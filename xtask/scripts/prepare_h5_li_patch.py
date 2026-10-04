"""Reproducible exact-package H563LI/H573LI overlay on the fixed STM32 PAC.

Pin reconstruction follows stm32-data caa36afd gpio_af.rs/generator.rs.
No package pin or peripheral pin list is copied from a ZI chip.
"""
import argparse
import copy
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import urllib.request
import xml.etree.ElementTree as ET
import zipfile

ROOT = Path(__file__).resolve().parents[2]
PATCH = ROOT / "data/patches/stm32h5-li"
SOURCES = PATCH / "sources"
PAC_REVISION = "e463add8cc54375f61c6f5f83d6b589e7fc68be2"
GENERATOR_REVISION = "caa36afd62510b0e6315ee0dccd1f9c65fbcac83"
NS = {"s": "http://dummy.com"}


def dump(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8", newline="\n")


def sha(data):
    return hashlib.sha256(data).hexdigest()


def verify_sources():
    manifest = json.loads((PATCH / "sources.json").read_text())
    for name, record in manifest["files"].items():
        if sha((SOURCES / name).read_bytes()) != record["sha256"]:
            raise ValueError(f"Pinned source checksum mismatch: {name}")
    return manifest


def pin_name(name):
    match = re.match(r"^P[A-Z]\d+(?:_C)?", name)
    return match[0] if match else None


def pin_sort(name):
    match = re.fullmatch(r"P([A-Z])(\d+)(?:_C)?", name)
    return match[1], int(match[2])


def signal_name(value):
    if value.startswith("USB_OTG_FS_"):
        peri, signal = "USB_OTG_FS", value[len("USB_OTG_FS_"):]
    elif value.startswith("USB_OTG_HS_"):
        peri, signal = "USB_OTG_HS", value[len("USB_OTG_HS_"):]
    elif value == "CEC":
        peri, signal = "CEC", "CEC"
    elif "_" in value:
        peri, signal = value.split("_", 1)
    else:
        return None
    if signal.startswith("EXTI"):
        return None
    peri = {"ADC": "ADC1", "DAC": "DAC1", "HRTIM": "HRTIM1",
            "HDMI_CEC": "CEC", "SUBGHZ": "SUBGHZSPI", "USB_DRD_FS": "USB",
            "SBS": "SYSCFG", "SPDIFRX": "SPDIFRX1", "RIF": "RIFSC"}.get(peri, peri)
    signal = re.sub(r"^(?:RMII_|MII_)", "", signal)
    return peri, signal


def gpio_af():
    tree = ET.parse(SOURCES / "GPIO-STM32H56x_gpio_v1_0_Modes.xml").getroot()
    result = {}
    for pin in tree.findall("s:GPIO_Pin", NS):
        name = pin_name(pin.get("Name", ""))
        if not name:
            continue
        for signal in pin.findall("s:PinSignal", NS):
            pair = signal_name(signal.get("Name"))
            if not pair:
                continue
            value = signal.findtext("s:SpecificParameter/s:PossibleValue", namespaces=NS)
            match = re.search(r"_AF(\d+)_", value or "")
            if not match:
                raise ValueError(f"Missing AF in GPIO source: {name} {signal.attrib}")
            key = (name, *pair)
            af = int(match[1])
            if key in result and result[key] != af:
                raise ValueError(f"Conflicting GPIO AF source: {key}")
            result[key] = af
    return result


def read_xml(name):
    return ET.parse(SOURCES / name).getroot()


def ip_set(tree):
    return sorted((p.get("InstanceName"), p.get("Name"), p.get("Version"))
                  for p in tree.findall("s:IP", NS))


def validate_die(li, zi):
    for field in ["Die", "Core", "Ram", "Flash"]:
        if li.findtext(f"s:{field}", namespaces=NS) != zi.findtext(f"s:{field}", namespaces=NS):
            raise ValueError(f"LI/ZI source mismatch: {field}")
    if ip_set(li) != ip_set(zi):
        raise ValueError("LI/ZI peripheral instance/version mismatch")
    if li.findtext("s:Die", namespaces=NS) != "DIE484":
        raise ValueError("Unexpected die")


def reconstruct_pins(tree, afs):
    positions, gpio, peris = {}, set(), {}
    for pin in tree.findall("s:Pin", NS):
        raw = pin.get("Name")
        clean = pin_name(raw)
        positions.setdefault(pin.get("Position"), []).append(clean or raw)
        if not clean:
            continue
        gpio.add(clean.replace("_C", ""))
        for signal in pin.findall("s:Signal", NS):
            raw_signal = signal.get("Name")
            if raw_signal in {"GPIO", "AUDIOCLK", "VDDTCXO"} or "EXTI" in raw_signal:
                continue
            pair = signal_name(raw_signal)
            if not pair:
                continue
            peri, sig = pair
            af = afs.get((clean, peri, sig))
            if af is None and sig == "CTS":
                af = afs.get((clean, peri, "CTS_NSS"))
            item = {"pin": clean, "signal": sig}
            if af is not None:
                item["af"] = af
            if peri.startswith("I2S"):
                peri = "SPI" + peri[3:]
                item["signal"] = "I2S_" + sig
            peris.setdefault(peri, []).append(item)
    # Upstream data/extra/STM32H5.yaml analog OPAMP override. This is a
    # documented source correction, filtered against the actual LI package.
    peris["OPAMP1"] = [{"pin": p, "signal": s} for p, s in [
        ("PC5", "VINM0"), ("PB1", "VINM1"), ("PB0", "VINP0"),
        ("PA0", "VINP2"), ("PA7", "VOUT")] if p in gpio]
    for peri, items in peris.items():
        items.sort(key=lambda p: (p["pin"], p["signal"], "af" not in p))
        unique = {}
        for item in items:
            unique.setdefault((item["pin"], item["signal"]), item)
        peris[peri] = list(unique.values())
    package = {"name": tree.get("RefName"), "package": tree.get("Package"), "pins": [
        {"position": pos, "signals": sorted(s for s in signals if s != "NC")}
        for pos, signals in sorted(positions.items(), key=lambda x: (0, int(x[0])) if x[0].isdigit() else (1, x[0]))]}
    return package, [{"name": p} for p in sorted(gpio, key=pin_sort)], peris


def fixed_file(checkout, relative):
    return subprocess.check_output(["git", "-C", str(checkout), "show", f"{PAC_REVISION}:{relative}"])


def die_metadata(chip):
    result = copy.deepcopy(chip)
    for key in ["name", "packages", "docs", "line"]:
        result.pop(key, None)
    for core in result["cores"]:
        core.pop("pins", None)
        for peri in core["peripherals"]:
            peri.pop("pins", None)
    return result


def build_json(checkout):
    verify_sources()
    afs = gpio_af()
    report = {"pac_revision": PAC_REVISION, "generator_revision": GENERATOR_REVISION, "chips": {}}
    for family in ["H563", "H573"]:
        name, baseline_name = f"STM32{family}LI", f"STM32{family}ZI"
        li, zi = read_xml(name + "HxQ.xml"), read_xml(baseline_name + "Tx.xml")
        validate_die(li, zi)
        raw = fixed_file(checkout, f"data/chips/{baseline_name}.json")
        baseline = json.loads(raw)
        package, pins, peris = reconstruct_pins(li, afs)
        # Upstream merges all packages sharing the ZI chip slug. The Q SMPS
        # package exposes PB11/PE1 where the LDO package uses supply pins.
        zi_q = read_xml(baseline_name + "TxQ.xml")
        if set(ip_set(zi)) | set(ip_set(zi_q)) != set(ip_set(li)):
            raise ValueError("Merged ZI package IP set differs from LI")
        zi_group = copy.deepcopy(zi)
        zi_group.extend(copy.deepcopy(zi_q.findall("s:Pin", NS)))
        _, _, reconstructed_zi = reconstruct_pins(zi_group, afs)
        changes = {}
        for peri in baseline["cores"][0]["peripherals"]:
            old = peri.get("pins", [])
            reconstructed = reconstructed_zi.get(peri["name"], [])
            if old != reconstructed:
                changes[peri["name"]] = {"baseline": old, "reconstructed": reconstructed}
        if changes:
            dump(PATCH / "zi-pin-differences.json", changes)
            raise ValueError("Reconstructed ZI pin data differs from baseline; review zi-pin-differences.json")
        chip = copy.deepcopy(baseline)
        chip["name"] = name
        chip["packages"] = [package]
        chip["cores"][0]["pins"] = pins
        for peri in chip["cores"][0]["peripherals"]:
            if peris.get(peri["name"]):
                peri["pins"] = peris[peri["name"]]
            else:
                peri.pop("pins", None)
        assert die_metadata(chip) == die_metadata(baseline)
        dump(PATCH / "chips" / f"{name}.json", chip)
        old_pins = {p["name"] for p in baseline["cores"][0]["pins"]}
        new_pins = {p["name"] for p in pins}
        pin_changes = {}
        old_peris = {p["name"]: p for p in baseline["cores"][0]["peripherals"]}
        for peri in chip["cores"][0]["peripherals"]:
            old = old_peris[peri["name"]].get("pins", [])
            new = peri.get("pins", [])
            added, removed = [p for p in new if p not in old], [p for p in old if p not in new]
            if added or removed:
                pin_changes[peri["name"]] = {"added": added, "removed": removed}
        report["chips"][name] = {
            "baseline": baseline_name, "baseline_sha256": sha(raw),
            "baseline_package_group": [baseline_name + "Tx", baseline_name + "TxQ"],
            "zi_q_omitted_ip": sorted(set(ip_set(zi)) - set(ip_set(zi_q))),
            "identical_ip_instances_versions": True, "ip_instances_versions": ip_set(li),
            "identical_die_level_metadata": True, "die_level_metadata_sha256": sha(json.dumps(die_metadata(chip), sort_keys=True).encode()),
            "added_gpio": sorted(new_pins - old_pins, key=pin_sort),
            "removed_gpio": sorted(old_pins - new_pins, key=pin_sort),
            "pin_af_changes": pin_changes,
            "required_supply": "smps", "package_suffix": "Q",
            "regulator_selection": "hardware_by_package",
            "hal_supply_switch_required": False,
            "startup_assumption": "cold_reset_with_VDDSMPS_powered_and_internal_regulator_not_bypassed",
            "supply_source": "https://www.st.com/resource/en/application_note/an5930-guidelines-for-power-management-on-stm32h5-mcus-stmicroelectronics.pdf",
            "supply_balls": [p for p in package["pins"] if any("SMPS" in s for s in p["signals"])],
            "peripheral_pin_counts": {p["name"]: len(p.get("pins", [])) for p in chip["cores"][0]["peripherals"] if p.get("pins")},
        }
    dump(PATCH / "comparison.json", report)
    dump(PATCH / "zi-pin-differences.json", {})
    return report


def build_generator():
    manifest = verify_sources()
    archive = ROOT / "data/sources/stm32-gaps/stm32-data-baseline.zip"
    if not archive.is_file():
        archive.parent.mkdir(parents=True, exist_ok=True)
        url = f"https://codeload.github.com/embassy-rs/stm32-data/zip/{GENERATOR_REVISION}"
        with urllib.request.urlopen(url, timeout=45) as response:
            archive.write_bytes(response.read())
    if sha(archive.read_bytes()) != manifest["generator_archive_sha256"]:
        raise ValueError("Pinned generator archive checksum mismatch")
    destination = (ROOT / "target/h5-li-generator-source").resolve()
    with zipfile.ZipFile(archive) as source:
        for member in source.infolist():
            path = (destination / member.filename).resolve()
            if not path.is_relative_to(destination):
                raise ValueError("Unsafe path in generator archive")
            if member.is_dir():
                path.mkdir(parents=True, exist_ok=True)
            else:
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(source.read(member))
    source = destination / f"stm32-data-{GENERATOR_REVISION}"
    target = ROOT / "target/h5-li-generator"
    subprocess.run(["cargo", "+1.98.1", "build", "--manifest-path", str(source / "Cargo.toml"),
                    "-p", "stm32-metapac-gen", "--locked", "--target-dir", str(target)],
                   cwd=ROOT, check=True)
    return target / "debug" / ("stm32-metapac-gen.exe" if os.name == "nt" else "stm32-metapac-gen")


def generate_pac(checkout, generator_binary):
    """Run the unmodified pinned upstream generator in a task-owned directory."""
    work = (ROOT / "target/h5-li-pac-generation").resolve()
    if not work.is_relative_to((ROOT / "target").resolve()):
        raise ValueError("Generator work directory escaped the workspace target directory")
    if (work / "build/stm32-metapac").is_symlink():
        raise ValueError("Refusing to regenerate through a symbolic link")
    data = work / "build/data"
    (data / "chips").mkdir(parents=True, exist_ok=True)
    (data / "registers").mkdir(parents=True, exist_ok=True)
    registers = set()
    for path in sorted((PATCH / "chips").glob("STM32H*LI.json")):
        chip = json.loads(path.read_text())
        shutil.copyfile(path, data / "chips" / path.name)
        for core in chip["cores"]:
            for peri in core["peripherals"]:
                if reg := peri.get("registers"):
                    registers.add(f"{reg['kind']}_{reg['version']}.json")
    for filename in registers:
        (data / "registers" / filename).write_bytes(fixed_file(checkout, "data/registers/" + filename))
    subprocess.run([str(generator_binary.resolve())], cwd=work, check=True)
    output = work / "build/stm32-metapac"
    # Use rustfmt through stdin to format each file without traversing modules.
    # Its output is only formatting; the generator is the original fixed binary.
    rust_files = list(output.rglob("*.rs"))
    for path in rust_files:
        process = subprocess.run(["rustfmt", "+1.98.1", "--edition", "2024", "--config", "max_width=120", "--emit", "stdout"],
                                 input=path.read_bytes(), stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=True)
        path.write_bytes(process.stdout.replace(b"\r\n", b"\n"))
    return output


def materialize_vendor(checkout, output):
    vendor = ROOT / "vendor/stm32-metapac"
    baseline = checkout / "stm32-metapac"
    # Never propagate local edits to the fixed upstream crate into the vendor.
    subprocess.run(["git", "-C", str(checkout), "diff", "--exit-code", PAC_REVISION, "--", "stm32-metapac"], check=True)
    vendor.mkdir(parents=True, exist_ok=True)
    shutil.copytree(baseline, vendor, dirs_exist_ok=True)
    changed = []
    # All generated shared register modules must still be the baseline versions.
    # Ignore whitespace only (formatting release may differ), retain all tokens.
    for folder in ["peripherals", "registers"]:
        for path in (output / "src" / folder).glob("*.rs"):
            old = baseline / "src" / folder / path.name
            if not old.is_file() or re.sub(rb"\s+", b"", path.read_bytes()) != re.sub(rb"\s+", b"", old.read_bytes()):
                raise ValueError(f"Generated register module differs from baseline: {folder}/{path.name}")
    shared = {}
    for path in sorted((output / "src/chips").glob("metadata_*.rs")):
        name = "metadata_li_" + path.name.removeprefix("metadata_")
        shared[path.name] = name
        dest = vendor / "src/chips" / name
        dest.write_bytes(path.read_bytes())
        changed.append(dest.relative_to(vendor).as_posix())
    for chip in ["stm32h563li", "stm32h573li"]:
        dest = vendor / "src/chips" / chip
        dest.mkdir(parents=True, exist_ok=True)
        for path in sorted((output / "src/chips" / chip).iterdir()):
            text = path.read_text()
            if path.name == "metadata.rs":
                for old, new in shared.items():
                    text = text.replace(f'../{old}', f'../{new}')
            (dest / path.name).write_text(text, encoding="utf-8", newline="\n")
            changed.append((dest / path.name).relative_to(vendor).as_posix())
    cargo = (baseline / "Cargo.toml").read_text()
    for name in ["stm32h563li", "stm32h573li"]:
        # Add an independent empty feature; do not forward to a ZI feature.
        anchor = next(line for line in cargo.splitlines() if line.startswith(name[:9]) and line > name + " = []")
        cargo = cargo.replace(anchor, name + " = []\n" + anchor, 1)
    cargo += "\n# This materialized dependency is isolated from the parent workspace.\n[workspace]\n"
    cargo = cargo.replace('    "README.md",', '    "README.md",\n    "LICENSE-*",\n    "ST-LICENSE",\n    "NOTICE",')
    (vendor / "Cargo.toml").write_text(cargo, encoding="utf-8", newline="\n")
    all_chips = (baseline / "src/all_chips.rs").read_text()
    for name in ["STM32H563LI", "STM32H573LI"]:
        lines = all_chips.splitlines()
        index = next(i for i, line in enumerate(lines) if line.strip().strip('\",') > name and line.strip().startswith('"'))
        lines.insert(index, f'    "{name}",')
        all_chips = "\n".join(lines) + "\n"
    (vendor / "src/all_chips.rs").write_text(all_chips, encoding="utf-8", newline="\n")
    for name in ["LICENSE-MIT", "LICENSE-APACHE", "ST-LICENSE"]:
        shutil.copyfile(SOURCES / name, vendor / name)
    (vendor / "NOTICE").write_text(
        "stm32-metapac baseline: e463add8cc54375f61c6f5f83d6b589e7fc68be2 (MIT OR Apache-2.0).\n"
        "All original baseline source files and notices are retained. The original snapshot contains no standalone license text.\n"
        "MIT/Apache texts are reproduced from the pinned Embassy project; provenance is recorded in data/patches/stm32h5-li/sources.json.\n"
        "New STM32H563LI/STM32H573LI package metadata derives from STMicroelectronics STM32_open_pin_data,\n"
        "revision 7d1f1514ed5583ec5007ad91236b4e1d377295b1; its BSD-3-Clause license is retained in ST-LICENSE.\n"
        "Copyright (c) 2026 STMicroelectronics. All rights reserved.\n"
        "Generated with unchanged stm32-data generator caa36afd62510b0e6315ee0dccd1f9c65fbcac83.\n",
        encoding="utf-8", newline="\n")
    changed += ["Cargo.toml", "src/all_chips.rs", "LICENSE-MIT", "LICENSE-APACHE", "ST-LICENSE", "NOTICE"]
    dump(PATCH / "vendor-overlay.json", {"baseline_revision": PAC_REVISION,
        "files": {p: sha((vendor / p).read_bytes()) for p in sorted(changed)}})
    return vendor


def write_vendor_manifest(checkout):
    vendor = ROOT / "vendor/stm32-metapac"
    files = {}
    paths = sorted(p for p in vendor.rglob("*") if p.is_file())
    for index, path in enumerate(paths, 1):
        files[path.relative_to(vendor).as_posix()] = sha(path.read_bytes())
        if index % 1000 == 0:
            print(f"Hashed {index}/{len(paths)} vendor files", flush=True)
    tree = subprocess.check_output(["git", "-C", str(checkout), "ls-tree", "-r", PAC_REVISION,
                                    "data/chips", "stm32-metapac"], text=True)
    chip_blobs, pac_blobs = {}, {}
    for line in tree.splitlines():
        info, path = line.split("\t", 1)
        oid = info.split()[2]
        if path.startswith("data/chips/"):
            chip_blobs[path.removeprefix("data/chips/")] = oid
        else:
            pac_blobs[path.removeprefix("stm32-metapac/")] = oid
    # Original Git blob IDs remain an independent verification reference for
    # any baseline JSON read from a Cargo cache or a separate source checkout.
    result = {
        "baseline_revision": PAC_REVISION, "generator_revision": GENERATOR_REVISION,
        "baseline_repository": "https://github.com/embassy-rs/stm32-data-generated",
        "baseline_chip_git_blobs": chip_blobs, "baseline_pac_git_blobs": pac_blobs,
        "vendor_files_sha256": files,
        "new_chip_jsons_sha256": {p.name: sha(p.read_bytes()) for p in sorted((PATCH / "chips").glob("*.json"))},
        "sources_manifest_sha256": sha((PATCH / "sources.json").read_bytes()),
        "comparison_sha256": sha((PATCH / "comparison.json").read_bytes()),
    }
    dump(PATCH / "vendor-manifest.json", result)
    print(f"Wrote complete manifest: {len(files)} vendor files, {len(chip_blobs)} original chip blobs", flush=True)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--pac-source", type=Path)
    parser.add_argument("--generator-binary", type=Path,
                        help="Run the fixed upstream PAC generator and materialize the complete vendor crate")
    parser.add_argument("--materialize", action="store_true", help="Verify/build the fixed generator and materialize vendor")
    parser.add_argument("--write-manifest", action="store_true", help="Hash all vendor files and record original Git blob provenance")
    args = parser.parse_args()
    checkout = args.pac_source
    if checkout is None:
        candidates = list((ROOT / ".tools/cargo/git/checkouts").glob("stm32-data-generated-*/e463add"))
        if len(candidates) != 1:
            raise ValueError("Pass --pac-source with the fixed generated checkout")
        checkout = candidates[0]
    actual = subprocess.check_output(["git", "-C", str(checkout), "rev-parse", "HEAD"], text=True).strip()
    if actual != PAC_REVISION:
        raise ValueError(f"Wrong PAC checkout revision: {actual}")
    build_json(checkout)
    print("Generated exact H563LI/H573LI JSON with verified die and reconstructed package pin/AF data")
    if args.generator_binary and args.materialize:
        raise ValueError("Choose --materialize or --generator-binary, not both")
    binary = build_generator() if args.materialize else args.generator_binary
    if binary:
        output = generate_pac(checkout, binary)
        vendor = materialize_vendor(checkout, output)
        print(f"Materialized complete baseline plus two genuine package PACs: {vendor}")
    if args.write_manifest:
        write_vendor_manifest(checkout)


if __name__ == "__main__":
    main()
