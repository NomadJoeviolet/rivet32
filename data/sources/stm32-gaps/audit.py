"""Recheck the downloaded evidence. Network probes are opt-in; no generated files are edited."""
import argparse
import concurrent.futures
import hashlib
import json
from pathlib import Path
import re
import urllib.error
import urllib.request
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[3]
CACHE = Path(__file__).resolve().parent
GENERATED = "4df045923746d724e8b76fb92d30c12e67ed208c"
REVISIONS = {
    "embassy-rs/stm32-data": "e6a417fa643efaf031abc79590ea3e76bc4edbf2",
    "embassy-rs/stm32-data-generated": GENERATED,
    "STMicroelectronics/STM32_open_pin_data": "7d1f1514ed5583ec5007ad91236b4e1d377295b1",
    "STMicroelectronics/cmsis-device-g4": "626ee412334a5ed2e5b320af5a8d77d69f03a558",
    "STMicroelectronics/cmsis-device-h5": "884b8dc78e41cbfca008363342b17f4a9e8641f7",
    "STMicroelectronics/cmsis-device-f7": "2352e888e821aa0f4fe549bd5ea81d29c67a3222",
}


def xml_summary(path):
    node = ET.parse(path).getroot()
    result = dict(node.attrib)
    wanted = {"Die", "Flash", "Ram", "CCMRam", "IONb", "Frequency"}
    for child in node:
        name = child.tag.split("}")[-1]
        if name in wanted:
            result[name] = child.text
    result["ip"] = [dict(x.attrib) for x in node if x.tag.split("}")[-1] == "IP"]
    return result


def probe(item):
    url = f"https://raw.githubusercontent.com/embassy-rs/stm32-data-generated/{GENERATED}/{item}"
    try:
        with urllib.request.urlopen(url, timeout=25) as response:
            code = response.status
    except urllib.error.HTTPError as exc:
        code = exc.code
    except (urllib.error.URLError, TimeoutError) as exc:
        code = str(exc)
    return {"url": url, "http_status": code}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--probe", action="store_true")
    args = parser.parse_args()
    missing = json.loads((ROOT / "data/generated/embassy-difference.json").read_text())["missing_embassy_features"]
    features = set(re.findall(r"^(stm32\w+)\s*=", (CACHE / "generated-latest-Cargo.toml").read_text(), re.M))
    result = {
        "audit_date": "2026-09-30",
        "revisions": REVISIONS,
        "gap_count": len(missing),
        "latest_present": sorted(set(missing) & features),
        "latest_missing": sorted(set(missing) - features),
        "xml": {p.name: xml_summary(p) for p in sorted(CACHE.glob("*.xml"))},
        "sha256": {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(CACHE.iterdir()) if p.is_file() and p.suffix in {".h", ".xml", ".pdf", ".toml"}},
    }
    if args.probe:
        chips = ["stm32f723rc", "stm32g411cc", "stm32g414rc", "stm32h543cg", "stm32h553cg", "stm32h563li", "stm32h573li", "stm32h5e4vj", "stm32h5e5zj", "stm32h5f4vj", "stm32h5f5zj", "stm32h563zi"]
        paths = [path for chip in chips for path in (f"data/chips/{chip.upper()}.json", f"stm32-metapac/src/chips/{chip}/pac.rs")]
        with concurrent.futures.ThreadPoolExecutor(max_workers=6) as executor:
            result["probes"] = list(executor.map(probe, paths))
    elif (CACHE / "audit-results.json").exists():
        previous = json.loads((CACHE / "audit-results.json").read_text())
        if "probes" in previous:
            result["probes"] = previous["probes"]
    (CACHE / "audit-results.json").write_text(json.dumps(result, indent=2), encoding="utf-8")
    print(f"gap_count={len(missing)} latest_present={len(result['latest_present'])} latest_missing={len(result['latest_missing'])}")
    for row in result.get("probes", []):
        print(row["http_status"], row["url"].split(GENERATED)[1])


if __name__ == "__main__":
    main()
