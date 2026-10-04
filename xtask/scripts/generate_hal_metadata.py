"""Extract memory variants from the pinned PAC source resolved by Cargo.

Run after cargo fetch. No capacity/address is inferred from a part number.
The output is build metadata, not datasheet or hardware validation.
"""
import hashlib
import json
import os
from pathlib import Path, PurePosixPath, PureWindowsPath
import subprocess
from urllib.parse import urlsplit

ROOT = Path(__file__).resolve().parents[2]
REVISION = "e463add8cc54375f61c6f5f83d6b589e7fc68be2"
TAG = "stm32-data-caa36afd62510b0e6315ee0dccd1f9c65fbcac83"
GENERATOR_REVISION = TAG.removeprefix("stm32-data-")


def baseline_checkout():
    cargo_home = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo"))
    candidates = list((cargo_home / "git/checkouts").glob("stm32-data-generated-*/*"))
    cache = ROOT / ".cache/pinned-pac-data"
    candidates.append(cache)
    for path in candidates:
        if (path / "data/chips").is_dir():
            actual = subprocess.check_output(["git", "-C", str(path), "rev-parse", "HEAD"], text=True).strip()
            if actual == REVISION:
                return path
    if cache.exists():
        raise ValueError("Existing PAC source cache has wrong revision; inspect it before replacing")
    cache.parent.mkdir(parents=True, exist_ok=True)
    # These JSON bytes are checked against Git blob identities. A developer's
    # global Windows autocrlf policy must not rewrite the pinned source checkout.
    # --config is installed in this clone before its initial checkout only.
    subprocess.run(["git", "clone", "--config", "core.autocrlf=false", "--depth", "1", "--branch", TAG,
                    "https://github.com/embassy-rs/stm32-data-generated", str(cache)], check=True)
    actual = subprocess.check_output(["git", "-C", str(cache), "rev-parse", "HEAD"], text=True).strip()
    if actual != REVISION:
        raise ValueError("PAC source tag no longer matches pinned revision")
    return cache


def checked_child_path(base, relative):
    # Manifests use portable forward-slash paths. Reject Windows roots/drives
    # even when verification runs on Linux, and resolve symlinks before reads.
    if not isinstance(relative, str) or not relative or "\\" in relative:
        raise ValueError(f"Invalid source path: {relative}")
    path = Path(relative)
    if (path.is_absolute() or PureWindowsPath(relative).drive or PureWindowsPath(relative).root or ".." in path.parts
            or not path.parts):
        raise ValueError(f"Invalid source path: {relative}")
    resolved = (base / path).resolve()
    if not resolved.is_relative_to(base.resolve()):
        raise ValueError(f"Invalid source path: {relative}")
    return resolved


def checked_patch_path(root, relative):
    resolved = checked_child_path(root, relative)
    if not resolved.is_relative_to((root / "data/patches").resolve()):
        raise ValueError(f"Invalid patch source path: {relative}")
    return resolved


def patch_input_record(manifest, filename):
    added = manifest["new_chip_jsons"].get(filename)
    corrected = manifest.get("corrected_chip_jsons", {}).get(filename)
    if added is not None and corrected is not None:
        raise ValueError(f"Chip is both newly added and corrected: {filename}")
    return corrected if corrected is not None else added


def patch_json_path(manifest, filename, root=ROOT):
    record = patch_input_record(manifest, filename)
    if record is None:
        return None
    path = checked_patch_path(root, record["path"])
    if path.name != filename:
        raise ValueError(f"Local chip path identity differs: {filename}")
    return path


def hash_bound_bytes(path, expected, label):
    try:
        raw = path.read_bytes()
    except OSError as error:
        raise ValueError(f"Missing or unreadable {label}: {path}") from error
    if hashlib.sha256(raw).hexdigest() != expected:
        raise ValueError(f"{label} checksum differs: {path}")
    return raw


def identified_chip(raw, filename):
    item = json.loads(raw)
    if item.get("name") != filename.removesuffix(".json"):
        raise ValueError(f"PAC JSON name differs from catalogue identity: {filename}")
    return item


def verified_package(root, record):
    """Verify every file of an immutable, complete local source package."""
    path = checked_patch_path(root, record['manifest_path'])
    if path.name != 'manifest.json':
        raise ValueError('Unexpected source package manifest path')
    package = json.loads(hash_bound_bytes(path, record['sha256'], 'source package manifest'))
    if package.get('completed') is not True:
        raise ValueError('Source package is not complete')
    names = package['files']
    folded = set()
    for name in names:
        if (str(PurePosixPath(name)) != name or name.casefold() in folded
                or name == 'manifest.json' or '__pycache__' in PurePosixPath(name).parts):
            raise ValueError('Invalid or colliding source package path: ' + name)
        checked_child_path(path.parent, name)
        folded.add(name.casefold())
    files = {}
    for child in path.parent.rglob('*'):
        if child.is_symlink():
            raise ValueError('Symbolic link in source package: ' + str(child))
        relative = child.relative_to(path.parent).as_posix()
        if child.is_file() and child != path and '__pycache__' not in child.relative_to(path.parent).parts:
            files[relative] = hashlib.sha256(child.read_bytes()).hexdigest()
    if files != names:
        raise ValueError('Source package inventory/hash differs')
    return path.parent, package


def verify_overlay_inputs(manifest, root, *, verify_documents=True):
    if 'f7_otp_overlay' in manifest:
        from f7_otp_overlay import validate_manifest
        _, _, previous = validate_manifest(manifest, root)
        manifest = json.loads(previous)
    elif 'corrected_chip_jsons' in manifest:
        raise ValueError('Corrected chip inputs lack their publication provenance')
    if 'h5_rtc_low_power_overlay' in manifest:
        from h5_rtc_overlay import validate_manifest
        validate_manifest(manifest, root, 'pac')
    expected_chips = {}
    documents = {}
    seen_ids, seen_paths = set(), set()
    for overlay in manifest["overlays"]:
        identifier = overlay["id"]
        path = checked_patch_path(root, overlay["manifest_path"])
        if not identifier or identifier in seen_ids or path in seen_paths:
            raise ValueError(f"Duplicate or empty PAC overlay identity: {identifier}")
        seen_ids.add(identifier)
        seen_paths.add(path)
        original = json.loads(hash_bound_bytes(path, overlay["manifest_sha256"], "PAC overlay provenance"))
        if (original["baseline_revision"] != REVISION
                or original["generator_revision"] != GENERATOR_REVISION):
            raise ValueError(f"PAC overlay revision differs: {identifier}")
        base = path.parent
        if 'source_package' in original:
            base, package = verified_package(root, original['source_package'])
            packaged_chips = {name.removeprefix('chips/'): value for name, value in package['files'].items()
                              if name.startswith('chips/') and name.endswith('.json')}
            if (packaged_chips != original['new_chip_jsons_sha256']
                    or package.get('chips') != len(packaged_chips) or not packaged_chips):
                raise ValueError('Source package chip JSON inventory differs from overlay')
            # This package contains its source evidence in full; no download-only
            # PDF records from the older overlay format are inferred here.
            sources = {}
        else:
            sources = json.loads(hash_bound_bytes(
                checked_child_path(base, "sources.json"), original["sources_manifest_sha256"], "PAC sources manifest"))
            hash_bound_bytes(checked_child_path(base, "comparison.json"), original["comparison_sha256"], "PAC comparison")
            if sources["pac_revision"] != REVISION or sources["generator_revision"] != GENERATOR_REVISION:
                raise ValueError(f"PAC source revision differs: {identifier}")
            source_dir = checked_child_path(base, "sources")
            for name, record in sources["files"].items():
                hash_bound_bytes(checked_child_path(source_dir, name), record["sha256"], "PAC source file")
        # URL-only references are retained as citations. A cached document with
        # a recorded digest is part of the same byte-verifiable evidence chain.
        for name, record in sources.get("documents", {}).items():
            if "cache_path" in record or "sha256" in record:
                if "cache_path" not in record or "sha256" not in record:
                    raise ValueError(f"Incomplete cached document provenance: {name}")
                document = checked_child_path(root, record["cache_path"])
                if not document.is_relative_to((root / "data/sources").resolve()):
                    raise ValueError(f"Invalid cached document path: {name}")
                url = record.get("retrieved_mirror_url") or record.get("url") or record.get("official_url")
                parsed = urlsplit(url or "")
                if parsed.scheme != "https" or not parsed.hostname or parsed.username or parsed.password or parsed.fragment:
                    raise ValueError(f"Missing or invalid fixed document download URL: {name}")
                document_record = {"cache_path": record["cache_path"], "sha256": record["sha256"], "url": url}
                if document in documents and documents[document] != document_record:
                    raise ValueError(f"Conflicting cached document provenance: {name}")
                documents[document] = document_record
                if verify_documents:
                    try:
                        hash_bound_bytes(document, record["sha256"], "PAC cached document")
                    except ValueError as error:
                        raise ValueError(f"{error}; run python xtask/scripts/fetch_source_documents.py") from error
        for filename, digest in original["new_chip_jsons_sha256"].items():
            path = checked_child_path(base / "chips", filename)
            if path.name != filename or not filename.endswith(".json") or filename in expected_chips:
                raise ValueError(f"Duplicate or invalid overlay chip identity: {filename}")
            expected_chips[filename] = (identifier, path, digest)
    if set(manifest["new_chip_jsons"]) != set(expected_chips):
        raise ValueError("Combined PAC chip JSON inventory differs from overlays")
    for filename, (identifier, expected_path, digest) in expected_chips.items():
        record = manifest["new_chip_jsons"][filename]
        path = patch_json_path(manifest, filename, root)
        if record["overlay"] != identifier or record["sha256"] != digest or path != expected_path:
            raise ValueError(f"Combined PAC JSON identity differs from its overlay: {filename}")
        identified_chip(hash_bound_bytes(path, digest, "Local PAC JSON"), filename)
    return list(documents.values())


def load_patch_manifest(root=ROOT):
    manifest = json.loads((root / "data/patches/stm32-metapac/vendor-manifest.json").read_text(encoding="utf-8"))
    if manifest["schema_version"] != 2:
        raise ValueError("Unsupported combined PAC patch manifest schema")
    if manifest["baseline_revision"] != REVISION or manifest["generator_revision"] != GENERATOR_REVISION:
        raise ValueError("PAC patch baseline differs")
    return manifest


def verified_source_documents(root=ROOT):
    """Resolve fixed download records after verifying all committed provenance.

    Only this explicit fetch preparation skips document bytes, which may not
    exist yet. Normal memory generation still requires and verifies every PDF.
    """
    return verify_overlay_inputs(load_patch_manifest(root), root, verify_documents=False)


def verify_vendor(package, root=ROOT):
    manifest = load_patch_manifest(root)
    vendor = root / "vendor/stm32-metapac"
    if Path(package["manifest_path"]).resolve() != (vendor / "Cargo.toml").resolve():
        raise ValueError("Cargo resolved an unexpected local PAC")
    actual_files = {p.relative_to(vendor).as_posix() for p in vendor.rglob("*") if p.is_file()}
    if actual_files != set(manifest["vendor_files_sha256"]):
        raise ValueError("PAC vendor inventory differs from generated patch")
    for name, expected in manifest["vendor_files_sha256"].items():
        path = checked_child_path(vendor, name)
        if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError(f"PAC vendor file differs from generated patch: {name}")
    verify_overlay_inputs(manifest, root)
    return manifest


def default_bank(memory):
    if len(memory) == 1:
        return None
    single = [m for m in memory if any("BANK_1" in r["name"] for r in m)
              and not any("BANK_2" in r["name"] for r in m)]
    dual = [m for m in memory if any("BANK_1" in r["name"] for r in m)
            and any("BANK_2" in r["name"] for r in m)]
    if len(single) != 1 or len(dual) != 1:
        raise ValueError("Unrecognised memory selection; review the pinned PAC schema")
    # Project build policy, not an assertion about a device's option bytes.
    return "dual-bank"


def generate():
    metadata = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--format-version", "1", "--locked",
         "--features", "embodied-app/stm32h723vg"], cwd=ROOT))
    package = next(p for p in metadata["packages"] if p["name"] == "stm32-metapac")
    if package["source"] is None:
        patch = verify_vendor(package)
        checkout = baseline_checkout()
    else:
        patch = None
        if TAG not in package["source"]:
            raise ValueError("Cargo resolved a different STM32 PAC tag")
        checkout = Path(package["manifest_path"]).parent.parent
    actual = subprocess.check_output(["git", "-C", str(checkout), "rev-parse", "HEAD"], text=True).strip()
    if actual != REVISION:
        raise ValueError(f"Unexpected PAC commit: {actual}")
    tree = subprocess.check_output(
        ["git", "-C", str(checkout), "ls-tree", "-r", "HEAD", "data/chips"], text=True)
    blobs = {line.split("\t", 1)[1]: line.split()[2] for line in tree.splitlines()}
    catalogue = json.loads((ROOT / "data/chips.json").read_text(encoding="utf-8"))
    chips = {}
    for row in catalogue["devices"]:
        filename = row["chip"].upper() + ".json"
        path = checkout / "data/chips" / filename
        local_path = None if patch is None else patch_json_path(patch, filename)
        local = local_path is not None
        if local:
            path = local_path
        if not path.exists():
            if local or path.relative_to(checkout).as_posix() in blobs:
                raise ValueError(f"Required pinned PAC chip JSON is missing: {filename}")
            continue
        raw = path.read_bytes()
        git_blob = hashlib.sha1(b"blob " + str(len(raw)).encode() + b"\0" + raw).hexdigest()
        if local:
            if hashlib.sha256(raw).hexdigest() != patch_input_record(patch, filename)["sha256"]:
                raise ValueError(f"Local PAC JSON differs from generated patch: {filename}")
        elif blobs.get(path.relative_to(checkout).as_posix()) != git_blob:
            raise ValueError(f"PAC data differs from the pinned Git object: {path.name}")
        item = identified_chip(raw, filename)
        chips[row["chip"]] = {
            "source_sha256": hashlib.sha256(raw).hexdigest(),
            "source_kind": ("local-metadata-correction" if local and filename in patch.get("corrected_chip_jsons", {})
                            else "local-generated-patch" if local else "pinned-upstream"),
            "default_bank": default_bank(item["memory"]),
            "memory": item["memory"],
            "cores": [c["name"] for c in item["cores"]],
        }
    result = {"source_revision": REVISION, "source_tag": TAG,
              "bank_policy": "explicit dual-bank build for configurable parts; option bytes must match on hardware",
              "chips": chips}
    path = ROOT / "data/generated/hal-memory.json"
    path.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(f"Extracted pinned PAC memory metadata for {len(chips)} devices")


if __name__ == "__main__":
    generate()
