"""CMSIS-first register IR for DIE 47C; DFP supplies corroborating access only.

No reset value is inferred from C qualifiers or from another device. Registers
without explicit corroborating SVD access are marked for Raw post-processing.
"""
from copy import deepcopy
import gzip
import json
from pathlib import Path
import re
import xml.etree.ElementTree as ET
import provenance

HERE=Path(__file__).resolve().parent

FAMILIES = ("STM32H543", "STM32H553")
ALIASES = {"ADC_Common": ("adccommon", "ADC_COMMON", "ADC"),
 "DMA_Channel": ("gpdma_native", "DMA_CHANNEL", "DMA"),
 "DMA": ("gpdma_native", "DMA", "DMA"),
 "FDCAN_Global": ("fdcan_native", "FDCAN", "FDCAN"),
 "FDCAN_Config": ("fdcan_native", "FDCAN_CONFIG", "FDCAN"),
 "USB_DRD": ("usb_native", "USB", "USB"),
 "USB_DRD_PMABuffDesc": ("usb_native", "USB_PMA_DESC", "USB"),
 "TIM": ("timer_native", "TIM", "TIM"),
 "SBS": ("syscfg", "SBS", "SBS"),
 "SAI_Block": ("sai_native", "SAI_BLOCK", "SAI"),
 "SAI": ("sai_native", "SAI", "SAI"),
 "HASH_DIGEST": ("hash_native", "HASH_DIGEST", "HASH"),
 "HASH": ("hash_native", "HASH", "HASH"),
 "COMP_Common": ("comp", "COMP_COMMON", "COMP"),
 "COMPOPT": ("comp", "COMPOPT", "COMP"),
 "COMP": ("comp", "COMP", "COMP")}


def load(path):
    return json.loads(Path(path).read_text(encoding="utf-8"))


def access_policy(access):
    return {"read-only": "Read", "write-only": "Write", "read-write": "ReadWrite"}.get(access, "Raw")


def type_info(ctype):
    n = re.sub(r"_?TypeDef$", "", ctype)
    kind, block, prefix = ALIASES.get(n, (n.lower(), n.upper(), n.split("_")[0]))
    # Pinned generator substitutes metadata module placeholders with [a-z0-9].
    return kind.replace("_", ""), block, prefix


def members(record, ctype):
    return [m for m in record["types"][ctype]["members"] if not m["name"].startswith("RESERVED")]


class Svd:
    def __init__(self, path):
        self.tree = ET.fromstring(gzip.decompress(path.read_bytes())) if path.suffix==".gz" else ET.parse(path).getroot()
        self.peripherals = {p.findtext("name"): p for p in self.tree.findall("peripherals/peripheral")}

    def registers(self, name):
        if name not in self.peripherals:
            return []
        p = self.peripherals[name]
        if p.get("derivedFrom"):
            return self.registers(p.get("derivedFrom"))
        return p.findall("registers/register")

    def at(self, name, offset):
        return next((r for r in self.registers(name) if int(r.findtext("addressOffset"), 0) == offset), None)


def macro_register(prefix, reg):
    if prefix == "FLASH":
        n = re.sub(r"_(CUR|PRG)$", "", reg)
        n = re.sub(r"^(NS|SEC)(CR|SR|CCR|KEYR|OBKKEYR|OBKCFGR|EPOCHR|BOOTR)$", r"\2", n)
        n = re.sub(r"^(SECBB|PRIVBB)[12]R[12]$", r"\1R", n)
        n = re.sub(r"^(SECWM|WRP|EDATA|HDP)[12]R$", r"\1R", n)
        return "ECCR" if n in ("ECCCORR", "ECCDETR") else n
    if prefix == "I3C" and re.fullmatch(r"DEVR[1-9]", reg):
        return "DEVRX"
    return reg


def native_fields(record, prefix, reg):
    key = prefix + "_" + macro_register(prefix, reg) + "_"
    result = []
    for name, value in record["bitfields"].items():
        if not name.startswith(key):
            continue
        position, width = value["position"], value["width"]
        if name in ("I3C_TIMINGR2_STALLS", "I3C_TIMINGR2_STALLL"):
            # CMSIS masks use the preceding field's position by mistake. DFP
            # independently agrees with the correctly named CMSIS _Pos values.
            position, width = (5 if name.endswith("STALLS") else 6), 1
        elif not value["contiguous"]:
            # Noncontiguous composite masks do not describe a scalar field.
            continue
        result.append({"name": name[len(key):], "bit_offset": position, "bit_size": width})
    return sorted(result, key=lambda x: (x["bit_offset"], -x["bit_size"], x["name"]))


def baseline_registers(ir, block):
    out = {}
    for item in ir["block/" + block]["items"]:
        array = item.get("array", {"len": 1, "stride": 0})
        for i in range(array["len"]):
            out[item["byte_offset"] + i * array["stride"]] = item
    return out


def reconcile_fields(native, old, ir):
    """Retain an old name/enum only for the same named hardware field.

    Array fields are retained only while consecutive exact CMSIS positions
    exist. Named moved/shrunk fields take the new CMSIS position/width.
    """
    pending = deepcopy(native)
    out = []
    rename = {"DCACHE1EN": "DCACHEEN", "DCACHE1LPEN": "DCACHELPEN",
              "OCTOSPISEL": "OCTOSPI1SEL", "CKERPSEL": "PERSEL",
              "DACSEL": "DACHOLDSEL", "FDCANSEL": "FDCAN12SEL",
              "PLL1M": "DIVM"}
    def canon(s):
        return re.sub(r"\d", "", s)
    for f in old:
        if "array" in f:
            hits = []
            for n in range(f["array"]["len"]):
                pos = f["bit_offset"] + n * f["array"]["stride"]
                match = next((x for x in pending if x["bit_offset"] == pos and x["bit_size"] == f["bit_size"]
                              and canon(x["name"]) == canon(f["name"])), None)
                if match is None:
                    break
                hits.append(match)
            if hits:
                a = deepcopy(f); a.pop("description", None)
                a["array"]["len"] = len(hits)
                out.append(a)
                for x in hits:
                    pending.remove(x)
        else:
            match = next((x for x in pending if rename.get(x["name"], re.sub(r"^PLL[12]", "PLL", x["name"])) == f["name"]), None)
            if match is not None:
                a = deepcopy(match); a["name"] = f["name"]
                if f.get("enum") and f["bit_size"] == a["bit_size"]:
                    a["enum"] = f["enum"]
                out.append(a); pending.remove(match)
    for f in pending:
        if any(x["name"] == f["name"] for x in out):
            f["name"] += "_BITS"
        out.append(f)
    return sorted(out, key=lambda x: (x["bit_offset"], -x["bit_size"], x["name"]))


def rcc_enums(ir, constants):
    aliases = {"OCTOSPI1": "OSPI", "DACHOLD": "DACLP", "FDCAN12": "FDCAN", "ETHCLK": "ETH", "ETHPTPCLK": "ETHPTP", "ETHREFCLK": "ETHREF"}
    def variant(s):
        s = {"CLKP": "PER", "PIN": "I2S_CKIN", "PLLCLK": "PLL1_P"}.get(s, s)
        return re.sub(r"^(PLL[12])([PQR])$", r"\1_\2", s)
    # ST's GetPeriphCLKFreq uses GetHCLKFreq for these three exact muxes.
    # These are names for their input clock, before ETHPTPDIV. Match each
    # bus view to its exact RCC enable register, never rename HCLK globally.
    names={"ADCDACSEL":{"HCLK":"HCLK2","SYSCLK":"SYS"},
           "OCTOSPI1SEL":{"HCLK":"HCLK4"},"ETHPTPCLKSEL":{"HCLK":"HCLK1"},
           "RTCSEL":{"NO_CLK":"DISABLE","HSE_DIVx":"HSE_DIV_RTCPRE"}}
    for register,field in (("AHB2ENR","ADCEN"),("AHB2ENR","DAC1EN"),("AHB4ENR","OCTOSPI1EN"),("AHB1ENR","ETHEN")):
        if field not in {f["name"] for f in ir["fieldset/"+register]["fields"]}:
            raise ValueError("Clock bus naming proof no longer holds: "+register+"."+field)
    for name, fs in list(ir.items()):
        if not name.startswith("fieldset/"):
            continue
        for f in fs["fields"]:
            if not f["name"].endswith("SEL"):
                continue
            peripheral = aliases.get(f["name"][:-3], f["name"][:-3])
            prefix = "RCC_" + peripheral + "CLKSOURCE_"
            choices = []
            mask = ((1 << f["bit_size"]) - 1) << f["bit_offset"]
            for key, value in constants.items():
                if key.startswith(prefix) and value & ~mask == 0:
                    original=key[len(prefix):]
                    choices.append({"name": names.get(f["name"],{}).get(original,variant(original)), "value": value >> f["bit_offset"]})
            if choices:
                ename = f["name"]
                f["enum"] = ename
                ir["enum/" + ename] = {"bit_size": f["bit_size"], "variants": sorted(choices, key=lambda x: (x["value"], x["name"]))}
    # Upstream h5 IR has 3-bit SW/SWS. Exact 47C CMSIS has 2-bit fields;
    # derive the narrowed enum from the separately GCC-verified HAL constants.
    system = [{"name":variant(k.removeprefix("RCC_SYSCLKSOURCE_")),"value":v}
              for k,v in constants.items() if k.startswith("RCC_SYSCLKSOURCE_") and "STATUS" not in k]
    for f in ir["fieldset/CFGR"]["fields"]:
        if f["name"] in ("SW","SWS"):
            if any(v["value"] >= 1 << f["bit_size"] for v in system):
                raise ValueError("ST system clock source exceeds exact field width")
            f["enum"]="SW"
            ir["enum/SW"]={"bit_size":f["bit_size"],"variants":sorted(system,key=lambda x:x["value"])}
    # No 47C source may expose a PLL3 choice, even in an unused inherited enum.
    for name, en in ir.items():
        if name.startswith("enum/"):
            en["variants"] = [v for v in en["variants"] if "PLL3" not in v["name"]]


def critical_ir(kind, record, svd, base, constants):
    block = kind.upper()
    old = deepcopy(base)
    ir = {k: deepcopy(v) for k, v in base.items() if k.startswith("enum/")}
    by_offset = baseline_registers(old, block)
    items, raw, facts = [], [], []
    handled = set()
    for m in members(record, block + "_TypeDef"):
        reg = m["name"]
        prior = by_offset.get(m["offset"])
        item = {"name": reg, "byte_offset": m["offset"]}
        fieldset = reg
        if prior:
            if prior["name"] in handled:
                continue
            item = deepcopy(prior); item.pop("description", None)
            if "array" in item:
                item["array"]["len"] = 2
            fieldset = prior.get("fieldset", reg)
            handled.add(prior["name"])
        if kind == "flash" and reg == "SECBOOTR_PRG":
            item["name"] = "SECBOOTR_PRG"
        native = native_fields(record, block, reg)
        prior_fields = old.get("fieldset/" + fieldset, {}).get("fields", [])
        fields = reconcile_fields(native, prior_fields, old)
        if fields:
            # Native register spellings may share one fieldset (FLASH views).
            candidate = {"fields": fields}
            existing = ir.get("fieldset/" + fieldset)
            if existing is not None and existing != candidate:
                fieldset = reg
            ir["fieldset/" + fieldset] = candidate
            item["fieldset"] = fieldset
        else:
            item.pop("fieldset", None)
        sr = svd.at(block, m["offset"])
        access = access_policy(sr.findtext("access") if sr is not None else None)
        item["access"] = "ReadWrite" if access == "Raw" else access
        if access == "Raw":
            raw.append(block + "." + item["name"])
        facts.append({"register": item["name"], "cmsis_member": reg, "offset": m["offset"],
                      "access": access, "svd_reset": sr.findtext("resetValue") if sr is not None else None,
                      "reset_api": False})
        items.append(item)
    ir["block/" + block] = {"items": items}
    if kind == "rcc":
        rcc_enums(ir, constants)
    # Drop unreferenced inherited enums, including stale whole-IP APIs.
    used = {f["enum"] for k, v in ir.items() if k.startswith("fieldset/") for f in v["fields"] if "enum" in f}
    ir = {k:v for k,v in ir.items() if not k.startswith("enum/") or k[5:] in used}
    return ir, raw, facts


def generic_ir(ctype, record, svd, svd_name):
    kind, block, prefix = type_info(ctype)
    ir, items, raw, facts = {}, [], [], []
    for m in members(record, ctype):
        name = m["name"]
        item = {"name": name, "byte_offset": m["offset"]}
        if m["ctype"] not in ("uint32_t", "uint16_t", "uint8_t"):
            # Explicit nested CMSIS type, never flatten guesses about layout.
            _, nested, _ = type_info(m["ctype"])
            item["block"] = nested
            if m["array"]:
                item["array"] = {"len": m["size"] // record["types"][m["ctype"]]["size"], "stride": record["types"][m["ctype"]]["size"]}
            items.append(item)
            continue
        bits = int(m["ctype"][4:-2])
        if bits != 32:
            item["bit_size"] = bits
        if m["array"]:
            item["array"] = {"len": m["size"] // (bits // 8), "stride": bits // 8}
        fields = native_fields(record, prefix, name)
        if m["array"] and not fields:
            variants = [native_fields(record, prefix, name + str(n)) for n in range(item["array"]["len"])]
            if variants and all(v == variants[0] for v in variants):
                fields = variants[0]
            else:
                # Distinct words of an array can have different fields (RNG
                # HTSR0 repetition errors vs HTSR1 adaptive errors). Expose
                # explicit typed views in addition to the raw array.
                for n, view_fields in enumerate(variants):
                    if view_fields:
                        view = name + str(n)
                        ir["fieldset/" + view] = {"fields": view_fields}
                        items.append({"name":view,"byte_offset":m["offset"] + n*(bits//8),"fieldset":view,"access":"ReadWrite"})
                        raw.append(block + "." + view)
                        facts.append({"register":block + "." + view,"offset":m["offset"] + n*(bits//8),"access":"Raw","svd_reset":None,"reset_api":False})
        fieldset = name
        if fields:
            ir["fieldset/" + fieldset] = {"fields": fields, **({"bit_size": bits} if bits != 32 else {})}
            item["fieldset"] = fieldset
        sr = svd.at(svd_name, m["offset"])
        access = access_policy(sr.findtext("access") if sr is not None else None)
        # A relocated CMSIS sub-block or a differently named SVD register is
        # insufficient evidence for copying access from that SVD offset.
        if sr is not None:
            sname = sr.findtext("name", "")
            if not (sname.endswith("_" + name) or sname == name or (m["array"] and sname.endswith("_" + name + "0"))):
                access = "Raw"
        item["access"] = "ReadWrite" if access == "Raw" else access
        if access == "Raw":
            raw.append(block + "." + name)
        facts.append({"register": block + "." + name, "offset": m["offset"], "access": access,
                      "svd_reset": sr.findtext("resetValue") if sr is not None else None, "reset_api": False})
        items.append(item)
    ir["block/" + block] = {"items": items}
    return kind, ir, raw, facts


def build(root):
    root = Path(root)
    provenance.verify(root)
    evidence = load(root / "data/patches/stm32h5-47c/evidence.json")
    base_dir = HERE / "sources/registers"
    output = {}
    for family in FAMILIES:
        rec = load(root / "data/patches/stm32h5-47c/register-inputs" / (family + ".json"))
        ev = evidence["families"][family]
        svd = Svd(HERE / "sources" / (family + ".svd.gz"))
        registers, raw, facts, ctypes = {}, {}, {}, {}
        for kind in ("rcc", "pwr", "flash"):
            registers[kind], raw[kind], facts[kind] = critical_ir(kind, rec, svd, load(base_dir / (kind + "_h5.json")), ev["rcc_inputs"]["source_constants"])
            ctypes[kind.upper() + "_TypeDef"] = {"kind":kind,"version":family.lower()+"_47c","block":kind.upper()}
        for ctype in rec["types"]:
            if ctype in ctypes:
                continue
            names = [n.removesuffix("_NS") for n,p in ev["pointers"].items() if p["ctype"] == ctype and n.endswith("_NS")]
            svd_name = next((n for n in names if n in svd.peripherals), type_info(ctype)[2])
            kind, ir, rr, ff = generic_ir(ctype, rec, svd, svd_name)
            dest = registers.setdefault(kind, {})
            # Shared groups need distinct fieldset namespaces; namespaced
            # types keep native register names in their respective blocks.
            if dest:
                block = type_info(ctype)[1]
                renames = {k[9:]: block + "_" + k[9:] for k in ir if k.startswith("fieldset/")}
                for v in ir.values():
                    for it in v.get("items", []):
                        if "fieldset" in it:
                            it["fieldset"] = renames[it["fieldset"]]
                ir = {( "fieldset/" + renames[k[9:]] if k.startswith("fieldset/") else k):v for k,v in ir.items()}
            if set(dest) & set(ir):
                raise ValueError("Register IR collision: " + ctype)
            dest.update(ir); raw.setdefault(kind, []).extend(rr); facts.setdefault(kind, []).extend(ff)
            ctypes[ctype] = {"kind":kind,"version":family.lower()+"_47c","block":type_info(ctype)[1]}
        unmapped = []
        mapped = 0
        for ctype, spec in ctypes.items():
            entries = registers[spec["kind"]]["block/"+spec["block"]]["items"]
            offsets = set()
            for it in entries:
                ar = it.get("array", {"len":1,"stride":0})
                offsets.update(it["byte_offset"] + i*ar["stride"] for i in range(ar["len"]))
            for member in members(rec, ctype):
                if member["offset"] not in offsets:
                    unmapped.append(ctype + "." + member["name"])
                else:
                    mapped += 1
        output[family] = {"registers":registers,"raw_registers":raw,"register_evidence":facts,
                          "ctypes":ctypes,"coverage":{"mapped_members":mapped,"unmapped_members":unmapped}}
    return output
