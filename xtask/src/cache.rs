//! Bounded, reconstructible Cargo caches. The caller holds the target build lock.
use std::{fs, path::Path};

pub const DEFAULT_LIMIT_MIB: u64 = 2048;

#[derive(Debug, PartialEq, Eq)]
pub struct PruneResult {
    pub bytes_before: u64,
    pub bytes_after: u64,
    pub removed_bytes: u64,
}

pub fn limit_bytes(value: Option<&str>) -> Result<u64, String> {
    let mib = value.map_or(Ok(DEFAULT_LIMIT_MIB), |v| {
        if v.is_empty() || !v.bytes().all(|b| b.is_ascii_digit()) {
            return Err("cache limit must be a positive MiB integer".to_owned());
        }
        v.parse::<u64>()
            .map_err(|_| "cache limit must be a positive MiB integer".to_owned())
    })?;
    if mib == 0 {
        return Err("cache limit must be positive".into());
    }
    mib.checked_mul(1024 * 1024)
        .ok_or_else(|| "cache limit overflow".into())
}

fn plain_metadata(path: &Path) -> Result<fs::Metadata, String> {
    let meta = fs::symlink_metadata(path).map_err(|e| format!("{}: {e}", path.display()))?;
    #[cfg(windows)]
    let reparse = {
        use std::os::windows::fs::MetadataExt;
        meta.file_attributes() & 0x400 != 0
    };
    #[cfg(not(windows))]
    let reparse = false;
    if meta.file_type().is_symlink() || reparse || !(meta.is_dir() || meta.is_file()) {
        return Err(format!(
            "cache contains a link, reparse point or non-regular entry: {}",
            path.display()
        ));
    }
    Ok(meta)
}

fn tree_bytes(path: &Path) -> Result<u64, String> {
    let meta = plain_metadata(path)?;
    if meta.is_file() {
        return Ok(meta.len());
    }
    let mut total = 0u64;
    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let size = tree_bytes(&entry.map_err(|e| e.to_string())?.path())?;
        total = total.checked_add(size).ok_or("cache size overflow")?;
    }
    Ok(total)
}

/// Called only while `.xtask-build.lock` is held, before starting the next Cargo
/// process. Previous successful images/reports are already outside this cache.
pub fn prune_locked(root: &Path, target: &str, limit: u64) -> Result<PruneResult, String> {
    if limit == 0 {
        return Err("cache limit must be positive".into());
    }
    if !matches!(
        target,
        "thumbv6m-none-eabi"
            | "thumbv7m-none-eabi"
            | "thumbv7em-none-eabi"
            | "thumbv7em-none-eabihf"
            | "thumbv8m.main-none-eabi"
            | "thumbv8m.main-none-eabihf"
    ) {
        return Err("invalid firmware cache target".into());
    }
    let root = root.canonicalize().map_err(|e| e.to_string())?;
    let mut cache = root.clone();
    // Reject links in every reserved path component, even if they lead to a
    // different directory inside the workspace. Containment alone is weaker.
    for component in ["target", "firmware-cache", "shared", target] {
        cache.push(component);
        if !plain_metadata(&cache)?.is_dir() {
            return Err(format!(
                "cache component is not a directory: {}",
                cache.display()
            ));
        }
    }
    let cache = cache.canonicalize().map_err(|e| e.to_string())?;
    if !cache.starts_with(&root) {
        return Err("firmware cache escaped workspace".into());
    }
    let before = tree_bytes(&cache)?;
    if before <= limit {
        return Ok(PruneResult {
            bytes_before: before,
            bytes_after: before,
            removed_bytes: 0,
        });
    }
    // Never remove the cache root or .xtask-build.lock. That lock's persistent
    // file identity protects writers that are waiting for the current holder.
    // Only Cargo's known host/ARM output trees are eligible for eviction.
    for name in ["release", target] {
        let child = cache.join(name);
        if !child.try_exists().map_err(|e| e.to_string())? {
            continue;
        }
        if !plain_metadata(&child)?.is_dir() {
            return Err(format!(
                "unexpected non-directory Cargo output: {}",
                child.display()
            ));
        }
        let resolved = child.canonicalize().map_err(|e| e.to_string())?;
        if resolved.parent() != Some(cache.as_path()) {
            return Err("Cargo output escaped selected firmware cache".into());
        }
        fs::remove_dir_all(&resolved).map_err(|e| format!("clear {}: {e}", resolved.display()))?;
    }
    let after = tree_bytes(&cache)?;
    Ok(PruneResult {
        bytes_before: before,
        bytes_after: after,
        removed_bytes: before
            .checked_sub(after)
            .ok_or("cache grew while exclusively locked")?,
    })
}
