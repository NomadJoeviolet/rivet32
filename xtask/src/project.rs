//! Self-contained App projects with a licensed snapshot of the framework source.
use crate::{
    Chip, Result,
    build::{select_bank, sha256},
};
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, io::Write, path::Path, process::Command};

type Files = BTreeMap<String, Vec<u8>>;

#[path = "project_dual.rs"]
mod dual;

fn read(root: &Path, name: &str) -> Result<Vec<u8>> {
    fs::read(root.join(name)).map_err(|e| format!("read {name}: {e}"))
}

fn snapshot(base: &Path, dir: &Path, prefix: &str, files: &mut Files) -> Result<()> {
    for entry in fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let kind = entry.file_type().map_err(|e| e.to_string())?;
        let path = entry.path();
        if kind.is_symlink() {
            return Err(format!("snapshot refuses symbolic link {}", path.display()));
        }
        if kind.is_dir() {
            if !matches!(
                entry.file_name().to_str(),
                Some("target" | ".git" | ".build" | "__pycache__")
            ) {
                snapshot(base, &path, prefix, files)?;
            }
        } else if kind.is_file() {
            let relative = path.strip_prefix(base).map_err(|e| e.to_string())?;
            let relative = relative
                .to_str()
                .ok_or("source path is not UTF-8")?
                .replace('\\', "/");
            files.insert(
                format!("{prefix}/{relative}"),
                fs::read(&path).map_err(|e| e.to_string())?,
            );
        }
    }
    Ok(())
}

/// Render and exclusively create a new directory. No existing project is overwritten.
/// The lockfile is a seed; [`normalize_lock`] must finish before using `--locked`.
pub fn write_project(
    root: &Path,
    chip: &Chip,
    dest: &Path,
    name: &str,
    bank: Option<&str>,
) -> Result<()> {
    chip.ensure_in_scope()?;
    if name.is_empty()
        || name.len() > 64
        || !name.as_bytes()[0].is_ascii_lowercase()
        || !name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'_')
        || name == "xtask"
        || name.starts_with("embodied-")
    {
        return Err("--name must start with a-z and contain only a-z, 0-9, - or _ (1..64 bytes); embodied-* and xtask are reserved".into());
    }
    if !chip.backend_available() {
        return Err(format!(
            "{} is missing from pinned Embassy; cannot generate a buildable App",
            chip.feature
        ));
    }
    let pair = dual::select(root, chip)?;
    if dest.as_os_str().is_empty() || dest.try_exists().map_err(|e| e.to_string())? {
        return Err(format!("destination must not exist: {}", dest.display()));
    }
    let memory: Value = serde_json::from_slice(&read(root, "data/generated/hal-memory.json")?)
        .map_err(|e| e.to_string())?;
    let bank = select_bank(&memory["chips"][&chip.chip], bank)?;
    let mut features = vec![chip.feature.clone()];
    features.extend(bank.selected.clone());
    let feature_list = serde_json::to_string(&features)
        .map_err(|e| e.to_string())?
        .replace(",", ", ");
    let mut files = Files::new();
    let original_manifest =
        String::from_utf8(read(root, "Cargo.toml")?).map_err(|e| e.to_string())?;
    let (_, inherited) = original_manifest
        .split_once("[workspace.package]")
        .ok_or("workspace.package table missing")?;
    let (members, defaults) = if pair.is_some() {
        (
            "\"App/cm7\", \"App/cm4\", \"tools/build-pair\", \"Framework/crates/*\"",
            "\"tools/build-pair\"",
        )
    } else {
        ("\"App\", \"Framework/crates/*\"", "\"App\"")
    };
    let manifest = format!("[workspace]\nresolver = \"3\"\nmembers = [{members}]\nexclude = [\"Framework/vendor/embassy-stm32\", \"Framework/vendor/stm32-metapac\"]\ndefault-members = [{defaults}]\n\n[workspace.package]{inherited}")
        .replace("path = \"crates/", "path = \"Framework/crates/")
        .replace("path = \"vendor/", "path = \"Framework/vendor/");
    files.insert("Cargo.toml".into(), manifest.into_bytes());
    snapshot(
        &root.join("crates"),
        &root.join("crates"),
        "Framework/crates",
        &mut files,
    )?;
    if root.join("vendor").is_dir() {
        snapshot(
            &root.join("vendor"),
            &root.join("vendor"),
            "Framework/vendor",
            &mut files,
        )?;
        // Keep NOTICE references and the original source checksums reviewable in
        // the independent project. These are evidence, never chip aliases.
        snapshot(
            &root.join("data/patches"),
            &root.join("data/patches"),
            "data/patches",
            &mut files,
        )?;
    }
    for file in [
        "LICENSE",
        "THIRD_PARTY_NOTICES.md",
        "rust-toolchain.toml",
        "Cargo.lock",
        "data/support-policy.json",
        "xtask/scripts/fetch_source_documents.py",
        "xtask/scripts/generate_hal_metadata.py",
    ] {
        files.insert(file.into(), read(root, file)?);
    }
    snapshot(&root.join("docs"), &root.join("docs"), "docs", &mut files)?;
    files.insert(
        "data/ST-LICENSE.txt".into(),
        read(root, "data/ST-LICENSE.txt")?,
    );
    let app = include_str!("../templates/App-Cargo.toml")
        .replace("@NAME@", name)
        .replace("@FEATURES@", &feature_list);
    files.insert("App/Cargo.toml".into(), app.into_bytes());
    for (path, text) in [
        ("App/src/main.rs", include_str!("../templates/main.rs")),
        (
            "App/src/boards/mod.rs",
            include_str!("../templates/board.rs"),
        ),
        (
            "App/src/tasks/mod.rs",
            include_str!("../templates/tasks.rs"),
        ),
        (
            ".cargo/config.toml",
            include_str!("../templates/config.toml"),
        ),
        (
            ".github/workflows/ci.yml",
            include_str!("../templates/ci.yml"),
        ),
        ("README.md", include_str!("../templates/README.md")),
    ] {
        files.insert(
            path.into(),
            text.replace("@NAME@", name)
                .replace("@CHIP@", &chip.feature)
                .replace("@TARGET@", &chip.target)
                .into_bytes(),
        );
    }
    files.insert(
        ".gitignore".into(),
        b"/target/\n/data/sources/\n.build/\n__pycache__/\n*.elf\n*.bin\n*.hex\n".to_vec(),
    );
    // The provenance hashes describe exact bytes, including upstream CRLF.
    // Git must preserve those bytes when this independent project is committed.
    files.insert(".gitattributes".into(), b"* -text\n".to_vec());
    files.insert(
        ".vscode/extensions.json".into(),
        read(root, ".vscode/extensions.json")?,
    );
    let editor_command = json!([
        "cargo",
        "check",
        "-p",
        name,
        "--bin",
        "firmware",
        "--target",
        chip.target,
        "--message-format=json",
        "--locked"
    ]);
    files.insert(
        ".vscode/settings.json".into(),
        serde_json::to_vec_pretty(&json!({
            "rust-analyzer.linkedProjects": ["App/Cargo.toml"],
            "rust-analyzer.cargo.target": chip.target,
            "rust-analyzer.check.allTargets": false,
            "rust-analyzer.check.overrideCommand": editor_command,
            "rust-analyzer.cargo.buildScripts.overrideCommand": editor_command,
        }))
        .map_err(|e| e.to_string())?,
    );
    let pair_metadata = pair
        .as_ref()
        .map(|pair| dual::render(root, pair, name, &mut files))
        .transpose()?;
    let hashes: BTreeMap<_, _> = files
        .iter()
        .map(|(path, content)| (path.clone(), sha256(content)))
        .collect();
    let mut report = json!({
        "schema_version": 1, "name": name, "chip": chip.feature, "target": chip.target,
        "bank": bank.selected, "bank_requirement": bank.hardware_requirement,
        "framework_source": "local licensed source snapshot; paths independent of original checkout",
        "source_file_sha256": hashes, "lock_status": "seeded; normalization required",
        "hardware_validation": "not-run"
    });
    if let Some(pair_metadata) = pair_metadata {
        report.as_object_mut().unwrap().extend(pair_metadata);
    }
    files.insert(
        "project.json".into(),
        serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
    );
    // create_dir is the exclusive reservation; never create_dir_all(dest).
    fs::create_dir(dest)
        .map_err(|e| format!("create {} (parent must exist): {e}", dest.display()))?;
    for (relative, content) in files {
        let path = dest.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .and_then(|mut f| f.write_all(&content))
            .map_err(|e| {
                format!(
                    "{}: {e}; partial generated project retained",
                    path.display()
                )
            })?;
    }
    Ok(())
}

/// Keep the framework lock's dependency versions while adapting the workspace roots.
/// Offline resolution prevents an implicit dependency upgrade or network requirement.
pub fn normalize_lock(dest: &Path, target: &str) -> Result<()> {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let output = Command::new(cargo)
        .current_dir(dest)
        .args([
            "metadata",
            "--format-version=1",
            "--offline",
            "--filter-platform",
            target,
        ])
        .output()
        .map_err(|e| format!("project written, Cargo lock normalization failed: {e}"))?;
    fs::write(dest.join("generation.log"), &output.stderr).map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(format!(
            "project retained at {}; lock normalization failed: {}. Resolve the logged error, run cargo metadata --offline --format-version=1, then cargo app-build",
            dest.display(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let path = dest.join("project.json");
    let mut report: Value = serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    report["lock_status"] = json!("normalized offline");
    let lock_hash = sha256(&fs::read(dest.join("Cargo.lock")).map_err(|e| e.to_string())?);
    report["cargo_lock_sha256"] = json!(lock_hash);
    report["source_file_sha256"]["Cargo.lock"] = json!(lock_hash);
    fs::write(
        path,
        serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}
