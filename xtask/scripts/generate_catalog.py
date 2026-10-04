"""Rebuild the catalogue from pinned, hash-checked upstream source data.

Development-time tool (Python >=3.11, stdlib only). The user-facing CLI is Rust.
Run from any directory; download caches live under data/sources/.
"""
import argparse
import collections
import hashlib
import json
from pathlib import Path
import re
import tomllib
import urllib.request
import xml.etree.ElementTree as ET
import zipfile

ROOT = Path(__file__).resolve().parents[2]
DATA = ROOT / "data"
ST_SHA = "7d1f1514ed5583ec5007ad91236b4e1d377295b1"
EMBASSY_SHA = "ae9e6f0672af84cec8e200a94c574041844396f0"
FAMILIES = {"STM32" + f for f in ("F0", "F1", "F2", "F3", "F4", "F7", "G0", "G4", "H5", "H7")}
INPUTS = {
    "st-pin-data.zip": (f"https://codeload.github.com/STMicroelectronics/STM32_open_pin_data/zip/{ST_SHA}",
                        "992c2ec12ef569361fe250d5803de42b94d5bef5ccf61329c03f020b8501475f"),
    "st-pin-tree.json": (f"https://api.github.com/repos/STMicroelectronics/STM32_open_pin_data/git/trees/{ST_SHA}?recursive=1", None),
    "embassy-stm32-Cargo.toml": (f"https://raw.githubusercontent.com/embassy-rs/embassy/{EMBASSY_SHA}/embassy-stm32/Cargo.toml",
                               "bf446fd7ee4705beaa5e17028417afe194e67622998dfdc41e175a6d28a1da96"),
}
TARGETS = {
    "cortex-m0": "thumbv6m-none-eabi", "cortex-m0+": "thumbv6m-none-eabi",
    "cortex-m3": "thumbv7m-none-eabi", "cortex-m4": "thumbv7em-none-eabihf",
    "cortex-m7": "thumbv7em-none-eabihf", "cortex-m33": "thumbv8m.main-none-eabihf",
}


def local_patch_features():
    """Availability is separate from upstream coverage and from compilation."""
    manifest_path = ROOT / "data/patches/embassy-stm32/manifest.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    if manifest["revision"] != EMBASSY_SHA:
        raise ValueError("HAL patch revision mismatch")
    hal_path = ROOT / "vendor/embassy-stm32/Cargo.toml"
    if hashlib.sha256(hal_path.read_bytes()).hexdigest() != manifest["files"]["Cargo.toml"]:
        raise ValueError("HAL patch feature manifest differs")
    root = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    if root["patch"]["https://github.com/embassy-rs/embassy"]["embassy-stm32"]["path"] != "vendor/embassy-stm32":
        raise ValueError("HAL patch is not active")
    if root["patch"]["https://github.com/embassy-rs/stm32-data-generated"]["stm32-metapac"]["path"] != "vendor/stm32-metapac":
        raise ValueError("PAC patch is not active")
    hal = tomllib.loads(hal_path.read_text(encoding="utf-8"))["features"]
    pac = tomllib.loads((ROOT / "vendor/stm32-metapac/Cargo.toml").read_text(encoding="utf-8"))["features"]
    patched = set(manifest["added_features"])
    for feature in patched:
        if hal[feature] != [f"stm32-metapac/{feature}"] or feature not in pac:
            raise ValueError(f"Patch must select its own exact PAC feature: {feature}")
    return patched


def expand_refname(value):
    """Expand only explicit ST choice lists; B-C-E never means a range."""
    match = re.fullmatch(r"(STM32[FGH][A-Z0-9]{3}[A-Z])\(([A-Z0-9](?:-[A-Z0-9])+)\)([A-Z]x[A-Z0-9]*)", value)
    if match:
        prefix, choices, suffix = match.groups()
        return [prefix + choice + suffix for choice in choices.split("-")]
    if not re.fullmatch(r"STM32[FGH][A-Z0-9]{3}[A-Z][A-Z0-9][A-Z]x[A-Z0-9]*", value):
        raise ValueError(f"Unrecognised RefName grammar: {value}")
    return [value]


def parse_mcu(xml):
    root = ET.fromstring(xml)
    if root.get("Family") not in FAMILIES:
        return []
    refname = root.attrib["RefName"]
    names = expand_refname(refname)
    children = collections.defaultdict(list)
    for child in root:
        children[child.tag.split("}")[-1]].append(child)
    cores = [c.text.strip().lower().removeprefix("arm ") for c in children["Core"]]
    if not cores or any(c not in TARGETS for c in cores):
        raise ValueError(f"Unknown core in {refname}: {cores}")

    def memory(kind):
        values = [int(c.text) for c in children[kind]]
        # New ST entries can omit capacity metadata. Preserve the gap; neither
        # a part-number suffix nor a clock-tree name is a linker specification.
        if not values:
            return [None] * len(names)
        if len(values) == 1:
            return values * len(names)
        if len(values) != len(names):
            raise ValueError(f"Ambiguous {kind} in {refname}: {values}")
        return values

    flash, ram = memory("Flash"), memory("Ram")
    return [{
        "chip": name[:11].lower(), "family": root.attrib["Family"],
        "line": root.attrib["Line"], "refname": refname, "expanded_refname": name,
        "package": root.attrib["Package"], "cores": cores,
        "flash_kib": flash[i], "ram_kib": ram[i],
        "peripherals": sorted({ip.attrib["InstanceName"] for ip in children["IP"]}),
    } for i, name in enumerate(names)]


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")


def write_features(path, lines):
    text = path.read_text(encoding="utf-8")
    begin, end = "# BEGIN GENERATED CHIP FEATURES", "# END GENERATED CHIP FEATURES"
    before, rest = text.split(begin)
    _, after = rest.split(end)
    path.write_text(before + begin + "\n" + "\n".join(lines) + "\n" + end + after, encoding="utf-8", newline="\n")


def inputs(offline):
    for name, (url, checksum) in INPUTS.items():
        path = DATA / "sources" / name
        if not path.exists():
            if offline:
                raise FileNotFoundError(f"Missing cache: {path}; rerun without --offline")
            path.parent.mkdir(parents=True, exist_ok=True)
            request = urllib.request.Request(url, headers={"User-Agent": "embodied-framework-catalog"})
            path.write_bytes(urllib.request.urlopen(request, timeout=90).read())
        if checksum and sha256(path.read_bytes()) != checksum:
            raise ValueError(f"SHA256 mismatch for {path}")


def generate(offline=False):
    inputs(offline)
    tree = json.loads((DATA / "sources/st-pin-tree.json").read_text())
    if tree.get("sha") != ST_SHA or tree.get("truncated"):
        raise ValueError("ST tree revision mismatch or truncated GitHub tree")
    blobs = {entry["path"]: entry["sha"] for entry in tree["tree"] if entry["type"] == "blob"}
    embassy = tomllib.loads((DATA / "sources/embassy-stm32-Cargo.toml").read_text())
    features = {name for name in embassy["features"] if re.fullmatch(r"stm32[fgh][a-z0-9]+(?:-cm[47])?", name)}
    patched = local_patch_features()
    available = features | patched
    grouped = collections.defaultdict(list)
    manifest = []
    with zipfile.ZipFile(DATA / "sources/st-pin-data.zip") as archive:
        prefix = f"STM32_open_pin_data-{ST_SHA}/"
        (DATA / "ST-LICENSE.txt").write_bytes(archive.read(prefix + "LICENSE"))
        for info in sorted(archive.infolist(), key=lambda v: v.filename):
            path = info.filename.removeprefix(prefix)
            # Filenames only select XML documents; every identity comes from XML.
            if not path.startswith("mcu/") or "/" in path[4:] or not path.endswith(".xml"):
                continue
            raw = archive.read(info)
            rows = parse_mcu(raw)
            if not rows:
                continue
            git_hash = hashlib.sha1(b"blob " + str(len(raw)).encode() + b"\0" + raw).hexdigest()
            if blobs.get(path) != git_hash:
                raise ValueError(f"XML does not match pinned Git tree: {path}")
            manifest.append({"path": path, "git_blob_sha1": git_hash, "sha256": sha256(raw),
                             "refname": rows[0]["refname"], "expanded_count": len(rows)})
            for row in rows:
                row["source_xml"] = path
                grouped[row["chip"]].append(row)
    devices = []
    builds = []
    for chip, variants in sorted(grouped.items()):
        first = variants[0]
        for row in variants:
            for field in ("family", "cores", "flash_kib"):
                if row[field] != first[field]:
                    raise ValueError(f"Conflicting {field} after package merge: {chip}")
        # RAM and IP lists are package-dependent metadata, never a linker map.
        devices.append({"chip": chip, "family": first["family"], "cores": first["cores"],
                        "flash_kib": first["flash_kib"], "variants": variants})
        for core in first["cores"]:
            feature = chip + ("-cm" + core.split("m")[-1] if len(first["cores"]) > 1 else "")
            issues = []
            if chip.startswith(("stm32h7r", "stm32h7s")):
                issues.append("embassy-h7rs-fdcan-unverified")
            builds.append({"feature": feature, "chip": chip, "family": first["family"], "core": core,
                           "target": TARGETS[core], "embassy_feature_available": feature in features,
                           "local_patch_available": feature in patched,
                           "catalogue": "verified", "compile": "unknown", "link": "unknown", "hil": "unknown",
                           "configuration": "trustzone-disabled" if chip.startswith("stm32h5") else "default",
                           "known_runtime_issues": issues})
    build_keys = {r["feature"] for r in builds}
    policy_path = DATA / "support-policy.json"
    policy = json.loads(policy_path.read_text(encoding="utf-8"))
    if policy["schema_version"] != 1:
        raise ValueError("Unsupported support policy schema")
    excluded = {}
    for group in policy["exclusions"]:
        if not group["reason"].strip():
            raise ValueError("Excluded models require a reason")
        for feature in group["features"]:
            if feature not in build_keys or feature in excluded:
                raise ValueError(f"Unknown or duplicate excluded feature: {feature}")
            excluded[feature] = group["reason"]
    for row in builds:
        if row["feature"] in excluded:
            row["exclusion_reason"] = excluded[row["feature"]]
    supported = build_keys - excluded.keys()
    selectable = supported & available
    missing = sorted(build_keys - features)
    extras = sorted(features - build_keys)
    counts = {"source_xml_files": len(manifest), "package_variants": sum(len(x) for x in grouped.values()),
              "normalized_devices": len(devices), "core_builds": len(builds), "embassy_features": len(features),
              "missing_embassy_features": len(missing), "embassy_features_without_st_device": len(extras),
              "local_patch_features": len(patched), "missing_backend_features": len(build_keys - available)}
    counts.update(supported_devices=len({r["chip"] for r in builds if r["feature"] in supported}),
                  supported_core_builds=len(supported), excluded_core_builds=len(excluded),
                  supported_missing_backend_features=len(supported - available))
    sources = {
        "support_policy": {"path": "data/support-policy.json", "sha256": sha256(policy_path.read_bytes()),
                           "changed_on": policy["changed_on"]},
        "st_pin_data": {"revision": ST_SHA, "repository": "https://github.com/STMicroelectronics/STM32_open_pin_data",
                        "archive_sha256": INPUTS["st-pin-data.zip"][1], "license": "data/ST-LICENSE.txt"},
        "embassy": {"revision": EMBASSY_SHA, "repository": "https://github.com/embassy-rs/embassy",
                    "cargo_toml_sha256": INPUTS["embassy-stm32-Cargo.toml"][1]},
        "local_patches": {"features": sorted(patched),
                          "hal_manifest_sha256": hashlib.sha256((ROOT / "data/patches/embassy-stm32/manifest.json").read_bytes()).hexdigest(),
                          "pac_manifest_sha256": hashlib.sha256((ROOT / "data/patches/stm32-metapac/vendor-manifest.json").read_bytes()).hexdigest()},
    }
    catalogue = {"schema_version": 1, "sources": sources, "statistics": counts,
                 "normalization": "Eleven-character STM32 device key from XML RefName; package/options retained in variants; dual cores separate build rows.",
                 "devices": devices, "builds": builds,
                 "differences": {"missing_embassy_features": missing, "embassy_features_without_st_device": extras,
                                 "locally_patched_features": sorted(patched), "missing_backend_features": sorted(build_keys - available)}}
    write_json(DATA / "chips.json", catalogue)
    write_json(DATA / "sources/xml-manifest.json", {"revision": ST_SHA, "files": manifest})
    write_json(DATA / "generated/target-map.json", {r["feature"]: {k: r[k] for k in ("target", "core", "chip", "embassy_feature_available", "local_patch_available")} for r in builds if r["feature"] in supported})
    generated = ["# Generated reference only: this is not framework support certification.",
                 f"# Embassy {EMBASSY_SHA}; regenerate with xtask/scripts/generate_catalog.py", "[features]"]
    generated += [f'{feature} = ["embassy-stm32/{feature}"]' for feature in sorted(selectable)]
    (DATA / "generated/stm32-features.toml").write_text("\n".join(generated) + "\n", encoding="utf-8", newline="\n")
    write_json(DATA / "generated/embassy-difference.json", catalogue["differences"])
    write_features(ROOT / "crates/embodied-stm32/Cargo.toml",
                   [f'{f} = ["hal", "embassy-stm32/{f}"]' for f in sorted(selectable)])
    write_features(ROOT / "App/Cargo.toml",
                   [f'{f} = ["firmware", "embodied-stm32/{f}"]' for f in sorted(selectable)])
    print(json.dumps(counts, indent=2))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--offline", action="store_true", help="Require already-downloaded source caches")
    generate(parser.parse_args().offline)
