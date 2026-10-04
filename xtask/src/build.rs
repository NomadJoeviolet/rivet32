use crate::{
    Catalogue, Chip, Result,
    elf::{inspect_elf, parse_memory_regions},
};
use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn unix_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

pub fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// OS-owned, exclusive lock released by closing the file, including on failure.
/// Keep the path on disk: deleting it could give a competing process a new inode.
/// All writers of xtask's cache/artifacts must participate in this protocol.
pub struct BuildLock {
    _file: fs::File,
}

impl BuildLock {
    pub fn acquire(path: &Path) -> Result<Self> {
        let file = fs::File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .map_err(|e| format!("open build lock {}: {e}", path.display()))?;
        file.lock()
            .map_err(|e| format!("acquire build lock {}: {e}", path.display()))?;
        Ok(Self { _file: file })
    }
}

/// Dual-core App partitions must be validated against their own linker script,
/// never the dependency's whole-chip layout. Both Cargo phases may repeat a path.
pub fn generated_memory_path<'a>(
    messages: impl IntoIterator<Item = &'a Value>,
    dual_core: bool,
) -> Result<PathBuf> {
    let mut paths = BTreeSet::new();
    for message in messages {
        let package = message["package_id"].as_str().unwrap_or_default();
        let selected = if dual_core {
            package.contains("embodied-app")
                && message["cfgs"]
                    .as_array()
                    .is_some_and(|cfgs| cfgs.iter().any(|cfg| cfg == "embodied_dual_core"))
        } else {
            package.contains("embassy-stm32")
        };
        if message["reason"] == "build-script-executed"
            && selected
            && let Some(out) = message["out_dir"].as_str()
        {
            let path = Path::new(out).join("memory.x");
            if path.is_file() {
                paths.insert(path);
            }
        }
    }
    if paths.len() != 1 {
        return Err(format!(
            "expected exactly one {} memory.x, found {}",
            if dual_core {
                "App dual-core partition"
            } else {
                "HAL"
            },
            paths.len()
        ));
    }
    Ok(paths.into_iter().next().unwrap())
}

#[derive(Debug, Serialize)]
pub struct BankSelection {
    pub selected: Option<String>,
    pub configuration_index: usize,
    pub source: &'static str,
    pub hardware_requirement: &'static str,
}

pub fn select_bank(metadata: &Value, requested: Option<&str>) -> Result<BankSelection> {
    let configurations = metadata["memory"]
        .as_array()
        .ok_or("missing chip HAL memory metadata")?;
    if configurations.is_empty() {
        return Err("empty HAL memory configurations".into());
    }
    let default = metadata["default_bank"].as_str();
    if default.is_some_and(|v| v != "dual-bank") {
        return Err("unrecognised HAL default bank policy".into());
    }
    if requested.is_some_and(|v| !matches!(v, "single-bank" | "dual-bank")) {
        return Err("bank must be single-bank or dual-bank".into());
    }
    if requested.is_some() && default.is_none() {
        return Err("chip does not support selectable single-bank/dual-bank configuration".into());
    }
    let selected = requested.or(default);
    let configuration_index = if selected == Some("dual-bank") { 1 } else { 0 };
    if configuration_index >= configurations.len()
        || (default.is_some() && configurations.len() != 2)
        || (default.is_none() && configurations.len() != 1)
    {
        return Err("HAL bank policy and memory configurations disagree".into());
    }
    Ok(BankSelection {
        selected: selected.map(str::to_owned),
        configuration_index,
        source: if requested.is_some() {
            "explicit CLI override"
        } else if default.is_some() {
            "project catalogue default policy"
        } else {
            "fixed chip layout"
        },
        hardware_requirement: if selected.is_some() {
            "hardware option bytes must match the selected bank configuration; not verified by compilation"
        } else {
            "no selectable bank feature applied"
        },
    })
}

fn probe(root: &Path, program: &str, args: &[&str]) -> Value {
    match Command::new(program).args(args).current_dir(root).output() {
        Ok(output) => {
            json!({"program": program, "args": args, "exit_code": output.status.code(), "stdout": String::from_utf8_lossy(&output.stdout).trim(), "stderr": String::from_utf8_lossy(&output.stderr).trim()})
        }
        Err(error) => json!({"program": program, "args": args, "error": error.to_string()}),
    }
}

fn save_report(path: &Path, report: &Value) -> Result<()> {
    fs::write(
        path,
        serde_json::to_vec_pretty(report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("write {}: {e}", path.display()))
}

struct CargoResult {
    success: bool,
    messages: Vec<Value>,
    record: Value,
}
fn run_cargo(root: &Path, dir: &Path, label: &str, args: &[String]) -> Result<CargoResult> {
    let program = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let started = unix_ms();
    let output = Command::new(&program).args(args).current_dir(root).output();
    let stdout_path = dir.join(format!("{label}.stdout.jsonl"));
    let stderr_path = dir.join(format!("{label}.stderr.log"));
    match output {
        Ok(output) => {
            fs::write(&stdout_path, &output.stdout).map_err(|e| e.to_string())?;
            fs::write(&stderr_path, &output.stderr).map_err(|e| e.to_string())?;
            let messages: Vec<Value> = String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter_map(|line| serde_json::from_str(line).ok())
                .collect();
            let diagnostics: String = messages
                .iter()
                .filter_map(|m| m["message"]["rendered"].as_str())
                .collect();
            fs::write(dir.join(format!("{label}.diagnostics.log")), diagnostics)
                .map_err(|e| e.to_string())?;
            let record = json!({"program": program.to_string_lossy(), "args": args, "cwd": root, "started_unix_ms": started, "finished_unix_ms": unix_ms(), "exit_code": output.status.code(), "stdout": stdout_path, "stderr": stderr_path});
            Ok(CargoResult {
                success: output.status.success(),
                messages,
                record,
            })
        }
        Err(error) => {
            fs::write(&stderr_path, error.to_string()).map_err(|e| e.to_string())?;
            Ok(CargoResult {
                success: false,
                messages: vec![],
                record: json!({"program": program.to_string_lossy(), "args": args, "cwd": root, "started_unix_ms": started, "finished_unix_ms": unix_ms(), "spawn_error": error.to_string(), "stderr": stderr_path}),
            })
        }
    }
}

/// A target-wide cache reuses host tools and chip-independent dependencies.
/// Its external lock must cover Cargo execution AND artifact validation/copying.
fn cargo_args(mode: &str, chip: &Chip, cache: &Path, bank: Option<&str>) -> Vec<String> {
    [
        mode.to_owned(),
        "--locked".into(),
        "--release".into(),
        "-p".into(),
        "embodied-app".into(),
        "--bin".into(),
        "minimal".into(),
        "--no-default-features".into(),
        "--features".into(),
        bank.map_or_else(
            || chip.feature.clone(),
            |bank| format!("{},{bank}", chip.feature),
        ),
        "--target".into(),
        chip.target.clone(),
        "--target-dir".into(),
        cache.to_string_lossy().into_owned(),
        "--message-format=json-render-diagnostics".into(),
    ]
    .into()
}

pub fn build_chip(
    root: &Path,
    catalogue: &Catalogue,
    chip: &Chip,
    bank_override: Option<&str>,
) -> Result<Value> {
    chip.ensure_in_scope()?;
    let configured_limit = match std::env::var("EMBODIED_CACHE_LIMIT_MIB") {
        Ok(value) => Some(value),
        Err(std::env::VarError::NotPresent) => None,
        Err(error) => return Err(format!("invalid cache limit environment: {error}")),
    };
    let cache_limit = crate::cache::limit_bytes(configured_limit.as_deref())?;
    let reports = root.join("target/reports").join(&chip.feature);
    let artifacts = root.join("target/artifacts").join(&chip.feature);
    let cache = root.join("target/firmware-cache/shared").join(&chip.target);
    fs::create_dir_all(&reports).map_err(|e| e.to_string())?;
    fs::create_dir_all(&artifacts).map_err(|e| e.to_string())?;
    // Lock before invalidating the last report/artifact for the same chip. A
    // second bank configuration of that chip uses the same report destination.
    let _report_lock = BuildLock::acquire(&reports.join(".build.lock"))?;
    // A failed current build must never leave an old artifact looking current.
    for name in ["minimal.elf", "minimal.map"] {
        let path = artifacts.join(name);
        if path.exists() {
            fs::remove_file(&path).map_err(|e| format!("invalidate {}: {e}", path.display()))?;
        }
    }
    let git_sha = probe(root, "git", &["rev-parse", "HEAD"]);
    let git_status = probe(
        root,
        "git",
        &["status", "--porcelain=v1", "--untracked-files=normal"],
    );
    let lock_hash = fs::read(root.join("Cargo.lock"))
        .ok()
        .map(|bytes| sha256(&bytes));
    let mut report = json!({
        "schema_version": 1, "chip": chip, "sources": catalogue.sources,
        "started_unix_ms": unix_ms(), "finished_unix_ms": null,
        "cargo_lock_sha256": lock_hash,
        "rustc_vv": probe(root, "rustc", &["-Vv"]),
        "git": {"head": git_sha, "status": git_status, "dirty": git_status["stdout"].as_str().map(|s| !s.is_empty())},
        "compile": {"status": "not-run"}, "link": {"status": "not-run"}, "elf_validation": {"status": "not-run"},
        "hardware_validation": "not-run", "peripheral_validation": "minimal heartbeat only; peripheral capabilities are not instantiated by this command",
        "commands": [], "artifacts": null,
    });
    let report_path = reports.join("result.json");
    save_report(&report_path, &report)?;
    let result = (|| -> Result<()> {
        if !chip.backend_available() {
            let reason = format!(
                "{} is missing from pinned Embassy; no compile or link was attempted",
                chip.feature
            );
            report["compile"] = json!({"status": "blocked", "reason": reason});
            return Err(reason);
        }
        let memory_metadata_path = root.join("data/generated/hal-memory.json");
        let memory_metadata: Value = serde_json::from_slice(
            &fs::read(&memory_metadata_path)
                .map_err(|e| format!("read HAL memory metadata: {e}"))?,
        )
        .map_err(|e| format!("invalid HAL memory metadata: {e}"))?;
        let chip_memory = &memory_metadata["chips"][&chip.chip];
        let bank = select_bank(chip_memory, bank_override).map_err(|reason| {
            report["compile"] = json!({"status":"blocked","reason":reason});
            reason
        })?;
        report["build_configuration"] = json!({"bank": bank, "hal_memory_revision": memory_metadata["source_revision"], "hal_memory_chip_sha256": chip_memory["source_sha256"]});
        fs::create_dir_all(&cache).map_err(|e| e.to_string())?;
        // Cargo's internal lock ends when Cargo exits; another chip could then
        // replace `release/minimal` before we read it. Hold ours through copy.
        let _cache_lock = BuildLock::acquire(&cache.join(".xtask-build.lock"))?;
        let retention = crate::cache::prune_locked(root, &chip.target, cache_limit)?;
        report["cache"] = json!({"path": cache,
            "policy": "shared per target; prune known Cargo outputs above threshold before build; exclusive lock through validation and artifact copy",
            "limit_bytes": cache_limit, "bytes_before": retention.bytes_before,
            "bytes_after": retention.bytes_after, "removed_bytes": retention.removed_bytes});
        if lock_hash.is_none() {
            return Err("Cargo.lock is required for a locked firmware build".into());
        }
        eprintln!("{}: cargo check", chip.feature);
        let check = run_cargo(
            root,
            &reports,
            "compile",
            &cargo_args("check", chip, &cache, bank.selected.as_deref()),
        )?;
        report["commands"]
            .as_array_mut()
            .unwrap()
            .push(check.record);
        report["compile"] = json!({"status": if check.success { "passed" } else { "failed" }});
        save_report(&report_path, &report)?;
        if !check.success {
            return Err(format!(
                "compile failed; see {}",
                reports.join("compile.diagnostics.log").display()
            ));
        }

        // Unique linker arguments force the final binary to link for this run,
        // while retaining the shared dependency cache. A fresh Cargo artifact
        // cannot otherwise recreate a deleted map from an earlier cached link.
        let map_path = reports.join(format!("minimal-{}-{}.map", unix_ms(), std::process::id()));
        let mut args = cargo_args("rustc", chip, &cache, bank.selected.as_deref());
        args.extend([
            "--".into(),
            "-C".into(),
            format!("link-arg=-Map={}", map_path.display()),
        ]);
        eprintln!("{}: cargo rustc / link", chip.feature);
        let link = run_cargo(root, &reports, "link", &args)?;
        report["commands"].as_array_mut().unwrap().push(link.record);
        report["link"] = json!({"status": if link.success { "passed" } else { "failed" }});
        save_report(&report_path, &report)?;
        if !link.success {
            return Err(format!(
                "link failed; see {}",
                reports.join("link.diagnostics.log").display()
            ));
        }

        // Cargo's artifact message must name both this feature and this binary.
        let artifact = link
            .messages
            .iter()
            .find(|m| {
                m["reason"] == "compiler-artifact"
                    && m["target"]["name"] == "minimal"
                    && m["features"]
                        .as_array()
                        .is_some_and(|f| f.iter().any(|f| f == &chip.feature))
                    && m["executable"].is_string()
            })
            .ok_or("cargo reported no matching minimal executable for the selected chip")?;
        let executable = PathBuf::from(artifact["executable"].as_str().unwrap());
        let canonical_cache = fs::canonicalize(&cache).map_err(|e| e.to_string())?;
        let canonical_executable = fs::canonicalize(&executable).map_err(|e| e.to_string())?;
        if !canonical_executable.starts_with(&canonical_cache) {
            return Err("cargo executable escaped the selected chip cache".into());
        }
        let dual_core = chip.feature.ends_with("-cm7") || chip.feature.ends_with("-cm4");
        let memory_path =
            generated_memory_path(check.messages.iter().chain(&link.messages), dual_core)?;
        if !fs::canonicalize(&memory_path)
            .map_err(|e| e.to_string())?
            .starts_with(&canonical_cache)
        {
            return Err("generated memory script escaped the selected build cache".into());
        }
        let memory = fs::read_to_string(&memory_path).map_err(|e| e.to_string())?;
        fs::write(reports.join("memory.x"), &memory).map_err(|e| e.to_string())?;
        let regions = parse_memory_regions(&memory)?;
        let elf_bytes = fs::read(&canonical_executable).map_err(|e| e.to_string())?;
        let elf = inspect_elf(&elf_bytes, &regions)?;
        let catalogue_flash = catalogue
            .devices
            .iter()
            .find(|d| d["chip"] == chip.chip)
            .and_then(|d| d["flash_kib"].as_u64());
        let mapped_flash: u64 = regions
            .iter()
            .filter(|r| r.name.to_ascii_uppercase().contains("FLASH"))
            .map(|r| r.length)
            .sum();
        if catalogue_flash.is_some_and(|kib| mapped_flash > kib * 1024) {
            return Err("linker FLASH region exceeds ST catalogue flash total".into());
        }
        if !map_path.is_file() {
            return Err("link succeeded but did not produce the requested linker map".into());
        }
        if fs::read(root.join("Cargo.lock"))
            .ok()
            .map(|bytes| sha256(&bytes))
            != lock_hash
        {
            return Err(
                "Cargo.lock changed during firmware build; rerun for consistent provenance".into(),
            );
        }
        report["elf_validation"] = json!({"status": "passed", "details": elf, "memory_regions": regions, "memory_source": memory_path, "catalogue_flash_kib": catalogue_flash, "flash_total_check": if catalogue_flash.is_some() { "passed" } else { "unknown: catalogue has no flash total" }});
        let output_elf = artifacts.join("minimal.elf");
        let output_map = artifacts.join("minimal.map");
        fs::write(&output_elf, &elf_bytes).map_err(|e| e.to_string())?;
        fs::copy(&map_path, &output_map).map_err(|e| e.to_string())?;
        report["artifacts"] = json!({"elf": output_elf, "map": output_map, "elf_bytes": elf_bytes.len(), "elf_sha256": sha256(&elf_bytes), "map_sha256": sha256(&fs::read(&output_map).map_err(|e| e.to_string())?), "feature": chip.feature, "target": chip.target, "bank": bank.selected});
        Ok(())
    })();
    if let Err(error) = &result {
        if report["link"]["status"] == "passed" && report["elf_validation"]["status"] != "passed" {
            report["elf_validation"] = json!({"status": "failed", "reason": error});
        }
        report["error"] = json!(error);
    }
    report["finished_unix_ms"] = json!(unix_ms());
    save_report(&report_path, &report)?;
    result?;
    println!(
        "{}: compile/link/ELF checks passed; {}",
        chip.feature,
        artifacts.join("minimal.elf").display()
    );
    Ok(report)
}
