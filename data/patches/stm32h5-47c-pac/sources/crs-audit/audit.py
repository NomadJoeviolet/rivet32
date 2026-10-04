"""Read-only source audit. All outputs are local to this target directory."""
from pathlib import Path
import hashlib
import importlib.util
import json
import re
import shutil
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE = ROOT / 'data/patches/stm32h5-47c'
REV = 'e463add8cc54375f61c6f5f83d6b589e7fc68be2'
GIT = ROOT / '.tools/cargo/git/checkouts/stm32-data-generated-a5ed3f1859eb8323/e463add'
spec = importlib.util.spec_from_file_location('essential_prepare', SOURCE / 'prepare.py')
prepare = importlib.util.module_from_spec(spec)
spec.loader.exec_module(prepare)
prepare.BUILD = HERE / 'build'
prepare.BUILD.mkdir(parents=True, exist_ok=True)
gcc = shutil.which('gcc')
assert gcc, 'GCC is required'
lock = json.loads((SOURCE / 'sources.json').read_text(encoding='utf-8'))
headers = {}
provenance = {'cmsis_revision': lock['revisions']['cmsis_h5'], 'hal_revision': lock['revisions']['hal_h5'], 'metapac_revision': REV, 'sources': {}, 'gcc': subprocess.check_output([gcc, '--version'], text=True).splitlines()[0]}

for chip in ('STM32H543', 'STM32H553', 'STM32H563'):
    filename = chip.lower() + 'xx.h'
    actual = hashlib.sha256((prepare.SOURCES / filename).read_bytes()).hexdigest()
    assert actual == lock['files'][filename]['sha256']
    provenance['sources'][filename] = lock['files'][filename]
    print('Compiling exact CMSIS layout and all integer macros:', chip, flush=True)
    headers[chip] = prepare.header_data(chip, gcc)

irs = {}
for ip, version in (('CRS', 'crs_v1'), ('EXTI', 'exti_h5')):
    blob = subprocess.check_output(['git', '-C', str(GIT), 'show', REV + ':data/registers/' + version + '.json'])
    (HERE / (version + '.json')).write_bytes(blob)
    irs[ip] = json.loads(blob)
    provenance['sources'][version + '.json'] = {'sha256': hashlib.sha256(blob).hexdigest(), 'bytes': len(blob), 'url': 'https://raw.githubusercontent.com/embassy-rs/stm32-data-generated/' + REV + '/data/registers/' + version + '.json'}

def expand(array):
    return [(n, n * array['stride']) for n in range(array['len'])] if array else [(None, 0)]

def compare_ir(chip, ip):
    h, ir = headers[chip], irs[ip]
    ctype = h['types'][ip + '_TypeDef']
    c_regs = {}
    for member in ctype['members']:
        if member['name'].startswith('RESERVED'):
            continue
        count = member['size'] // 4 if member['array'] else 1
        for n in range(count):
            name = member['name'] + str(n + 1) if member['array'] else member['name']
            c_regs[member['offset'] + n * 4] = (name, member['access'])
    rows = []
    for item in ir['block/' + ip]['items']:
        for n, offset in expand(item.get('array')):
            offset += item['byte_offset']
            cname, caccess = c_regs.pop(offset, (None, None))
            name = item['name'] + (str(n + 1) if n is not None else '')
            # ST names the type members SECCFGR/PRIVCFGR but their macros
            # SECENR/PRIVENR in all three exact headers. Record this alias.
            macro_reg = name.replace('SECCFGR', 'SECENR').replace('PRIVCFGR', 'PRIVENR')
            fields = {k: v for k, v in h['bitfields'].items() if k.startswith(ip + '_' + macro_reg + '_')}
            c_mask = 0
            for field in fields.values():
                c_mask |= field['mask']
            leaf_mask = 0
            for key, field in fields.items():
                if ip != 'EXTI' or re.search(r'\d+$', key) or name == 'LOCKR':
                    leaf_mask |= field['mask']
            i_fields = []
            i_mask = 0
            for field in ir['fieldset/' + item['fieldset']]['fields']:
                for f_index, bit_offset in expand(field.get('array')):
                    pos = bit_offset + field['bit_offset']
                    mask = ((1 << field['bit_size']) - 1) << pos
                    i_mask |= mask
                    i_fields.append({'name': field['name'], 'index': f_index, 'position': pos, 'width': field['bit_size'], 'mask': mask, 'enum': field.get('enum')})
            named_diffs = []
            if ip == 'CRS':
                for field in i_fields:
                    actual = fields.get(ip + '_' + name + '_' + field['name'])
                    if actual is None or actual['position'] != field['position'] or actual['width'] != field['width']:
                        named_diffs.append({'ir': field, 'cmsis': actual})
                assert len(fields) == len(i_fields)
            rows.append({'register': name, 'cmsis_member': cname, 'cmsis_macro_register': macro_reg, 'offset': offset, 'cmsis_access': caccess, 'ir_access': item.get('access', 'ReadWrite'), 'cmsis_mask': c_mask, 'cmsis_individual_field_mask': leaf_mask, 'ir_mask': i_mask, 'ir_extra_mask': i_mask & ~c_mask, 'cmsis_extra_mask': c_mask & ~i_mask, 'cmsis_fields': fields, 'ir_fields': i_fields, 'named_field_differences': named_diffs})
    return {'registers': rows, 'unmapped_cmsis_registers': c_regs}

results = {}
hal_name = 'stm32h5xx_hal_rcc_ex.h'
assert hashlib.sha256((prepare.SOURCES / hal_name).read_bytes()).hexdigest() == lock['files'][hal_name]['sha256']
provenance['sources'][hal_name] = lock['files'][hal_name]
hal_source = prepare.strip_includes((prepare.SOURCES / hal_name).read_text(encoding='utf-8'))
for chip, header in headers.items():
    row = {'numeric_oracle': header['numeric_oracle'], 'ip': {}}
    probe = prepare.BUILD / (chip + '-crs-hal.h')
    probe.write_text(header['source'] + '\n' + hal_source, encoding='utf-8')
    macros = prepare.macros_from(subprocess.check_output([gcc, '-E', '-dM', '-x', 'c', str(probe)], text=True, encoding='utf-8'))
    enum_macros = {k: v for k, v in macros.items() if k.startswith('RCC_CRS_SYNC_SOURCE_')}
    constants = prepare.Constants(macros)
    values = {k: constants.value(k) for k in enum_macros}
    row['crs_hal_source_selection'] = {'macro_expansions': enum_macros, 'encoded_values': values, 'field_values': {k: v >> 28 for k, v in values.items()}, 'oracle': prepare.gcc_verify_values(chip + '-crs-hal', header['source'], values, gcc, enum_macros)}
    for ip in irs:
        prefix = ip + '_'
        numerical = {k: v for k, v in header['numeric_macros'].items() if k.startswith(prefix)}
        row['ip'][ip] = {
            'vs_h563': prepare.compare(header, headers['STM32H563'], ip),
            'integer_macro_count': len(numerical),
            'numeric_macros': numerical,
            'unsupported_object_macros': {k: v for k, v in header['unsupported_object_macros'].items() if k.startswith(prefix)},
            'type': header['types'][ip + '_TypeDef'],
            'pointers': {k: v for k, v in header['pointers'].items() if k.startswith(prefix)},
            'interrupts': {k: v for k, v in header['interrupts'].items() if k.startswith(ip)},
            'ir_comparison': compare_ir(chip, ip),
        }
    results[chip] = row
    prepare.write_json(HERE / (chip + '.json'), row)
    print(chip, {ip: {'identical': row['ip'][ip]['vs_h563']['identical'], 'macros': row['ip'][ip]['integer_macro_count'], 'differences': row['ip'][ip]['vs_h563']['macro_counts']} for ip in irs}, flush=True)
prepare.write_json(HERE / 'provenance.json', provenance)
prepare.write_json(HERE / 'comparison.json', {k: {ip: v['ip'][ip]['vs_h563'] for ip in irs} for k, v in results.items()})
for chip in results:
    crs = results[chip]['ip']['CRS']
    assert crs['vs_h563']['identical']
    assert not crs['ir_comparison']['unmapped_cmsis_registers']
    assert all(not r['named_field_differences'] and not r['ir_extra_mask'] and not r['cmsis_extra_mask'] for r in crs['ir_comparison']['registers'])
assert all(results['STM32H543']['ip'][ip] == results['STM32H553']['ip'][ip] for ip in irs)
print('Assertions passed: H543/H553 CRS+EXTI evidence identical; CRS individual fields/offsets/masks match pinned IR.', flush=True)
