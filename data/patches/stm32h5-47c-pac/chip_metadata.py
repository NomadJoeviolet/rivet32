"""Exact DIE47C chip/package/IRQ/DMA scaffolds for stm32-metapac-gen.

This module does not supply registers or RCC mappings and does not claim HAL
support. Constants not present in evidence.json are checked against its pinned
ST source headers; external-memory windows are never treated as installed RAM.
"""
from collections import defaultdict
from functools import lru_cache
import hashlib
import json
from pathlib import Path
import re


def _natural(value):
    return [int(part) if part.isdigit() else part for part in re.split(r"(\d+)", value)]


@lru_cache(maxsize=2)
def source_facts(family):
    if family not in ("STM32H543", "STM32H553"):
        raise ValueError("this generator accepts only exact DIE47C families")
    root = next((p for p in Path(__file__).resolve().parents
                 if (p / "data/patches/stm32h5-47c/evidence.json").is_file()), None)
    if root is None:
        raise ValueError("DIE47C pinned source evidence is missing")
    base = root / "data/patches/stm32h5-47c"
    source_lock = json.loads((base / "sources.json").read_text(encoding="utf-8"))
    names = (family.lower() + "xx.h", "stm32h5xx_hal_flash.h", "stm32h5xx_hal_flash_ex.h")
    sources = {}
    for name in names:
        raw = (base / "sources" / name).read_bytes()
        if hashlib.sha256(raw).hexdigest() != source_lock["files"][name]["sha256"]:
            raise ValueError("source SHA mismatch: " + name)
        sources[name] = raw.decode("utf-8")
    header = sources[names[0]]
    def literal(name):
        match = re.search(r"^#define\s+" + re.escape(name) + r"\s+\(?(0x[\dA-Fa-f]+|\d+)[ULul]*\)?\s", header, re.M)
        if match is None:
            raise ValueError("missing literal source fact: " + name)
        return int(match[1], 0)
    bank = re.search(r"^#define\s+FLASH_BANK_SIZE\s+\(FLASH_SIZE\s*>>\s*1U\)", header, re.M)
    if bank is None:
        raise ValueError("DIE47C bank-size policy no longer matches the source")
    flash = sources["stm32h5xx_hal_flash.h"]
    if not (re.search(r"^#define\s+FLASH_TYPEPROGRAM_QUADWORD\s+FLASH_CR_PG\b", flash, re.M)
            and re.search(r"^#define\s+FLASH_TYPEPROGRAM_HALFWORD_OTP\s+", flash, re.M)):
        raise ValueError("main flash/OTP programming granularity source changed")
    for name in ("FLASH_BANK_1", "FLASH_BANK_2"):
        if not re.search(r"^#define\s+" + name + r"\s", sources["stm32h5xx_hal_flash_ex.h"], re.M):
            raise ValueError("missing flash bank source: " + name)
    # C preprocessing joins escaped physical lines before parsing a macro.
    logical_header = re.sub(r"\\\r?\n", "", header)
    two_d = re.search(r"^#define[ \t]+IS_DMA_2D_ADDRESSING_INSTANCE\([^\r\n]+", logical_header, re.M)
    if two_d is None:
        raise ValueError("missing exact DMA 2D capability predicate")
    return dict(nvic_priority_bits=literal("__NVIC_PRIO_BITS"),
                otp_address=literal("FLASH_OTP_BASE"), otp_size=literal("FLASH_OTP_SIZE"),
                channels_2d=set(re.findall(r"(GPDMA[12]_Channel\d+)_NS", two_d[0])),
                source_sha256={n: source_lock["files"][n]["sha256"] for n in names})


def _validate_chip(chip):
    family = chip["family"]
    if (family not in ("STM32H543", "STM32H553") or chip["die"] != "DIE47C"
            or not chip["name"].startswith(family) or chip["flash_bytes"] not in (512*1024,1024*1024)
            or chip["flash_base_ns"] != 0x08000000 or chip["flash_sector_size"] != 8192
            or not chip["packages"]):
        raise ValueError("inconsistent exact DIE47C chip record")
    for package in chip["packages"]:
        if (package["refname"][:11] != chip["name"] or package["die"] != chip["die"]
                or package["flash_bytes"] != chip["flash_bytes"]
                or package["ram_bytes"] != sum(b["size"] for b in chip["ram"] if b["name"] != "BKPSRAM")):
            raise ValueError("package capacity/identity does not match its exact chip")


def chip_scaffold(chip_record, family_evidence):
    _validate_chip(chip_record)
    family = chip_record["family"]
    if family_evidence["header"] != family.lower() + "xx.h":
        raise ValueError("chip and register-evidence families differ")
    facts = source_facts(family)
    packages = []
    pins = set()
    for package in sorted(chip_record["packages"], key=lambda p: p["refname"]):
        physical = []
        for pin in package["pins"]:
            gpio = pin["gpio"]
            if gpio:
                if not re.fullmatch(r"P[A-K](?:[0-9]|1[0-5])", gpio):
                    raise ValueError("invalid GPIO identity: " + gpio)
                pins.add(gpio)
            physical.append(dict(position=pin["Position"], signals=[gpio or pin["Name"]]))
        physical.sort(key=lambda p: _natural(p["position"]))
        packages.append(dict(name=package["refname"], package=package["package"], pins=physical))
    half = chip_record["flash_bytes"] // 2
    memory = [dict(name=f"BANK_{i+1}", kind="flash", address=chip_record["flash_base_ns"] + i*half,
                   size=half, settings=dict(erase_size=chip_record["flash_sector_size"], write_size=16, erase_value=255))
              for i in range(2)]
    memory.append(dict(name="OTP", kind="flash", address=facts["otp_address"], size=facts["otp_size"],
                       settings=dict(erase_size=0, write_size=2, erase_value=255)))
    memory.extend(dict(name=bank["name"], kind="ram", address=bank["address_ns"], size=bank["size"])
                  for bank in chip_record["ram"])
    irqs = family_evidence["interrupts"]
    if len(set(irqs.values())) != len(irqs) or any(number < 0 for number in irqs.values()):
        raise ValueError("duplicate or negative device interrupt number")
    channels = []
    for controller in (1, 2):
        dma = f"GPDMA{controller}"
        for channel in range(8):
            pointer = f"{dma}_Channel{channel}"
            if pointer + "_NS" not in family_evidence["pointers"]:
                raise ValueError("required channel absent from exact CMSIS: " + pointer)
            channels.append(dict(name=f"{dma}_CH{channel}", dma=dma, channel=channel,
                                 supports_2d=pointer in facts["channels_2d"]))
    return dict(name=chip_record["name"], family="STM32H5", line=family, die=chip_record["die"],
                device_id=0x47C, packages=packages, memory=[memory], docs=[],
                cores=[dict(name="cm33", peripherals=[], nvic_priority_bits=facts["nvic_priority_bits"],
                            interrupts=[dict(name=name, number=number) for name,number in sorted(irqs.items(),key=lambda x:(x[1],x[0]))],
                            dma_channels=channels, pins=[dict(name=pin) for pin in sorted(pins,key=_natural)])])


def _signal(name):
    if "_" not in name:
        return None
    instance, signal = name.split("_", 1)
    if re.fullmatch(r"I2S[123]", instance):
        return "SPI" + instance[3:], "I2S_" + signal
    if instance == "ETH":
        signal = re.sub(r"^(?:MII|RMII)_", "", signal)
    if instance == "RCC" and signal in ("MCO1", "MCO2"):
        signal = "MCO_" + signal[-1]
    return instance, signal


def peripheral_pins(chip_record):
    _validate_chip(chip_record)
    rows = defaultdict(set)
    for package in chip_record["packages"]:
        afs = defaultdict(set)
        bonded = {pin["gpio"] for pin in package["pins"] if pin["gpio"]}
        for row in package["alternate_functions"]:
            if row["pin"] not in bonded or not 0 <= row["af"] <= 15:
                raise ValueError("AF record is not a bonded GPIO/valid AF")
            afs[row["pin"],row["signal"]].add(row["af"])
        if any(len(values) != 1 for values in afs.values()):
            raise ValueError("conflicting AF values for one package signal")
        for pin in package["pins"]:
            if not pin["gpio"]:
                continue
            for entry in pin["signals"]:
                parsed = _signal(entry["Name"])
                if parsed is None:
                    # GPIO is represented by core pins. AUDIOCLK is a shared
                    # clock input rather than an independently owned instance.
                    continue
                instance, signal = parsed
                af = next(iter(afs.get((pin["gpio"], entry["Name"]), {None})))
                rows[instance].add((pin["gpio"], signal, af))
    result = {}
    for instance, values in sorted(rows.items()):
        by_signal = defaultdict(set)
        for pin,signal,af in values:
            by_signal[pin,signal].add(af)
        if any(len(afs) != 1 for afs in by_signal.values()):
            raise ValueError("package variants disagree on a signal AF: " + instance)
        result[instance] = [dict(pin=pin,signal=signal,**({} if af is None else dict(af=af)))
                            for pin,signal,af in sorted(values,key=lambda x:(_natural(x[0]),x[1],-1 if x[2] is None else x[2]))]
    return result


def peripheral_dma(family_evidence):
    result = defaultdict(list)
    for macro, request in sorted(family_evidence["dma_requests"].items()):
        match = re.fullmatch(r"(GPDMA[12])_REQUEST_([A-Z0-9]+)(?:_(.+))?", macro)
        if match is None or not isinstance(request,int) or not 0 <= request <= 255:
            raise ValueError("unrecognized exact DMA request: " + macro)
        dma, instance, signal = match.groups()
        if instance + "_NS" not in family_evidence["pointers"]:
            raise ValueError("DMA request lacks a matching peripheral: " + instance)
        result[instance].append(dict(signal=signal or instance,dma=dma,request=request))
    return {instance:sorted(rows,key=lambda x:(x["signal"],x["dma"],x["request"]))
            for instance,rows in sorted(result.items())}
