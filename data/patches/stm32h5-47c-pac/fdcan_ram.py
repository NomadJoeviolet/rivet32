"""DIE47C FDCAN RAM allocation from exact ST HAL macros and GCC evidence."""
import ast
import json
from pathlib import Path
import re

import provenance

HERE=Path(__file__).resolve().parent
REGIONS=(("FLSSA","FLSSA","FLS"),("FLESA","FLESA","FLE"),("RXFIFO0","RF0SA","RF0"),
         ("RXFIFO1","RF1SA","RF1"),("TXEFIFO","TEFSA","TEF"),("TXBUF","TFQSA","TFQ"))


class Constants:
    """Only positive +/* arithmetic within INT32_MAX, U suffix and uint32 cast.

    Every operand/intermediate must fit nonnegative signed 32-bit range. Thus
    removing the known uint32 cast cannot hide truncation or signed overflow.
    This intentionally does not implement general C integer expression rules.
    """
    def __init__(self,definitions):
        self.definitions=definitions;self.cache={};self.active=set()

    def value(self,name):
        if name in self.cache:return self.cache[name]
        if name in self.active or name not in self.definitions:raise ValueError("unknown/cyclic SRAMCAN constant: "+name)
        self.active.add(name)
        try:
            expression=re.sub(r"\(\s*uint32_t\s*\)","",self.definitions[name])
            expression=re.sub(r"\b(0[xX][0-9a-fA-F]+|[0-9]+)U\b",r"\1",expression)
            try:node=ast.parse(expression.strip(),mode="eval").body
            except SyntaxError as error:raise ValueError("unsupported SRAMCAN expression") from error
            def evaluate(n):
                if isinstance(n,ast.Constant) and type(n.value) is int:v=n.value
                elif isinstance(n,ast.Name):v=self.value(n.id)
                elif isinstance(n,ast.BinOp) and isinstance(n.op,(ast.Add,ast.Mult)):
                    a,b=evaluate(n.left),evaluate(n.right);v=a+b if isinstance(n.op,ast.Add) else a*b
                else:raise ValueError("unsupported SRAMCAN expression semantics")
                if not 0<=v<=0x7fffffff:raise ValueError("SRAMCAN arithmetic exceeds supported nonnegative int32 range")
                return v
            result=evaluate(node);self.cache[name]=result;return result
        finally:self.active.remove(name)


def load(path):return json.loads(path.read_text(encoding="utf-8"))


def hal_parts(raw):
    source=raw.decode("utf-8")
    no_comments=re.sub(r"/\*.*?\*/","",source,flags=re.S)
    definitions=dict(re.findall(r"^#define[ \t]+(SRAMCAN_[A-Z0-9_]+)[ \t]+([^\r\n]+)",no_comments,re.M))
    expected={"SRAMCAN_SIZE"}|{"SRAMCAN_"+offset for _,offset,_ in REGIONS}|{"SRAMCAN_"+group+suffix for _,_,group in REGIONS for suffix in ("_NBR","_SIZE")}
    if set(definitions)!=expected:raise ValueError("Unexpected HAL SRAMCAN macro inventory")
    marker="static void FDCAN_CalcultateRamBlockAddresses(FDCAN_HandleTypeDef *hfdcan)\n{"
    if source.count(marker)!=1:raise ValueError("Unexpected FDCAN allocation function")
    start=source.index(marker)+len(marker);end=source.index("  /* Standard filter list start address */",start)
    prefix=source[start:end]
    normalized=re.sub(r"\s+","",re.sub(r"/\*.*?\*/","",prefix,flags=re.S))
    expected_prefix="""uint32_t RAMcounter; uint32_t SramCanInstanceBase = SRAMCAN_BASE;
        #if defined(FDCAN2) if (hfdcan->Instance == FDCAN2) { SramCanInstanceBase += SRAMCAN_SIZE; } #endif
        #if defined(FDCAN3) if (hfdcan->Instance == FDCAN3) { SramCanInstanceBase += SRAMCAN_SIZE * 2U; } #endif"""
    if normalized!=re.sub(r"\s+","",expected_prefix):raise ValueError("Unreviewed HAL per-instance RAM selection semantics")
    return definitions,prefix


def derive(root,family):
    if family not in ("STM32H543","STM32H553"):raise ValueError("Exact DIE47C family required")
    provenance.verify(root)
    folder=HERE/"sources/fdcan-ram"
    definitions,_=hal_parts((folder/"stm32h5xx_hal_fdcan.c").read_bytes())
    constants=Constants(definitions)
    size=constants.value("SRAMCAN_SIZE")
    offsets=[constants.value("SRAMCAN_"+offset) for _,offset,_ in REGIONS]
    ir=load(HERE/"sources/registers/fdcanram_v1.json")
    items=ir["block/FDCANRAM"]["items"]
    if [i["name"] for i in items]!=[name for name,_,_ in REGIONS]:raise ValueError("FDCAN RAM IR region inventory differs")
    regions=[]
    for item,(name,offset,group) in zip(items,REGIONS):
        expected_offset=constants.value("SRAMCAN_"+offset)
        bytes_=constants.value("SRAMCAN_"+group+"_NBR")*constants.value("SRAMCAN_"+group+"_SIZE")
        if item["byte_offset"]!=expected_offset or item["array"]!={"len":bytes_//4,"stride":4} or bytes_%4:
            raise ValueError("FDCAN RAM IR differs from exact HAL byte layout: "+name)
        regions.append({"name":name,"offset":expected_offset,"bytes":bytes_})
    if any(a["offset"]+a["bytes"]!=b["offset"] for a,b in zip(regions,regions[1:])) or regions[-1]["offset"]+regions[-1]["bytes"]!=size:
        raise ValueError("Non-contiguous or truncated FDCAN RAM IR")
    base=root/"data/patches/stm32h5-47c"
    evidence=load(base/"evidence.json")["families"][family]
    present={name for name in evidence["pointers"] if re.fullmatch(r"FDCAN\d+_NS",name)}
    if present!={"FDCAN1_NS","FDCAN2_NS"}:raise ValueError("HAL allocation requires exactly two verified FDCAN instances")
    registers=load(base/"register-inputs"/(family+".json"))
    report=load(folder/"verified-layout.json")
    if report["hal_source_sha256"]!=provenance.digest((folder/"stm32h5xx_hal_fdcan.c").read_bytes()):raise ValueError("GCC report uses another HAL source")
    if report["cmsis_commit"]!=load(base/"sources.json")["revisions"]["cmsis_h5"]:raise ValueError("GCC report uses another CMSIS revision")
    cases={(c["chip"],c["security_alias"]):c for c in report["cases"]}
    if len(cases)!=4 or len(report["cases"])!=4:raise ValueError("Missing or duplicate GCC allocation cases")
    addresses={}
    for security in ("NS","S"):
        address=registers["numeric_macros"]["SRAMCAN_BASE_"+security]
        case=cases[family.lower()+"xx",security]
        header=(base/"sources"/(family.lower()+"xx.h")).read_bytes()
        if case["cmsis_sha256"]!=provenance.digest(header):raise ValueError("GCC case uses another exact CMSIS header")
        expected={"FDCANRAM1":address,"FDCANRAM2":address+size,"SRAMCAN_SIZE":size}
        expected.update({"SRAMCAN_"+offset:value for (_,offset,_),value in zip(REGIONS,offsets)})
        if case["values"]!=expected:raise ValueError("Exact source derivation disagrees with GCC arithmetic/selection")
        if security=="NS":addresses={name:expected[name] for name in ("FDCANRAM1","FDCANRAM2")}
    return {"registers":ir,"spec":{"kind":"fdcanram","version":"v1","block":"FDCANRAM"},
            "addresses":addresses,"size":size,"offsets":offsets,"evidence":{"family":family,"regions":regions,
            "hal_source_sha256":provenance.digest((folder/"stm32h5xx_hal_fdcan.c").read_bytes()),
            "gcc_oracle_sha256":provenance.digest((folder/"verified-layout.json").read_bytes()),"security_domain":"NS"}}
