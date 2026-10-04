//! Recorded minimum-firmware coverage. Artifact hashes establish retention,
//! not a repeat ELF inspection, a current-source build, or hardware validation.
use crate::{Catalogue, Chip, build::sha256};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path};

const STAGES: [&str; 3] = ["compile", "link", "elf_validation"];

pub fn collect(root: &Path, catalogue: &Catalogue) -> Value {
    let mut evidence: BTreeMap<&str, BTreeMap<String, usize>> = STAGES
        .into_iter()
        .map(|stage| (stage, BTreeMap::new()))
        .collect();
    let mut integrity_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut rows = Vec::with_capacity(catalogue.builds.len());
    for chip in catalogue.builds.iter().filter(|chip| chip.in_scope()) {
        let path = root
            .join("target/reports")
            .join(&chip.feature)
            .join("result.json");
        let (report_info, report) = read_report(&path, chip);
        let mut recorded = serde_json::Map::new();
        for stage in STAGES {
            let status = report
                .as_ref()
                .and_then(|r| r[stage]["status"].as_str())
                .unwrap_or("unknown");
            *evidence
                .get_mut(stage)
                .unwrap()
                .entry(status.into())
                .or_default() += 1;
            recorded.insert(stage.into(), json!(status));
        }
        let report_value = report.as_ref().unwrap_or(&Value::Null);
        recorded.insert(
            "hardware_validation".into(),
            json!(
                report_value["hardware_validation"]
                    .as_str()
                    .unwrap_or("unknown")
            ),
        );
        let integrity = report.as_ref().map_or_else(
            || json!({"status":"not-checked"}),
            |report| verify_artifacts(root, chip, &report["artifacts"]),
        );
        *integrity_counts
            .entry(integrity["status"].as_str().unwrap().into())
            .or_default() += 1;
        rows.push(json!({
            "feature":chip.feature, "chip":chip.chip, "family":chip.family,
            "core":chip.core, "target":chip.target,
            "backend":if chip.local_patch_available { "local-patch" } else if chip.embassy_feature_available { "upstream" } else { "missing" },
            "known_runtime_issues":chip.known_runtime_issues,
            "report":report_info, "recorded":recorded, "artifact_integrity":integrity,
            "provenance":{
                "sources":report_value["sources"], "cargo_lock_sha256":report_value["cargo_lock_sha256"],
                "started_unix_ms":report_value["started_unix_ms"], "finished_unix_ms":report_value["finished_unix_ms"],
                "rustc_vv":report_value["rustc_vv"]["stdout"], "build_configuration":report_value["build_configuration"]
            }
        }));
    }
    json!({
        "schema_version":1, "core_builds":rows.len(),
        "catalogue_core_builds":catalogue.builds.len(),
        "excluded_core_builds":catalogue.builds.iter().filter(|c| !c.in_scope()).count(),
        "excluded_builds":catalogue.builds.iter().filter_map(|c| c.exclusion_reason.as_ref().map(|reason| json!({
            "feature":c.feature,"chip":c.chip,"family":c.family,"reason":reason,
            "known_runtime_issues":c.known_runtime_issues
        }))).collect::<Vec<_>>(),
        "missing_embassy_features":catalogue.builds.iter().filter(|c| c.in_scope() && !c.embassy_feature_available).count(),
        "local_patch_features":catalogue.builds.iter().filter(|c| c.in_scope() && c.local_patch_available).count(),
        "missing_backend_features":catalogue.builds.iter().filter(|c| c.in_scope() && !c.backend_available()).count(),
        "catalogue_statistics":catalogue.statistics, "sources":catalogue.sources,
        "build_evidence":evidence,
        "build_evidence_scope":"latest recorded reports; inspect each report's source and toolchain provenance",
        "artifact_integrity":integrity_counts,
        "artifact_integrity_scope":"SHA-256 of retained ELF and MAP bytes only; not a new ELF inspection or current-source verification",
        "peripheral_validation":"not aggregated here; use the separate per-chip peripheral reports",
        "hardware_validation":"not inferred from compilation", "builds":rows
    })
}

fn read_report(path: &Path, chip: &Chip) -> (Value, Option<Value>) {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) => {
            return (
                json!({"path":path,"status":if error.kind() == std::io::ErrorKind::NotFound { "missing" } else { "unreadable" },"error":error.to_string()}),
                None,
            );
        }
    };
    let hash = sha256(&bytes);
    let report: Value = match serde_json::from_slice(&bytes) {
        Ok(report) => report,
        Err(error) => {
            return (
                json!({"path":path,"sha256":hash,"status":"invalid-json","error":error.to_string()}),
                None,
            );
        }
    };
    let status = if report["schema_version"] != 1 {
        "unsupported-schema"
    } else if report["chip"]["feature"] != chip.feature || report["chip"]["target"] != chip.target {
        "identity-mismatch"
    } else {
        "available"
    };
    (
        json!({"path":path,"sha256":hash,"status":status}),
        (status == "available").then_some(report),
    )
}

fn verify_artifacts(root: &Path, chip: &Chip, artifacts: &Value) -> Value {
    if artifacts.is_null() {
        return json!({"status":"not-produced"});
    }
    if artifacts["feature"] != chip.feature || artifacts["target"] != chip.target {
        return json!({"status":"identity-mismatch"});
    }
    let mut files = serde_json::Map::new();
    let mut status = "verified";
    for name in ["elf", "map"] {
        let record = match (
            artifacts[name].as_str(),
            artifacts[format!("{name}_sha256")].as_str(),
        ) {
            (Some(path), Some(expected))
                if !path.is_empty()
                    && expected.len() == 64
                    && expected.bytes().all(|b| b.is_ascii_hexdigit()) =>
            {
                let path = root.join(path);
                match hash_file(&path) {
                    Ok(actual) => {
                        json!({"status":if actual.eq_ignore_ascii_case(expected) { "verified" } else { "hash-mismatch" }, "path":path, "expected_sha256":expected, "actual_sha256":actual})
                    }
                    Err(error) => {
                        json!({"status":"unreadable","path":path,"error":error.to_string()})
                    }
                }
            }
            _ => json!({"status":"unverifiable","reason":"missing path or invalid SHA-256"}),
        };
        let file_status = record["status"].as_str().unwrap();
        if status == "verified" && file_status != "verified" {
            status = match file_status {
                "hash-mismatch" => "hash-mismatch",
                "unreadable" => "unreadable",
                _ => "unverifiable",
            };
        }
        files.insert(name.into(), record);
    }
    json!({"status":status,"files":files})
}

fn hash_file(path: &Path) -> std::io::Result<String> {
    let mut file = fs::File::open(path)?;
    if !file.metadata()?.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "artifact is not a regular file",
        ));
    }
    let mut hash = Sha256::new();
    let mut buffer = [0; 64 * 1024];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hash.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hash.finalize()))
}
