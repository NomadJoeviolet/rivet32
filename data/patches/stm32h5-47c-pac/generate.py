"""Generate an independent 14-part PAC in this task's private output tree."""
import argparse
from contextlib import contextmanager
from copy import deepcopy
import hashlib
import json
from pathlib import Path
import re
import subprocess
import os
import shutil

import chip_metadata
import register_ir
import provenance
import derived_ip
import fdcan_ram
import crs_ip

HERE = Path(__file__).resolve().parent


def dump(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8", newline="\n")


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def rcc_metadata(name, ir):
    field = {f["name"]:(k[9:],f) for k,v in ir.items() if k.startswith("fieldset/") for f in v["fields"]}
    stem = {"ADC1":"ADC", "ADC2":"ADC", "DAC1":"DAC1", "FDCAN1":"FDCAN", "FDCAN2":"FDCAN", "USB":"USB", "SYSCFG":"SBS", "DCACHE1":"DCACHE"}.get(name,name)
    enable = next((field[n] for n in (stem+"EN", stem+"1EN") if n in field), None)
    if enable is None:
        return None
    reg, ef = enable
    bus = next((clock for prefix,clock in (("AHB1","HCLK1"),("AHB2","HCLK2"),("AHB4","HCLK4"),("APB1","PCLK1"),("APB2","PCLK2"),("APB3","PCLK3")) if reg.startswith(prefix)), None)
    if bus is None:
        return None
    result = {"bus_clock":bus,"kernel_clock":bus,"enable":{"register":reg,"field":ef["name"]}}
    for candidate in (stem+"RST", stem+"1RST"):
        if candidate in field:
            rr,rf = field[candidate]; result["reset"]={"register":rr,"field":rf["name"]}; break
    mux = {"ADC1":"ADCDACSEL","ADC2":"ADCDACSEL","ADC3":"ADCDACSEL","DAC1":"ADCDACSEL", "FDCAN1":"FDCAN12SEL","FDCAN2":"FDCAN12SEL"}.get(name,name+"SEL")
    if mux in field:
        reg,f = field[mux];result["kernel_clock"]={"register":reg,"field":f["name"]}
    elif re.fullmatch(r"TIM\d+",name):
        result["kernel_clock"] = bus + "_TIM"
    return result


def peripheral_interrupts(name, ev, baseline):
    irqs = ev["interrupts"]
    # Reuse only functional signal labels whose exact IRQ exists on DIE47C.
    old = baseline.get(name, {})
    rows = [deepcopy(x) for x in old.get("interrupts",[]) if x["interrupt"] in irqs]
    if rows:
        return rows
    if name in irqs:
        rows.append({"signal":"GLOBAL","interrupt":name})
    for suffix,signal in (("_EV","EV"),("_ER","ER"),("_S","SECURE")):
        if name+suffix in irqs:
            rows.append({"signal":signal,"interrupt":name+suffix})
    return rows


def build_inputs(root, output):
    evidence = register_ir.load(root / "data/patches/stm32h5-47c/evidence.json")
    products = register_ir.build(root)
    derived={family:derived_ip.derive(root,family) for family in products}
    can_ram={family:fdcan_ram.derive(root,family) for family in products}
    crs={family:crs_ip.derive(root,family) for family in products}
    expected_chips={r["name"]+".json" for r in evidence["chips"]}
    existing_chips={f.name for f in (output/"build/data/chips").glob("*.json")}
    if existing_chips-expected_chips:raise ValueError("Unexpected chip input files: "+repr(existing_chips-expected_chips))
    baseline_chip = register_ir.load(HERE / "sources/baseline-chip.json")
    baseline = {p["name"]:p for p in baseline_chip["cores"][0]["peripherals"]}
    base_dir = HERE / "sources/registers"
    reused = {}
    # These IPs have both exact C layout and all prefixed numeric definitions
    # identical to H563, as already GCC-checked in the frozen evidence.
    reuse = {"GPIO_TypeDef":("GPIO", "GPIOA"),"USART_TypeDef":("USART", "USART1"),
             "DCACHE_TypeDef":("DCACHE", "DCACHE1"),"ICACHE_TypeDef":("ICACHE", "ICACHE"),
             "DMA_TypeDef":("DMA", "GPDMA1"),"FDCAN_GlobalTypeDef":("FDCAN", "FDCAN1")}
    expected_regs={kind+"_"+family.lower()+"_47c.json" for family,p in products.items() for kind in p["registers"]}
    expected_regs.update(baseline[example]["registers"]["kind"]+"_"+baseline[example]["registers"]["version"]+".json" for _,example in reuse.values())
    expected_regs.update(kind+"_"+version+".json" for d in derived.values() for kind,version in d["register_versions"].items())
    expected_regs.add("fdcanram_v1.json")
    expected_regs.add("crs_v1.json")
    existing_regs={f.name for f in (output/"build/data/registers").glob("*.json")}
    if existing_regs-expected_regs:raise ValueError("Unexpected register input files: "+repr(existing_regs-expected_regs))
    for family, p in products.items():
        ev = evidence["families"][family]
        reused[family] = {}
        for ctype, (group, example) in reuse.items():
            if not ev["vs_h563"][group]["identical"]:
                raise ValueError("Unproven IP reuse: " + family + " " + group)
            spec = deepcopy(baseline[example]["registers"])
            source = base_dir / (spec["kind"]+"_"+spec["version"]+".json")
            data = source.read_bytes()
            dest = output / "build/data/registers" / source.name
            dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(data)
            p["ctypes"][ctype] = spec
            reused[family][ctype] = {"registers":spec,"sha256":hashlib.sha256(data).hexdigest(),"evidence_group":group}
        for kind, ir in p["registers"].items():
            dump(output / "build/data/registers" / (kind+"_"+family.lower()+"_47c.json"),ir)
        for kind,ir in derived[family]["registers"].items():
            dest=output/"build/data/registers"/(kind+"_"+derived[family]["register_versions"][kind]+".json")
            if dest.exists() and register_ir.load(dest)!=ir:raise ValueError("Shared derived version differs between families")
            dump(dest,ir)
        dump(output/"build/data/registers/fdcanram_v1.json",can_ram[family]["registers"])
        dump(output/"build/data/registers/crs_v1.json",crs[family]["registers"])
        p["ctypes"]["CRS_TypeDef"]=deepcopy(crs[family]["spec"])
    for record in evidence["chips"]:
        family = record["family"];ev = evidence["families"][family];product = products[family]
        chip = chip_metadata.chip_scaffold(record,ev)
        pins = chip_metadata.peripheral_pins(record);dma = chip_metadata.peripheral_dma(ev)
        peripherals = chip["cores"][0]["peripherals"]
        for ptr, value in sorted(ev["pointers"].items()):
            if not ptr.endswith("_NS"):
                continue
            original = ptr[:-3]
            if original=="ETH_MAC":
                actual=ev["pointers"].get("ETH_NS",{})
                if any(actual.get(k)!=value[k] for k in ("address","ctype")):raise ValueError("ETH_MAC is no longer an exact alias of ETH")
                continue
            name = {"SBS":"SYSCFG","USB_DRD_FS":"USB"}.get(original,original)
            spec = deepcopy(product["ctypes"][value["ctype"]])
            if name in derived[family]["instances"]:
                spec=deepcopy(derived[family]["instances"][name])
            if name == "LPUART1" and spec["kind"] == "usart" and spec["version"] == "v4":
                spec["block"] = "LPUART"
            peri = {"name":name,"address":value["address"],"registers":spec,
                    "pins":pins.get(original, pins.get(name,[])),"dma_channels":dma.get(original,[]),
                    "interrupts":peripheral_interrupts(name,ev,baseline)}
            rcc = rcc_metadata(name, product["registers"]["rcc"])
            if rcc:
                peri["rcc"] = rcc
            peripherals.append(peri)
        # Debug block is not security-aliased in ST CMSIS.
        rec = register_ir.load(root / "data/patches/stm32h5-47c/register-inputs" / (family+".json"))
        peripherals.append({"name":"DBGMCU","address":rec["numeric_macros"]["DBGMCU_BASE"],"registers":deepcopy(product["ctypes"]["DBGMCU_TypeDef"])})
        peripherals.append({"name":"USBRAM","address":derived[family]["usb_ram_address"],"registers":derived[family]["instances"]["USBRAM"]})
        for name,address in can_ram[family]["addresses"].items():
            peripherals.append({"name":name,"address":address,"registers":deepcopy(can_ram[family]["spec"])})
        dump(output / "build/data/chips" / (chip["name"]+".json"),chip)
    dump(output / "register-evidence.json",{f:{k:v for k,v in p.items() if k!="registers"} for f,p in products.items()})
    dump(output / "ip-reuse.json",reused)
    dump(output / "derived-ips.json",{family:d["evidence"] for family,d in derived.items()})
    dump(output / "fdcan-ram.json",{family:{k:v for k,v in d.items() if k!="registers"} for family,d in can_ram.items()})
    dump(output / "crs-proof.json",{family:d["evidence"] for family,d in crs.items()})
    provenance.verify(root)
    return products


RAW_API = '''
/// Access semantics have not been independently established for this register.
/// Does not implement Read or Write: safe read/modify/default-write are absent.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Raw {}
impl sealed::Access for Raw {}
impl Access for Raw {}
impl<T: Copy> Reg<T, Raw> {
    /// Caller must establish that a read is permitted and its side effects are intended.
    pub unsafe fn read_unchecked(self) -> T { unsafe { core::ptr::read_volatile(self.ptr.cast::<T>()) } }
    /// Caller must establish allowed bits, write semantics and peripheral state.
    pub unsafe fn write_unchecked(self, value: T) { unsafe { core::ptr::write_volatile(self.ptr.cast::<T>(), value) } }
}
'''


def guard_raw(output, products):
    pac = output / "build/stm32-metapac"
    common = pac / "src/common.rs"
    common.write_text(common.read_text(encoding="utf-8")+RAW_API,encoding="utf-8",newline="\n")
    metadata = pac / "src/metadata.rs"
    meta = metadata.read_text(encoding="utf-8")
    meta,n = re.subn(r"(pub enum Access \{)",r"\1\n        /// Access requires independent hardware verification and unsafe code.\n        Raw,",meta,count=1)
    if n != 1:
        raise ValueError("pinned metadata Access declaration changed")
    metadata.write_text(meta,encoding="utf-8",newline="\n")
    replacements = 0
    for family, p in products.items():
        for kind, names in p["raw_registers"].items():
            path = pac / "src/peripherals" / (kind+"_"+family.lower()+"_47c.rs")
            if not path.exists():
                # Original generator emits only referenced IR kinds.
                continue
            source = path.read_text(encoding="utf-8")
            metadata_path = pac / "src/registers" / path.name
            meta = metadata_path.read_text(encoding="utf-8")
            for full in names:
                block, method = full.split(".")
                # Match an impl section, then the precise register getter.
                impls = list(re.finditer(r"impl (\w+) \{", source))
                target = next((m for m in impls if m[1].lower() == block.replace("_","").lower()),None)
                if target is None:
                    raise ValueError("missing generated block: "+full)
                end = next((m.start() for m in impls if m.start()>target.start()),len(source))
                segment = source[target.start():end]
                rx = re.compile(r"(pub const fn "+re.escape(method.lower())+r"\([^)]*\)\s*->\s*crate::common::Reg<.*?,\s*crate::common::)(RW|R|W)(>)",re.S)
                segment,n = rx.subn(r"\1Raw\3",segment,count=1)
                if n != 1:
                    raise ValueError("missing generated register: "+full)
                source = source[:target.start()]+segment+source[end:];replacements+=1
                blocks = list(re.finditer(r'Block \{\s*name: "(\w+)"',meta))
                mb = next((m for m in blocks if m[1].lower()==block.replace("_","").lower()),None)
                if mb is None:
                    raise ValueError("missing register metadata block: "+full)
                me = next((m.start() for m in blocks if m.start()>mb.start()),len(meta))
                section = meta[mb.start():me]
                mr = re.compile(r'(BlockItem \{\s*name: "'+re.escape(method.lower())+r'",.*?access: Access::)(ReadWrite|Read|Write)',re.S)
                section,n=mr.subn(r'\1Raw',section,count=1)
                if n!=1:
                    raise ValueError("missing register metadata item: "+full)
                meta=meta[:mb.start()]+section+meta[me:]
            path.write_text(source,encoding="utf-8",newline="\n")
            metadata_path.write_text(meta,encoding="utf-8",newline="\n")
    return replacements


@contextmanager
def output_lock(output):
    output.mkdir(parents=True,exist_ok=True)
    path=output/"generation.lock"
    try:fd=os.open(path,os.O_CREAT|os.O_EXCL|os.O_WRONLY)
    except FileExistsError:
        raise RuntimeError("Output is locked; no artifact was read. Remove a stale lock only after its owner and all cargo/rustc/generator/rustfmt descendants have exited: "+str(path)) from None
    os.write(fd,str(os.getpid()).encode("ascii"));os.close(fd)
    keep=False
    try:yield
    except (KeyboardInterrupt,SystemExit):
        keep=True
        raise
    finally:
        if not keep:path.unlink()


def run_generation(root, output, inputs_only=False, rebuild_generator=False, log=None):
    if not output.is_relative_to((root/"target").resolve()):raise ValueError("Generation output must remain under this workspace target directory")
    if output.is_symlink() or (output/"build/stm32-metapac").is_symlink():raise ValueError("Generation through a symbolic link is not allowed")
    products=build_inputs(root,output)
    if inputs_only:
        return
    binary,tool_provenance=provenance.generator(root,rebuild_generator)
    subprocess.run([str(binary)],cwd=output,check=True,stdout=log,stderr=subprocess.STDOUT if log else None)
    pac=output / "build/stm32-metapac"
    # Formatting through stdin avoids recursively loading chip-specific modules.
    for path in pac.rglob("*.rs"):
        run=subprocess.run(["rustfmt","+1.98.1","--edition","2024","--emit","stdout"],input=path.read_bytes(),capture_output=True,check=True)
        path.write_bytes(run.stdout.replace(b"\r\n",b"\n"))
    count=guard_raw(output,products)
    manifest=pac / "Cargo.toml"
    manifest.write_text(manifest.read_text(encoding="utf-8").replace('    "README.md",','    "README.md",\n    "LICENSE-*",\n    "ST-LICENSE",\n    "DFP-LICENSE",\n    "NOTICE",')+"\n[workspace]\n",encoding="utf-8",newline="\n")
    for name in ("LICENSE-MIT","LICENSE-APACHE","ST-LICENSE","DFP-LICENSE"):
        shutil.copyfile(HERE/"sources"/name,pac/name)
    shutil.copyfile(HERE/"sources/pac-Cargo.lock",pac/"Cargo.lock")
    (pac/"NOTICE").write_text("Independent STM32H543/H553 DIE47C PAC, 14 exact part selections.\nGenerated with unchanged stm32-data caa36afd62510b0e6315ee0dccd1f9c65fbcac83, with explicit Raw access post-processing.\nRegister layouts/masks: ST CMSIS H5 884b8dc78e41cbfca008363342b17f4a9e8641f7 (Apache-2.0).\nSupplementary corrected SVD: Open-CMSIS-Pack STM32H5xx_DFP f954be2f9afd93c90a2c47d6c339a67a7569ed77 (Apache-2.0).\nPackages/AF: ST STM32_open_pin_data 7d1f1514ed5583ec5007ad91236b4e1d377295b1 (BSD-3-Clause; ST-LICENSE).\nBaseline register IR and shared PAC: stm32-data-generated e463add8cc54375f61c6f5f83d6b589e7fc68be2 (MIT OR Apache-2.0).\nSee sources.json and register-evidence.json in data/patches/stm32h5-47c-pac for precise corrections, provenance and limitations.\n",encoding="utf-8",newline="\n")
    dump(output / "generation.json",{"generator_revision":"caa36afd62510b0e6315ee0dccd1f9c65fbcac83","generator_binary_sha256":sha(binary),"generator":tool_provenance,"raw_register_getters":count,"chips":14,"hal_supported":False})
    print("Generated 14 PAC chips; conservative Raw getters:",count)


def main():
    ap=argparse.ArgumentParser();ap.add_argument("--workspace",type=Path);ap.add_argument("--output",type=Path);ap.add_argument("--inputs-only",action="store_true");ap.add_argument("--rebuild-generator",action="store_true")
    args=ap.parse_args();root=(args.workspace or provenance.workspace()).resolve();output=(args.output or root/"target/h5-47c-pac-generation").resolve()
    with output_lock(output):run_generation(root,output,args.inputs_only,args.rebuild_generator)


if __name__ == "__main__":
    main()
