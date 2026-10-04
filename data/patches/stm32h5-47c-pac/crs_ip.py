"""Reuse CRS v1 only after exact CMSIS and independent GCC comparisons."""
import json
from pathlib import Path
import provenance

HERE=Path(__file__).resolve().parent
def load(path):return json.loads(path.read_text(encoding="utf-8"))


def check_evidence(actual,baseline,ir):
    a,b=actual["ip"]["CRS"],baseline["ip"]["CRS"]
    if a["type"]!=b["type"] or a["numeric_macros"]!=b["numeric_macros"]:
        raise ValueError("CRS exact CMSIS layout/macros differ from proved v1 IP")
    if a["integer_macro_count"]!=86 or len(a["numeric_macros"])!=86:
        raise ValueError("Unexpected CRS macro inventory")
    members={m["name"]:m for m in a["type"]["members"]}
    if a["type"]["size"]!=16 or set(members)!={i["name"] for i in ir["block/CRS"]["items"]}:
        raise ValueError("CRS register inventory differs")
    for item in ir["block/CRS"]["items"]:
        member=members[item["name"]]
        if member["offset"]!=item["byte_offset"] or member["size"]!=4 or member["array"]:
            raise ValueError("CRS register offset/width differs")
        prefix="CRS_"+item["name"]+"_"
        field_names={n.removeprefix(prefix).removesuffix("_Pos") for n in a["numeric_macros"] if n.startswith(prefix) and n.endswith("_Pos")}
        fields=ir["fieldset/"+item["fieldset"]]["fields"]
        if field_names!={f["name"] for f in fields}:raise ValueError("CRS field inventory differs")
        for field in fields:
            key=prefix+field["name"]
            if a["numeric_macros"][key+"_Pos"]!=field["bit_offset"] or a["numeric_macros"][key+"_Msk"]!=((1<<field["bit_size"])-1)<<field["bit_offset"]:
                raise ValueError("CRS field offset/mask differs")
    enum=actual["crs_hal_source_selection"]
    values={name.removeprefix("RCC_CRS_SYNC_SOURCE_"):value for name,value in enum["field_values"].items()}
    if values!={v["name"]:v["value"] for v in ir["enum/SYNCSRC"]["variants"]}:
        raise ValueError("CRS synchronization source enum differs from ST HAL")
    if enum["oracle"]["verified"]!=3 or enum["oracle"]["mismatches"]!=0:
        raise ValueError("CRS GCC enum oracle failed")
    if a["interrupts"]!={"CRS":75}:raise ValueError("Unexpected exact CRS interrupt")


def derive(root,family):
    if family not in ("STM32H543","STM32H553"):raise ValueError("Exact DIE47C family required")
    provenance.verify(root)
    folder=HERE/"sources/crs-audit"
    origin=load(folder/"provenance.json")
    frozen=root/"data/patches/stm32h5-47c"
    lock=load(frozen/"sources.json")
    for name in ("stm32h543xx.h","stm32h553xx.h","stm32h563xx.h","stm32h5xx_hal_rcc_ex.h"):
        if origin["sources"][name]["sha256"]!=lock["files"][name]["sha256"] or provenance.digest((frozen/"sources"/name).read_bytes())!=origin["sources"][name]["sha256"]:
            raise ValueError("CRS proof uses another ST source: "+name)
    ir=load(HERE/"sources/registers/crs_v1.json")
    if provenance.digest((HERE/"sources/registers/crs_v1.json").read_bytes())!=origin["sources"]["crs_v1.json"]["sha256"]:
        raise ValueError("CRS proof uses another register IR")
    actual=load(folder/(family+".json"));baseline=load(folder/"STM32H563.json")
    check_evidence(actual,baseline,ir)
    native=load(frozen/"register-inputs"/(family+".json"))
    if native["types"]["CRS_TypeDef"]!=actual["ip"]["CRS"]["type"]:
        raise ValueError("CRS audit and original frozen CMSIS layout disagree")
    macros={k:v for k,v in native["numeric_macros"].items() if k.startswith("CRS_")}
    if macros!=actual["ip"]["CRS"]["numeric_macros"]:raise ValueError("CRS audit and original frozen macros disagree")
    return {"registers":ir,"spec":{"kind":"crs","version":"v1","block":"CRS"},"evidence":{"family":family,"integer_macros":86,"typedef_bytes":16,"irq":75,"ir_sha256":origin["sources"]["crs_v1.json"]["sha256"],"gcc_report_sha256":provenance.digest((folder/(family+".json")).read_bytes()),"access_policy":"Pinned v1 access; no new permission inferred from CMSIS __IO"}}
