"""Derive three driver-facing DIE47C IPs from verified register-level deltas.

No chip alias is introduced. TIM instance capabilities come from each exact
CMSIS predicate; removed controllers are never copied from the baseline chip.
"""
import ast
from copy import deepcopy
import hashlib
import json
from pathlib import Path
import re

HERE = Path(__file__).resolve().parent
REVISION = "e463add8cc54375f61c6f5f83d6b589e7fc68be2"


def members(header, name, prefix):
    """Accept only an OR-list of exact instance equalities from a C macro."""
    logical = re.sub(r"\\\r?\n", "", re.sub(r"/\*.*?\*/", "", header, flags=re.S))
    matches = list(re.finditer(r"^#define[ \t]+" + re.escape(name) + r"\((\w+)\)[ \t]*([^\r\n]+)", logical, re.M))
    if len(matches) != 1:
        raise ValueError("missing/ambiguous instance predicate: " + name)
    parameter, body = matches[0].groups()
    found = []
    pattern = r"\(\s*" + re.escape(parameter) + r"\s*\)\s*==\s*(" + prefix + r"\d+)(?:_(NS|S))?\b"
    def replace(match):
        found.append(match[1])
        return "True"
    expression = re.sub(pattern, replace, body).replace("||", " or ").strip()
    try:
        tree = ast.parse(expression, mode="eval")
    except SyntaxError as error:
        raise ValueError("unsupported instance predicate: " + name) from error
    if not found or any(not isinstance(node, (ast.Expression, ast.BoolOp, ast.Or, ast.Constant))
                        or (isinstance(node, ast.Constant) and node.value is not True)
                        for node in ast.walk(tree)):
        raise ValueError("unreviewed instance predicate semantics: " + name)
    return set(found)


def load_inputs(root, family, derived_inputs=None):
    if family not in ("STM32H543", "STM32H553"):
        raise ValueError("exact DIE47C family required")
    base = root / "data/patches/stm32h5-47c"
    sources = json.loads((base / "sources.json").read_bytes())["files"]
    headers = {}
    for chip in (family.lower(), "stm32h563"):
        name = chip + "xx.h"
        raw = (base / "sources" / name).read_bytes()
        if hashlib.sha256(raw).hexdigest() != sources[name]["sha256"]:
            raise ValueError("CMSIS source SHA mismatch: " + name)
        headers[chip] = raw.decode()
    directory = derived_inputs or HERE / "sources/derived-inputs"
    source_lock = json.loads((directory / "sources.json").read_bytes())
    if source_lock["baseline_revision"] != REVISION:
        raise ValueError("unreviewed baseline IR revision")
    baseline = {}
    for name in ("timer_v2.json", "spi_v5_i2s.json", "usb_v4.json", "usbram_32_2048.json"):
        raw = (directory / name).read_bytes()
        lock = source_lock["files"][name]
        if hashlib.sha256(raw).hexdigest() != lock["sha256"] or hashlib.sha1(b"blob " + str(len(raw)).encode() + b"\0" + raw).hexdigest() != lock["git_blob"]:
            raise ValueError("baseline IR source differs from fixed Git blob: " + name)
        baseline[name.split("_")[0]] = json.loads(raw)
    return dict(family=family, family_evidence=json.loads((base / "evidence.json").read_bytes())["families"][family],
                registers=json.loads((base / "register-inputs" / (family + ".json")).read_bytes()),
                header=headers[family.lower()], baseline_header=headers["stm32h563"],
                baseline=baseline, baseline_sources=source_lock)


def derive(root, family, derived_inputs=None):
    return derive_inputs(load_inputs(root, family, derived_inputs))


def derive_inputs(inputs):
    evidence = inputs["family_evidence"]
    expected = {
        "TIM": dict(added={}, changed={}, removed={"TIM_OR1_RTCPREEN": 2, "TIM_OR1_RTCPREEN_Msk": 2, "TIM_OR1_RTCPREEN_Pos": 1}),
        "SPI": dict(added={"SPI_CFG1_DRDS": 1 << 24, "SPI_CFG1_DRDS_Msk": 1 << 24, "SPI_CFG1_DRDS_Pos": 24}, changed={}, removed={}),
        "USB": dict(added={}, changed={}, removed={"USB_CNTR_L1XACT": 64, "USB_CNTR_L1XACT_Msk": 64, "USB_CNTR_L1XACT_Pos": 6}),
    }
    for kind, delta in expected.items():
        comparison = evidence["vs_h563"][kind]
        if not comparison["typedefs_identical"] or comparison["macros"] != delta:
            raise ValueError("unreviewed IP layout/field difference: " + kind)
    registers = deepcopy(inputs["baseline"])
    removed = {}
    for kind, field in (("timer", "RTCPREEN"), ("usb", "L1XACT")):
        count = 0
        for name, value in registers[kind].items():
            if name.startswith("fieldset/"):
                count += sum(f["name"] == field for f in value["fields"])
                value["fields"] = [f for f in value["fields"] if f["name"] != field]
        # The current baseline already omits both reserved positions.
        removed[field] = count
    spi = registers["spi"]["fieldset/CFG1"]["fields"]
    if any(field["name"] == "DRDS" or field["bit_offset"] <= 24 < field["bit_offset"] + field["bit_size"] for field in spi):
        raise ValueError("SPI DRDS would overlap a baseline field")
    spi.append(dict(name="DRDS", description="Delay Read Data Sampling", bit_offset=24, bit_size=1))
    spi.sort(key=lambda field: field["bit_offset"])
    header = inputs["header"]
    pointers = evidence["pointers"]
    timers = {name[:-3] for name in pointers if re.fullmatch(r"TIM\d+_NS", name)}
    if members(header, "IS_TIM_INSTANCE", "TIM") != timers:
        raise ValueError("TIM predicates and physical instances disagree")
    channels = {n: members(header, f"IS_TIM_CC{n}_INSTANCE", "TIM") for n in range(1, 5)}
    wide = members(header, "IS_TIM_32B_COUNTER_INSTANCE", "TIM")
    brake = members(header, "IS_TIM_BREAK_INSTANCE", "TIM")
    advanced = members(header, "IS_TIM_ADVANCED_INSTANCE", "TIM")
    # Validate the baseline IP block's essential capabilities for each real
    # instance, not for absent TIM13/14/16/17 entries in the older chip.
    capabilities = {"IS_TIM_32B_COUNTER_INSTANCE": wide, "IS_TIM_BREAK_INSTANCE": brake,
                    "IS_TIM_ADVANCED_INSTANCE": advanced,
                    **{f"IS_TIM_CC{n}_INSTANCE": values for n,values in channels.items()}}
    for predicate, values in capabilities.items():
        if values - timers or (members(inputs["baseline_header"], predicate, "TIM") & timers) != values:
            raise ValueError("instance capability differs from baseline IP block: " + predicate)
    instances = {}
    for timer in sorted(timers):
        count = sum(timer in values for values in channels.values())
        if any((timer in channels[n]) != (n <= count) for n in range(1, 5)):
            raise ValueError("non-contiguous TIM channel capability")
        if timer in advanced:
            if count != 4 or timer not in brake: raise ValueError("invalid advanced timer capabilities")
            block = "TIM_ADV"
        elif count == 4 and timer not in brake:
            block = "TIM_GP32" if timer in wide else "TIM_GP16"
        elif count == 2 and timer not in wide:
            block = "TIM_2CH_CMP" if timer in brake else "TIM_2CH"
        elif count == 0 and timer not in wide and timer not in brake:
            block = "TIM_BASIC"
        else:
            raise ValueError("no verified TIM IP block for " + timer)
        instances[timer] = dict(kind="timer", version="h5_47c", block=block)
    spis = {name[:-3] for name in pointers if re.fullmatch(r"SPI\d+_NS", name)}
    if members(header, "IS_SPI_ALL_INSTANCE", "SPI") != spis:
        raise ValueError("SPI predicate and exact instances disagree")
    for predicate in ("IS_SPI_LIMITED_INSTANCE", "IS_SPI_FULL_INSTANCE", "IS_I2S_ALL_INSTANCE"):
        # I2S predicates compare SPI controller names.
        if members(header, predicate, "SPI") != (members(inputs["baseline_header"], predicate, "SPI") & spis):
            raise ValueError("SPI/I2S instance capability differs from baseline")
    instances.update({name:dict(kind="spi",version="h5_47c",block="SPI") for name in sorted(spis)})
    numeric = inputs["registers"]["numeric_macros"]
    ram = registers["usbram"]["block/USBRAM"]["items"]
    if len(ram) != 1 or ram[0]["byte_offset"] != 0 or ram[0]["array"] != dict(len=512,stride=4) or numeric["USB_DRD_PMA_SIZE"] != 2048:
        raise ValueError("USB PMA geometry differs from the verified 32-bit IP")
    instances["USB"] = dict(kind="usb", version="h5_47c", block="USB")
    instances["USBRAM"] = dict(kind="usbram", version="32_2048", block="USBRAM")
    return dict(registers=registers, register_versions=dict(timer="h5_47c",spi="h5_47c",usb="h5_47c",usbram="32_2048"),
                instances=instances, usb_ram_address=numeric["USB_DRD_PMAADDR_NS"], usb_ram_bytes=2048,
                evidence=dict(family=inputs["family"], baseline_sources=inputs["baseline_sources"],
                              numeric_deltas=expected, removed_exposed_fields=removed,
                              timer_predicates={name:sorted(values) for name,values in capabilities.items()},
                              spi_instances=sorted(spis), driver_compatibility="requires explicit HAL cfg adaptation; no HIL"))
