"""Publish a completed independent PAC snapshot; root vendor is never touched."""
import argparse
import json
from pathlib import Path
import shutil
import io
import zipfile
import re

import audit_outputs
import generate
import provenance
import verify

HERE=Path(__file__).resolve().parent


def archive_existing(destination,version):
    """Validate every prior published byte, then keep a deterministic snapshot."""
    if not re.fullmatch(r"v[1-9][0-9]*",version):raise ValueError("Snapshot version must be v followed by a positive integer")
    manifest_path=destination/"manifest.json"
    manifest_raw=manifest_path.read_bytes();manifest=json.loads(manifest_raw)
    if not manifest.get("completed"):raise ValueError("Previous publication was incomplete")
    expected=set(manifest["files"])|{"manifest.json"}
    actual={p.relative_to(destination).as_posix() for p in destination.rglob("*") if p.is_file() and "__pycache__" not in p.parts}
    if actual!=expected:raise ValueError("Previous snapshot inventory differs from manifest")
    contents={"manifest.json":manifest_raw}
    for name,expected_hash in manifest["files"].items():
        p=(destination/name).resolve()
        if not p.is_relative_to(destination.resolve()):raise ValueError("Unsafe previous manifest path")
        raw=p.read_bytes()
        if provenance.digest(raw)!=expected_hash:raise ValueError("Previous snapshot changed: "+name)
        contents[name]=raw
    memory=io.BytesIO()
    with zipfile.ZipFile(memory,"w",compression=zipfile.ZIP_DEFLATED,compresslevel=9) as z:
        for name,raw in sorted(contents.items()):
            info=zipfile.ZipInfo(name,date_time=(1980,1,1,0,0,0));info.compress_type=zipfile.ZIP_DEFLATED;info.external_attr=0o100644<<16
            z.writestr(info,raw)
    raw=memory.getvalue()
    receipt={"version":version,"archive_sha256":provenance.digest(raw),"previous_manifest_sha256":provenance.digest(manifest_raw),"files":len(contents)}
    return raw,receipt


def main():
    ap=argparse.ArgumentParser();ap.add_argument("--workspace",type=Path);ap.add_argument("--output",type=Path);ap.add_argument("--replace-version")
    args=ap.parse_args();root=(args.workspace or provenance.workspace()).resolve();output=(args.output or root/"target/h5-47c-pac-generation").resolve()
    destination=root/"data/patches/stm32h5-47c-pac"
    previous=None
    if destination.exists() and any(destination.iterdir()):
        if not args.replace_version:raise ValueError("Replacing a published snapshot requires --replace-version to archive it")
        previous=archive_existing(destination,args.replace_version)
    with generate.output_lock(output):
        result=json.loads((output/"validation.json").read_text(encoding="utf-8"))
        if not result.get("completed") or result.get("deterministic_replays")!=2 or len(result.get("arm_checks",[]))!=14:
            raise ValueError("Complete deterministic 14-part validation is required before publishing")
        if verify.tree_hash(output/"build/stm32-metapac")!=result["generated_files"]:raise ValueError("PAC changed since validation")
        for folder,key in (("chips","chip_inputs"),("registers","register_inputs")):
            if verify.tree_hash(output/"build/data"/folder)!=result[key]:raise ValueError("PAC input changed since validation")
        if {p.name:provenance.digest(p.read_bytes()) for p in HERE.glob("*.py")}!=result["scripts"]:raise ValueError("Scripts changed since validation")
        if provenance.digest((HERE/"sources.json").read_bytes())!=result["sources_lock_sha256"]:raise ValueError("Source lock changed since validation")
        provenance.verify(root)
        audit_outputs.audit(root,output)
        files={}
        for p in HERE.glob("*.py"):files[p.name]=p
        for name in ("README.md","PLAN.md","CLOCK-NAMES.md","sources.json"):files[name]=HERE/name
        for folder in ("sources","fixtures"):
            for p in (HERE/folder).rglob("*"):
                if p.is_file() and "__pycache__" not in p.parts:files[p.relative_to(HERE).as_posix()]=p
        for source,folder in ((output/"build/data/chips","chips"),(output/"build/data/registers","registers"),(output/"build/stm32-metapac","generated-pac")):
            for p in source.rglob("*"):
                if p.is_file():files[folder+"/"+p.relative_to(source).as_posix()]=p
        for name in ("generation.json","validation.json","register-evidence.json","ip-reuse.json","derived-ips.json","fdcan-ram.json","crs-proof.json","api-diff.json","selected-versions.json"):
            files[name]=output/name
        hashes={}
        if previous:
            raw,receipt=previous
            folder=destination/"history"/args.replace_version
            if folder.exists():raise ValueError("Previous version history already exists")
            folder.mkdir(parents=True)
            (folder/"source-snapshot.zip").write_bytes(raw)
            generate.dump(folder/"receipt.json",receipt)
        for path in (destination/"history").rglob("*"):
            if path.is_file():hashes[path.relative_to(destination).as_posix()]=provenance.digest(path.read_bytes())
        for name,path in sorted(files.items()):
            if path.suffix.lower() in (".pdf",".exe",".rlib",".o",".pyc"):raise ValueError("Unexpected non-source publication artifact: "+name)
            raw=path.read_bytes();dest=destination/name;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(raw)
            hashes[name]=provenance.digest(raw)
        # The old bytes have already been verified and archived. Remove only
        # formerly manifested paths superseded by this precise publication.
        if previous:
            prior=json.loads((destination/"manifest.json").read_text(encoding="utf-8"))
            for name in set(prior["files"])-set(hashes):
                obsolete=(destination/name).resolve()
                if not obsolete.is_relative_to(destination.resolve()):raise ValueError("Unsafe obsolete manifest path")
                obsolete.unlink()
        generate.dump(destination/"manifest.json",{"schema":1,"version":3,"completed":True,"hal_integrated":False,"files":hashes,
            "sources_lock_sha256":result["sources_lock_sha256"],"chips":14,"pac_sha256":result["generated_files"]})
        print("Published",len(hashes),"files to",destination)


if __name__=="__main__":main()
