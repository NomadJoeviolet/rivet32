"""Compose exact H5E/F PAC selections with the already-integrated local backend."""
import argparse
import json
from pathlib import Path, PurePosixPath
import re

from prepare_h5_47c_pac import (CHIP_HEADER, VERSION_HEADER, chip_features, chips,
                                inventory, normalized, safe_names, sha, versions)

EXACT_CHIPS = tuple('stm32' + name for name in (
    'h5e4aj', 'h5e4ak', 'h5e4ij', 'h5e4ik', 'h5e4vj', 'h5e4vk', 'h5e4zj', 'h5e4zk',
    'h5e5ij', 'h5e5ik', 'h5e5lj', 'h5e5lk', 'h5e5vj', 'h5e5vk', 'h5e5zj', 'h5e5zk',
    'h5f4aj', 'h5f4ij', 'h5f4vj', 'h5f4zj', 'h5f5ij', 'h5f5lj', 'h5f5vj', 'h5f5zj'))


def metadata_access(base, generated, added, existing_features, normalize):
    """Append Raw only for its existing selections plus the new exact chips."""
    pattern = re.compile(r'pub enum Access\s*\{(?P<body>.*?)\n[ \t]*\}', re.S)
    old, new = base.decode(), generated.decode()
    old_matches, new_matches = list(pattern.finditer(old)), list(pattern.finditer(new))
    if len(old_matches) != 1 or len(new_matches) != 1:
        raise ValueError('Raw metadata Access enum missing or repeated')
    before, after = old_matches[0], new_matches[0]
    raw = re.compile(r'\s*/// Access requires independent hardware verification and unsafe code\.\s*'
                     r'#\[cfg\(any\((.*?)\)\)\]\s*Raw,\s*', re.S)
    matches = list(raw.finditer(before['body']))
    if len(matches) != 1:
        raise ValueError('Raw baseline conditional variant is not recognized')
    config = matches[0][1]
    if not re.fullmatch(r'\s*(?:feature\s*=\s*"[a-z0-9_-]+",\s*)+', config):
        raise ValueError('Raw baseline feature expression is not a simple list')
    retained = re.findall(r'feature\s*=\s*"([a-z0-9_-]+)"', config)
    if len(retained) != len(set(retained)) or not set(retained) <= existing_features:
        raise ValueError('Raw baseline features are duplicated or not existing chips')
    plain_old = raw.sub('', before['body'])
    if len(re.findall(r'\bRaw\s*,', after['body'])) != 1:
        raise ValueError('Raw generated variant missing or repeated')
    plain_new = re.sub(r'\bRaw\s*,', '', after['body'])
    expected = 'ReadWrite,Read,Write,'
    if re.sub(r'\s+', '', plain_old) != expected or re.sub(r'\s+', '', plain_new) != expected:
        raise ValueError('Raw metadata modifies legacy variants or values')
    canonical = 'pub enum Access { ReadWrite, Read, Write, }'
    a = (old[:before.start()] + canonical + old[before.end():]).encode()
    b = (new[:after.start()] + canonical + new[after.end():]).encode()
    if a != b and normalize(a) != normalize(b):
        raise ValueError('Raw metadata modifies definitions outside Access')
    enabled = sorted(set(retained) | set(added))
    enum = ('pub enum Access {\n        ReadWrite,\n        Read,\n        Write,\n'
            '        /// Access requires independent hardware verification and unsafe code.\n'
            '        #[cfg(any(\n' + ''.join(f'            feature = "{chip}",\n' for chip in enabled)
            + '        ))]\n        Raw,\n    }')
    return (old[:before.start()] + enum + old[before.end():]).encode(), retained


def compose(base, generated, added, normalize):
    safe_names(base)
    safe_names(generated)
    added = sorted(added)
    selected = set(added)
    if not added or len(added) != len(selected) or not selected <= set(EXACT_CHIPS):
        raise ValueError('invalid exact H5E/F chip list')
    existing_features = chip_features(base['Cargo.toml'])
    if selected & existing_features:
        raise ValueError('existing chip feature collision')
    if chip_features(generated['Cargo.toml']) != selected or chips(generated['src/all_chips.rs']) != {s.upper() for s in selected}:
        raise ValueError('generated chip feature/list identity mismatch')
    directories = {PurePosixPath(n).parts[2] for n in generated if n.startswith('src/chips/') and len(PurePosixPath(n).parts) > 3}
    if directories != selected:
        raise ValueError('generated chip directory identity mismatch')
    if base['src/common.rs'] != generated['src/common.rs']:
        raise ValueError('Raw common register API differs from existing backend')
    metadata, retained = metadata_access(base['src/metadata.rs'], generated['src/metadata.rs'], added, existing_features, normalize)
    out = dict(base)
    evidence = dict(added_chips=added, shared_modules=[], metadata_names={}, retained_raw_features=sorted(retained))
    handled = {'src/common.rs', 'src/metadata.rs'}
    names = {n.casefold() for n in out}

    def add(name, content):
        if name.casefold() in names:
            raise ValueError('output collision: ' + name)
        out[name] = content
        names.add(name.casefold())

    mapping = {}
    for name, content in sorted(generated.items()):
        match = re.fullmatch(r'src/chips/metadata_(\d+)\.rs', name)
        if match:
            target = 'src/chips/metadata_h5_47a_' + match[1] + '.rs'
            add(target, content)
            mapping[PurePosixPath(name).name] = PurePosixPath(target).name
            handled.add(name)
    if not mapping:
        raise ValueError('missing shared chip metadata')
    evidence['metadata_names'] = mapping
    for name, content in sorted(generated.items()):
        path = PurePosixPath(name)
        if name.startswith('src/chips/') and len(path.parts) == 4:
            if path.name == 'metadata.rs':
                text = content.decode()
                refs = re.findall(r'include!\("\.\./(metadata_\d+\.rs)"\);', text)
                if len(refs) != 1 or refs[0] not in mapping:
                    raise ValueError('unknown shared chip metadata reference')
                content = text.replace('../' + refs[0], '../' + mapping[refs[0]]).encode()
            add(name, content)
            handled.add(name)
    for chip in added:
        if any(f'src/chips/{chip}/{name}' not in out for name in ['pac.rs', 'metadata.rs', 'device.x']):
            raise ValueError('missing exact chip PAC output')
    for name, content in sorted(generated.items()):
        if re.fullmatch(r'src/(peripherals|registers)/[a-z0-9_]+\.rs', name):
            if name in base:
                same = content == base[name]
                if not same and normalize(content) != normalize(base[name]):
                    raise ValueError('shared IP semantic source mismatch: ' + name)
                evidence['shared_modules'].append(dict(path=name, baseline_sha256=sha(base[name]), generated_sha256=sha(content), identical=same))
            else:
                add(name, content)
            handled.add(name)
    out['src/metadata.rs'] = metadata
    cargo = base['Cargo.toml'].decode()
    if cargo.count('[features]\n') != 1:
        raise ValueError('unexpected baseline feature table')
    out['Cargo.toml'] = cargo.replace('[features]\n', '[features]\n' + ''.join(f'{s} = []\n' for s in added), 1).encode()
    out['src/all_chips.rs'] = (CHIP_HEADER + '\n' + ''.join(f'    "{s}",\n' for s in sorted(chips(base['src/all_chips.rs']) | {s.upper() for s in selected})) + '];\n').encode()
    merged = versions(base['src/all_peripheral_versions.rs'])
    for kind, values in versions(generated['src/all_peripheral_versions.rs']).items():
        merged.setdefault(kind, set()).update(values)
    out['src/all_peripheral_versions.rs'] = (VERSION_HEADER + '\n' + ''.join(
        '    ("' + kind + '", &[' + ', '.join('"' + v + '"' for v in sorted(values)) + ']),\n'
        for kind, values in sorted(merged.items())) + '];\n').encode()
    out['NOTICE'] = base['NOTICE'] + b'\n' + generated['NOTICE']
    handled.update(['Cargo.toml', 'src/all_chips.rs', 'src/all_peripheral_versions.rs', 'NOTICE'])
    for name in generated.keys() - handled:
        if name not in base:
            raise ValueError('unhandled generated file: ' + name)
        if generated[name] != base[name] and (name != 'build.rs' or normalize(generated[name]) != normalize(base[name])):
            raise ValueError('unhandled shared source difference: ' + name)
    changed = {n for n in base if out[n] != base[n]}
    if changed != {'Cargo.toml', 'src/all_chips.rs', 'src/all_peripheral_versions.rs', 'src/metadata.rs', 'NOTICE'}:
        raise ValueError('unexpected existing source modification set')
    evidence['changed_existing'] = sorted(changed)
    safe_names(out)
    return out, evidence


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument('--generated', type=Path, required=True)
    parser.add_argument('--generation', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    root, generated_path, output = args.root.resolve(), args.generated.resolve(), args.output.resolve()
    baseline_path = root / 'vendor/stm32-metapac'
    for source in [baseline_path, generated_path]:
        if output.is_relative_to(source) or source.is_relative_to(output):
            raise ValueError('composition output overlaps its inputs')
    manifest = root / 'data/patches/stm32-metapac/vendor-manifest.json'
    tool_paths = [Path(__file__).resolve(), Path(__file__).resolve().with_name('prepare_h5_47c_pac.py')]
    tool_hashes = {str(p):sha(p.read_bytes()) for p in tool_paths}
    manifest_raw, generation_raw = manifest.read_bytes(), args.generation.read_bytes()
    base, generated = inventory(baseline_path), inventory(generated_path)
    if {n:sha(v) for n,v in base.items()} != json.loads(manifest_raw)['vendor_files_sha256']:
        raise ValueError('main PAC differs from its source manifest')
    generation = json.loads(generation_raw)
    if {n:sha(v) for n,v in generated.items()} != generation['pac_sha256'] or set(generation['chips']) != set(EXACT_CHIPS):
        raise ValueError('H5E/F generation inventory or exact chip set mismatch')
    files, report = compose(base, generated, generation['chips'], normalized)
    if inventory(baseline_path) != base or inventory(generated_path) != generated:
        raise ValueError('composition source changed during derivation')
    if manifest.read_bytes() != manifest_raw or args.generation.read_bytes() != generation_raw:
        raise ValueError('composition provenance changed during derivation')
    # Exclusive creation prevents a concurrent process from being overwritten.
    output.mkdir(parents=True, exist_ok=False)
    receipt = output / 'composition.json'
    report.update(completed=False, hal_integrated=False, hil='not-run',
                  baseline_manifest_sha256=sha(manifest_raw), generation_sha256=sha(generation_raw),
                  runner_sha256=tool_hashes[str(Path(__file__).resolve())], sources=tool_hashes)
    with receipt.open('x', encoding='utf-8', newline='\n') as handle:
        handle.write(json.dumps(report, indent=2) + '\n'); handle.flush()
        pac = output / 'stm32-metapac'
        for name, raw in files.items():
            path = pac / name
            path.parent.mkdir(parents=True, exist_ok=True)
            with path.open('xb') as destination: destination.write(raw)
        if inventory(pac) != files:
            raise ValueError('written composition differs from derived PAC')
        if inventory(baseline_path) != base or inventory(generated_path) != generated:
            raise ValueError('composition source changed while writing output')
        if manifest.read_bytes() != manifest_raw or args.generation.read_bytes() != generation_raw:
            raise ValueError('composition provenance changed while writing output')
        if any(sha(Path(n).read_bytes()) != h for n,h in tool_hashes.items()):
            raise ValueError('composition tool changed while running')
        report.update(completed=True, sources_unchanged=True, files={n:sha(v) for n,v in sorted(files.items())})
        handle.seek(0); handle.write(json.dumps(report, indent=2) + '\n'); handle.truncate()
    print('Composed', len(files), 'PAC files; retained legacy and 47C APIs; added 24 exact H5E/F parts')


if __name__ == '__main__': main()
