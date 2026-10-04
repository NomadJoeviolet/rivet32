//! Rendering for two exact H7 chip/core packages; Cargo builds each independently.
use super::{Files, read};
use crate::{Catalogue, Chip, Result};
use serde_json::{Map, Value, json};
use std::path::Path;

pub(super) struct Pair {
    part: String,
    images: [Chip; 2],
}

fn select_catalogue(catalogue: &Catalogue, chip: &Chip) -> Result<Option<Pair>> {
    let Some((part, core)) = chip.feature.rsplit_once('-') else {
        return Ok(None);
    };
    if !matches!(core, "cm7" | "cm4") {
        return Ok(None);
    }
    if !["stm32h745", "stm32h747", "stm32h755", "stm32h757"]
        .iter()
        .any(|prefix| part.starts_with(prefix))
    {
        return Err(format!("no paired startup policy for {}", chip.feature));
    }
    let images = [
        catalogue.chip(&format!("{part}-cm7"))?.clone(),
        catalogue.chip(&format!("{part}-cm4"))?.clone(),
    ];
    for image in &images {
        if image.chip != part
            || image.target != "thumbv7em-none-eabihf"
            || image.target != chip.target
            || !image.backend_available()
        {
            return Err(format!(
                "unavailable or inconsistent paired chip/core {}",
                image.feature
            ));
        }
    }
    Ok(Some(Pair {
        part: part.into(),
        images,
    }))
}

pub(super) fn select(root: &Path, chip: &Chip) -> Result<Option<Pair>> {
    if !chip.feature.ends_with("-cm7") && !chip.feature.ends_with("-cm4") {
        return Ok(None);
    }
    select_catalogue(&Catalogue::read(root)?, chip)
}

pub(super) fn render(
    root: &Path,
    pair: &Pair,
    name: &str,
    files: &mut Files,
) -> Result<Map<String, Value>> {
    // Replace the single-core App before hashing or reserving the destination.
    files.retain(|path, _| !path.starts_with("App/"));
    let startup = read(root, "App/src/dual_core.rs")?;
    let support = read(root, "App/build_support.rs")?;
    let build = String::from_utf8(read(root, "App/build.rs")?).map_err(|e| e.to_string())?;
    if build.matches("cargo:rustc-link-arg-bin=minimal=").count() != 1 {
        return Err("App dual-core link-script interface changed; review project renderer".into());
    }
    let build = build
        .replace("    #[cfg(feature = \"firmware\")]\n", "")
        .replace("#[cfg(feature = \"firmware\")]\n", "");
    let mut images = Vec::new();
    for (image, core, timer) in [
        (&pair.images[0], "cm7", "TIM5"),
        (&pair.images[1], "cm4", "TIM2"),
    ] {
        let render = |template: &str| {
            template
                .replace("@NAME@", name)
                .replace("@CORE@", core)
                .replace("@CHIP@", &image.feature)
                .replace("@TARGET@", &image.target)
        };
        for (path, contents) in [
            (
                "Cargo.toml",
                include_str!("../templates/dual/App-Cargo.toml"),
            ),
            ("src/main.rs", include_str!("../templates/dual/main.rs")),
            (
                "src/boards/mod.rs",
                include_str!("../templates/dual/board.rs"),
            ),
            ("src/tasks/mod.rs", include_str!("../templates/tasks.rs")),
        ] {
            files.insert(format!("App/{core}/{path}"), render(contents).into_bytes());
        }
        files.insert(format!("App/{core}/build_support.rs"), support.clone());
        files.insert(
            format!("App/{core}/build.rs"),
            build
                .replace(
                    "cargo:rustc-link-arg-bin=minimal=",
                    &format!("cargo:rustc-link-arg-bin=firmware-{core}="),
                )
                .into_bytes(),
        );
        images.push(json!({
            "core": core, "chip": image.feature, "package": format!("{name}-{core}"),
            "manifest": format!("App/{core}/Cargo.toml"), "binary": format!("firmware-{core}"),
            "target": image.target, "time_driver": timer,
        }));
    }
    files.insert("App/shared/dual_core.rs".into(), startup);
    // Keep the proven ELF checks and lock lifetime as an exact source snapshot.
    files.insert(
        "scripts/check_dual_core.py".into(),
        read(root, "scripts/check_dual_core.py")?,
    );
    for (path, contents) in [
        (
            ".cargo/config.toml",
            include_str!("../templates/dual/config.toml"),
        ),
        (
            ".github/workflows/ci.yml",
            include_str!("../templates/dual/ci.yml"),
        ),
        ("README.md", include_str!("../templates/dual/README.md")),
        (
            "scripts/build_pair.py",
            include_str!("../templates/dual/build_pair.py"),
        ),
        (
            "tools/build-pair/Cargo.toml",
            include_str!("../templates/dual/launcher-Cargo.toml"),
        ),
        (
            "tools/build-pair/src/main.rs",
            include_str!("../templates/dual/launcher.rs"),
        ),
    ] {
        files.insert(
            path.into(),
            contents
                .replace("@NAME@", name)
                .replace("@PART@", &pair.part)
                .replace("@TARGET@", &pair.images[0].target)
                .into_bytes(),
        );
    }
    for core in ["cm7", "cm4"] {
        let command = json!([
            "cargo",
            "check",
            "-p",
            format!("{name}-{core}"),
            "--bin",
            format!("firmware-{core}"),
            "--target",
            pair.images[0].target,
            "--message-format=json",
            "--locked",
            "--target-dir",
            format!("target/editor-{core}")
        ]);
        let settings = json!({
            "rust-analyzer.linkedProjects": [format!("App/{core}/Cargo.toml")],
            "rust-analyzer.cargo.target": pair.images[0].target,
            "rust-analyzer.check.allTargets": false,
            "rust-analyzer.check.overrideCommand": command,
            "rust-analyzer.cargo.buildScripts.overrideCommand": command,
        });
        if core == "cm7" {
            files.insert(
                ".vscode/settings.json".into(),
                serde_json::to_vec_pretty(&settings).map_err(|e| e.to_string())?,
            );
        }
        files.insert(
            format!("{core}.code-workspace"),
            serde_json::to_vec_pretty(&json!({
                "folders": [{"path": "."}], "settings": settings,
            }))
            .map_err(|e| e.to_string())?,
        );
    }
    Ok(json!({"schema_version": 2, "topology": "dual-core-pair", "part": pair.part, "images": images})
        .as_object().unwrap().clone())
}
