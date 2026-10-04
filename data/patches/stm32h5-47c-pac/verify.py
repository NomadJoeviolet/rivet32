"""Deterministic replay, real PAC compilation and fail-closed access regressions."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import time

import provenance
import generate
import verify_fdcan_layout

HERE=Path(__file__).resolve().parent


def run(command, logfile, expected=0):
    proc=subprocess.run(command,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
    logfile.parent.mkdir(parents=True,exist_ok=True);logfile.write_bytes(proc.stdout)
    if (proc.returncode==0)!=(expected==0):
        raise RuntimeError("Unexpected command result: "+repr(command)+"\n"+proc.stdout.decode(errors="replace"))
    return {"command":command,"exit_code":proc.returncode,"log_sha256":provenance.digest(proc.stdout)}


def tree_hash(directory):
    return {p.relative_to(directory).as_posix():provenance.digest(p.read_bytes()) for p in sorted(directory.rglob("*")) if p.is_file() and p.name!="Cargo.lock"}


def raw_gates(pac, output):
    output.mkdir(parents=True,exist_ok=True)
    common=(pac/"src/common.rs").as_posix()
    head='#![no_std]\n#[path="'+common+'"] mod common;\n'
    cases={"read":"r.read();","write":"r.write(|_| {});","modify":"r.modify(|_| {});","write_value":"r.write_value(0);","reset":"r.reset();","unsafe_required":"r.read_unchecked();"}
    results={}
    for name,body in cases.items():
        path=output/(name+".rs");path.write_text(head+'fn check(r:common::Reg<u32,common::Raw>) { '+body+' }',encoding="utf-8")
        result=run(["rustc","+1.98.1","--edition","2024","--crate-type","lib","--target","thumbv8m.main-none-eabihf",str(path),"-o",str(output/(name+".rlib"))],output/(name+".log"),expected=1)
        diagnostic=(output/(name+".log")).read_text(encoding="utf-8")
        required="E0133" if name=="unsafe_required" else "E0599"
        if required not in diagnostic:raise RuntimeError("Raw gate failed for an unrelated reason: "+name)
        results[name]=result
    path=output/"unsafe_allowed.rs"
    path.write_text(head+'pub unsafe fn check(r:common::Reg<u32,common::Raw>) -> u32 { unsafe {r.write_unchecked(0);r.read_unchecked()} }',encoding="utf-8")
    results["unsafe_allowed"]=run(["rustc","+1.98.1","--edition","2024","--crate-type","lib","--target","thumbv8m.main-none-eabihf",str(path),"-o",str(output/"unsafe_allowed.rlib")],output/"unsafe_allowed.log")
    return results


def verify(root, output, existing=False):
    report={"completed":False,"hal_integrated":False};generate.dump(output/"validation.json",report)
    lock=provenance.verify(root)
    scripts={p.name:provenance.digest(p.read_bytes()) for p in sorted(HERE.glob("*.py"))}
    report["python_tests"]=run([sys.executable,"-m","unittest","discover","-s",str(HERE),"-p","test_*.py"],output/"logs/python-tests.log")
    report["fdcan_layout_gcc"]=verify_fdcan_layout.verify(root,output/"fdcan-gcc")
    if not existing:
        for index in (1,2):
            log=output/"logs"/("generate-"+str(index)+".log")
            with log.open("wb") as handle:generate.run_generation(root,output,log=handle)
            report["generate_"+str(index)]={"exit_code":0,"log_sha256":provenance.digest(log.read_bytes())}
            if index==1:first=tree_hash(output/"build/stm32-metapac")
        second=tree_hash(output/"build/stm32-metapac")
        if first!=second:raise ValueError("PAC regeneration is nondeterministic")
        report["deterministic_replays"]=2
    pac=output/"build/stm32-metapac"
    report["generated_files"]=tree_hash(pac)
    report["chip_inputs"]=tree_hash(output/"build/data/chips")
    report["register_inputs"]=tree_hash(output/"build/data/registers")
    chips=sorted(p.stem for p in (output/"build/data/chips").glob("*.json"))
    expected=sorted(c["name"] for c in generate.register_ir.load(root/"data/patches/stm32h5-47c/evidence.json")["chips"])
    if chips!=expected:raise ValueError("Generated chip inventory is not the exact 14 DIE47C parts")
    report["arm_checks"]=[]
    for chip in chips:
        command=["cargo","+1.98.1","check","--manifest-path",str(pac/"Cargo.toml"),"--target","thumbv8m.main-none-eabihf","--features",chip.lower()+",metadata,rt","--offline","--target-dir",str(root/"target/h5-47c-pac-check")]
        if (pac/"Cargo.lock").is_file():command.append("--locked")
        report["arm_checks"].append({"chip":chip,**run(command,output/"logs"/(chip+"-check.log"))})
        print(chip,"ARM check PASS",flush=True)
    fixtures=output/"fixtures";fixtures.mkdir(exist_ok=True)
    for name in ("Cargo.toml","Cargo.lock","registers.rs"):
        shutil.copyfile(HERE/"fixtures"/name,fixtures/name)
    report["rust_golden_tests"]=run(["cargo","+1.98.1","test","--manifest-path",str(fixtures/"Cargo.toml"),"--offline","--locked","--target-dir",str(root/"target/h5-47c-fixtures")],output/"logs/rust-golden.log")
    report["raw_gate_tests"]=raw_gates(pac,output/"raw-gates")
    provenance.verify(root)
    if scripts!={p.name:provenance.digest(p.read_bytes()) for p in sorted(HERE.glob("*.py"))}:raise ValueError("Preparation scripts changed during validation")
    report["scripts"]=scripts
    report.update(completed=True,sources_lock_sha256=provenance.digest((HERE/"sources.json").read_bytes()),source_files=len(lock["files"]),frozen_inputs=len(lock["frozen_inputs"]))
    generate.dump(output/"validation.json",report)
    print("14 ARM checks, Rust golden tests and Raw gates PASS",flush=True)


def main():
    ap=argparse.ArgumentParser();ap.add_argument("--workspace",type=Path);ap.add_argument("--output",type=Path);ap.add_argument("--existing",action="store_true")
    args=ap.parse_args();root=(args.workspace or provenance.workspace()).resolve();output=(args.output or root/"target/h5-47c-pac-generation").resolve()
    with generate.output_lock(output):verify(root,output,args.existing)

if __name__=="__main__":main()
