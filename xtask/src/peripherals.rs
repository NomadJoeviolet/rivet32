//! CLI bridge to the source-bound, per-category ARM constructor matrix.
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::Command,
};

use crate::Result;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Options {
    pub chip: Option<String>,
    pub family: Option<String>,
    pub package: Option<String>,
    pub shard: Option<(usize, usize)>,
    pub limit: Option<usize>,
    pub output: Option<PathBuf>,
    pub plan_only: bool,
}

pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Options> {
    let mut args = args.into_iter();
    let mut flags = BTreeMap::new();
    let mut plan_only = false;
    while let Some(flag) = args.next() {
        if flag == "--plan-only" {
            if plan_only {
                return Err("duplicate argument --plan-only".into());
            }
            plan_only = true;
            continue;
        }
        if !matches!(
            flag.as_str(),
            "--chip" | "--family" | "--package" | "--shard" | "--limit" | "--output"
        ) {
            return Err(format!("unrecognised argument {flag} for peripherals"));
        }
        let value = args
            .next()
            .filter(|value| !value.starts_with("--") && !value.is_empty())
            .ok_or_else(|| format!("missing value for {flag}"))?;
        if flags.insert(flag.clone(), value).is_some() {
            return Err(format!("duplicate argument {flag}"));
        }
    }
    let chip = flags
        .remove("--chip")
        .map(|value| value.to_ascii_lowercase());
    let family = flags.remove("--family").map(|value| {
        let value = value.to_ascii_uppercase();
        if value.starts_with("STM32") {
            value
        } else {
            format!("STM32{value}")
        }
    });
    let package = flags.remove("--package");
    let shard = flags
        .remove("--shard")
        .map(|value| {
            let (index, count) = value
                .split_once('/')
                .ok_or("--shard must be zero-based index/count")?;
            let index = index.parse::<usize>().map_err(|_| "invalid shard index")?;
            let count = count.parse::<usize>().map_err(|_| "invalid shard count")?;
            if count == 0 || index >= count {
                return Err("shard must satisfy 0 <= index < count");
            }
            Ok((index, count))
        })
        .transpose()
        .map_err(str::to_owned)?;
    let limit = flags
        .remove("--limit")
        .map(|value| {
            value
                .parse::<usize>()
                .ok()
                .filter(|value| *value > 0)
                .ok_or("--limit must be positive".to_owned())
        })
        .transpose()?;
    if chip.is_some() && (family.is_some() || shard.is_some()) {
        return Err("--chip cannot be combined with --family or --shard".into());
    }
    if package.is_some() && chip.is_none() {
        return Err("--package requires --chip".into());
    }
    Ok(Options {
        chip,
        family,
        package,
        shard,
        limit,
        output: flags.remove("--output").map(PathBuf::from),
        plan_only,
    })
}

impl Options {
    pub fn command(&self, root: &Path) -> Command {
        let mut command = Command::new(if cfg!(windows) { "python" } else { "python3" });
        command
            .current_dir(root)
            .arg("xtask/scripts/peripheral_matrix.py");
        for (flag, value) in [
            ("--chip", &self.chip),
            ("--family", &self.family),
            ("--package", &self.package),
        ] {
            if let Some(value) = value {
                command.arg(flag).arg(value);
            }
        }
        if let Some((index, count)) = self.shard {
            command.arg("--shard").arg(format!("{index}/{count}"));
        }
        if let Some(limit) = self.limit {
            command.arg("--limit").arg(limit.to_string());
        }
        if let Some(output) = &self.output {
            command.arg("--output").arg(output);
        }
        if self.plan_only {
            command.arg("--plan-only");
        }
        command
    }

    pub fn run(&self, root: &Path) -> Result<()> {
        let status = self
            .command(root)
            .status()
            .map_err(|error| format!("start peripheral matrix (Python 3.11+ required): {error}"))?;
        if status.success() {
            Ok(())
        } else {
            Err(format!(
                "peripheral matrix exited with {status}; inspect its per-chip reports"
            ))
        }
    }
}
