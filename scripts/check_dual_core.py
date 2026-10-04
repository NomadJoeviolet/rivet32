"""Build independent H7 core images and validate their actual ELF allocation.

Run after scripts/env.ps1. Outputs are under target/dual-core/<chip>/.
No board is connected by this script, and a successful link is never HIL.
"""
import argparse
from contextlib import contextmanager
import hashlib
import json
import os
from pathlib import Path
import shutil
import struct
import subprocess
import sys
import time
import uuid

ROOT = Path(__file__).resolve().parents[1]
TARGET = "thumbv7em-none-eabihf"
CACHE = ROOT / "target/dual-core/cache"


@contextmanager
def cache_lock(path):
    path.parent.mkdir(parents=True, exist_ok=True)
    try:
        descriptor = os.open(path, os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
    except FileExistsError:
        raise RuntimeError(f"dual-core cache lock exists: {path}; no artifacts were read. "
                           "Before removing a stale lock, confirm its owner and all Cargo/rustc descendants have stopped.") from None
    owner = json.dumps(dict(pid=os.getpid(), token=uuid.uuid4().hex, started_unix=time.time())).encode()
    release = True
    try:
        with os.fdopen(descriptor, "wb") as output:
            output.write(owner)
        yield
    except (KeyboardInterrupt, SystemExit):
        # Killing/waiting for Cargo alone need not reap its Windows rustc
        # descendants. Quarantine this cache instead of admitting another build.
        release = False
        print(f"Interrupted dual-core check: retaining {path}; confirm the entire Cargo/rustc "
              "process tree has stopped before manually removing this lock.", file=sys.stderr)
        raise
    finally:
        if release and path.exists() and path.read_bytes() == owner:
            path.unlink()


def within(address, size, region):
    return region["origin"] <= address and address + size <= region["origin"] + region["length"]


def elf_info(path, layout):
    raw = path.read_bytes()
    if len(raw) < 52 or raw[:7] != b"\x7fELF\x01\x01\x01":
        raise ValueError("expected little-endian ELF32")
    header = struct.unpack_from("<HHIIIIIHHHHHH", raw, 16)
    typ, machine, _, entry, phoff, shoff, _, _, phsize, phnum, shsize, shnum, strings_index = header
    if typ != 2 or machine != 40 or phsize != 32 or shsize != 40:
        raise ValueError("expected ARM32 executable with standard headers")
    sections = [struct.unpack_from("<10I", raw, shoff + i * shsize) for i in range(shnum)]
    strings = sections[strings_index]
    names = raw[strings[4]:strings[4] + strings[5]]
    def name_at(table, offset):
        return table[offset:table.index(0, offset)].decode("utf-8")
    named = {name_at(names, s[0]): s for s in sections}
    flash, ram, shared = (layout[k] for k in ("flash", "ram", "shared"))
    vector = named[".vector_table"]
    if vector[3] != flash["origin"] or vector[5] < 8:
        raise ValueError("vector table is outside this core's Flash bank")
    sp, reset = struct.unpack_from("<II", raw, vector[4])
    if sp % 8 or not (ram["origin"] < sp <= ram["origin"] + ram["length"]):
        raise ValueError("initial stack is outside this core's private RAM")
    if not reset & 1 or not within(reset & ~1, 2, flash) or entry != reset:
        raise ValueError("invalid Thumb reset/entry or reset in the peer bank")
    shared_section = named[".embassy_shared"]
    if shared_section[1] != 8 or shared_section[3] != shared["origin"] or not 0 < shared_section[5] <= shared["length"]:
        raise ValueError("SharedData must be fixed-address NOBITS/NOLOAD")
    segments = []
    for index in range(phnum):
        kind, offset, vaddr, paddr, filesz, memsz, flags, align = struct.unpack_from("<8I", raw, phoff + index * phsize)
        if kind != 1 or memsz == 0:
            continue
        if not any(within(vaddr, memsz, r) for r in (flash, ram, shared)):
            raise ValueError(f"PT_LOAD escapes this core's allocation: {vaddr:#x}+{memsz:#x}")
        if filesz and not within(paddr, filesz, flash):
            raise ValueError("load bytes would overwrite peer/shared RAM")
        segments.append(dict(vaddr=vaddr, paddr=paddr, file_size=filesz, memory_size=memsz, flags=flags))
    symbols = {}
    for section in sections:
        if section[1] != 2:
            continue
        table = sections[section[6]]
        strings = raw[table[4]:table[4] + table[5]]
        for offset in range(section[4], section[4] + section[5], section[9]):
            name, value, size, _, _, _ = struct.unpack_from("<IIIBBH", raw, offset)
            symbols[name_at(strings, name)] = (value, size)
    if symbols["__embodied_shared_data"][0] != shared["origin"]:
        raise ValueError("HAL SharedData does not start at the fixed shared address")
    if symbols[layout["time_driver"]][0] == symbols["DefaultHandler"][0]:
        raise ValueError("assigned time driver IRQ is not installed")
    return dict(entry=entry, initial_sp=sp, shared_address=shared_section[3], shared_size=shared_section[5],
                time_driver=layout["time_driver"], segments=segments, sha256=hashlib.sha256(raw).hexdigest())


def build(chip):
    directory = ROOT / "target/dual-core" / chip
    directory.mkdir(parents=True, exist_ok=True)
    report = directory / "result.json"
    report.unlink(missing_ok=True)
    map_path = directory / "minimal.map"
    map_path.unlink(missing_ok=True)
    started = time.time()
    command = ["cargo", "rustc", "--locked", "--offline", "--release", "-p", "embodied-app", "--bin", "minimal",
               "--features", chip, "--target", TARGET, "--target-dir", str(CACHE), "--message-format=json", "--", f"-Clink-arg=-Map={map_path}"]
    run = subprocess.run(command, cwd=ROOT, text=True, encoding="utf-8", stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    (directory / "cargo.stdout.jsonl").write_text(run.stdout, encoding="utf-8")
    (directory / "cargo.stderr.log").write_text(run.stderr, encoding="utf-8")
    result = dict(chip=chip, command=command, started_unix=started, finished_unix=time.time(), cargo_exit_code=run.returncode,
                  hil="not_run", status="failed")
    try:
        if run.returncode:
            raise RuntimeError(run.stderr[-4000:])
        messages = [json.loads(line) for line in run.stdout.splitlines() if line.startswith("{")]
        apps = [m for m in messages if m.get("reason") == "build-script-executed" and "embodied-app" in m["package_id"]]
        if len(apps) != 1 or "embodied_dual_core" not in apps[0]["cfgs"]:
            raise ValueError("missing exact App dual-core build-script output")
        out = Path(apps[0]["out_dir"])
        layout = json.loads((out / "dual-core-layout.json").read_text(encoding="utf-8"))
        if layout["chip"] != chip:
            raise ValueError("stale or mismatched layout")
        artifacts = [m for m in messages if m.get("reason") == "compiler-artifact" and m.get("executable") and m["target"]["name"] == "minimal"]
        if len(artifacts) != 1:
            raise ValueError("missing exact minimal artifact")
        elf = directory / "minimal.elf"
        shutil.copyfile(artifacts[0]["executable"], elf)
        for filename in ("memory.x", "dual-shared.x", "dual-core-layout.json"):
            shutil.copyfile(out / filename, directory / filename)
        result.update(layout=layout, elf=elf_info(elf, layout), map_sha256=hashlib.sha256(map_path.read_bytes()).hexdigest(),
                      status="link_and_elf_passed")
    except Exception as error:
        result["error"] = str(error)
    report.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(chip, result["status"], result.get("error", ""), flush=True)
    return result


def inspect(chip):
    """Validate copied, hash-matched xtask artifacts without starting Cargo."""
    reports = ROOT / "target/reports" / chip
    report_path = reports / "result.json"
    report_bytes = report_path.read_bytes()
    upstream = json.loads(report_bytes)
    if upstream["chip"]["feature"] != chip or any(upstream[stage]["status"] != "passed" for stage in ("compile", "link", "elf_validation")):
        raise ValueError(f"{chip}: xtask stages did not all pass")
    messages = [json.loads(line) for line in (reports / "link.stdout.jsonl").read_text(encoding="utf-8").splitlines() if line.startswith("{")]
    apps = [m for m in messages if m.get("reason") == "build-script-executed" and "embodied-app" in m["package_id"]]
    if len(apps) != 1 or "embodied_dual_core" not in apps[0]["cfgs"]:
        raise ValueError("missing exact dual-core App build-script output")
    layout = json.loads((Path(apps[0]["out_dir"]) / "dual-core-layout.json").read_text(encoding="utf-8"))
    if layout["chip"] != chip:
        raise ValueError("mismatched layout")
    directory = ROOT / "target/dual-core/inspected" / chip
    directory.mkdir(parents=True, exist_ok=True)
    (directory / "result.json").unlink(missing_ok=True)
    for kind, extension in (("elf", "elf"), ("map", "map")):
        data = Path(upstream["artifacts"][kind]).read_bytes()
        if hashlib.sha256(data).hexdigest() != upstream["artifacts"][kind + "_sha256"]:
            raise ValueError(f"{chip}: current {kind} differs from the xtask report")
        (directory / ("minimal." + extension)).write_bytes(data)
    if report_path.read_bytes() != report_bytes:
        raise ValueError("xtask report changed during artifact snapshot")
    result = dict(chip=chip, status="link_and_elf_passed", hil="not_run", layout=layout,
                  elf=elf_info(directory / "minimal.elf", layout),
                  xtask_report_sha256=hashlib.sha256(report_bytes).hexdigest(), xtask_report=str(report_path))
    (directory / "result.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(chip, "inspected", result["status"], flush=True)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("parts", nargs="*", help="unsuffixed real H745/H747/H755/H757 parts; default: stm32h747xi")
    parser.add_argument("--inspect-only", action="store_true", help="inspect hash-matched existing xtask artifacts; do not invoke Cargo")
    parser.add_argument("--all", action="store_true", help="check every real H7 dual-core catalogue pair")
    args = parser.parse_args()
    catalogue = {build["feature"] for build in json.loads((ROOT / "data/chips.json").read_text(encoding="utf-8"))["builds"]}
    if args.all:
        if args.parts:
            parser.error("--all cannot be combined with an explicit part list")
        args.parts = sorted(feature.rsplit("-", 1)[0] for feature in catalogue if feature.endswith("-cm7") and feature.startswith(("stm32h745", "stm32h747", "stm32h755", "stm32h757")))
    elif not args.parts:
        args.parts = ["stm32h747xi"]
    for part in args.parts:
        if not part.startswith(("stm32h745", "stm32h747", "stm32h755", "stm32h757")) or any(part + suffix not in catalogue for suffix in ("-cm7", "-cm4")):
            parser.error(f"unknown dual-core part {part}")
    all_results = []
    with cache_lock(CACHE / ".dual-core.lock"):
        for part in args.parts:
            operation = inspect if args.inspect_only else build
            primary, secondary = (operation(part + suffix) for suffix in ("-cm7", "-cm4"))
            if primary["status"] == secondary["status"] == "link_and_elf_passed":
                if (primary["elf"]["shared_address"], primary["elf"]["shared_size"]) != (secondary["elf"]["shared_address"], secondary["elf"]["shared_size"]):
                    raise ValueError(f"{part}: incompatible SharedData location/size")
                if primary["elf"]["time_driver"] == secondary["elf"]["time_driver"]:
                    raise ValueError(f"{part}: both cores use same time driver")
            all_results.extend([primary, secondary])
    if any(result["status"] != "link_and_elf_passed" for result in all_results):
        raise SystemExit(1)


if __name__ == "__main__":
    main()
