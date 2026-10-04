"""Build real, exact-chip/package peripheral constructors and inspect ARM ELFs.

Run after scripts/env.ps1 (or env.sh). Example:
  python xtask/scripts/peripheral_matrix.py --chip stm32f103c8
  python xtask/scripts/peripheral_matrix.py --family F0
Reports always contain all seven categories. Passing means constructor-linked,
not a transfer, electrical, clock-tree, board, or hardware-in-the-loop test.
"""
import argparse
from contextlib import ExitStack, contextmanager
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import struct
import subprocess
import sys
import time
import uuid

import generate_hal_metadata as provenance
import cache_retention
from peripheral_recipes import CATEGORIES, DEFAULT_CLOCK_SOURCE, plan

ROOT = Path(__file__).resolve().parents[2]
REV = "ae9e6f0672af84cec8e200a94c574041844396f0"
CARGO = os.environ.get("CARGO", "cargo")


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git_blob(raw):
    return hashlib.sha1(b"blob " + str(len(raw)).encode() + b"\0" + raw).hexdigest()


def source_snapshot(paths):
    return {str(p.resolve()): sha(p) for p in paths}


def ensure_snapshot(saved):
    for name, expected in saved.items():
        if not Path(name).is_file() or sha(Path(name)) != expected:
            raise ValueError(f"source changed during probe build: {name}")


def bound_bytes(path, saved):
    """Read for parsing without silently replacing the batch's first digest."""
    raw = path.read_bytes()
    if hashlib.sha256(raw).hexdigest() != saved[str(path.resolve())]:
        raise ValueError(f"source changed before it could be consumed: {path}")
    return raw


def bind_input(path, saved):
    name = str(path.resolve())
    if name not in saved:
        saved[name] = sha(path)
    return bound_bytes(path, saved)


def tree_snapshot(roots):
    return {str(root.resolve()): {p.relative_to(root).as_posix(): sha(p) for p in sorted(root.rglob("*"))
                                 if p.is_file() and not set(p.relative_to(root).parts) & {"target", ".git", "__pycache__", ".build"}}
            for root in roots}


def ensure_trees(saved):
    if tree_snapshot([Path(name) for name in saved]) != saved:
        raise ValueError("source tree inventory or bytes changed during probe build")


@contextmanager
def cache_lock(path):
    path.parent.mkdir(parents=True, exist_ok=True)
    owner = json.dumps(dict(pid=os.getpid(), token=uuid.uuid4().hex, started_unix=time.time())).encode()
    with path.open("xb") as handle:
        handle.write(owner)
    release = True
    try:
        yield
    except KeyboardInterrupt:
        release = False
        print(f"Interrupted; retained {path}. Confirm all Cargo/rustc children stopped before removing.", file=sys.stderr)
        raise
    finally:
        if release and path.exists() and path.read_bytes() == owner:
            path.unlink()


def select_chips(rows, chip, family, limit, shard=None):
    if chip is not None:
        for row in rows:
            if row["feature"] == chip and row.get("exclusion_reason") is not None:
                raise ValueError(f"{chip} is excluded from framework support: {row['exclusion_reason']}")
    selected = sorted([r for r in rows if r.get("exclusion_reason") is None and (chip is None or r["feature"] == chip)
                       and (family is None or r["family"].upper().removeprefix("STM32") == family.upper().removeprefix("STM32"))],
                      key=lambda r: r["feature"])
    if not selected:
        raise ValueError("unknown or empty exact chip/core or family selection")
    if shard is not None:
        if chip is not None or not re.fullmatch(r"\d+/\d+", shard):
            raise ValueError("--shard requires index/count and cannot accompany --chip")
        index, count = map(int, shard.split("/"))
        if not 0 <= index < count:
            raise ValueError("--shard requires 0 <= index < count")
        selected = [row for ordinal, row in enumerate(selected) if ordinal % count == index]
        if not selected:
            raise ValueError("empty shard: select a smaller shard count or another index")
    return selected[:limit] if limit else selected


def dual_pair(chip, rows):
    feature = chip["feature"]
    if not feature.endswith(("-cm7", "-cm4")):
        return None
    part = feature.rsplit("-", 1)[0]
    if not re.fullmatch(r"stm32h7(?:45|47|55|57)[a-z][gi]", part):
        raise ValueError("no audited paired startup for " + feature)
    pair = []
    for core in ("cm7", "cm4"):
        candidates = [r for r in rows if r["feature"] == part + "-" + core]
        if len(candidates) != 1 or candidates[0]["target"] != "thumbv7em-none-eabihf" or not (
                candidates[0]["embassy_feature_available"] or candidates[0].get("local_patch_available")):
            raise ValueError("missing/unavailable exact peer or inconsistent target: " + part + "-" + core)
        pair.append(candidates[0])
    return pair


def collect_artifact(messages, project, category, hal_id):
    artifacts = [m for m in messages if m.get("reason") == "compiler-artifact" and m.get("executable")
                 and Path(m.get("manifest_path", "")).resolve() == (project / "Cargo.toml").resolve()
                 and m.get("target", {}).get("name") == "probe-" + category]
    scripts = [m for m in messages if m.get("reason") == "build-script-executed" and m.get("package_id") == hal_id]
    if len(artifacts) != 1 or len(scripts) != 1:
        raise ValueError("missing or ambiguous exact probe artifact / HAL memory build-script identity")
    return Path(artifacts[0]["executable"]), Path(scripts[0]["out_dir"]) / "memory.x"


def collect_dual_artifact(messages, project, category, core):
    artifacts = [m for m in messages if m.get("reason") == "compiler-artifact" and m.get("executable")
                 and Path(m.get("manifest_path", "")).resolve() == (project / "Cargo.toml").resolve()
                 and m.get("target", {}).get("name") == "probe-" + category]
    if len(artifacts) != 1:
        raise ValueError("missing/ambiguous exact dual probe artifact")
    scripts = [m for m in messages if m.get("reason") == "build-script-executed"
               and m.get("package_id") == artifacts[0]["package_id"]]
    if len(scripts) != 1 or not {"embodied_dual_core", "embodied_core_" + core}.issubset(scripts[0].get("cfgs", [])):
        raise ValueError("missing exact package/core dual probe layout")
    return Path(artifacts[0]["executable"]), Path(scripts[0]["out_dir"])


def dual_checks():
    # Load the source-bound App ELF checker and established paired-project
    # allocation checker, without invoking either file's standalone CLI.
    def load(name, path):
        spec = importlib.util.spec_from_file_location(name, path)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        return module
    checker = load("peripheral_dual_elf", ROOT / "scripts/check_dual_core.py")
    previous = sys.modules.get("check_dual_core")
    sys.modules["check_dual_core"] = checker
    try:
        pair = load("peripheral_dual_pair", ROOT / "xtask/templates/dual/build_pair.py")
    finally:
        if previous is None:
            del sys.modules["check_dual_core"]
        else:
            sys.modules["check_dual_core"] = previous
    return checker.elf_info, pair.validate_pair


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + ".new")
    temporary.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")
    temporary.replace(path)


def run(command, cwd, log):
    env = os.environ.copy()
    # An unrelated host's RUSTFLAGS must not alter the link contract. The local
    # scalar .cargo config supplies the two pinned linker scripts on ARM only.
    env.pop("RUSTFLAGS", None)
    env.pop("CARGO_ENCODED_RUSTFLAGS", None)
    env["DEFMT_LOG"] = "off"
    process = subprocess.run(command, cwd=cwd, env=env, capture_output=True, text=True, encoding="utf-8", errors="replace")
    log.parent.mkdir(parents=True, exist_ok=True)
    log.with_suffix(".stdout").write_text(process.stdout, encoding="utf-8")
    log.with_suffix(".stderr").write_text(process.stderr, encoding="utf-8")
    if process.returncode:
        raise RuntimeError(f"exit {process.returncode}: {' '.join(command)}; logs {log}.stdout/.stderr\n{process.stderr[-1500:]}")
    return process.stdout


def patches():
    return (f'[patch."https://github.com/embassy-rs/stm32-data-generated"]\nstm32-metapac = {{ path = {json.dumps((ROOT / "vendor/stm32-metapac").as_posix())} }}\n'
            f'[patch."https://github.com/embassy-rs/embassy"]\nembassy-stm32 = {{ path = {json.dumps((ROOT / "vendor/embassy-stm32").as_posix())} }}\n')


def normalize(project, log):
    shutil.copyfile(ROOT / "Cargo.lock", project / "Cargo.lock")
    return json.loads(run([CARGO, "metadata", "--format-version", "1", "--offline"], project, log))


def prepare_project(directory, chip, recipes, bank):
    directory.mkdir(parents=True, exist_ok=True)
    (directory / "src/bin").mkdir(parents=True, exist_ok=True)
    (directory / ".cargo").mkdir(exist_ok=True)
    features = [chip, "time-driver-" + next(r["time_driver"].lower() for r in recipes.values() if r["status"] == "planned")]
    if bank:
        features.append(bank)
    manifest = f'''[workspace]
resolver = "3"
[package]
name = "peripheral-probe-{chip}"
version = "0.0.0"
edition = "2024"
publish = false
autobins = false
[dependencies]
embodied-stm32 = {{ path = {json.dumps((ROOT / "crates/embodied-stm32").as_posix())}, features = {json.dumps(features)} }}
embodied-core = {{ path = {json.dumps((ROOT / "crates/embodied-core").as_posix())} }}
embassy-time = {{ git = "https://github.com/embassy-rs/embassy", rev = "{REV}", features = ["tick-hz-32_768"] }}
embassy-usb = {{ git = "https://github.com/embassy-rs/embassy", rev = "{REV}", optional = true }}
cortex-m = {{ version = "0.7.7", features = ["inline-asm", "critical-section-single-core"] }}
cortex-m-rt = {{ version = "0.7.5", features = ["set-vtor"] }}
defmt-rtt = "1.0.0"
static_cell = {{ version = "2.1", optional = true }}
portable-atomic = {{ version = "1", features = ["critical-section"], optional = true }}
[features]
usb = ["dep:embassy-usb", "embodied-stm32/usb", "dep:static_cell", "dep:portable-atomic"]
[profile.release]
opt-level = "s"
lto = true
codegen-units = 1
debug = 2
panic = "abort"
'''
    for category, recipe in recipes.items():
        if recipe["status"] == "planned":
            manifest += f'\n[[bin]]\nname = "probe-{category}"\npath = "src/bin/{category}.rs"\n'
            if category == "usb_cdc":
                manifest += 'required-features = ["usb"]\n'
            (directory / f"src/bin/{category}.rs").write_text(recipe["source"], encoding="utf-8")
    (directory / "Cargo.toml").write_text(manifest + patches(), encoding="utf-8")
    (directory / ".cargo/config.toml").write_text('[net]\noffline = true\n[target.\'cfg(all(target_arch = "arm", target_os = "none"))\']\nrustflags = "-C link-arg=-Tlink.x -C link-arg=-Tdefmt.x"\n', encoding="utf-8")


def prepare_dual_projects(directory, owner, pair, recipes, bank):
    projects = {}
    for image in pair:
        feature = image["feature"]
        core = feature.rsplit("-", 1)[1]
        project = directory / core
        core_recipes = {}
        for category, recipe in recipes.items():
            if recipe["status"] != "planned":
                continue
            source = recipe["source"] if feature == owner else (
                '#![no_std]\n#![no_main]\nuse defmt_rtt as _;\nuse embodied_stm32::hal;\n'
                + recipe.get("clock_source", DEFAULT_CLOCK_SOURCE) +
                '#[path = "../dual_startup.rs"]\nmod dual_startup;\n'
                '#[panic_handler]\nfn panic(_: &core::panic::PanicInfo) -> ! { loop { cortex_m::asm::wfi(); } }\n'
                f'#[cortex_m_rt::entry]\nfn main() -> ! {{ probe_companion_{category}() }}\n'
                f'#[inline(never)]\n#[unsafe(no_mangle)]\npub fn probe_companion_{category}() -> ! {{\n'
                'let _p = dual_startup::init(); loop { cortex_m::asm::wfi(); }\n}\n')
            core_recipes[category] = dict(recipe, source=source, time_driver={"cm7": "TIM5", "cm4": "TIM2"}[core])
        prepare_project(project, feature, core_recipes, bank)
        path = project / "Cargo.toml"
        manifest = path.read_text(encoding="utf-8").replace('[features]\n', f'[features]\ndefault = ["{feature}"]\n{feature} = []\n')
        manifest += ('\n[build-dependencies]\n'
                     'stm32-metapac = { git = "https://github.com/embassy-rs/stm32-data-generated", '
                     f'tag = "stm32-data-caa36afd62510b0e6315ee0dccd1f9c65fbcac83", default-features = false, features = ["metadata", "{feature}"] }}\n')
        path.write_text(manifest, encoding="utf-8")
        build = (ROOT / "App/build.rs").read_text(encoding="utf-8")
        if build.count("cargo:rustc-link-arg-bin=minimal=") != 1:
            raise ValueError("App dual-core linker contract changed")
        build = re.sub(r'^\s*#\[cfg\(feature = "firmware"\)\]\n', '', build, flags=re.MULTILINE)
        (project / "build.rs").write_text(build.replace("cargo:rustc-link-arg-bin=minimal=", "cargo:rustc-link-arg="), encoding="utf-8")
        shutil.copyfile(ROOT / "App/build_support.rs", project / "build_support.rs")
        shutil.copyfile(ROOT / "xtask/templates/peripherals/dual_startup.rs", project / "src/dual_startup.rs")
        projects[core] = project
    return projects


def inspector(base):
    project = base / "inspector"
    cache = base / "host-cache"
    with cache_lock(base / "locks/host.lock"):
        (project / "src").mkdir(parents=True, exist_ok=True)
        (project / "Cargo.toml").write_text(f'''[workspace]
[package]
name = "peripheral-elf-inspect"
version = "0.0.0"
edition = "2024"
[dependencies]
xtask = {{ path = {json.dumps((ROOT / "xtask").as_posix())} }}
serde_json = "1"
''', encoding="utf-8")
        shutil.copyfile(ROOT / "xtask/templates/peripherals/inspect.rs", project / "src/main.rs")
        normalize(project, project / "metadata")
        output = run([CARGO, "build", "--offline", "--locked", "--target-dir", str(cache), "--message-format=json"], project, project / "build")
        matches = [m for line in output.splitlines() if line.startswith("{") for m in [json.loads(line)]
                   if m.get("reason") == "compiler-artifact" and m.get("executable")
                   and Path(m.get("manifest_path", "")).resolve() == (project / "Cargo.toml").resolve()
                   and m.get("target", {}).get("name") == "peripheral-elf-inspect"]
        if len(matches) != 1:
            raise ValueError("missing exact host ELF inspector artifact")
        # Each invocation owns a private executable; another cache user cannot
        # replace the inspector while this invocation is checking ARM images.
        executable = base / "inspectors" / (uuid.uuid4().hex + (".exe" if os.name == "nt" else ""))
        executable.parent.mkdir(exist_ok=True)
        shutil.copyfile(matches[0]["executable"], executable)
        return executable


def check_constructor(raw, name):
    """Require a nonempty executable function symbol, not a source-only claim."""
    shoff = struct.unpack_from("<I", raw, 32)[0]
    shsize, count = struct.unpack_from("<HH", raw, 46)
    sections = [struct.unpack_from("<10I", raw, shoff + i * shsize) for i in range(count)]
    matches = []
    for section in sections:
        if section[1] != 2:
            continue
        strings = sections[section[6]]
        table = raw[strings[4]:strings[4] + strings[5]]
        for offset in range(section[4], section[4] + section[5], section[9]):
            pos, value, size, info, _, index = struct.unpack_from("<IIIBBH", raw, offset)
            symbol = table[pos:table.find(b"\0", pos)].decode("utf-8", "replace")
            if symbol == name and size and info & 15 == 2 and index < len(sections) and sections[index][2] & 4:
                matches.append(dict(symbol=name, address=value, size=size))
    if len(matches) != 1:
        raise ValueError("constructor marker is absent/ambiguous or not executable in linked ELF")
    return matches[0]


def load_data(checkout, patch, chip):
    filename = chip.upper().split("-")[0] + ".json"
    path = provenance.patch_json_path(patch, filename)
    if path:
        record = provenance.patch_input_record(patch, filename)
        raw = provenance.hash_bound_bytes(path, record["sha256"], "chip JSON")
        source = dict(kind="local-metadata-correction" if filename in patch.get("corrected_chip_jsons", {})
                      else "local-overlay", overlay=record["overlay"])
    else:
        path = checkout / "data/chips" / filename
        raw = path.read_bytes()
        expected = subprocess.check_output(["git", "-C", str(checkout), "rev-parse", provenance.REVISION + ":data/chips/" + filename], text=True).strip()
        if git_blob(raw) != expected:
            raise ValueError("pinned chip JSON differs from its Git object")
        source = dict(kind="pinned-git", revision=provenance.REVISION, git_blob=expected)
    return provenance.identified_chip(raw, filename), dict(source, path=str(path), sha256=hashlib.sha256(raw).hexdigest())


def afio_widths(checkout):
    path = checkout / "data/registers/afio_f1.json"
    expected = subprocess.check_output(["git", "-C", str(checkout), "rev-parse", provenance.REVISION + ":data/registers/afio_f1.json"], text=True).strip()
    if git_blob(path.read_bytes()) != expected:
        raise ValueError("AFIO register widths differ from pinned Git data")
    data = json.loads(path.read_bytes())
    registers = data["block/AFIO"]["items"]
    return {(register["name"], field["name"]): field["bit_size"]
            for register in registers if register.get("fieldset")
            for field in data["fieldset/" + register["fieldset"]]["fields"]}


def build_category(base, executable, project, chip, category, recipe, metadata):
    budget = cache_retention.limit_bytes(os.environ.get("EMBODIED_CACHE_LIMIT_MIB"))
    directory = base / "reports" / chip["feature"] / category
    directory.mkdir(parents=True, exist_ok=True)
    cache = base / "cache" / chip["target"]
    map_path = directory / ("link-" + uuid.uuid4().hex + ".map")
    command = [CARGO, "rustc", "--release", "--locked", "--offline", "--bin", "probe-" + category,
               "--target", chip["target"], "--target-dir", str(cache), "--message-format=json"]
    if category == "usb_cdc":
        command += ["--features", "usb"]
    command += ["--", "-Clink-arg=-Map=" + str(map_path)]
    recipe["command"] = command
    with cache_lock(base / "locks" / (chip["target"] + ".lock")):
        recipe["cache_retention"] = cache_retention.prune_locked(base, cache, chip["target"], budget)
        try:
            output = run(command, project, directory / "cargo")
            messages = [json.loads(line) for line in output.splitlines() if line.startswith("{")]
            hal = [p for p in metadata["packages"] if p["name"] == "embassy-stm32"
                   and Path(p["manifest_path"]).resolve() == (ROOT / "vendor/embassy-stm32/Cargo.toml").resolve()]
            if len(hal) != 1:
                raise ValueError("unexpected HAL dependency identity")
            elf_path, memory_path = collect_artifact(messages, project, category, hal[0]["id"])
            for source, name in [(elf_path, "firmware.elf"), (memory_path, "memory.x"), (map_path, "firmware.map")]:
                shutil.copyfile(source, directory / name)
            inspection = json.loads(run([str(executable), str(directory / "firmware.elf"), str(directory / "memory.x")], project, directory / "elf-inspect"))
            marker = check_constructor((directory / "firmware.elf").read_bytes(), "probe_constructor_" + category)
            return dict(status="constructor-linked", reason="real HAL/framework constructor linked and exact ARM ELF validated; hardware not exercised",
                        elf=inspection, constructor=marker,
                        artifacts={name: dict(path=str(directory / name), sha256=sha(directory / name))
                                   for name in ("firmware.elf", "firmware.map", "memory.x")}, hil="not-run")
        finally:
            map_path.unlink(missing_ok=True)


def build_dual_category(base, executable, projects, pair, owner, category, recipe):
    budget = cache_retention.limit_bytes(os.environ.get("EMBODIED_CACHE_LIMIT_MIB"))
    elf_info, validate_pair = dual_checks()
    images = []
    with cache_lock(base / "locks/dual-cache.lock"):
        for chip in pair:
            core = chip["feature"].rsplit("-", 1)[1]
            project = projects[core]
            directory = base / "reports" / owner / category / core
            directory.mkdir(parents=True, exist_ok=True)
            map_path = directory / ("link-" + uuid.uuid4().hex + ".map")
            command = [CARGO, "rustc", "--release", "--locked", "--offline", "--bin", "probe-" + category,
                       "--target", chip["target"], "--target-dir", str(base / "dual-cache" / core), "--message-format=json"]
            if category == "usb_cdc":
                command += ["--features", "usb"]
            command += ["--", "-Clink-arg=-Map=" + str(map_path)]
            retention = cache_retention.prune_locked(base, base / "dual-cache" / core, chip["target"], budget)
            try:
                output = run(command, project, directory / "cargo")
                messages = [json.loads(line) for line in output.splitlines() if line.startswith("{")]
                elf, out = collect_dual_artifact(messages, project, category, core)
                for source_path, filename in [(elf, "firmware.elf"), (map_path, "firmware.map"),
                                              *((out / name, name) for name in ("memory.x", "dual-shared.x", "dual-core-layout.json"))]:
                    shutil.copyfile(source_path, directory / filename)
                layout = json.loads((directory / "dual-core-layout.json").read_bytes())
                if any(layout.get(key) != value for key, value in dict(chip=chip["feature"], core=core,
                       time_driver={"cm7": "TIM5", "cm4": "TIM2"}[core]).items()):
                    raise ValueError("layout differs from the exact paired core/timebase")
                checked = elf_info(directory / "firmware.elf", layout)
                inspection = json.loads(run([str(executable), str(directory / "firmware.elf"), str(directory / "memory.x")], project, directory / "elf-inspect"))
                role = "constructor-owner" if chip["feature"] == owner else "startup-companion"
                marker = check_constructor((directory / "firmware.elf").read_bytes(),
                                           ("probe_constructor_" if role == "constructor-owner" else "probe_companion_") + category)
                images.append(dict(chip=chip["feature"], role=role, layout=layout, elf=checked, inspection=inspection,
                                   entry_symbol=marker, command=command, cache_retention=retention,
                                   artifacts={name: dict(path=str(directory / name), sha256=sha(directory / name))
                                              for name in ("firmware.elf", "firmware.map", "memory.x", "dual-shared.x", "dual-core-layout.json")}))
            finally:
                map_path.unlink(missing_ok=True)
        validate_pair(*images)
    target = next(image for image in images if image["role"] == "constructor-owner")
    return dict(status="constructor-linked", reason="target-core HAL/framework constructor linked; exact companion ELF and pair layout validated; hardware not exercised",
                constructor=target["entry_symbol"], elf=target["inspection"], artifacts=target["artifacts"], hil="not-run",
                pair=dict(status="link-and-pair-validated", owner=owner, companion_counts_as_constructor=False,
                          startup_barrier="HAL publication bit0; secondary init complete bit1; both-init release bit2", images=images))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    selector = parser.add_mutually_exclusive_group()
    selector.add_argument("--chip")
    selector.add_argument("--family")
    parser.add_argument("--package", help="exact ST order-code pattern from chip JSON; defaults to lexically first real package")
    parser.add_argument("--limit", type=int)
    parser.add_argument("--shard", help="zero-based index/count after family sort and before limit")
    parser.add_argument("--plan-only", action="store_true")
    parser.add_argument("--output", type=Path, default=ROOT / "target/peripheral-probes")
    args = parser.parse_args()
    if args.limit is not None and args.limit <= 0:
        parser.error("--limit must be positive")
    if args.package and not args.chip:
        parser.error("--package requires --chip")
    base = args.output.resolve()
    base.mkdir(parents=True, exist_ok=True)
    selection = (args.chip or (args.family or "all").lower()) + ("-shard-" + args.shard.replace("/", "-of-") if args.shard else "")
    summary_path = base / ("summary-" + selection + ".json")
    summary = dict(schema_version=1, selection=vars(args) | {"output": str(base)}, status="incomplete", chips=[])
    saved, batch_trees, records = {}, {}, []
    executable = None

    def checkpoint():
        ensure_snapshot(saved)
        ensure_trees(batch_trees)

    def finish(status, error=None):
        summary.update(status=status, source_sha256=dict(saved), finished_unix=time.time())
        if error is not None:
            summary["error"] = str(error) or type(error).__name__
        summary["chips"] = [dict(chip=result["chip"], status=result["status"], report=str(path), sha256=sha(path))
                            for path, result in records]
        write_json(summary_path, summary)

    # One summary and all selected per-chip report locks remain owned until
    # batch source validation finishes, including failure-report publication.
    with cache_lock(base / "locks" / ("summary-" + selection + ".lock")):
        write_json(summary_path, summary)
        try:
            # Capture trees BEFORE parsing the catalogue or running any source
            # verifier, metadata resolver, host inspector, or ARM compiler.
            batch_trees = tree_snapshot([ROOT / "vendor/embassy-stm32", ROOT / "vendor/stm32-metapac",
                                         ROOT / "data/patches", ROOT / "crates", ROOT / "xtask/src",
                                         ROOT / "xtask/templates", ROOT / "xtask/scripts", ROOT / "App"])
            paths = [ROOT / "Cargo.toml", ROOT / "Cargo.lock", ROOT / ".cargo/config.toml", ROOT / "rust-toolchain.toml",
                     ROOT / "xtask/Cargo.toml", ROOT / "data/chips.json", ROOT / "data/patches/stm32-metapac/vendor-manifest.json",
                     ROOT / "xtask/scripts/generate_hal_metadata.py", ROOT / "xtask/scripts/peripheral_matrix.py",
                     ROOT / "xtask/scripts/peripheral_recipes.py"]
            saved = source_snapshot(paths)
            catalogue = json.loads(bound_bytes(ROOT / "data/chips.json", saved))
            chips = select_chips(catalogue["builds"], args.chip, args.family, args.limit, args.shard)
            batch_path = base / "source-snapshots" / (uuid.uuid4().hex + ".json")
            write_json(batch_path, batch_trees)
            summary["source_tree_snapshot"] = dict(path=str(batch_path), sha256=sha(batch_path))
            with ExitStack() as report_locks:
                # Acquire all locks before touching any selected report. If a
                # conflicting invocation owns one, its evidence stays intact.
                for chip in chips:
                    report_locks.enter_context(cache_lock(base / "locks" / (chip["feature"] + ".lock")))
                for chip in chips:
                    result = dict(schema_version=1, chip=chip["feature"], target=chip["target"], status="incomplete", started_unix=time.time(),
                                  scope="HAL and framework constructors linked; no electrical or transfer test", hil="not-run",
                                  source_sha256=dict(saved), source_tree_snapshot=summary["source_tree_snapshot"],
                                  categories={c: dict(status="blocked", hardware_present=None, reason="exact source/recipe verification not completed") for c in CATEGORIES})
                    path = base / "reports" / chip["feature"] / "result.json"
                    records.append((path, result))
                    write_json(path, result)
                try:
                    # Resolve additional immutable inputs without rebasing any
                    # existing digest. The patch chain and its source inventory
                    # are already captured in the data/patches tree above.
                    manifest = json.loads(bound_bytes(ROOT / "data/patches/stm32-metapac/vendor-manifest.json", saved))
                    checkout = provenance.baseline_checkout()
                    bind_input(checkout / "data/registers/afio_f1.json", saved)
                    for chip in chips:
                        if chip["embassy_feature_available"] or chip.get("local_patch_available"):
                            filename = chip["feature"].upper().split("-")[0] + ".json"
                            path = provenance.patch_json_path(manifest, filename, ROOT) or checkout / "data/chips" / filename
                            bind_input(path, saved)
                    for document in provenance.verified_source_documents(ROOT):
                        path = provenance.checked_child_path(ROOT, document["cache_path"])
                        try:
                            raw = bind_input(path, saved)
                        except OSError as error:
                            raise ValueError(f"Missing source document {path}; run python xtask/scripts/fetch_source_documents.py") from error
                        if hashlib.sha256(raw).hexdigest() != document["sha256"]:
                            raise ValueError(f"source document differs from its bound provenance: {path}")
                    checkpoint()
                    patch = provenance.verify_vendor({"manifest_path": str(ROOT / "vendor/stm32-metapac/Cargo.toml")})
                    if patch != manifest:
                        raise ValueError("verified PAC manifest differs from batch input")
                    widths = afio_widths(checkout)
                    executable = None if args.plan_only else inspector(base)
                    checkpoint()
                    for chip, (report_path, result) in zip(chips, records):
                        feature = chip["feature"]
                        result["source_sha256"] = dict(saved)
                        checkpoint()
                        try:
                            if not (chip["embassy_feature_available"] or chip.get("local_patch_available")):
                                result["categories"] = {c: dict(status="blocked", reason="exact chip/core has no verified PAC/HAL backend", hardware_present=None) for c in CATEGORIES}
                            else:
                                data, source = load_data(checkout, patch, feature)
                                if saved.get(str(Path(source["path"]).resolve())) != source["sha256"]:
                                    raise ValueError("chip metadata differs from batch input")
                                result["chip_metadata"] = source
                                pair = dual_pair(chip, catalogue["builds"])
                                recipes = plan(data, feature, args.package, widths, paired=pair is not None)
                                result["categories"] = {c: {k: v for k, v in r.items() if k not in {"source", "clock_source"}} for c, r in recipes.items()}
                                if pair:
                                    result["topology"] = dict(kind="dual-core-owner-and-companion", owner=feature,
                                                              images=[p["feature"] for p in pair],
                                                              ownership={p["feature"]: dict(time_driver={"cm7": "TIM5", "cm4": "TIM2"}[p["feature"].rsplit("-", 1)[1]],
                                                                                           target_constructor=p["feature"] == feature) for p in pair})
                                if not args.plan_only and any(r["status"] == "planned" for r in recipes.values()):
                                    project = base / "projects" / feature
                                    if pair:
                                        projects = prepare_dual_projects(project, feature, pair, recipes, provenance.default_bank(data["memory"]))
                                        for core_project in projects.values():
                                            normalize(core_project, core_project / "metadata")
                                        metadata = None
                                        project_saved = source_snapshot([p for directory in projects.values() for p in directory.rglob("*")
                                                                         if p.is_file() and p.suffix in {".toml", ".lock", ".rs"}])
                                    else:
                                        prepare_project(project, feature, recipes, provenance.default_bank(data["memory"]))
                                        metadata = normalize(project, project / "metadata")
                                        project_saved = source_snapshot([project / "Cargo.toml", project / "Cargo.lock", project / ".cargo/config.toml", *sorted((project / "src/bin").glob("*.rs"))])
                                    result["probe_source_sha256"] = project_saved
                                    for category, recipe in recipes.items():
                                        if recipe["status"] != "planned":
                                            continue
                                        row = result["categories"][category]
                                        try:
                                            row.update(build_dual_category(base, executable, projects, pair, feature, category, row) if pair else
                                                       build_category(base, executable, project, chip, category, row, metadata))
                                            if category == "can" and chip.get("known_runtime_issues"):
                                                row.update(status="blocked", link_status="constructor-linked", reason="catalogue runtime issue requires resolution", known_runtime_issues=chip["known_runtime_issues"])
                                        except Exception as error:
                                            row.update(status="failed", reason=str(error))
                                        ensure_snapshot(project_saved)
                                        write_json(report_path, result)
                                        print(feature, category, row["status"], flush=True)
                            result["status"] = ("planned" if args.plan_only else "constructor-matrix-passed") if all(r["status"] in {"constructor-linked", "not-present", "planned" if args.plan_only else "not-present"} for r in result["categories"].values()) else "incomplete"
                        except Exception as error:
                            result.update(status="failed", error=str(error))
                        # Global source failures escape the per-chip handler:
                        # earlier linked categories cannot turn the batch green.
                        checkpoint()
                        result["finished_unix"] = time.time()
                        write_json(report_path, result)
                        print(feature, result["status"], report_path, flush=True)
                    checkpoint()
                    passed = all(result["status"] in {"constructor-matrix-passed", "planned"} for _, result in records)
                    finish(("planned" if args.plan_only else "constructor-matrix-passed") if passed else "incomplete")
                except BaseException as error:
                    for path, result in records:
                        result.update(status="failed", batch_error=str(error) or type(error).__name__,
                                      source_sha256=dict(saved), finished_unix=time.time())
                        write_json(path, result)
                    finish("failed", error)
                    raise
        except Exception as error:
            if summary["status"] != "failed":
                finish("failed", error)
            print(str(error), file=sys.stderr)
            return 1
        finally:
            if executable:
                executable.unlink(missing_ok=True)
        return 0 if summary["status"] in {"constructor-matrix-passed", "planned"} else 1


if __name__ == "__main__":
    sys.exit(main())
