"""One-time capture from already pinned caches; never downloads manuals."""
from pathlib import Path
import argparse
import gzip
import hashlib
import json
import subprocess
import zipfile

HERE = Path(__file__).resolve().parent
PAC = "e463add8cc54375f61c6f5f83d6b589e7fc68be2"
GEN = "caa36afd62510b0e6315ee0dccd1f9c65fbcac83"
ARCHIVE_SHA = "8b56ac7e2969e0b651662d047ae994afb5e008526425ceb8d7e8b70a03ca3071"
REGISTERS = ("rcc_h5", "pwr_h5", "flash_h5", "gpio_v2", "usart_v4", "can_fdcan_v1", "gpdma_v1", "dcache_v1", "icache_v1_4crr")


def digest(data):
    return hashlib.sha256(data).hexdigest()


def main():
    ap=argparse.ArgumentParser();ap.add_argument("--workspace",required=True,type=Path)
    root=ap.parse_args().workspace.resolve()
    # Supplemental reviews have their own fixed source receipts. Re-capture
    # may preserve them only byte-for-byte; it must not silently erase them
    # from the inventory or certify changed local files with fresh hashes.
    previous=json.loads((HERE/"sources.json").read_text(encoding="utf-8"))
    supplemental={n:r for n,r in previous["files"].items() if n.startswith(("fdcan-ram/","crs-audit/","clock-names/")) or n in ("registers/fdcanram_v1.json","registers/crs_v1.json")}
    for name,record in supplemental.items():
        if digest((HERE/"sources"/name).read_bytes())!=record["sha256"]:
            raise ValueError("Supplemental source changed; recapture cannot certify it: "+name)
    checkout=next((root/".tools/cargo/git/checkouts/stm32-data-generated-a5ed3f1859eb8323").iterdir())
    records={};sources=HERE/"sources"
    def save(name,raw,origin,**extra):
        path=sources/name;path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(raw)
        records[name]={"sha256":digest(raw),"bytes":len(raw),"origin":origin,**extra}
    def git_file(path):
        return subprocess.check_output(["git","-C",str(checkout),"show",PAC+":"+path])
    for name in REGISTERS:
        relative="data/registers/"+name+".json"
        save("registers/"+name+".json",git_file(relative),"https://raw.githubusercontent.com/embassy-rs/stm32-data-generated/"+PAC+"/"+relative)
    derived={"baseline_revision":PAC,"files":{}}
    for name in ("timer_v2.json","spi_v5_i2s.json","usb_v4.json","usbram_32_2048.json"):
        relative="data/registers/"+name;raw=git_file(relative)
        record={"sha256":digest(raw),"git_blob":hashlib.sha1(b"blob "+str(len(raw)).encode()+b"\0"+raw).hexdigest(),"source_path":relative,"url":"https://github.com/embassy-rs/stm32-data-generated/blob/"+PAC+"/"+relative}
        derived["files"][name]=record
        save("derived-inputs/"+name,raw,record["url"],git_blob=record["git_blob"])
    save("derived-inputs/sources.json",(json.dumps(derived,indent=2)+"\n").encode(),"Manifest of four exact Git blobs from "+PAC)
    lock_file=sources/"pac-Cargo.lock"
    if not lock_file.is_file():raise ValueError("The committed PAC Cargo.lock is required; do not silently re-resolve dependencies")
    save("pac-Cargo.lock",lock_file.read_bytes(),"Cargo resolution for pinned generated manifest with Rust 1.98.1; frozen before replay validation")
    save("baseline-chip.json",git_file("data/chips/STM32H563ZI.json"),"https://raw.githubusercontent.com/embassy-rs/stm32-data-generated/"+PAC+"/data/chips/STM32H563ZI.json")
    licenselock=json.loads((root/"data/patches/stm32h5-li/sources.json").read_text(encoding="utf-8"))
    for name in ("LICENSE-MIT","LICENSE-APACHE","ST-LICENSE"):
        source=licenselock["files"][name];raw=(root/"data/patches/stm32h5-li/sources"/name).read_bytes()
        if digest(raw)!=source["sha256"]:raise ValueError("License source mismatch: "+name)
        save(name,raw,source["url"])
    svdlock=json.loads((root/"target/h5-next-audit/sources.json").read_text(encoding="utf-8"))
    for name in ("STM32H543.svd","STM32H553.svd","DFP-LICENSE"):
        source=svdlock["files"][name];raw=(root/"target/h5-next-audit"/name).read_bytes()
        if digest(raw)!=source["sha256"]:raise ValueError("DFP source mismatch: "+name)
        if name.endswith(".svd"):
            save(name+".gz",gzip.compress(raw,mtime=0),source["url"],uncompressed_sha256=source["sha256"],license="Apache-2.0")
        else:save(name,raw,source["url"],license="Apache-2.0")
    frozen=root/"data/patches/stm32h5-47c"
    files=("evidence.json","register-inputs/STM32H543.json","register-inputs/STM32H553.json","sources.json","sources/stm32h543xx.h","sources/stm32h553xx.h","sources/stm32h563xx.h","sources/stm32h5xx_hal_flash.h","sources/stm32h5xx_hal_flash_ex.h")
    archive=root/"data/sources/stm32-gaps/stm32-data-baseline.zip"
    if digest(archive.read_bytes())!=ARCHIVE_SHA:raise ValueError("Generator archive mismatch")
    checked=0
    with zipfile.ZipFile(archive) as z:
        for entry in z.infolist():
            if not entry.is_dir():
                if (root/"target/h5-li-generator-source"/entry.filename).read_bytes()!=z.read(entry):
                    raise ValueError("Built generator source differs from fixed archive: "+entry.filename)
                checked+=1
    records.update(supplemental)
    lock={"schema":1,"files":records,"frozen_inputs":{n:digest((frozen/n).read_bytes()) for n in files},
          "pac_revision":PAC,"svd_revision":svdlock["commit"],"generator":{"revision":GEN,"archive_sha256":ARCHIVE_SHA,
          "url":"https://codeload.github.com/embassy-rs/stm32-data/zip/"+GEN,"verified_extracted_source_files":checked,
          "observed_windows_executable_sha256":digest((root/"target/h5-li-generator/debug/stm32-metapac-gen.exe").read_bytes()),
          "build_command":"cargo +1.98.1 build -p stm32-metapac-gen --locked","profile":"debug"}}
    (HERE/"sources.json").write_text(json.dumps(lock,indent=2,sort_keys=True)+"\n",encoding="utf-8",newline="\n")
    print("Captured",len(records),"sources; verified",checked,"generator source files")


if __name__=="__main__":main()
