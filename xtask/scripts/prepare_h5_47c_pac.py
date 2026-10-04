"""Compose exact DIE47C PAC data without replacing existing chip/IP definitions.

Staging tool: writes only a new output directory. Inputs must match their full
source manifests. The fixed source generator remains in stm32h5-47c-pac.
"""
import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import subprocess
import tomllib
from h5_47c_baseline import read_baseline

RAW_EXTENSION = b'''
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
ACCESS_ANCHOR = b'        ReadWrite,\n'
ACCESS_ADDITION = b'        /// Access requires independent hardware verification and unsafe code.\n        Raw,\n'
VERSION_HEADER = 'pub static ALL_PERIPHERAL_VERSIONS: &[(&str, &[&str])] = &['
CHIP_HEADER = 'pub static ALL_CHIPS: &[&str] = &['


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def string_list(text):
    result = json.loads('[' + text.strip().removesuffix(',') + ']')
    if (not result or any(not isinstance(x, str) or not re.fullmatch('[A-Za-z0-9_-]+', x) for x in result)
            or len(result) != len(set(result))):
        raise ValueError('invalid or repeated string list value')
    return result


def body(raw, header):
    text = raw.decode('utf-8').strip()
    if not text.startswith(header) or not text.endswith('];'):
        raise ValueError('unexpected generated static layout')
    return text[len(header):-2]


def versions(raw):
    rest = body(raw, VERSION_HEADER).strip()
    result = {}
    pattern = re.compile(r'\(\s*"([a-z0-9_]+)"\s*,\s*&\[([^\[\]]*)\]\s*,?\s*\)\s*,\s*')
    while rest:
        match = pattern.match(rest)
        if not match or match[1] in result:
            raise ValueError('unrecognized or duplicate peripheral version entry')
        result[match[1]] = set(string_list(match[2]))
        rest = rest[match.end():]
    return result


def chips(raw):
    return set(string_list(body(raw, CHIP_HEADER)))


def chip_features(raw):
    features = tomllib.loads(raw.decode('utf-8'))['features']
    return {name for name in features if name.startswith('stm32')}


def safe_names(files):
    seen = set()
    for name in files:
        p = PurePosixPath(name)
        if (not name or str(p) != name or p.is_absolute() or '..' in p.parts or '\\' in name
                or ':' in name or name.casefold() in seen):
            raise ValueError('invalid or colliding input path: ' + name)
        seen.add(name.casefold())


def compose(base, generated, added, normalize):
    safe_names(base)
    safe_names(generated)
    added = sorted(added)
    if not added or len(set(added)) != len(added) or any(not re.fullmatch('stm32h5[45]3[a-z][eg]', s) for s in added):
        raise ValueError('invalid exact 47C chip list')
    selected = set(added)
    if chip_features(generated['Cargo.toml']) != selected or chips(generated['src/all_chips.rs']) != {s.upper() for s in selected}:
        raise ValueError('generated chip feature/list identity mismatch')
    directories = {PurePosixPath(n).parts[2] for n in generated
                   if n.startswith('src/chips/') and len(PurePosixPath(n).parts) > 3}
    if directories != selected:
        raise ValueError('generated chip directory identity mismatch')
    if chip_features(base['Cargo.toml']) & selected:
        raise ValueError('existing chip feature collision')

    out = dict(base)
    evidence = dict(added_chips=added, shared_modules=[], metadata_names={}, changed_existing=[])
    handled = set()

    def add(name, raw):
        if name.casefold() in {n.casefold() for n in out}:
            raise ValueError('output collision: ' + name)
        out[name] = raw

    # Shared metadata indices are generator-local: give the new batch a stable namespace.
    mapping = {}
    for name, raw in sorted(generated.items()):
        match = re.fullmatch(r'src/chips/metadata_(\d+)\.rs', name)
        if match:
            dest = 'src/chips/metadata_h5_47c_' + match[1] + '.rs'
            add(dest, raw)
            mapping[PurePosixPath(name).name] = PurePosixPath(dest).name
            handled.add(name)
    if not mapping:
        raise ValueError('missing shared chip metadata')
    evidence['metadata_names'] = mapping
    for name, raw in sorted(generated.items()):
        path = PurePosixPath(name)
        if name.startswith('src/chips/') and len(path.parts) == 4:
            if path.parts[2] not in selected:
                raise ValueError('unexpected chip directory')
            if path.name == 'metadata.rs':
                text = raw.decode('utf-8')
                refs = re.findall(r'include!\("\.\./(metadata_\d+\.rs)"\);', text)
                if len(refs) != 1 or refs[0] not in mapping:
                    raise ValueError('unknown shared chip metadata reference')
                text = text.replace('../' + refs[0], '../' + mapping[refs[0]])
                raw = text.encode('utf-8')
            add(name, raw)
            handled.add(name)
    for chip in added:
        for name in ['pac.rs', 'metadata.rs', 'device.x']:
            if f'src/chips/{chip}/{name}' not in out:
                raise ValueError('missing exact chip PAC output')

    for name, raw in sorted(generated.items()):
        if re.fullmatch(r'src/(peripherals|registers)/[a-z0-9_]+\.rs', name):
            if name in base:
                identical = raw == base[name]
                if not identical and normalize(raw) != normalize(base[name]):
                    raise ValueError('shared IP semantic source mismatch: ' + name)
                evidence['shared_modules'].append(dict(path=name, baseline_sha256=sha(base[name]),
                                                       generated_sha256=sha(raw), identical=identical))
            else:
                add(name, raw)
            handled.add(name)

    if generated['src/common.rs'] != base['src/common.rs'] + RAW_EXTENSION:
        raise ValueError('Raw common extension modifies existing register API')
    if (base['src/metadata.rs'].count(ACCESS_ANCHOR) != 1 or generated['src/metadata.rs'] !=
            base['src/metadata.rs'].replace(ACCESS_ANCHOR, ACCESS_ADDITION + ACCESS_ANCHOR)):
        raise ValueError('Raw metadata extension modifies existing access variants')
    out['src/common.rs'] = generated['src/common.rs']
    # Older chip metadata retains the original exhaustive enum and values.
    # Only the exact new selections need Raw; append it after the old variants.
    tail = b'        Write,\n'
    if base['src/metadata.rs'].count(tail) != 1:
        raise ValueError('unexpected baseline Access enum tail')
    raw_variant = ('        /// Access requires independent hardware verification and unsafe code.\n'
                   '        #[cfg(any(\n' + ''.join(f'            feature = "{chip}",\n' for chip in added)
                   + '        ))]\n        Raw,\n').encode('utf-8')
    out['src/metadata.rs'] = base['src/metadata.rs'].replace(tail, tail + raw_variant)
    handled.update(['src/common.rs', 'src/metadata.rs'])
    evidence['metadata_access_policy'] = 'Raw is appended and enabled only for the exact added chip features; old variants and exhaustive matches are preserved'

    text = base['Cargo.toml'].decode('utf-8')
    if text.count('[features]\n') != 1:
        raise ValueError('unexpected baseline chip feature table')
    text = text.replace('[features]\n', '[features]\n' + ''.join(f'{chip} = []\n' for chip in added), 1)
    out['Cargo.toml'] = text.encode('utf-8')
    old_chips = chips(base['src/all_chips.rs'])
    out['src/all_chips.rs'] = (CHIP_HEADER + '\n' + ''.join(f'    "{chip}",\n' for chip in sorted(old_chips | {s.upper() for s in added})) + '];\n').encode()
    combined = versions(base['src/all_peripheral_versions.rs'])
    for kind, values in versions(generated['src/all_peripheral_versions.rs']).items():
        combined.setdefault(kind, set()).update(values)
    out['src/all_peripheral_versions.rs'] = (VERSION_HEADER + '\n' + ''.join(
        '    ("' + kind + '", &[' + ', '.join('"' + value + '"' for value in sorted(values)) + ']),\n'
        for kind, values in sorted(combined.items())) + '];\n').encode()
    out['NOTICE'] = base['NOTICE'] + b'\n' + generated['NOTICE']
    if 'DFP-LICENSE' in generated:
        add('DFP-LICENSE', generated['DFP-LICENSE'])
        handled.add('DFP-LICENSE')
        # Keep the new third-party notice in packaged source distributions too.
        marker = b'    "ST-LICENSE",\n'
        if out['Cargo.toml'].count(marker) == 1:
            out['Cargo.toml'] = out['Cargo.toml'].replace(marker, marker + b'    "DFP-LICENSE",\n')
    handled.update(['Cargo.toml', 'src/all_chips.rs', 'src/all_peripheral_versions.rs', 'NOTICE'])

    for name in generated.keys() - handled:
        if name not in base:
            raise ValueError('unhandled generated file: ' + name)
        if generated[name] != base[name]:
            if name != 'build.rs' or normalize(generated[name]) != normalize(base[name]):
                raise ValueError('unhandled shared source difference: ' + name)
    evidence['changed_existing'] = sorted(name for name in base if out[name] != base[name])
    if set(evidence['changed_existing']) != {'Cargo.toml', 'src/all_chips.rs', 'src/all_peripheral_versions.rs',
                                           'src/common.rs', 'src/metadata.rs', 'NOTICE'}:
        raise ValueError('unexpected existing source modification set')
    safe_names(out)
    return out, evidence


def inventory(root):
    files = {}
    for path in sorted(root.rglob('*')):
        if path.is_symlink():
            raise ValueError('symbolic link in source: ' + str(path))
        if path.is_file():
            files[path.relative_to(root).as_posix()] = path.read_bytes()
    safe_names(files)
    return files


def normalized(raw):
    result = subprocess.run(['rustfmt', '+1.98.1', '--edition', '2024', '--config', 'max_width=120', '--emit', 'stdout'],
                            input=raw, capture_output=True, check=True)
    return result.stdout.replace(b'\r\n', b'\n')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path.cwd())
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    root, output = args.root.resolve(), args.output.resolve()
    if output.exists():
        raise ValueError('refusing to replace an existing composition')
    pack = root / 'data/patches/stm32h5-47c-pac'
    for source in [root / 'vendor/stm32-metapac', pack]:
        if output.is_relative_to(source) or source.is_relative_to(output):
            raise ValueError('composition output overlaps its source tree')
    baseline, base_manifest = read_baseline(root, 'pac')
    pack_manifest = (pack / 'manifest.json').read_bytes()
    if {p: sha(raw) for p, raw in baseline.items()} != json.loads(base_manifest)['vendor_files_sha256']:
        raise ValueError('baseline PAC inventory/hash mismatch')
    package = inventory(pack)
    package.pop('manifest.json')
    # Python may have created interpreter bytecode; it is never a source input.
    package = {p: raw for p, raw in package.items() if '__pycache__' not in PurePosixPath(p).parts}
    if {p: sha(raw) for p, raw in package.items()} != json.loads(pack_manifest)['files']:
        raise ValueError('formal 47C package inventory/hash mismatch')
    generated = {p.removeprefix('generated-pac/'): raw for p, raw in package.items() if p.startswith('generated-pac/')}
    names = sorted(Path(name).stem.lower() for name in package if re.fullmatch(r'chips/STM32H5[45]3[A-Z][EG]\.json', name))
    if len(names) != 14:
        raise ValueError('missing exact 14-chip input set')
    files, evidence = compose(baseline, generated, names, normalized)
    evidence.update(baseline_manifest_sha256=sha(base_manifest), package_manifest_sha256=sha(pack_manifest),
                    vendor_files_sha256={p: sha(raw) for p, raw in sorted(files.items())},
                    generator_sha256=sha(Path(__file__).read_bytes()), hal_integrated=False, hil='not-run')
    # Re-read inputs immediately before writing the derived output.
    if read_baseline(root, 'pac') != (baseline, base_manifest):
        raise ValueError('baseline changed while composing')
    if (pack / 'manifest.json').read_bytes() != pack_manifest:
        raise ValueError('package manifest changed while composing')
    current = inventory(pack)
    current.pop('manifest.json')
    if {p: raw for p, raw in current.items() if '__pycache__' not in PurePosixPath(p).parts} != package:
        raise ValueError('package source changed while composing')
    # The existence check above is only an early diagnostic. This exclusive
    # directory creation arbitrates writers after the potentially long reads.
    output.mkdir(parents=True, exist_ok=False)
    for name, raw in files.items():
        path = output / 'stm32-metapac' / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(raw)
    (output / 'composition.json').write_text(json.dumps(evidence, indent=2) + '\n', encoding='utf-8')
    (output / 'baseline-manifest.json').write_bytes(base_manifest)
    (output / 'package-manifest.json').write_bytes(pack_manifest)
    if inventory(output / 'stm32-metapac') != files:
        raise ValueError('written composition differs from derived sources')
    print(f'Composed {len(files)} PAC files; added {len(names)} exact chips; preserved existing chip/IP sources')


if __name__ == '__main__':
    main()
