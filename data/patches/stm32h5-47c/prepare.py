"""Build exact DIE47C PAC-input evidence, without generating/aliasing a working PAC.

Python stdlib plus GCC. All writes stay beside this script. Register sizes and
offsets are measured by compiling the actual ST typedefs, not guessed from H563.
"""
import argparse
import ast
import hashlib
import json
import operator
from pathlib import Path
import re
import subprocess
import xml.etree.ElementTree as ET

HERE = Path(__file__).resolve().parent
SOURCES = HERE / "sources"
BUILD = HERE / ".build"
NS = {"m": "http://dummy.com"}
PARTS = {"STM32H543" + suffix for suffix in ("CE", "CG", "RE", "RG", "UG", "VE", "VG", "ZE", "ZG")} | {
    "STM32H553" + suffix for suffix in ("CG", "RG", "UG", "VG", "ZG")}
OPS = {ast.Add: operator.add, ast.Sub: operator.sub, ast.Mult: operator.mul,
       ast.LShift: operator.lshift, ast.RShift: operator.rshift,
       ast.BitOr: operator.or_, ast.BitAnd: operator.and_, ast.BitXor: operator.xor}


class Constants:
    """Bounded nonnegative candidates, NOT general C integer semantics.

    Reject complements, negatives, overflow and all non-literal casts. Every
    published candidate is additionally checked against actual GCC evaluation.
    """
    def __init__(self, macros):
        self.macros = macros
        self.cache = {}
        self.active = set()

    def value(self, name):
        if name in self.cache:
            return self.cache[name]
        if name in self.active or name not in self.macros:
            raise ValueError(f"Unresolved or cyclic macro {name}")
        self.active.add(name)
        try:
            def literal_cast(match):
                value = int(match[2], 0)
                if value >= 1 << int(match[1]):
                    raise ValueError("Truncating casts are outside the accepted subset")
                return match[2]
            expression = re.sub(r"\(\s*uint(8|16|32|64)_t\s*\)\s*(0[xX][0-9A-Fa-f]+|[0-9]+)[uUlL]*\b", literal_cast, self.macros[name])
            expression = re.sub(r"\b(0[xX][0-9A-Fa-f]+|[0-9]+)[uUlL]+\b", r"\1", expression)
            value = self.node(ast.parse(expression, mode="eval").body)
            self.cache[name] = value
            return value
        except (SyntaxError, TypeError, ZeroDivisionError, OverflowError) as error:
            raise ValueError(f"Not a constant: {name}") from error
        finally:
            self.active.remove(name)

    def node(self, node):
        value = self.operation(node)
        if not 0 <= value <= 0xFFFFFFFF:
            raise ValueError("Value outside supported nonnegative 32-bit subset")
        return value

    def operation(self, node):
        if isinstance(node, ast.Constant) and type(node.value) is int:
            return node.value
        if isinstance(node, ast.Name):
            return self.value(node.id)
        if isinstance(node, ast.BinOp) and type(node.op) in OPS:
            return OPS[type(node.op)](self.node(node.left), self.node(node.right))
        if isinstance(node, ast.UnaryOp) and isinstance(node.op, ast.UAdd):
            return self.node(node.operand)
        raise ValueError("Unsupported C constant expression")


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8", newline="\n")


def verify_source_inventory(folder, source_lock):
    actual = {p.relative_to(folder).as_posix() for p in folder.rglob("*") if p.is_file()}
    expected = set(source_lock["files"])
    if actual != expected:
        raise ValueError(f"Source inventory differs from lock: extra={sorted(actual - expected)}, missing={sorted(expected - actual)}")
    for name, info in source_lock["files"].items():
        if hashlib.sha256((folder / name).read_bytes()).hexdigest() != info["sha256"]:
            raise ValueError(f"Source hash mismatch: {name}")


def expected_output_paths(folder):
    expected = {"evidence.json", *("chip-inputs/" + part + ".json" for part in PARTS),
                "register-inputs/STM32H543.json", "register-inputs/STM32H553.json"}
    actual = {p.relative_to(folder).as_posix() for sub in ("chip-inputs", "register-inputs")
              for p in (folder / sub).rglob("*.json")}
    if actual - expected:
        raise ValueError(f"Unexpected generated outputs: {sorted(actual - expected)}")
    return [folder / name for name in sorted(expected)]


def strip_includes(text):
    return re.sub(r"^\s*#\s*include[^\n]*", "", text, flags=re.M)


def macros_from(text):
    return dict(re.findall(r"^#define\s+(\w+)(?:[ \t]+([^\n]+))?$", text, re.M))


def gcc_verify_values(label, source, values, gcc, extra_macros=None):
    """Check every emitted integer candidate using ST expressions and C types."""
    probe = ["#include <stdint.h>", "#include <stdio.h>", "#define __IO volatile",
             "#define __I volatile const", "#define __O volatile", "#define __IM volatile const", source]
    for name, value in (extra_macros or {}).items():
        probe.append(f"#define {name} {value}")
    probe.append("int main(void) {")
    for name in sorted(values):
        probe.append(f'printf("{name} %llu\\n", (unsigned long long)({name}));')
    probe += ["return 0;", "}"]
    path, binary = BUILD / (label + "-constants.c"), BUILD / (label + "-constants.exe")
    path.write_text("\n".join(probe), encoding="utf-8")
    subprocess.run([gcc, "-std=c11", "-Werror", str(path), "-o", str(binary)], check=True)
    output = subprocess.check_output([str(binary)], text=True)
    actual = {name: int(number) for name, number in (line.split() for line in output.splitlines())}
    differences = {name: {"candidate": value, "gcc": actual.get(name)}
                   for name, value in values.items() if actual.get(name) != value}
    if differences:
        raise ValueError(f"C integer semantics mismatch: {differences}")
    return {"verified": len(values), "mismatches": 0,
            "values_sha256": hashlib.sha256(json.dumps(actual, sort_keys=True).encode()).hexdigest()}


def register_typedefs(raw):
    """Only primitive register blocks; exclude ROM API tables and other structs."""
    typedefs = {}
    for match in re.finditer(r"typedef\s+struct\s*\{([^{}]*)\}\s*(\w+)\s*;", raw, re.S):
        if not match[2].endswith("TypeDef"):
            continue
        body = re.sub(r"/\*.*?\*/", "", match[1], flags=re.S)
        members = []
        declarations = [s.strip() for s in body.split(";") if s.strip()]
        for declaration in declarations:
            field = re.fullmatch(r"(?:(__IO|__I|__O)\s+)?(u?int(?:8|16|32|64)_t)\s+(\w+)\s*(\[[^\]]+\])?", declaration)
            if not field:
                break
            members.append({"name": field[3], "ctype": field[2], "access": field[1] or "unqualified",
                            "array": bool(field[4])})
        else:
            if members:
                typedefs[match[2]] = members
    for source, alias in re.findall(r"typedef\s+(\w+TypeDef)\s+(\w+TypeDef)\s*;", raw):
        if source in typedefs:
            typedefs[alias] = typedefs[source]
    return typedefs


def header_data(family, gcc):
    name = family.lower() + "xx.h"
    raw = (SOURCES / name).read_text(encoding="utf-8")
    source = strip_includes(raw)
    c_path = BUILD / (family + ".h")
    c_path.write_text(source, encoding="utf-8")
    macro_text = subprocess.check_output([gcc, "-E", "-dM", "-x", "c", str(c_path)], text=True, encoding="utf-8")
    macros = macros_from(macro_text)
    constants = Constants(macros)
    typedefs = register_typedefs(raw)
    probe = ["#include <stdint.h>", "#include <stddef.h>", "#include <stdio.h>",
             "#define __IO volatile", "#define __I volatile const", "#define __O volatile", "#define __IM volatile const",
             source, "int main(void) {"]
    for typename, fields in typedefs.items():
        probe.append(f'printf("S {typename} %zu\\n", sizeof({typename}));')
        for field in fields:
            member = field["name"]
            probe.append(f'printf("F {typename} {member} %zu %zu\\n", offsetof({typename}, {member}), sizeof((({typename}*)0)->{member}));')
    probe += ["return 0;", "}"]
    probe_path = BUILD / (family + "-layout.c")
    executable = BUILD / (family + "-layout.exe")
    probe_path.write_text("\n".join(probe), encoding="utf-8")
    subprocess.run([gcc, "-std=c11", "-Werror", str(probe_path), "-o", str(executable)], check=True)
    observed = subprocess.check_output([str(executable)], text=True)
    types = {}
    by_member = {(t, f["name"]): f for t, fields in typedefs.items() for f in fields}
    for line in observed.splitlines():
        pieces = line.split()
        if pieces[0] == "S":
            types[pieces[1]] = {"size": int(pieces[2]), "members": []}
        else:
            _, typename, member, offset, size = pieces
            field = {**by_member[typename, member], "offset": int(offset), "size": int(size)}
            types[typename]["members"].append(field)
    numeric = {}
    declared_names = set(re.findall(r"^\s*#\s*define\s+(\w+)", raw, re.M))
    for name in macros:
        if name in declared_names and not name.startswith("__"):
            try:
                numeric[name] = constants.value(name)
            except ValueError:
                pass
    numeric_oracle = gcc_verify_values(family, source, numeric, gcc)
    interrupts = {name.removesuffix("_IRQn"): int(number) for name, number in
                  re.findall(r"\b(\w+_IRQn)\s*=\s*(-?\d+)\s*,?", raw) if int(number) >= 0}
    pointers = {}
    for name, value in macros.items():
        found = re.fullmatch(r"\(\(\s*(\w+TypeDef)\s*\*\s*\)\s*(\w+)\s*\)", value.strip())
        if found and (name.endswith("_NS") or name.endswith("_S")):
            try:
                pointers[name] = {"ctype": found[1], "address": constants.value(found[2]), "base_macro": found[2]}
            except ValueError:
                raise ValueError(f"Could not resolve peripheral pointer {name}")
    missing_types = {p["ctype"] for p in pointers.values()} - types.keys()
    if missing_types:
        raise ValueError(f"Peripheral layouts not measured: {sorted(missing_types)}")
    fields = {}
    for name, pos in numeric.items():
        if name.endswith("_Pos") and (mask_name := name[:-4] + "_Msk") in numeric:
            mask = numeric[mask_name]
            shifted = mask >> pos
            fields[name[:-4]] = {"position": pos, "mask": mask, "width": shifted.bit_length(),
                                 "contiguous": shifted > 0 and shifted & (shifted + 1) == 0}
    return {"header": family.lower() + "xx.h", "interrupts": interrupts, "pointers": pointers,
            "types": types, "bitfields": fields, "numeric_macros": numeric, "numeric_oracle": numeric_oracle,
            "unsupported_object_macros": {k: v for k, v in macros.items() if k in declared_names and k not in numeric and not k.startswith("__")},
            "raw_macros": macros, "raw_header": raw, "source": source}


def dictionary_diff(new, old):
    return {"added": {k: new[k] for k in sorted(new.keys() - old.keys())},
            "removed": {k: old[k] for k in sorted(old.keys() - new.keys())},
            "changed": {k: {"new": new[k], "old": old[k]} for k in sorted(new.keys() & old.keys()) if new[k] != old[k]}}


def compare(new, old, prefix):
    new_types = {k: v for k, v in new["types"].items() if k.startswith(prefix + "_") or k == prefix + "TypeDef"}
    old_types = {k: v for k, v in old["types"].items() if k.startswith(prefix + "_") or k == prefix + "TypeDef"}
    new_macros = {k: v for k, v in new["numeric_macros"].items() if k.startswith(prefix + "_")}
    old_macros = {k: v for k, v in old["numeric_macros"].items() if k.startswith(prefix + "_")}
    difference = dictionary_diff(new_macros, old_macros)
    return {"identical": bool(new_types) and new_types == old_types and new_macros == old_macros,
            "typedefs_identical": new_types == old_types, "new_typedefs": list(new_types),
            "macro_counts": {key: len(value) for key, value in difference.items()}, "macros": difference,
            "typedefs": dictionary_diff(new_types, old_types)}


def dma_requests(header, gcc):
    source = header["source"] + "\n" + strip_includes((SOURCES / "stm32h5xx_hal_dma.h").read_text(encoding="utf-8"))
    source += "\n#if IS_DMA_REQUEST(GPDMA1_REQUEST_ADC3)\n#define EVIDENCE_ADC3_VALID 1\n#else\n#define EVIDENCE_ADC3_VALID 0\n#endif\n"
    path = BUILD / (header["header"] + "-dma.c")
    path.write_text(source, encoding="utf-8")
    text = subprocess.check_output([gcc, "-E", "-dM", "-x", "c", str(path)], text=True, encoding="utf-8")
    macros = macros_from(text)
    constants = Constants(macros)
    result = {name: constants.value(name) for name in macros if re.fullmatch(r"GPDMA[12]_REQUEST_\w+", name)}
    extras = {k: v for k, v in macros.items() if k not in header["raw_macros"] and k.startswith(("GPDMA", "DMA_"))}
    oracle = gcc_verify_values(header["header"] + "-dma", header["source"], result, gcc, extras)
    return dict(sorted(result.items())), bool(constants.value("EVIDENCE_ADC3_VALID")), oracle


def rcc_inputs(header, gcc):
    source = header["source"]
    for name in ("stm32h5xx_hal_rcc.h", "stm32h5xx_hal_rcc_ex.h"):
        source += "\n" + strip_includes((SOURCES / name).read_text(encoding="utf-8"))
    path = BUILD / (header["header"] + "-rcc.c")
    path.write_text(source, encoding="utf-8")
    text = subprocess.check_output([gcc, "-E", "-dM", "-x", "c", str(path)], text=True, encoding="utf-8")
    macros = macros_from(text)
    constants = Constants(macros)
    clock_sources, unresolved = {}, {}
    for name in sorted(macros):
        if name.startswith("RCC_") and "CLKSOURCE_" in name:
            try:
                clock_sources[name] = constants.value(name)
            except ValueError:
                unresolved[name] = macros[name]
    extras = {k: v for k, v in macros.items() if k not in header["raw_macros"] and k.startswith("RCC_")}
    oracle = gcc_verify_values(header["header"] + "-rcc", header["source"], clock_sources, gcc, extras)
    return {"source_constants": clock_sources, "unresolved_source_expressions": unresolved, "numeric_oracle": oracle,
            "fields": {k: v for k, v in header["bitfields"].items() if k.startswith("RCC_")},
            "layout": header["types"]["RCC_TypeDef"],
            "status": "raw_encoded_fields_and_ST_HAL_values_not_normalized_clock_tree"}


def gpio_af():
    root = ET.parse(SOURCES / "GPIO-STM32H5(4-5)3x_gpio_v1_0_Modes.xml").getroot()
    result = {}
    for pin in root.findall("m:GPIO_Pin", NS):
        for signal in pin.findall("m:PinSignal", NS):
            for parameter in signal.findall("m:SpecificParameter", NS):
                if parameter.get("Name") != "GPIO_AF":
                    continue
                for possible in parameter.findall("m:PossibleValue", NS):
                    match = re.fullmatch(r"GPIO_AF(\d+)_.+", possible.text or "")
                    if not match:
                        raise ValueError(f"Unrecognized AF value {possible.text}")
                    result[pin.get("Name"), signal.get("Name")] = int(match[1])
    return result


def package_data(path, afs):
    root = ET.parse(path).getroot()
    ip = [dict(x.attrib) for x in root.findall("m:IP", NS)]
    pins, functions = [], []
    for pin in root.findall("m:Pin", NS):
        item = dict(pin.attrib)
        match = re.match(r"^(P[A-I]\d+)(?:\b|[-/ (])", pin.get("Name", ""))
        gpio = match[1] if match else None
        item["gpio"] = gpio
        item["signals"] = [dict(s.attrib) for s in pin.findall("m:Signal", NS)]
        pins.append(item)
        for signal in item["signals"]:
            if (gpio, signal["Name"]) in afs:
                functions.append({"pin": gpio, "signal": signal["Name"], "af": afs[gpio, signal["Name"]]})
    return {"refname": root.get("RefName"), "package": root.get("Package"), "xml": path.name,
            "die": root.findtext("m:Die", namespaces=NS), "flash_bytes": int(root.findtext("m:Flash", namespaces=NS)) * 1024,
            "ram_bytes": int(root.findtext("m:Ram", namespaces=NS)) * 1024, "ip": ip, "pins": pins,
            "alternate_functions": sorted(functions, key=lambda f: (f["pin"], f["signal"], f["af"]))}


def produce(gcc="gcc"):
    source_lock = json.loads((HERE / "sources.json").read_text(encoding="utf-8"))
    verify_source_inventory(SOURCES, source_lock)
    expected_output_paths(HERE)
    BUILD.mkdir(exist_ok=True)
    headers = {family: header_data(family, gcc) for family in ("STM32H543", "STM32H553", "STM32H563")}
    afs = gpio_af()
    chips = {}
    for path in sorted(SOURCES.glob("STM32H5[45]3*.xml")):
        package = package_data(path, afs)
        name = package["refname"][:11]
        family = name[:9]
        header = headers[family]
        macro = header["numeric_macros"]
        ram = [{"name": bank, "address_ns": macro[bank + "_BASE_NS"], "address_s": macro[bank + "_BASE_S"],
                "size": macro[bank + "_SIZE"]} for bank in ("SRAM1", "SRAM2", "SRAM3", "BKPSRAM")]
        record = {"name": name, "family": family, "die": package["die"], "flash_bytes": package["flash_bytes"],
                  "flash_base_ns": macro["FLASH_BASE_NS"], "flash_base_s": macro["FLASH_BASE_S"],
                  "flash_sector_size": macro["FLASH_SECTOR_SIZE"], "ram": ram, "packages": []}
        if name not in chips:
            chips[name] = record
        else:
            assert {k: v for k, v in chips[name].items() if k != "packages"} == {k: v for k, v in record.items() if k != "packages"}
        assert package["ram_bytes"] == sum(x["size"] for x in ram if x["name"] != "BKPSRAM")
        chips[name]["packages"].append(package)
    if set(chips) != PARTS:
        raise ValueError("Generated chip set differs from exact fourteen-part scope")
    families = {}
    prefixes = ("RCC", "PWR", "FLASH", "GPIO", "ADC", "DMA", "GTZC", "RNG", "I3C", "OCTOSPI", "XSPI",
                "ICACHE", "DCACHE", "RAMCFG", "USART", "SPI", "TIM", "FDCAN", "USB", "HASH", "PKA", "PLAY", "AES", "SAES", "CCB")
    for family in ("STM32H543", "STM32H553"):
        header = headers[family]
        compatibility = {prefix: compare(header, headers["STM32H563"], prefix) for prefix in prefixes}
        requests, dma_adc3_valid, dma_oracle = dma_requests(header, gcc)
        family_data = {"header": header["header"], "interrupts": header["interrupts"], "pointers": header["pointers"],
                       "dma_requests": requests, "dma_adc3_passes_st_hal_validator": dma_adc3_valid, "dma_numeric_oracle": dma_oracle,
                       "rcc_inputs": rcc_inputs(header, gcc), "vs_h563": compatibility,
                       "irq_vs_h563": dictionary_diff(header["interrupts"], headers["STM32H563"]["interrupts"])}
        families[family] = family_data
        registers = {"schema": "cmsis-typedef-bitfield-input-v1", "family": family, "layout_oracle": "GCC sizeof/offsetof on ST typedefs",
                     "types": header["types"], "bitfields": header["bitfields"], "numeric_macros": header["numeric_macros"],
                     "numeric_oracle": header["numeric_oracle"],
                     "unsupported_object_macros": header["unsupported_object_macros"],
                     "source_header_sha256": source_lock["files"][header["header"]]["sha256"],
                     "warning": "Raw CMSIS evidence; reset values, side effects and mux enumeration semantics need RM0481 review. Not an existing stm32-data register version."}
        write_json(HERE / "register-inputs" / (family + ".json"), registers)
    result = {"schema": "stm32-h5-47c-pac-input-v1", "status": "exact_inputs_pending_register_and_hal_integration",
              "revisions": source_lock["revisions"], "chips": [chips[k] for k in sorted(chips)], "families": families,
              "not_a_generated_chip_json": True}
    verify_source_inventory(SOURCES, source_lock)
    write_json(HERE / "evidence.json", result)
    for chip in result["chips"]:
        write_json(HERE / "chip-inputs" / (chip["name"] + ".json"), {"schema": result["schema"], **chip,
                   "family_evidence": "../evidence.json#/families/" + chip["family"],
                   "register_input": "../register-inputs/" + chip["family"] + ".json"})
    print(f"Produced exact inputs: {len(chips)} chips, {sum(len(c['packages']) for c in chips.values())} packages")
    for family, evidence in families.items():
        print(family, "IRQs", len(evidence["interrupts"]), "DMA requests", len(evidence["dma_requests"]))
        print({p: (evidence["vs_h563"][p]["identical"], evidence["vs_h563"][p]["macro_counts"]) for p in prefixes[:10]})


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--gcc", default="gcc")
    produce(parser.parse_args().gcc)
