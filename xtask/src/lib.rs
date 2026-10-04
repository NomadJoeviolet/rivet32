pub mod build;
pub mod cache;
pub mod coverage;
pub mod elf;
pub mod peripherals;
pub mod project;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

pub type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chip {
    pub feature: String,
    pub chip: String,
    pub family: String,
    pub core: String,
    pub target: String,
    pub embassy_feature_available: bool,
    #[serde(default)]
    pub local_patch_available: bool,
    pub catalogue: String,
    pub configuration: String,
    pub known_runtime_issues: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusion_reason: Option<String>,
}
impl Chip {
    pub fn in_scope(&self) -> bool {
        self.exclusion_reason.is_none()
    }
    pub fn ensure_in_scope(&self) -> Result<()> {
        match &self.exclusion_reason {
            Some(reason) => Err(format!(
                "{} is excluded from framework support: {reason}",
                self.feature
            )),
            None => Ok(()),
        }
    }
    pub fn backend_available(&self) -> bool {
        self.embassy_feature_available || self.local_patch_available
    }
}
#[derive(Deserialize)]
pub struct Catalogue {
    pub schema_version: u32,
    pub sources: Value,
    pub statistics: Value,
    pub builds: Vec<Chip>,
    pub devices: Vec<Value>,
}
impl Catalogue {
    pub fn read(root: &Path) -> Result<Self> {
        let path = root.join("data/chips.json");
        let data = fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut catalogue: Self =
            serde_json::from_slice(&data).map_err(|e| format!("invalid chip catalogue: {e}"))?;
        if catalogue.schema_version != 1 {
            return Err("unsupported chip catalogue schema".into());
        }
        catalogue.builds.sort_by(|a, b| a.feature.cmp(&b.feature));
        let mut seen = std::collections::BTreeSet::new();
        for chip in &catalogue.builds {
            if !chip.feature.starts_with("stm32")
                || !chip
                    .feature
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
                || !seen.insert(&chip.feature)
            {
                return Err(format!(
                    "invalid or duplicate chip feature {}",
                    chip.feature
                ));
            }
            if !matches!(
                chip.target.as_str(),
                "thumbv6m-none-eabi"
                    | "thumbv7m-none-eabi"
                    | "thumbv7em-none-eabi"
                    | "thumbv7em-none-eabihf"
                    | "thumbv8m.main-none-eabihf"
                    | "thumbv8m.main-none-eabi"
            ) {
                return Err(format!("unsupported target {}", chip.target));
            }
        }
        Ok(catalogue)
    }
    pub fn chip(&self, feature: &str) -> Result<&Chip> {
        let chip = self
            .builds
            .iter()
            .find(|c| c.feature == feature)
            .ok_or_else(|| format!("unknown chip/core feature {feature}; use list-chips"))?;
        chip.ensure_in_scope()?;
        Ok(chip)
    }
    pub fn filtered(&self, family: Option<&str>) -> Result<Vec<&Chip>> {
        let result: Vec<_> = self
            .builds
            .iter()
            .filter(|c| c.in_scope() && family.is_none_or(|f| c.family == f))
            .collect();
        if result.is_empty() {
            return Err(format!(
                "unknown or empty family {}",
                family.unwrap_or("all")
            ));
        }
        Ok(result)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Cli {
    Peripherals(peripherals::Options),
    New {
        chip: String,
        path: PathBuf,
        name: String,
        bank: Option<String>,
    },
    List {
        chip: Option<String>,
        family: Option<String>,
    },
    Coverage,
    Build {
        chip: String,
        bank: Option<String>,
    },
    Matrix {
        family: Option<String>,
        shard: Option<(usize, usize)>,
        limit: Option<usize>,
        bank: Option<String>,
    },
}
pub fn parse_args(args: impl IntoIterator<Item = String>) -> Result<Cli> {
    let mut args = args.into_iter();
    let command = args
        .next()
        .ok_or("usage: cargo xtask list-chips|coverage|build|matrix|peripherals|new [options]")?;
    if command == "peripherals" {
        return peripherals::parse(args).map(Cli::Peripherals);
    }
    if !matches!(
        command.as_str(),
        "list-chips" | "coverage" | "build" | "matrix" | "new"
    ) {
        return Err(format!("unknown command {command}"));
    }
    let mut flags = BTreeMap::new();
    while let Some(flag) = args.next() {
        let allowed = match command.as_str() {
            "list-chips" => matches!(flag.as_str(), "--chip" | "--family"),
            "build" => matches!(flag.as_str(), "--chip" | "--bank"),
            "new" => matches!(flag.as_str(), "--chip" | "--path" | "--name" | "--bank"),
            "matrix" => matches!(flag.as_str(), "--family" | "--shard" | "--limit" | "--bank"),
            _ => false,
        };
        if !allowed {
            return Err(format!("unrecognised argument {flag} for {command}"));
        }
        let value = args
            .next()
            .filter(|s| !s.starts_with("--"))
            .ok_or_else(|| format!("missing value for {flag}"))?;
        if flags.insert(flag.clone(), value).is_some() {
            return Err(format!("duplicate argument {flag}"));
        }
    }
    let family = flags.remove("--family").map(|f| {
        let f = f.to_ascii_uppercase();
        if f.starts_with("STM32") {
            f
        } else {
            format!("STM32{f}")
        }
    });
    let chip = flags.remove("--chip").map(|c| c.to_ascii_lowercase());
    let bank = flags.remove("--bank");
    if bank
        .as_ref()
        .is_some_and(|bank| !matches!(bank.as_str(), "single-bank" | "dual-bank"))
    {
        return Err("--bank must be single-bank or dual-bank".into());
    }
    match command.as_str() {
        "new" => Ok(Cli::New {
            chip: chip.ok_or("new requires --chip <catalogue feature>")?,
            path: PathBuf::from(
                flags
                    .remove("--path")
                    .ok_or("new requires --path <new directory>")?,
            ),
            name: flags.remove("--name").unwrap_or_else(|| "robot-app".into()),
            bank,
        }),
        "list-chips" => Ok(Cli::List { chip, family }),
        "coverage" => Ok(Cli::Coverage),
        "build" => Ok(Cli::Build {
            chip: chip.ok_or("build requires --chip <catalogue feature>")?,
            bank,
        }),
        "matrix" => {
            let shard = flags
                .remove("--shard")
                .map(|s| {
                    let (i, n) = s
                        .split_once('/')
                        .ok_or("--shard must be zero-based index/count")?;
                    let i = i.parse::<usize>().map_err(|_| "invalid shard index")?;
                    let n = n.parse::<usize>().map_err(|_| "invalid shard count")?;
                    if n == 0 || i >= n {
                        return Err("shard must satisfy 0 <= index < count");
                    }
                    Ok((i, n))
                })
                .transpose()
                .map_err(str::to_owned)?;
            let limit = flags
                .remove("--limit")
                .map(|s| {
                    s.parse::<usize>()
                        .ok()
                        .filter(|n| *n > 0)
                        .ok_or("--limit must be positive".to_owned())
                })
                .transpose()?;
            Ok(Cli::Matrix {
                family,
                shard,
                limit,
                bank,
            })
        }
        _ => unreachable!(),
    }
}

pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
pub fn run(cli: Cli, root: &Path) -> Result<()> {
    let catalogue = Catalogue::read(root)?;
    match cli {
        Cli::Peripherals(options) => {
            if let Some(feature) = &options.chip {
                catalogue.chip(feature)?;
            }
            options.run(root)
        }
        Cli::New {
            chip,
            path,
            name,
            bank,
        } => {
            let chip = catalogue.chip(&chip)?;
            project::write_project(root, chip, &path, &name, bank.as_deref())?;
            project::normalize_lock(&path, &chip.target)?;
            println!(
                "Generated {}: cd into this directory, then run cargo app-build",
                path.display()
            );
            Ok(())
        }
        Cli::List { chip, family } => {
            if let Some(feature) = &chip {
                for row in catalogue
                    .builds
                    .iter()
                    .filter(|c| c.feature == *feature || c.chip == *feature)
                {
                    row.ensure_in_scope()?;
                }
            }
            let mut rows = catalogue.filtered(family.as_deref())?;
            if let Some(chip) = chip {
                rows.retain(|c| c.feature == chip || c.chip == chip);
                if rows.is_empty() {
                    return Err(format!("unknown chip/core feature {chip}"));
                }
            }
            println!("feature\tfamily\tcore\ttarget\tembassy\tconfiguration");
            for row in rows {
                println!(
                    "{}\t{}\t{}\t{}\t{}\t{}",
                    row.feature,
                    row.family,
                    row.core,
                    row.target,
                    if row.local_patch_available {
                        "local-patch"
                    } else if row.embassy_feature_available {
                        "available"
                    } else {
                        "missing"
                    },
                    row.configuration
                );
            }
            Ok(())
        }
        Cli::Coverage => {
            let output = coverage::collect(root, &catalogue);
            println!(
                "{}",
                serde_json::to_string_pretty(&output).map_err(|e| e.to_string())?
            );
            Ok(())
        }
        Cli::Build { chip, bank } => {
            let chip = catalogue.chip(&chip)?;
            build::build_chip(root, &catalogue, chip, bank.as_deref()).map(|_| ())
        }
        Cli::Matrix {
            family,
            shard,
            limit,
            bank,
        } => {
            let selected: Vec<_> = catalogue
                .filtered(family.as_deref())?
                .into_iter()
                .enumerate()
                .filter(|(index, _)| shard.is_none_or(|(i, n)| index % n == i))
                .map(|(_, chip)| chip)
                .take(limit.unwrap_or(usize::MAX))
                .collect();
            if selected.is_empty() {
                return Err("matrix selection is empty".into());
            }
            let started = build::unix_ms();
            let mut rows = vec![];
            for (index, chip) in selected.iter().enumerate() {
                eprintln!("[{}/{}] {}", index + 1, selected.len(), chip.feature);
                match build::build_chip(root, &catalogue, chip, bank.as_deref()) {
                    Ok(_) => rows.push(json!({"feature": chip.feature, "status": "passed"})),
                    Err(error) => {
                        eprintln!("{}: {error}", chip.feature);
                        rows.push(
                            json!({"feature": chip.feature, "status": "failed", "error": error}),
                        );
                    }
                }
            }
            let failures = rows.iter().filter(|r| r["status"] == "failed").count();
            let report = json!({"started_unix_ms": started, "finished_unix_ms": build::unix_ms(), "family": family, "shard": shard, "bank_override": bank, "rows": rows, "failures": failures, "hardware_validation": "not-run"});
            let report_dir = root.join("target/reports");
            fs::create_dir_all(&report_dir).map_err(|e| e.to_string())?;
            let name = shard.map_or_else(
                || "matrix.json".to_owned(),
                |(i, n)| format!("matrix-{i}-of-{n}.json"),
            );
            fs::write(
                report_dir.join(&name),
                serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
            println!(
                "matrix: {} passed, {failures} failed; {}",
                selected.len() - failures,
                report_dir.join(name).display()
            );
            if failures == 0 {
                Ok(())
            } else {
                Err(format!(
                    "{failures} matrix rows failed; all selected rows were recorded"
                ))
            }
        }
    }
}
