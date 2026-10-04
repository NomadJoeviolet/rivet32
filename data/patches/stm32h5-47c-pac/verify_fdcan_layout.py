"""Replay the locked ST FDCAN allocation prefix in GCC, without MMIO."""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess

import fdcan_ram
import provenance

HERE=Path(__file__).resolve().parent


def verify(root,output):
    output=output.resolve()
    if not output.is_relative_to((root/"target").resolve()):
        raise ValueError("GCC probes must stay in the ignored workspace target directory")
    provenance.verify(root)
    output.mkdir(parents=True,exist_ok=True)
    folder=HERE/"sources/fdcan-ram"
    raw=(folder/"stm32h5xx_hal_fdcan.c").read_bytes()
    definitions,prefix=fdcan_ram.hal_parts(raw)
    macros="\n".join("#define "+name+" "+value for name,value in definitions.items())
    oracle=json.loads((folder/"verified-layout.json").read_text(encoding="utf-8"))
    fields=["FDCANRAM1","FDCANRAM2"]+["SRAMCAN_"+offset for _,offset,_ in fdcan_ram.REGIONS]+["SRAMCAN_SIZE"]
    report={"hal_source_sha256":provenance.digest(raw),"gcc":subprocess.check_output(["gcc","--version"]).decode("utf-8",errors="replace").splitlines()[0],"cases":[]}
    for family in ("STM32H543","STM32H553"):
        fdcan_ram.derive(root,family)
        chip=family.lower()+"xx"
        header_raw=(root/"data/patches/stm32h5-47c/sources"/(chip+".h")).read_bytes()
        header=re.sub(r"/\*.*?\*/","",header_raw.decode("utf-8"),flags=re.S)
        for security in ("NS","S"):
            names=[name+"_"+security for name in ("PERIPH_BASE","APB1PERIPH_BASE","SRAMCAN_BASE","FDCAN1_BASE","FDCAN2_BASE","FDCAN1","FDCAN2")]
            chosen=[]
            for name in names:
                rows=re.findall(r"^#define[ \t]+"+name+r"[ \t]+[^\r\n]+",header,re.M)
                if len(rows)!=1:raise ValueError("Exact CMSIS macro not unique: "+name)
                chosen.extend(rows)
            for name in ("SRAMCAN_BASE","FDCAN1","FDCAN2"):
                rows=re.findall(r"^#define[ \t]+"+name+r"[ \t]+"+name+"_"+security+r"[ \t]*$",header,re.M)
                if len(rows)!=1:raise ValueError("Exact security alias not unique: "+name)
                chosen.extend(rows)
            if re.search(r"^#define[ \t]+FDCAN3[ \t]",header,re.M):raise ValueError("Unexpected third FDCAN")
            c="#include <stdio.h>\n#include <stdint.h>\ntypedef struct {uint32_t dummy;} FDCAN_GlobalTypeDef;\n"
            c+="\n".join(chosen)+"\n"+macros+"\n"
            c+="typedef struct {FDCAN_GlobalTypeDef *Instance;} FDCAN_HandleTypeDef;\n"
            c+="static uint32_t selected_base(FDCAN_HandleTypeDef *hfdcan) {"+prefix+"\n(void)sizeof(RAMcounter);return SramCanInstanceBase;}\n"
            c+='int main(void) {FDCAN_HandleTypeDef a={FDCAN1},b={FDCAN2};printf("%u %u",selected_base(&a),selected_base(&b));\n'
            for field in fields[2:]:c+='printf(" %u",(unsigned)'+field+');\n'
            c+='return 0;}\n'
            stem=output/(chip+"-"+security.lower())
            source=stem.with_suffix(".c");binary=stem.with_suffix(".exe" if os.name=="nt" else ".out")
            source.write_text(c,encoding="utf-8",newline="\n")
            command=["gcc","-std=c11","-Wall","-Wextra","-Werror",str(source),"-o",str(binary)]
            compiled=subprocess.run(command,capture_output=True)
            if compiled.returncode:raise RuntimeError(compiled.stderr.decode("utf-8",errors="replace"))
            values=dict(zip(fields,map(int,subprocess.check_output([str(binary)]).split()),strict=True))
            expected=next(case for case in oracle["cases"] if case["chip"]==chip and case["security_alias"]==security)
            if values!=expected["values"]:raise ValueError("Fresh GCC allocation disagrees with locked original evidence")
            report["cases"].append({"chip":chip,"security_alias":security,"cmsis_sha256":provenance.digest(header_raw),"values":values,"source_sha256":provenance.digest(c.encode()),"compile_command":command})
    provenance.verify(root)
    return report


if __name__=="__main__":
    ap=argparse.ArgumentParser();ap.add_argument("--workspace",type=Path);ap.add_argument("--output",type=Path)
    args=ap.parse_args();root=(args.workspace or provenance.workspace()).resolve()
    output=args.output or root/"target/h5-47c-fdcan-gcc"
    result=verify(root,output)
    (output/"fresh-layout.json").write_text(json.dumps(result,indent=2,sort_keys=True)+"\n",encoding="utf-8",newline="\n")
    print("Four fresh GCC layout/instance cases PASS")
