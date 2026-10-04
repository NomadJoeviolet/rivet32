"""Create a release bundle using the LLVM tools from the selected rustc sysroot."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import tomllib
import zipfile

ROOT = Path(__file__).resolve().parents[1]


def output(*args):
    return subprocess.check_output(args, cwd=ROOT, text=True).strip()


def check_tag():
    version = tomllib.loads((ROOT / "Cargo.toml").read_text())["workspace"]["package"]["version"]
    tag = os.environ.get("RELEASE_TAG", "")
    if tag != "v" + version:
        raise ValueError(f"Release tag {tag!r} must equal workspace version v{version}")
    return tag


def validate_artifacts(chip, commit, stage=None):
    source = stage if stage is not None else ROOT / "target/artifacts" / chip
    report = stage / "result.json" if stage is not None else ROOT / "target/reports" / chip / "result.json"
    elf, link_map = source / "minimal.elf", source / "minimal.map"
    for path in (elf, link_map, report):
        if not path.is_file():
            raise FileNotFoundError(path)
    result = json.loads(report.read_text(encoding="utf-8"))
    if result["chip"]["feature"] != chip:
        raise ValueError("Build report chip does not match requested artifact")
    for step in ("compile", "link", "elf_validation"):
        if result[step]["status"] != "passed":
            raise ValueError(f"Build report has no successful {step}")
    head = result["git"]["head"]
    if (result["git"]["dirty"] is not False or result["git"]["status"]["exit_code"] != 0
            or head["exit_code"] != 0 or head["stdout"].strip() != commit):
        raise ValueError("Firmware must come from this exact, clean source commit")
    expected = ((elf, result["artifacts"]["elf_sha256"]),
                (link_map, result["artifacts"]["map_sha256"]),
                ((stage if stage is not None else ROOT) / "Cargo.lock", result["cargo_lock_sha256"]))
    for path, digest in expected:
        if hashlib.sha256(path.read_bytes()).hexdigest() != digest:
            raise ValueError(f"Artifact differs from successful build report: {path.name}")
    return [elf, link_map, report]


def package(chip):
    tag = check_tag()
    # Validate identity before using it in any output path.
    catalog = json.loads((ROOT / "data/chips.json").read_text(encoding="utf-8"))
    selected = next((row for row in catalog["builds"] if row["feature"] == chip), None)
    if selected is None:
        raise ValueError(f"Unknown chip/core: {chip}")
    if selected.get("exclusion_reason") is not None:
        raise ValueError(f"Chip/core {chip} is excluded from support: {selected['exclusion_reason']}")
    commit = output("git", "rev-parse", "HEAD")
    if output("git", "status", "--porcelain", "--untracked-files=normal"):
        raise ValueError("Release packaging requires a clean, committed source tree")
    files = validate_artifacts(chip, commit)
    sysroot = Path(output("rustc", "--print", "sysroot"))
    host = next(line.removeprefix("host: ") for line in output("rustc", "-Vv").splitlines() if line.startswith("host: "))
    objcopy = sysroot / "lib/rustlib" / host / "bin" / ("llvm-objcopy.exe" if os.name == "nt" else "llvm-objcopy")
    if not objcopy.is_file():
        raise FileNotFoundError("Run rustup component add llvm-tools --toolchain 1.98.1")
    destination = ROOT / "dist"
    destination.mkdir(exist_ok=True)
    archive_path = destination / f"embodied-{tag}-{chip}.zip"
    # A new directory on every invocation excludes stale output by construction.
    with tempfile.TemporaryDirectory(prefix="release-", dir=ROOT / "target") as temporary:
        stage = Path(temporary)
        for item in files:
            shutil.copy2(item, stage / item.name)
        for name in ("Cargo.lock", "rust-toolchain.toml", "LICENSE", "THIRD_PARTY_NOTICES.md"):
            shutil.copy2(ROOT / name, stage / name)
        # Revalidate the private snapshot, then derive every format from it.
        # A parallel build is free to replace the original files after copying.
        validate_artifacts(chip, commit, stage)
        for fmt, suffix in (("binary", "bin"), ("ihex", "hex")):
            subprocess.run([str(objcopy), "-O", fmt, str(stage / "minimal.elf"), str(stage / f"minimal.{suffix}")], check=True)
        (stage / "rustc.txt").write_text(output("rustc", "-Vv") + "\n", encoding="utf-8")
        (stage / "source.txt").write_text(commit + "\n", encoding="utf-8")
        staged = sorted(p for p in stage.iterdir() if p.is_file())
        sums = "".join(f"{hashlib.sha256(p.read_bytes()).hexdigest()}  {p.name}\n" for p in staged)
        (stage / "SHA256SUMS").write_text(sums, encoding="utf-8")
        with zipfile.ZipFile(archive_path, "w", zipfile.ZIP_DEFLATED) as archive:
            for item in sorted(stage.iterdir()):
                archive.write(item, item.name)
    digest = hashlib.sha256(archive_path.read_bytes()).hexdigest()
    archive_path.with_suffix(".zip.sha256").write_text(f"{digest}  {archive_path.name}\n", encoding="utf-8")
    print(archive_path)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check-tag", action="store_true")
    parser.add_argument("--chip")
    args = parser.parse_args()
    if args.check_tag:
        print(check_tag())
    elif args.chip:
        package(args.chip)
    else:
        parser.error("Specify --check-tag or --chip")
