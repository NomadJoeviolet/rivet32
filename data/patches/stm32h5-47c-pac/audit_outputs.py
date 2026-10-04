"""Summarize actual emitted APIs and bindings; never changes the PAC."""
import argparse
import json
from pathlib import Path
import re

import generate
import provenance

HERE=Path(__file__).resolve().parent


def apis(ir):
    out={}
    for key,value in ir.items():
        if key.startswith("block/"):
            for item in value["items"]:
                row={k:v for k,v in item.items() if k not in ("name","description")}
                row.setdefault("access","ReadWrite")
                out[key+"."+item["name"]]=row
        if key.startswith("fieldset/"):
            for field in value["fields"]:
                out[key+"."+field["name"]]={k:v for k,v in field.items() if k not in ("name","description")}
        if key.startswith("enum/"):
            out[key]={"bit_size":value["bit_size"]}
            for variant in value["variants"]:out[key+"."+variant["name"]]=variant["value"]
    return out


def audit(root,output):
    provenance.verify(root)
    reg=output/"build/data/registers"
    evidence=generate.register_ir.load(output/"register-evidence.json")
    result={}
    for family,facts in evidence.items():
        family_report={}
        for kind in ("rcc","pwr","flash"):
            old=apis(generate.register_ir.load(HERE/"sources/registers"/(kind+"_h5.json")))
            new=apis(generate.register_ir.load(reg/(kind+"_"+family.lower()+"_47c.json")))
            family_report[kind]={"added":{k:new[k] for k in new.keys()-old.keys()},"removed":{k:old[k] for k in old.keys()-new.keys()},
                "changed":{k:{"old":old[k],"new":new[k]} for k in old.keys()&new.keys() if old[k]!=new[k]},"raw_registers":facts["raw_registers"][kind]}
        result[family]=family_report
    generate.dump(output/"api-diff.json",result)
    selected={}
    for file in sorted((output/"build/data/chips").glob("*.json")):
        chip=generate.register_ir.load(file)
        for peripheral in chip["cores"][0]["peripherals"]:
            r=peripheral["registers"];key=r["kind"]+"_"+r["version"]
            selected.setdefault(key,{"kind":r["kind"],"version":r["version"],"instances":set(),"blocks":set()})
            selected[key]["instances"].add(peripheral["name"]);selected[key]["blocks"].add(r["block"])
    for key,row in selected.items():
        row["instances"]=sorted(row["instances"]);row["blocks"]=sorted(row["blocks"])
        code=output/"build/stm32-metapac/src/peripherals"/(key+".rs")
        source=code.read_text(encoding="utf-8")
        row["source_sha256"]=provenance.digest(code.read_bytes())
        row["raw_getters"]=len(re.findall(r"->\s*crate::common::Reg<[^>]*crate::common::Raw>",source))
        row["driver_status"]="requires explicit HAL adaptation" if "47c" in row["version"] else "register IP reused after evidence comparison; full backend not asserted"
    generate.dump(output/"selected-versions.json",selected)


def main():
    ap=argparse.ArgumentParser();ap.add_argument("--workspace",type=Path);ap.add_argument("--output",type=Path)
    args=ap.parse_args();root=(args.workspace or provenance.workspace()).resolve();output=(args.output or root/"target/h5-47c-pac-generation").resolve()
    audit(root,output)


if __name__=="__main__":main()
