"""Verify the captured public G411/G414 source comparison without network access."""
import hashlib
import json
from pathlib import Path
import re
import xml.etree.ElementTree as ET

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def main():
    provenance = json.loads((HERE/'sources.json').read_bytes())
    report = json.loads((HERE/'memory-audit.json').read_bytes())
    assert report['sources'] == provenance
    for kind, group in provenance.items():
        for row in group['selected']:
            raw = (HERE/kind/row['path']).read_bytes()
            assert len(raw) == row['bytes']
            assert hashlib.sha256(raw).hexdigest() == row['sha256']
    pdsc = HERE/'dfp/Keil.STM32G4xx_DFP.pdsc'
    assert hashlib.sha256(pdsc.read_bytes()).hexdigest() == report['pdsc_sha256']
    devices = {}

    def walk(node, inherited):
        state = {k: dict(v) for k, v in inherited.items()}
        for child in node:
            if child.tag in ['memory', 'debug', 'processor', 'compile']:
                state[child.tag+':'+child.get('name', '')] = dict(child.attrib)
        name = node.get('Dname', '')
        if re.match(r'STM32G41[14]', name):
            assert name not in devices or devices[name] == state
            devices[name] = state
        for child in node:
            if child.tag in ['devices', 'family', 'subFamily', 'device', 'variant']:
                walk(child, state)

    walk(ET.parse(pdsc).getroot().find('devices'), {})
    conflicts = []
    for chip, row in report['parts'].items():
        raw = (ROOT/row['header']).read_bytes()
        assert hashlib.sha256(raw).hexdigest() == row['header_sha256']
        macros = {name: int(value, 16) for name, value in re.findall(
            r'^#define\s+(FLASH_BASE|SRAM1_BASE|SRAM2_BASE|CCMSRAM_BASE|SRAM1_SIZE_MAX|SRAM2_SIZE|CCMSRAM_SIZE)\s+\(?(0x[0-9A-Fa-f]+)',
            raw.decode(), re.M)}
        assert macros == row['header_memory_macros']
        maximum = sum(macros.get(k, 0) for k in ['SRAM1_SIZE_MAX', 'SRAM2_SIZE', 'CCMSRAM_SIZE'])
        assert maximum == row['cmsis_max_ram_bytes'] and not row['memory_verified']
        expected_names = {name for name in devices if name[:11].lower() == chip}
        assert {p['order_code'] for p in row['dfp_packages']} == expected_names
        for package in row['dfp_packages']:
            state = devices[package['order_code']]
            assert int(state['memory:Main_Flash']['size'], 0) == package['flash_bytes']
            assert int(state['memory:SRAM']['size'], 0) == package['dfp_sram_bytes']
            assert int(state['memory:SRAM']['start'], 0) == package['dfp_sram_start']
            assert state['debug:']['svd'] == package['debug_svd']
        agree = {p['dfp_sram_bytes'] for p in row['dfp_packages']} == {maximum}
        assert agree == row['cmsis_dfp_max_ram_agree']
        if not agree: conflicts.append(chip)
    assert conflicts == report['known_conflicts']
    assert len(report['parts']) == report['exact_part_count'] == 23
    assert sum(len(r['dfp_packages']) for r in report['parts'].values()) == report['exact_package_count'] == 32
    print('Verified 23 parts / 32 packages; 8 G414 RAM conflicts remain unresolved. No HAL support asserted.')


if __name__ == '__main__': main()
