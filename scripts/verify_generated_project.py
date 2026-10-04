"""Verify an unmodified generated project's snapshot and normalized lockfile.

This checks the generation baseline, not subsequent user edits. It does not
claim that a project compiles or that its firmware has run on hardware.
"""
import argparse
import hashlib
import json
import os
import re
from pathlib import Path, PurePosixPath, PureWindowsPath


def required_manifests(report):
    """Validate the topology before trusting a pair's source-file inventory."""
    schema = report.get("schema_version")
    if schema == 1:
        return ["App/Cargo.toml"]
    if schema != 2 or report.get("topology") != "dual-core-pair":
        raise ValueError("unsupported generated project topology")
    part = report.get("part")
    images = report.get("images")
    target = "thumbv7em-none-eabihf"
    if (not isinstance(part, str) or not re.fullmatch(r"stm32h7[45][57][a-z][a-z0-9]", part)
            or report.get("target") != target or not isinstance(images, list) or len(images) != 2
            or not all(isinstance(image, dict) for image in images)):
        raise ValueError("invalid dual-core project identity")
    if {image.get("core") for image in images} != {"cm7", "cm4"}:
        raise ValueError("paired project requires exactly one cm7 and one cm4 image")
    for image in images:
        core = image["core"]
        expected = dict(chip=f"{part}-{core}", manifest=f"App/{core}/Cargo.toml",
                        binary=f"firmware-{core}", target=target,
                        time_driver="TIM5" if core == "cm7" else "TIM2")
        if (any(image.get(key) != value for key, value in expected.items())
                or not isinstance(image.get("package"), str) or not image["package"]):
            raise ValueError(f"inconsistent paired image metadata: {core}")
    if (images[0]["package"] == images[1]["package"]
            or report.get("chip") not in {image["chip"] for image in images}):
        raise ValueError("paired project package or requested chip mismatch")
    return [image["manifest"] for image in images]


def verify(project, baseline=None):
    root = Path(project).resolve(strict=True)
    manifest_bytes = (root / "project.json").read_bytes()
    if baseline is not None and manifest_bytes != Path(baseline).read_bytes():
        raise ValueError("project.json changed from the independent generation baseline")
    report = json.loads(manifest_bytes)
    if report.get("lock_status") != "normalized offline":
        raise ValueError("project generation/lock normalization is not complete")
    manifests = required_manifests(report)
    entries = report.get("source_file_sha256")
    if not isinstance(entries, dict) or not entries:
        raise ValueError("source file inventory is missing")
    for required in ["Cargo.lock", "Cargo.toml", "rust-toolchain.toml", *manifests]:
        if required not in entries:
            raise ValueError(f"required snapshot file is missing: {required}")
    actual_files = set()
    for directory, dirs, names in os.walk(root, followlinks=False):
        here = Path(directory)
        dirs[:] = [name for name in dirs if name not in ("__pycache__", ".build")]
        if here == root:
            dirs[:] = [name for name in dirs if name not in ("target", ".git")]
        if here == root / "data":
            dirs[:] = [name for name in dirs if name != "sources"]
        for name in dirs + names:
            path = here / name
            relative = path.relative_to(root).as_posix()
            if relative in ("project.json", "generation.log", ".git"):
                continue
            if path.is_symlink():
                raise ValueError(f"snapshot contains a symbolic link: {relative}")
            if path.is_file():
                actual_files.add(relative)
    if actual_files != set(entries):
        raise ValueError(f"snapshot inventory differs: extra={sorted(actual_files - set(entries))}, "
                         f"missing={sorted(set(entries) - actual_files)}")
    for relative, expected in entries.items():
        path = PurePosixPath(relative)
        windows = PureWindowsPath(relative)
        if (not relative or "\\" in relative or path.is_absolute() or windows.drive
                or windows.root or ".." in path.parts or path.as_posix() != relative):
            raise ValueError(f"invalid snapshot path: {relative}")
        actual_path = (root / path).resolve(strict=True)
        if not actual_path.is_relative_to(root):
            raise ValueError(f"snapshot path escapes project: {relative}")
        actual = hashlib.sha256(actual_path.read_bytes()).hexdigest()
        if actual != expected:
            raise ValueError(f"snapshot SHA256 mismatch: {relative}")
    if report.get("cargo_lock_sha256") != entries["Cargo.lock"]:
        raise ValueError("normalized Cargo.lock hashes disagree")
    result = {"source_file_count": len(entries), "source_hash_validation": "passed",
              "cargo_lock_sha256": entries["Cargo.lock"], "chip": report["chip"],
              "target": report["target"], "hardware_validation": "not-run"}
    if report["schema_version"] == 2:
        result.update(topology=report["topology"], images=report["images"])
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("project", type=Path)
    parser.add_argument("--baseline", type=Path,
                        help="independent copy of project.json captured before building")
    args = parser.parse_args()
    print(json.dumps(verify(args.project, args.baseline), indent=2))


if __name__ == "__main__":
    main()
