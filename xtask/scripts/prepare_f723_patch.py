"""Build exact F723RC/RE PACs over the existing, immutable LI overlay.

Source correction policy follows fixed stm32-data's XML/CMSIS intersection.
The package pin maps are reconstructed from ST's actual VQFPN68 XML, never
from another package. Run --materialize --write-manifest after the LI patch.
"""
import argparse
import ast
import copy
import json
from pathlib import Path
import re
import shutil
import subprocess
import xml.etree.ElementTree as ET

import prepare_h5_li_patch as common

ROOT = Path(__file__).resolve().parents[2]
PATCH = ROOT / "data/patches/stm32f723-r"
SOURCES = PATCH / "sources"
COMPOSED = ROOT / "data/patches/stm32-metapac/vendor-manifest.json"
VENDOR = ROOT / "vendor/stm32-metapac"
PAC_REVISION = common.PAC_REVISION
GENERATOR_REVISION = common.GENERATOR_REVISION
NS = common.NS
CHIPS = ["STM32F723RC", "STM32F723RE"]
REMOVED = {"FMC", "LPTIM1", "SPI4", "UART7", "UART8"}
SHARED_CHANGED = {"Cargo.toml", "src/all_chips.rs", "NOTICE"}


def verify_sources():
    result = json.loads((PATCH / "sources.json").read_text())
    for name, record in result["files"].items():
        if common.sha((SOURCES / name).read_bytes()) != record["sha256"]:
            raise ValueError(f"Pinned source checksum mismatch: {name}")
    return result


def read_xml(name):
    return ET.parse(SOURCES / name).getroot()


def gpio_af():
    result = {}
    tree = read_xml("GPIO-STM32F72x_gpio_v1_0_Modes.xml")
    for pin in tree.findall("s:GPIO_Pin", NS):
        clean = common.pin_name(pin.get("Name", ""))
        if not clean:
            continue
        for signal in pin.findall("s:PinSignal", NS):
            pair = common.signal_name(signal.get("Name"))
            if not pair:
                continue
            value = signal.findtext("s:SpecificParameter/s:PossibleValue", namespaces=NS)
            match = re.search(r"_AF(\d+)_", value or "")
            if not match:
                raise ValueError(f"Missing GPIO alternate function: {clean} {pair}")
            key, af = (clean, *pair), int(match[1])
            if key in result and result[key] != af:
                raise ValueError(f"Conflicting alternate function: {key}")
            result[key] = af
    return result


def reconstruct_pins(tree, afs):
    positions, gpio, peris, suppressed = {}, set(), {}, []
    for pin in tree.findall("s:Pin", NS):
        raw = pin.get("Name")
        clean = common.pin_name(raw)
        positions.setdefault(pin.get("Position"), []).append(clean or raw)
        if not clean:
            continue
        gpio.add(clean)
        for signal in pin.findall("s:Signal", NS):
            name = signal.get("Name")
            if name in {"GPIO", "AUDIOCLK", "VDDTCXO"} or "EXTI" in name:
                continue
            pair = common.signal_name(name)
            if not pair:
                continue
            peri, sig = pair
            # DS11853 Rev9 Table10, page80 footnote4 explicitly excludes
            # ULPI signals on F723. Current shared F72x XML lists them anyway.
            if peri == "USB_OTG_HS" and sig.startswith("ULPI_"):
                suppressed.append({"pin": clean, "signal": sig, "reason": "DS11853 Rev9 p80 footnote4"})
                continue
            af = afs.get((clean, peri, sig))
            if af is None and sig == "CTS":
                af = afs.get((clean, peri, "CTS_NSS"))
            item = {"pin": clean, "signal": sig}
            if af is not None:
                item["af"] = af
            if peri.startswith("I2S"):
                peri = "SPI" + peri[3:]
                item["signal"] = "I2S_" + sig
            if peri == "QUADSPI" and sig == "NCS":
                item["signal"] = "BK1_NCS"
            peris.setdefault(peri, []).append(item)
    for peri, items in peris.items():
        items.sort(key=lambda p: (p["pin"], p["signal"], "af" not in p))
        unique = {}
        for item in items:
            unique.setdefault((item["pin"], item["signal"]), item)
        peris[peri] = list(unique.values())
    package = {"name": tree.get("RefName"), "package": tree.get("Package"), "pins": [
        {"position": pos, "signals": sorted(set(signals) - {"NC"})}
        for pos, signals in sorted(positions.items(), key=lambda p: (0, int(p[0])) if p[0].isdigit() else (1, p[0]))]}
    return package, [{"name": p} for p in sorted(gpio, key=common.pin_sort)], peris, suppressed


class Header:
    """Evaluate only numeric CMSIS macro expressions, without Python eval."""
    def __init__(self, text):
        text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
        self.defines = dict(re.findall(r"^#define\s+(\w+)\s+([^\n]+)$", text, re.M))
        self.irqs = {name: int(number) for name, number in
                     re.findall(r"\b(\w+)_IRQn\s*=\s*(-?\d+)", text) if int(number) >= 0}

    def value(self, name, seen=()):
        if name in seen or name not in self.defines:
            raise ValueError(f"Unresolved CMSIS constant: {name}")
        value = self.defines[name]
        value = re.sub(r"\(\s*(?:\w+_TypeDef|u?int\d+_t)\s*\*?\s*\)", "", value)
        value = re.sub(r"(0[xX][0-9a-fA-F]+|\d+)[uUlL]+\b", r"\1", value)

        def number(node):
            if isinstance(node, ast.Constant) and isinstance(node.value, int):
                return node.value
            if isinstance(node, ast.Name):
                return self.value(node.id, (*seen, name))
            if isinstance(node, ast.BinOp):
                a, b = number(node.left), number(node.right)
                if isinstance(node.op, ast.Add):
                    return a + b
                if isinstance(node.op, ast.Sub):
                    return a - b
                if isinstance(node.op, ast.LShift):
                    return a << b
                if isinstance(node.op, ast.BitOr):
                    return a | b
            raise ValueError(f"Unsupported CMSIS expression: {name}")
        return number(ast.parse(value.strip(), mode="eval").body)

    def address(self, name):
        aliases = {"FLASH": ["FLASH_R_BASE"], "FMC": ["FMC_R_BASE"],
                   "QUADSPI": ["QSPI_R_BASE", "QUADSPI_R_BASE"],
                   "VREFINTCAL": ["VREFINT_CAL_ADDR_CMSIS"],
                   "TS_CAL1": ["TEMPSENSOR_CAL1_ADDR_CMSIS"],
                   "TS_CAL2": ["TEMPSENSOR_CAL2_ADDR_CMSIS"]}
        for key in aliases.get(name, []) + [name + "_BASE", name + "_PERIPH_BASE", name]:
            if key in self.defines:
                try:
                    return key, self.value(key)
                except (SyntaxError, ValueError):
                    continue
        raise ValueError(f"No exact CMSIS address for {name}")


def verify_die(target, baseline_xml, suffix):
    for key, expected in [("Die", "DIE452"), ("Core", "Arm Cortex-M7"), ("Ram", "192"), ("CCMRam", "64")]:
        if target.findtext("s:" + key, namespaces=NS) != expected:
            raise ValueError(f"Unexpected target {key}")
        if baseline_xml.findtext("s:" + key, namespaces=NS) != expected:
            raise ValueError(f"Unexpected baseline {key}")
    if target.findtext("s:Flash", namespaces=NS) != {"C": "256", "E": "512"}[suffix]:
        raise ValueError("Wrong exact-part flash capacity")
    old, new = set(common.ip_set(baseline_xml)), set(common.ip_set(target))
    if new - old != {("AES", "AES", "aes3_v1_0_F7_Cube")}:
        raise ValueError("Unreviewed additional/changed target IP")
    if {p[0] for p in old - new} != REMOVED:
        raise ValueError("Unexpected missing target IP")
    # Matching names must retain all exact IP versions, including RCC and DMA.
    if len(new - old) != 1 or len(old - new) != 5:
        raise ValueError("Unreviewed IP version difference")
    return {"added_xml_ip": sorted(new - old), "omitted_xml_ip": sorted(old - new),
            "shared_ip_instances_versions": sorted(old & new)}


def validate_header(chip, header, rcc_registers):
    core, = chip["cores"]
    if {p["name"]: p["number"] for p in core["interrupts"]} != header.irqs:
        raise ValueError("Baseline interrupt vector differs from exact F723 CMSIS")
    if "AES" in header.defines or "AES_BASE" in header.defines:
        raise ValueError("F723 CMSIS now declares AES: re-review the XML conflict")
    addresses, clocks = {}, {}
    for peri in core["peripherals"]:
        macro, address = header.address(peri["name"])
        if address != peri["address"]:
            raise ValueError(f"CMSIS address differs for {peri['name']}")
        addresses[peri["name"]] = {"macro": macro, "address": address}
        for kind in ["enable", "reset"]:
            field = peri.get("rcc", {}).get(kind)
            if field:
                # PAC normalizes these names; CMSIS retains the OTG spelling.
                value = field["field"].replace("USB_OTG_HS", "OTGHS").replace("USB_OTG_FS", "OTGFS").replace("QUADSPI", "QSPI")
                if value == "OTGHSRST":
                    value = "OTGHRST"
                macro = f"RCC_{field['register']}_{value}"
                definition = next(f for f in rcc_registers["fieldset/" + field["register"]]["fields"]
                                  if f["name"] == field["field"])
                mask = ((1 << definition["bit_size"]) - 1) << definition["bit_offset"]
                if mask != header.value(macro):
                    raise ValueError(f"RCC register bit mask differs from exact CMSIS: {macro}")
                clocks[f"{peri['name']}.{kind}"] = {"macro": macro, "mask": mask,
                                                  "matches_pinned_rcc_bit_definition": True}
    return {"peripheral_addresses": addresses, "rcc_gate_reset_masks": clocks,
            "irq_count": len(header.irqs), "interrupts_identical_to_exact_cmsis": True}


def build_json(checkout):
    verify_sources()
    afs = gpio_af()
    header = Header((SOURCES / "stm32f723xx.h").read_text())
    rcc_registers = json.loads(common.fixed_file(checkout, "data/registers/rcc_f7.json"))
    baseline_xml = read_xml("STM32F723V(C-E)Tx.xml")
    group = copy.deepcopy(baseline_xml)
    group.extend(copy.deepcopy(read_xml("STM32F723V(C-E)Yx.xml").findall("s:Pin", NS)))
    _, _, reference_pins, _ = reconstruct_pins(group, afs)
    report = {"baseline_revision": PAC_REVISION, "generator_revision": GENERATOR_REVISION, "chips": {}}
    for suffix in "CE":
        name, baseline_name = "STM32F723R" + suffix, "STM32F723V" + suffix
        target = read_xml(name + "Vx.xml")
        ip = verify_die(target, baseline_xml, suffix)
        raw = common.fixed_file(checkout, f"data/chips/{baseline_name}.json")
        baseline = json.loads(raw)
        discrepancies = {}
        for peri in baseline["cores"][0]["peripherals"]:
            a, b = peri.get("pins", []), reference_pins.get(peri["name"], [])
            if a != b:
                discrepancies[peri["name"]] = {"only_baseline": [p for p in a if p not in b],
                                               "only_current_st_xml": [p for p in b if p not in a]}
        # DS11853 Rev9 p63 Table10 and current ST XML both show PC1 TAMP3,
        # not TS. Preserve neither a stale baseline pin nor an invented one.
        if discrepancies != {"RTC": {"only_baseline": [{"pin": "PC1", "signal": "TS"}], "only_current_st_xml": []}}:
            common.dump(PATCH / "unexpected-baseline-pin-differences.json", discrepancies)
            raise ValueError("Unreviewed reference-package pin reconstruction difference")
        package, pins, peris, suppressed = reconstruct_pins(target, afs)
        if len(package["pins"]) != 68 or len(pins) != 51:
            raise ValueError("Unexpected VQFPN68 pin count")
        chip = copy.deepcopy(baseline)
        chip["name"], chip["packages"] = name, [package]
        core = chip["cores"][0]
        core["pins"] = pins
        core["peripherals"] = [p for p in core["peripherals"] if p["name"] not in REMOVED]
        for peri in core["peripherals"]:
            if peris.get(peri["name"]):
                peri["pins"] = peris[peri["name"]]
            else:
                peri.pop("pins", None)
        checks = validate_header(chip, header, rcc_registers)
        # All retained non-pin metadata must be byte-for-byte structurally equal;
        # only explicitly reviewed package-unavailable peripherals are removed.
        expected = common.die_metadata(baseline)
        expected["cores"][0]["peripherals"] = [p for p in expected["cores"][0]["peripherals"] if p["name"] not in REMOVED]
        if common.die_metadata(chip) != expected:
            raise ValueError("Unexpected die-level metadata mutation")
        old = {p["name"]: p for p in baseline["cores"][0]["peripherals"]}
        changes = {}
        for peri in core["peripherals"]:
            a, b = old[peri["name"]].get("pins", []), peri.get("pins", [])
            if a != b:
                changes[peri["name"]] = {"added": [p for p in b if p not in a], "removed": [p for p in a if p not in b]}
        common.dump(PATCH / "chips" / (name + ".json"), chip)
        report["chips"][name] = {"baseline": baseline_name, "baseline_json_sha256": common.sha(raw),
            **ip, **checks, "removed_peripherals": sorted(REMOVED),
            "rejected_xml_ip": {"AES": "No AES address or instance in exact stm32f723xx.h; follows upstream address intersection"},
            "baseline_pin_source_differences": discrepancies,
            "suppressed_unsupported_ulpi_signals": suppressed, "pin_af_changes": changes,
            "gpio_count": len(pins), "position_count": len(package["pins"]),
            "retained_die_metadata_identical": True,
            "memory_identical_to_same_capacity_f723": True,
            "power_pins": [dict(p.attrib) for p in target.findall("s:Pin", NS) if p.get("Type") in {"Power", "MonoIO"}],
            "supply": "internal F7 regulator / VCAP_1; separate VDDPHYHS and V12_PHYHS for integrated HS PHY; no H5 Q/SMPS selector",
            "ordering_status": "official ST XML; updated full 68-pin ordering/electrical table not available in audited DS11853 Rev9",
            "hardware_validation": "not-run"}
    common.dump(PATCH / "comparison.json", report)
    return report


def generate_pac(checkout, binary):
    work = ROOT / "target/f723-r-pac-generation"
    data = work / "build/data"
    (data / "chips").mkdir(parents=True, exist_ok=True)
    (data / "registers").mkdir(parents=True, exist_ok=True)
    if (work / "build/stm32-metapac").is_symlink():
        raise ValueError("Refusing to generate through a symbolic link")
    registers = set()
    for name in CHIPS:
        path = PATCH / "chips" / (name + ".json")
        chip = json.loads(path.read_text())
        shutil.copyfile(path, data / "chips" / path.name)
        for peri in chip["cores"][0]["peripherals"]:
            if reg := peri.get("registers"):
                registers.add(f"{reg['kind']}_{reg['version']}.json")
    for name in registers:
        (data / "registers" / name).write_bytes(common.fixed_file(checkout, "data/registers/" + name))
    subprocess.run([str(binary.resolve())], cwd=work, check=True)
    output = work / "build/stm32-metapac"
    for path in output.rglob("*.rs"):
        p = subprocess.run(["rustfmt", "+1.98.1", "--edition", "2024", "--config", "max_width=120", "--emit", "stdout"],
                           input=path.read_bytes(), stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=True)
        path.write_bytes(p.stdout.replace(b"\r\n", b"\n"))
    return output


def materialize_vendor(checkout, output):
    li = json.loads((common.PATCH / "vendor-manifest.json").read_text())
    # Reject unknown local changes before touching any vendor file. On a repeat
    # run verify the complete composed inventory, not the earlier LI inventory.
    actual_files = {p.relative_to(VENDOR).as_posix() for p in VENDOR.rglob("*") if p.is_file()}
    # A clean regeneration may have rebuilt only the LI stage so far even
    # when the repository still carries the final composed evidence file.
    previous = li if actual_files == set(li["vendor_files_sha256"]) else (
        json.loads(COMPOSED.read_text()) if COMPOSED.exists() else li)
    if actual_files != set(previous["vendor_files_sha256"]):
        raise ValueError("Current PAC file inventory differs from previous manifest")
    for name, digest in previous["vendor_files_sha256"].items():
        if common.sha((VENDOR / name).read_bytes()) != digest:
            raise ValueError(f"Refusing to overwrite changed PAC source: {name}")
    for folder in ["peripherals", "registers"]:
        for path in (output / "src" / folder).glob("*.rs"):
            original = common.fixed_file(checkout, f"stm32-metapac/src/{folder}/{path.name}")
            if re.sub(rb"\s+", b"", path.read_bytes()) != re.sub(rb"\s+", b"", original):
                raise ValueError(f"Generated register module differs from fixed baseline: {path}")
    changed, shared = set(), {}
    for path in sorted((output / "src/chips").glob("metadata_*.rs")):
        name = "metadata_f723_r_" + path.name.removeprefix("metadata_")
        shared[path.name] = name
        dest = VENDOR / "src/chips" / name
        dest.write_bytes(path.read_bytes())
        changed.add(dest.relative_to(VENDOR).as_posix())
    for chip in [c.lower() for c in CHIPS]:
        dest = VENDOR / "src/chips" / chip
        dest.mkdir(parents=True, exist_ok=True)
        for path in sorted((output / "src/chips" / chip).iterdir()):
            text = path.read_text()
            if path.name == "metadata.rs":
                for old, new in shared.items():
                    text = text.replace(f"../{old}", f"../{new}")
            (dest / path.name).write_text(text, encoding="utf-8", newline="\n")
            changed.add((dest / path.name).relative_to(VENDOR).as_posix())
    cargo = (VENDOR / "Cargo.toml").read_text()
    for chip in [c.lower() for c in CHIPS]:
        if f"\n{chip} = []\n" not in cargo:
            anchor = next(l for l in cargo.splitlines() if l.startswith("stm32f723") and l > chip + " = []")
            cargo = cargo.replace(anchor, chip + " = []\n" + anchor, 1)
    (VENDOR / "Cargo.toml").write_text(cargo, encoding="utf-8", newline="\n")
    text = (VENDOR / "src/all_chips.rs").read_text()
    for chip in CHIPS:
        if f'    "{chip}",' not in text:
            lines = text.splitlines()
            index = next(i for i, line in enumerate(lines) if line.strip().startswith('"') and line.strip().strip('\",') > chip)
            lines.insert(index, f'    "{chip}",')
            text = "\n".join(lines) + "\n"
    (VENDOR / "src/all_chips.rs").write_text(text, encoding="utf-8", newline="\n")
    notice = (VENDOR / "NOTICE").read_text()
    addition = ("New STM32F723RC/STM32F723RE VQFPN68 package metadata derives from the same pinned ST XML repository.\n"
                "Exact F723 CMSIS and DS11853 constrain inconsistent XML AES/ULPI entries; see data/patches/stm32f723-r.\n"
                "Combined LI + F723 provenance: data/patches/stm32-metapac/vendor-manifest.json.\n")
    if addition not in notice:
        (VENDOR / "NOTICE").write_text(notice + addition, encoding="utf-8", newline="\n")
    changed.update(SHARED_CHANGED)
    # Every original LI-only file, including PAC/metadata/licenses, stays exact.
    for name, digest in li["vendor_files_sha256"].items():
        if name not in SHARED_CHANGED and common.sha((VENDOR / name).read_bytes()) != digest:
            raise ValueError(f"LI overlay changed unexpectedly: {name}")
    common.dump(PATCH / "vendor-overlay.json", {"baseline_revision": PAC_REVISION,
                "files": {p: common.sha((VENDOR / p).read_bytes()) for p in sorted(changed)}})


def write_manifests():
    li_path = common.PATCH / "vendor-manifest.json"
    li = json.loads(li_path.read_text())
    overlay_path = PATCH / "vendor-manifest.json"
    overlay_files = json.loads((PATCH / "vendor-overlay.json").read_text())["files"]
    expected = {**li["vendor_files_sha256"], **overlay_files}
    paths = sorted(p for p in VENDOR.rglob("*") if p.is_file())
    if {p.relative_to(VENDOR).as_posix() for p in paths} != set(expected):
        raise ValueError("Composed vendor has missing or unreviewed extra files")
    files = {}
    for index, path in enumerate(paths, 1):
        name = path.relative_to(VENDOR).as_posix()
        files[name] = common.sha(path.read_bytes())
        if files[name] != expected[name]:
            raise ValueError(f"Refusing to certify an unreviewed vendor modification: {name}")
        if index % 1000 == 0:
            print(f"Hashed {index}/{len(paths)} composed vendor files", flush=True)
    common.dump(overlay_path, {"schema_version": 1, "baseline_revision": PAC_REVISION,
        "generator_revision": GENERATOR_REVISION,
        "files": overlay_files,
        "new_chip_jsons_sha256": {p.name: common.sha(p.read_bytes()) for p in sorted((PATCH / "chips").glob("*.json"))},
        "sources_manifest_sha256": common.sha((PATCH / "sources.json").read_bytes()),
        "comparison_sha256": common.sha((PATCH / "comparison.json").read_bytes())})
    result = {"schema_version": 2, "baseline_revision": PAC_REVISION,
              "generator_revision": GENERATOR_REVISION,
              "baseline_repository": li["baseline_repository"],
              "baseline_chip_git_blobs": li["baseline_chip_git_blobs"],
              "baseline_pac_git_blobs": li["baseline_pac_git_blobs"],
              "vendor_files_sha256": files, "overlays": [], "new_chip_jsons": {}}
    for name, path in [("stm32h5-li", li_path), ("stm32f723-r", overlay_path)]:
        result["overlays"].append({"id": name, "manifest_path": path.relative_to(ROOT).as_posix(),
                                   "manifest_sha256": common.sha(path.read_bytes())})
        original = json.loads(path.read_text())
        for filename, digest in original["new_chip_jsons_sha256"].items():
            chip_path = path.parent / "chips" / filename
            if common.sha(chip_path.read_bytes()) != digest:
                raise ValueError(f"Overlay chip JSON changed: {filename}")
            result["new_chip_jsons"][filename] = {"path": chip_path.relative_to(ROOT).as_posix(),
                                                "sha256": digest, "overlay": name}
    common.dump(COMPOSED, result)
    print(f"Composed {len(paths)} vendor files and {len(result['new_chip_jsons'])} exact chip inputs", flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--pac-source", type=Path)
    parser.add_argument("--materialize", action="store_true")
    parser.add_argument("--generator-binary", type=Path)
    parser.add_argument("--write-manifest", action="store_true")
    args = parser.parse_args()
    checkout = args.pac_source
    if checkout is None:
        candidates = list((ROOT / ".tools/cargo/git/checkouts").glob("stm32-data-generated-*/e463add"))
        if len(candidates) != 1:
            raise ValueError("Pass --pac-source with the fixed generated checkout")
        checkout = candidates[0]
    revision = subprocess.check_output(["git", "-C", str(checkout), "rev-parse", "HEAD"], text=True).strip()
    if revision != PAC_REVISION:
        raise ValueError("Wrong fixed PAC source revision")
    build_json(checkout)
    print("Generated exact F723RC/RE VQFPN68 JSON and source-difference audit", flush=True)
    if args.materialize and args.generator_binary:
        raise ValueError("Choose --materialize or --generator-binary")
    binary = common.build_generator() if args.materialize else args.generator_binary
    if binary:
        materialize_vendor(checkout, generate_pac(checkout, binary))
        print("Materialized two independent F723 package PACs; LI sources preserved", flush=True)
    if args.write_manifest:
        write_manifests()


if __name__ == "__main__":
    main()
