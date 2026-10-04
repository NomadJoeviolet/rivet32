"""Build the two packages separately and validate this invocation's exact ELF pair."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import time
import uuid

from check_dual_core import cache_lock, elf_info

ROOT = Path(__file__).resolve().parents[1]


def validate_project(project):
    part = project.get("part", "")
    if project.get("topology") != "dual-core-pair" or not re.fullmatch(r"stm32h7(?:45|47|55|57)[a-z][gi]", part):
        raise ValueError("expected an exact supported H7 dual-core pair")
    images = project.get("images", [])
    if len(images) != 2:
        raise ValueError("pair requires exactly two images")
    for image, core, timer in zip(images, ("cm7", "cm4"), ("TIM5", "TIM2")):
        if any(image.get(key) != value for key, value in dict(core=core, chip=part + "-" + core,
               manifest=f"App/{core}/Cargo.toml", binary=f"firmware-{core}",
               target="thumbv7em-none-eabihf", time_driver=timer).items()):
            raise ValueError(f"inconsistent {core} image identity or resource assignment")
        if not re.fullmatch(r"[a-z][a-z0-9_-]*", image.get("package", "")):
            raise ValueError("invalid package name")
    if images[0]["package"] == images[1]["package"]:
        raise ValueError("the two images require separate packages")
    return images


def cargo_command(root, image, check):
    command = [os.environ.get("CARGO", "cargo"), "check" if check else "rustc", "--locked", "--offline",
               "--release", "-p", image["package"], "--bin", image["binary"],
               "--target", image["target"], "--target-dir", str(root / "target/pair-cache" / image["core"]),
               "--message-format=json"]
    if not check:
        # The unique link argument reruns the final linker even when Cargo could
        # otherwise return a Fresh ELF after the prior report/map was removed.
        command += ["--", "-Clink-arg=-Map=" + str(root / "target/pair" / image["core"] / ("link-" + uuid.uuid4().hex + ".map"))]
    return command


def overlap(left, right):
    return left["origin"] < right["origin"] + right["length"] and right["origin"] < left["origin"] + left["length"]


def validate_pair(primary, secondary):
    layouts = [primary["layout"], secondary["layout"]]
    if layouts[0]["shared"] != layouts[1]["shared"] or any(primary["elf"][key] != secondary["elf"][key]
                                                        for key in ("shared_address", "shared_size")):
        raise ValueError("incompatible shared region or SharedData ABI")
    if [(layout["core"], layout["time_driver"]) for layout in layouts] != [("cm7", "TIM5"), ("cm4", "TIM2")]:
        raise ValueError("incorrect core/time-driver ownership")
    private = [layout[kind] for layout in layouts for kind in ("flash", "ram")]
    for index, region in enumerate(private):
        if region["length"] <= 0 or overlap(region, layouts[0]["shared"]) or any(overlap(region, other) for other in private[index + 1:]):
            raise ValueError("overlapping private/shared image allocations")


def collect_image(root, image, messages, directory):
    expected_manifest = (root / image["manifest"]).resolve()
    artifacts = [m for m in messages if m.get("reason") == "compiler-artifact"
                 and m.get("executable") and m.get("target", {}).get("name") == image["binary"]
                 and Path(m.get("manifest_path", "")).resolve() == expected_manifest]
    if len(artifacts) != 1:
        raise ValueError("missing exact package/binary ELF artifact")
    artifact = artifacts[0]
    scripts = [m for m in messages if m.get("reason") == "build-script-executed"
               and m.get("package_id") == artifact["package_id"]]
    if len(scripts) != 1 or not {"embodied_dual_core", "embodied_core_" + image["core"]}.issubset(scripts[0]["cfgs"]):
        raise ValueError("missing exact core build-script output")
    output = Path(scripts[0]["out_dir"])
    layout = json.loads((output / "dual-core-layout.json").read_text(encoding="utf-8"))
    for key in ("chip", "core", "time_driver"):
        if layout.get(key) != image[key]:
            raise ValueError("stale/mismatched layout " + key)
    elf = directory / "firmware.elf"
    shutil.copyfile(artifact["executable"], elf)
    for filename in ("memory.x", "dual-shared.x", "dual-core-layout.json"):
        shutil.copyfile(output / filename, directory / filename)
    checked = elf_info(elf, layout)
    return dict(layout=layout, elf=checked, status="link_and_elf_passed", hil="not_run",
                map_sha256=hashlib.sha256((directory / "firmware.map").read_bytes()).hexdigest())


def build_image(root, image, check):
    directory = root / "target/pair" / image["core"]
    directory.mkdir(parents=True, exist_ok=True)
    # A failed build must not leave old linked bytes looking like a fresh image.
    if not check:
        for filename in ("firmware.elf", "firmware.map", "memory.x", "dual-shared.x", "dual-core-layout.json"):
            (directory / filename).unlink(missing_ok=True)
    command = cargo_command(root, image, check)
    run = subprocess.run(command, cwd=root, text=True, encoding="utf-8", stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    prefix = "check" if check else "build"
    (directory / (prefix + ".stdout.jsonl")).write_text(run.stdout, encoding="utf-8")
    (directory / (prefix + ".stderr.log")).write_text(run.stderr, encoding="utf-8")
    if run.returncode:
        raise RuntimeError(f"{image['chip']}: Cargo failed ({run.returncode}); see {directory}\n{run.stderr[-4000:]}")
    if check:
        return dict(chip=image["chip"], status="compile_checked", command=command, hil="not_run")
    messages = [json.loads(line) for line in run.stdout.splitlines() if line.startswith("{")]
    map_path = Path(command[-1].removeprefix("-Clink-arg=-Map="))
    try:
        shutil.copyfile(map_path, directory / "firmware.map")
        return dict(chip=image["chip"], command=command, **collect_image(root, image, messages, directory))
    finally:
        map_path.unlink(missing_ok=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="check each package without claiming linked-pair validation")
    args = parser.parse_args()
    directory = ROOT / "target/pair"
    with cache_lock(directory / ".pair.lock"):
        source = (ROOT / "project.json").read_bytes()
        lockfile = (ROOT / "Cargo.lock").read_bytes()
        images = validate_project(json.loads(source))
        report = directory / ("check-result.json" if args.check else "result.json")
        result = dict(status="failed", hil="not_run", started_unix=time.time(), images=[],
                      project_sha256=hashlib.sha256(source).hexdigest(),
                      cargo_lock_sha256=hashlib.sha256(lockfile).hexdigest())
        report.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
        try:
            for image in images:
                result["images"].append(build_image(ROOT, image, args.check))
            if not args.check:
                validate_pair(*result["images"])
            if source != (ROOT / "project.json").read_bytes() or lockfile != (ROOT / "Cargo.lock").read_bytes():
                raise ValueError("project metadata or Cargo.lock changed while building the pair")
            result["status"] = "compile_checked" if args.check else "link_and_elf_passed"
        except BaseException as error:
            result["error"] = str(error) or type(error).__name__
            raise
        finally:
            result["finished_unix"] = time.time()
            report.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
        print(result["status"], report)


if __name__ == "__main__":
    main()
