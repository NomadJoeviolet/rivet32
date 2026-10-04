"""Fail-closed source inventory and pinned generator provenance."""
import hashlib
import gzip
import json
import os
from pathlib import Path
import subprocess
import urllib.request
import zipfile

HERE=Path(__file__).resolve().parent
FROZEN_INPUTS={"evidence.json","register-inputs/STM32H543.json","register-inputs/STM32H553.json","sources.json","sources/stm32h543xx.h","sources/stm32h553xx.h","sources/stm32h563xx.h","sources/stm32h5xx_hal_flash.h","sources/stm32h5xx_hal_flash_ex.h"}


def digest(data):return hashlib.sha256(data).hexdigest()


def workspace():
    root=next((p for p in HERE.parents if (p/"data/patches/stm32h5-47c/evidence.json").is_file()),None)
    if root is None:raise ValueError("Workspace with frozen DIE47C evidence not found; supply --workspace")
    return root


def verify(root, directory=HERE):
    lock=json.loads((directory/"sources.json").read_text(encoding="utf-8"))
    if set(lock["frozen_inputs"])!=FROZEN_INPUTS:raise ValueError("Frozen input inventory differs from required inputs")
    files={p.relative_to(directory/"sources").as_posix() for p in (directory/"sources").rglob("*") if p.is_file()}
    if files!=set(lock["files"]):raise ValueError("Source inventory differs from lock: "+repr(files ^ set(lock["files"])))
    for name,source in lock["files"].items():
        raw=(directory/"sources"/name).read_bytes()
        if digest(raw)!=source["sha256"]:raise ValueError("Source SHA mismatch: "+name)
        if "uncompressed_sha256" in source and digest(gzip.decompress(raw))!=source["uncompressed_sha256"]:raise ValueError("Uncompressed SVD SHA mismatch: "+name)
    for name,expected in lock["frozen_inputs"].items():
        if digest((root/"data/patches/stm32h5-47c"/name).read_bytes())!=expected:raise ValueError("Frozen input SHA mismatch: "+name)
    return lock


def generator(root, rebuild=False):
    lock=verify(root)["generator"]
    old=root/"target/h5-li-generator/debug"/("stm32-metapac-gen.exe" if os.name=="nt" else "stm32-metapac-gen")
    if not rebuild and old.is_file() and digest(old.read_bytes())==lock["observed_windows_executable_sha256"]:
        return old,{"method":"verified previously built binary","revision":lock["revision"],"archive_sha256":lock["archive_sha256"],"binary_sha256":digest(old.read_bytes())}
    archive=root/"data/sources/stm32-gaps/stm32-data-baseline.zip"
    if not archive.is_file():
        archive.parent.mkdir(parents=True,exist_ok=True)
        with urllib.request.urlopen(lock["url"],timeout=60) as response:raw=response.read()
        if digest(raw)!=lock["archive_sha256"]:raise ValueError("Downloaded generator archive SHA mismatch")
        archive.write_bytes(raw)
    if digest(archive.read_bytes())!=lock["archive_sha256"]:raise ValueError("Generator archive SHA mismatch")
    destination=(root/"target/h5-47c-generator-source").resolve()
    with zipfile.ZipFile(archive) as z:
        for entry in z.infolist():
            path=(destination/entry.filename).resolve()
            if not path.is_relative_to(destination):raise ValueError("Unsafe generator archive path")
            if entry.is_dir():path.mkdir(parents=True,exist_ok=True)
            else:path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(z.read(entry))
    source=destination/("stm32-data-"+lock["revision"])
    target=root/"target/h5-47c-generator"
    command=["cargo","+1.98.1","build","--manifest-path",str(source/"Cargo.toml"),"-p","stm32-metapac-gen","--locked","--target-dir",str(target)]
    subprocess.run(command,check=True,cwd=root)
    binary=target/"debug"/("stm32-metapac-gen.exe" if os.name=="nt" else "stm32-metapac-gen")
    return binary,{"method":"built unmodified verified archive","command":command,"revision":lock["revision"],"archive_sha256":lock["archive_sha256"],"binary_sha256":digest(binary.read_bytes())}
