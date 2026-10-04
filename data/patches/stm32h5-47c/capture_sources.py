"""Capture pinned public ST evidence in this directory; shared sources are read-only."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import os
import tempfile
import urllib.request
import zipfile

HERE = Path(__file__).resolve().parent
PIN_REV = "7d1f1514ed5583ec5007ad91236b4e1d377295b1"
CMSIS_REV = "884b8dc78e41cbfca008363342b17f4a9e8641f7"
HAL_REV = "ba20038d938ecc31399e49d60fa6bcc8e82db5da"
GEN_REV = "e6a417fa643efaf031abc79590ea3e76bc4edbf2"
GPIO = "GPIO-STM32H5(4-5)3x_gpio_v1_0_Modes.xml"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def resolve_workspace(location, explicit=None):
    if explicit is not None:
        return explicit
    workspace = next((p for p in location.parents if (p / "data/sources/st-pin-data.zip").exists()), None)
    if workspace is None:
        raise ValueError("Cannot discover source cache; pass --workspace explicitly")
    return workspace


def capture(workspace, fetch_documents=False):
    output = HERE / "sources"
    output.mkdir(parents=True, exist_ok=True)
    old = json.loads((HERE / "sources.json").read_text(encoding="utf-8")) if (HERE / "sources.json").exists() else {}
    files = {}
    def save(name, data, url, **extra):
        sha = digest(data)
        if name in old.get("files", {}) and old["files"][name]["sha256"] != sha:
            raise ValueError(f"Pinned source changed: {name}")
        path = output / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
        files[name] = {"url": url, "sha256": sha, "bytes": len(data), **extra}
    tree = json.loads((workspace / "data/sources/st-pin-tree.json").read_text(encoding="utf-8"))
    assert tree["sha"] == PIN_REV and not tree["truncated"]
    blobs = {x["path"]: x["sha"] for x in tree["tree"] if x["type"] == "blob"}
    with zipfile.ZipFile(workspace / "data/sources/st-pin-data.zip") as archive:
        prefix = f"STM32_open_pin_data-{PIN_REV}/"
        paths = sorted(p for p in blobs if p.startswith(("mcu/STM32H543", "mcu/STM32H553")))
        assert len(paths) == 29
        paths += ["mcu/IP/" + GPIO, "LICENSE", "mcu/STM32H563ZITx.xml"]
        for path in paths:
            data = archive.read(prefix + path)
            blob_hash = hashlib.sha1(b"blob " + str(len(data)).encode() + b"\0" + data).hexdigest()
            assert blob_hash == blobs[path], path
            name = "ST-pin-LICENSE" if path == "LICENSE" else Path(path).name
            save(name, data, f"https://raw.githubusercontent.com/STMicroelectronics/STM32_open_pin_data/{PIN_REV}/{path}", git_blob_sha1=blob_hash)
    cache = workspace / "data/sources/stm32-gaps"
    audit = json.loads((cache / "audit-results.json").read_text(encoding="utf-8"))
    for name in ("stm32h543xx.h", "stm32h553xx.h", "stm32h563xx.h"):
        data = (cache / name).read_bytes()
        assert digest(data) == audit["sha256"][name]
        save(name, data, f"https://raw.githubusercontent.com/STMicroelectronics/cmsis-device-h5/{CMSIS_REV}/Include/{name}")
    network = {"ST-cmsis-LICENSE": f"https://raw.githubusercontent.com/STMicroelectronics/cmsis-device-h5/{CMSIS_REV}/LICENSE.md",
               "ST-hal-LICENSE": f"https://raw.githubusercontent.com/STMicroelectronics/stm32h5xx-hal-driver/{HAL_REV}/LICENSE.md"}
    for name in ("stm32h5xx_hal_dma.h", "stm32h5xx_hal_dma_ex.h", "stm32h5xx_ll_dma.h", "stm32h5xx_hal_flash.h", "stm32h5xx_hal_flash_ex.h", "stm32h5xx_hal_pwr_ex.h", "stm32h5xx_hal_rcc.h", "stm32h5xx_hal_rcc_ex.h"):
        network[name] = f"https://raw.githubusercontent.com/STMicroelectronics/stm32h5xx-hal-driver/{HAL_REV}/Inc/{name}"
    for name, url in network.items():
        path = output / name
        if path.exists():
            data = path.read_bytes()
        else:
            with urllib.request.urlopen(url, timeout=45) as response:
                data = response.read()
        save(name, data, url)
    generator = cache / f"stm32-data-{GEN_REV}"
    gen_paths = ["data/header_map.yaml", "stm32-data-gen/src/" + "perimap.rs",
                 "stm32-data-gen/src/memory.rs", "stm32-data-gen/src/chips.rs", "stm32-data-gen/src/dma.rs",
                 "data/registers/rcc_h5.yaml", "data/registers/pwr_h5.yaml", "data/registers/flash_h5.yaml",
                 "data/registers/gpdma_v1.yaml", "data/registers/i3c_v1.yaml", "data/registers/rng_v4.yaml"]
    generator_sources = {}
    for path in gen_paths:
        data = (generator / path).read_bytes()
        generator_sources[path] = {"sha256": digest(data), "bytes": len(data),
            "url": f"https://raw.githubusercontent.com/embassy-rs/stm32-data/{GEN_REV}/{path}"}
    rng_yaml = (generator / "data/registers/rng_v4.yaml").read_text(encoding="utf-8")
    rng_nscr = rng_yaml.split("fieldset/NSCR:\n", 1)[1].split("fieldset/NSMR:", 1)[0]
    count = re.search(r"\blen:\s*(\d+)", rng_nscr)
    if not count or "- name: EN_OSC" not in rng_nscr:
        raise ValueError("Pinned RNG YAML structure changed; review extraction")
    comparison = {"schema": "stm32-h5-47c-generator-comparison-inputs-v1", "revision": GEN_REV,
                  "redistribution": "reference_metadata_and_facts_only_no_generator_source_copy",
                  "files": generator_sources, "facts": {"rng_v4_NSCR_EN_OSC_array_length": int(count[1])}}
    (HERE / "generator-inputs.json").write_text(json.dumps(comparison, indent=2) + "\n", encoding="utf-8", newline="\n")
    documents = {
        "DS15168-H543.pdf": "https://www.st.com/resource/en/datasheet/stm32h543cg.pdf",
        "DS15167-H553.pdf": "https://www.st.com/resource/en/datasheet/stm32h553cg.pdf",
        "ES0683.pdf": "https://www.st.com/resource/en/errata_sheet/es0683-stm32h543xx-stm32h553xx-device-errata-stmicroelectronics.pdf",
        "RM0481-H543-H553.pdf": "https://www.st.com/resource/en/reference_manual/rm0481-stm32h523533xx-stm32h543553xx-stm32h562xx-and-stm32h563573xx-armbased-32bit-mcus-stmicroelectronics.pdf",
    }
    document_results = {d["name"]: d for d in old.get("documents", [])} if not fetch_documents else {}
    document_cache = workspace / "data/sources/stm32h5-47c-documents"
    for name, url in documents.items():
        try:
            cached_path = document_cache / name
            if cached_path.exists():
                data = cached_path.read_bytes()
            elif fetch_documents:
                with urllib.request.urlopen(url, timeout=45) as response:
                    data = response.read()
            else:
                continue
            if not data.startswith(b"%PDF-"):
                raise ValueError("response is not PDF")
            if not cached_path.exists():
                document_cache.mkdir(parents=True, exist_ok=True)
                with tempfile.NamedTemporaryFile(dir=document_cache, suffix=".tmp", delete=False) as temporary:
                    temporary.write(data)
                    pending = Path(temporary.name)
                try:
                    os.replace(pending, cached_path)
                finally:
                    pending.unlink(missing_ok=True)
            document_results[name] = {"name": name, "url": url, "status": "cached_outside_patch", "sha256": digest(data),
                                      "cache_path": "data/sources/stm32h5-47c-documents/" + name}
        except (OSError, ValueError) as error:
            document_results[name] = {"name": name, "url": url, "status": "unavailable", "error": str(error)}
    result = {"schema": "stm32-h5-47c-source-lock-v1", "revisions": {"pin_data": PIN_REV, "cmsis_h5": CMSIS_REV,
              "hal_h5": HAL_REV, "generator": GEN_REV},
              "licenses": {"pin_data": {"spdx": "BSD-3-Clause", "file": "ST-pin-LICENSE"},
                           "cmsis_h5": {"spdx": "Apache-2.0", "file": "ST-cmsis-LICENSE"},
                           "hal_h5": {"spdx": "BSD-3-Clause", "file": "ST-hal-LICENSE"}},
              "files": files, "documents": list(document_results.values())}
    (HERE / "sources.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(f"Captured {len(files)} sources, including 29 exact package XMLs")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--workspace", type=Path)
    parser.add_argument("--fetch-documents", action="store_true")
    args = parser.parse_args()
    capture(resolve_workspace(HERE, args.workspace), args.fetch_documents)
