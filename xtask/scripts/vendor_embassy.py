"""Reproduce the STM32 HAL fork from the pinned Embassy archive (stdlib only).

Relocate sibling dependencies and expose independently generated chip PACs.
Recorded source edits are reapplied from hash-bound patch hunks.
"""
import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath, PureWindowsPath
import re
import urllib.request
import zipfile

ROOT = Path(__file__).resolve().parents[2]
REV = "ae9e6f0672af84cec8e200a94c574041844396f0"
ARCHIVE_SHA = "98c70001d38674e8ecc40ebf359e238d6e73a0c24caddd4d5b1f40746afc0793"
ADDED = ["stm32f723rc", "stm32f723re", "stm32h563li", "stm32h573li"]
SOURCE_PATCHES = ("dual-core.json", "fdcan-flush.json")


def patched_manifest(raw):
    text = raw.decode("utf-8")
    text, count = re.subn(r'path = "\.\./embassy-[a-z-]+"',
                         f'git = "https://github.com/embassy-rs/embassy", rev = "{REV}"', text)
    if count != 12:
        raise ValueError(f"Unexpected sibling dependency count: {count}")
    for feature in ADDED:
        anchor = feature.replace("li", "zi") if feature.endswith("li") else feature.replace("stm32f723r", "stm32f723v")
        line = f'{anchor} = [ "stm32-metapac/{anchor}" ]'
        if text.count(line) != 1 or f'\n{feature} =' in text:
            raise ValueError(f"Unexpected original feature layout: {feature}")
        text = text.replace(line, line + f'\n{feature} = ["stm32-metapac/{feature}"]')
    return text.encode()


def patch_source(raw, record):
    if hashlib.sha256(raw).hexdigest() != record["upstream_sha256"]:
        raise ValueError(f"HAL source patch baseline mismatch: {record['path']}")
    text = raw.decode("utf-8")
    for replacement in record["replacements"]:
        if text.count(replacement["old"]) != 1:
            raise ValueError(f"HAL source patch does not match exactly once: {record['path']}")
        text = text.replace(replacement["old"], replacement["new"])
    result = text.encode("utf-8")
    if hashlib.sha256(result).hexdigest() != record["patched_sha256"]:
        raise ValueError(f"HAL source patch result mismatch: {record['path']}")
    return result


def new_source(record, patch_directory):
    """Load a hash-bound added source without accepting arbitrary host paths."""
    for key in ("path", "source"):
        value = record[key]
        if (not isinstance(value, str) or not value or "\\" in value or ":" in value
                or PurePosixPath(value).is_absolute() or PureWindowsPath(value).drive
                or not PurePosixPath(value).parts or ".." in PurePosixPath(value).parts
                or str(PurePosixPath(value)) != value):
            raise ValueError(f"Invalid HAL added source path: {value}")
    if record["path"].casefold() == "cargo.toml":
        raise ValueError("HAL added source cannot replace the manifest")
    base = patch_directory.resolve()
    path = (base / record["source"]).resolve()
    if not path.is_relative_to(base) or not path.is_file():
        raise ValueError(f"Missing or uncontained HAL added source path: {path}")
    raw = path.read_bytes()
    if hashlib.sha256(raw).hexdigest() != record["sha256"]:
        raise ValueError(f"HAL added source checksum mismatch: {record['source']}")
    return raw


def compose_h5_47c(files, manifest, root):
    # Keep this import local: the candidate replay also uses patch_source above.
    from prepare_h5_47c_hal import apply, checked_package
    from generate_hal_metadata import verified_package
    from h5_47c_baseline import INTEGRATION, sha

    integration_path = root / INTEGRATION
    _, integration = verified_package(root, {'manifest_path': INTEGRATION, 'sha256': sha(integration_path.read_bytes())})
    package = root / 'data/patches/stm32h5-47c-hal'
    inputs, package_sha = checked_package(package)
    spec = json.loads(inputs['patch.json'])
    raw_manifest = (json.dumps(manifest, indent=2) + '\n').encode()
    if (spec['schema_version'] != 1 or spec['base_revision'] != REV
            or sha(raw_manifest) != spec['baseline_manifest_sha256']
            or sha(raw_manifest) != integration['baselines']['hal']['manifest_sha256']
            or {n: sha(raw) for n, raw in files.items()} != manifest['files']):
        raise ValueError('47C HAL generator baseline provenance differs')
    _, pac_sha = checked_package(root / 'data/patches/stm32h5-47c-pac')
    if pac_sha != spec['pac_manifest_sha256'] or pac_sha != integration['source_package']['sha256']:
        raise ValueError('47C HAL generator PAC package differs')
    if sorted(spec['added_chips']) != sorted(n.removesuffix('.json').lower() for n in integration['new_chip_jsons_sha256']):
        raise ValueError('47C HAL generator chip identities differ')
    additions = {r['path']: new_source(r, package) for r in spec['new_files']}
    output = apply(files, spec, additions)
    if {n: sha(raw) for n, raw in output.items()} != integration['baselines']['hal']['composed_files_sha256']:
        raise ValueError('47C HAL generator differs from verified composition')
    provenance = dict(manifest)
    provenance['added_features'] = sorted(set(manifest['added_features']) | set(spec['added_chips']))
    provenance['changes'] += '; exact H543/H553 DIE47C platform overlay'
    provenance['layered_patches'] = [{'manifest_path': 'data/patches/stm32h5-47c-hal/manifest.json',
                                      'sha256': package_sha, 'baseline_manifest_sha256': sha(raw_manifest)}]
    provenance['replay_baseline'] = {'manifest_path': INTEGRATION, 'sha256': sha(integration_path.read_bytes())}
    provenance['files'] = {n: sha(raw) for n, raw in sorted(output.items())}
    return output, provenance


def derive_baseline():
    archive_path = ROOT / "data/sources/embassy.zip"
    if not archive_path.exists():
        archive_path.parent.mkdir(parents=True, exist_ok=True)
        request = urllib.request.Request(f"https://codeload.github.com/embassy-rs/embassy/zip/{REV}", headers={"User-Agent": "embodied-framework"})
        with urllib.request.urlopen(request, timeout=120) as response:
            archive_path.write_bytes(response.read())
    if hashlib.sha256(archive_path.read_bytes()).hexdigest() != ARCHIVE_SHA:
        raise ValueError("Pinned Embassy archive hash mismatch")
    files = {}
    original = {}
    with zipfile.ZipFile(archive_path) as archive:
        for license_name in ["LICENSE-MIT", "LICENSE-APACHE"]:
            raw = archive.read(f"embassy-{REV}/{license_name}")
            files[license_name] = raw
            original[license_name] = hashlib.sha256(raw).hexdigest()
        prefix = f"embassy-{REV}/embassy-stm32/"
        for name in archive.namelist():
            if name.startswith(prefix) and not name.endswith("/"):
                relative = name[len(prefix):]
                if ".." in PurePosixPath(relative).parts:
                    raise ValueError("Unsafe archive path")
                raw = archive.read(name)
                original[relative] = hashlib.sha256(raw).hexdigest()
                files[relative] = patched_manifest(raw) if relative == "Cargo.toml" else raw
    if not files:
        raise ValueError("HAL missing from archive")
    seen = set()
    source_patches = []
    for patch_name in SOURCE_PATCHES:
        source_patch_path = ROOT / "data/patches/embassy-stm32" / patch_name
        patch_bytes = source_patch_path.read_bytes()
        source_patch = json.loads(patch_bytes)
        if source_patch["schema_version"] != 1 or source_patch["base_revision"] != REV:
            raise ValueError("HAL source patch schema or revision mismatch")
        for record in source_patch["files"]:
            relative = record["path"]
            if relative not in original or relative == "Cargo.toml" or relative in seen:
                raise ValueError("HAL source patch file must be unique, original and non-manifest")
            seen.add(relative)
            files[relative] = patch_source(files[relative], record)
        for record in source_patch.get("new_files", []):
            relative = record["path"]
            raw = new_source(record, source_patch_path.parent)
            if relative.casefold() in {name.casefold() for name in original.keys() | seen}:
                raise ValueError("HAL added source must be unique and absent from the original archive")
            seen.add(relative)
            files[relative] = raw
        source_patches.append({"path": source_patch_path.relative_to(ROOT).as_posix(),
                               "sha256": hashlib.sha256(patch_bytes).hexdigest()})
    manifest = {"revision": REV, "archive_sha256": ARCHIVE_SHA, "added_features": ADDED,
                "changes": "Cargo source relocation, four exact generated chip features, dual-core startup publication, distinct H7 time drivers, explicit board timer selection over any fallback, FDCAN flush mailbox indexing/bounds, and classic CAN timestamp scaling from requested bit timing",
                "source_patches": source_patches,
                "upstream_sha256": original,
                "files": {name: hashlib.sha256(raw).hexdigest() for name, raw in sorted(files.items())}}
    return files, manifest


def generate(check=False):
    files, manifest = derive_baseline()
    if (ROOT / 'data/patches/stm32h5-47c-integration/manifest.json').exists():
        files, manifest = compose_h5_47c(files, manifest, ROOT)
    from hal_post_patches import PATCH, apply as apply_post_patch
    if (ROOT / PATCH).exists():
        files, manifest = apply_post_patch(files, manifest, ROOT)
    from h5_47a_integration import INTEGRATION as H5_47A, apply_hal
    if (ROOT / H5_47A).exists():
        files, manifest = apply_hal(files, manifest, ROOT)
    from h5_rtc_overlay import PACKAGE as RTC_PACKAGE, apply as apply_rtc
    if (ROOT / RTC_PACKAGE).exists():
        files, manifest = apply_rtc(files, manifest, ROOT, 'hal')
    from hal_platform_fixes import PATCH as PLATFORM_PATCH, apply as apply_platform
    if (ROOT / PLATFORM_PATCH).exists():
        files, manifest = apply_platform(files, manifest, ROOT)
    vendor = ROOT / "vendor/embassy-stm32"
    actual = {path.relative_to(vendor).as_posix() for path in vendor.rglob("*") if path.is_file()}
    # Refuse stale/unreviewed additions even when materializing; never silently
    # delete local files or certify only a subset of the resulting directory.
    if actual - set(files) or (check and actual != set(files)):
        raise ValueError("HAL fork file inventory differs from reproducible output")
    for relative, raw in files.items():
        target = vendor / relative
        if check:
            if not target.exists() or target.read_bytes() != raw:
                raise ValueError(f"HAL fork differs from reproducible output: {relative}")
        else:
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(raw)
    target = ROOT / "data/patches/embassy-stm32/manifest.json"
    raw = (json.dumps(manifest, indent=2) + "\n").encode()
    if check:
        if target.read_bytes() != raw:
            raise ValueError("HAL fork provenance manifest differs")
    else:
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(raw)
    print(f"{'Verified' if check else 'Vendored'} {len(files)} HAL files; new exact features: {', '.join(manifest['added_features'])}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    generate(parser.parse_args().check)
