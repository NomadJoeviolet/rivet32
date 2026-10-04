"""Replay the portable exact H5E/F PAC overlay into a new candidate directory.

The package retains both fixed generator inputs and its generated output. This
command composes that output with the bound current baseline; it does not run the
upstream generator. Regeneration is a separate reproducibility check.
"""
import argparse
import io
import json
from pathlib import Path
import stat
import zipfile

from prepare_h5_47a_pac import EXACT_CHIPS, compose
from prepare_h5_47c_hal import checked_package
from prepare_h5_47c_pac import inventory, normalized, safe_names, sha
from replay_h5_47a_hal import hash_inventory
from h5_47a_integration import read_baseline

PACKAGE = 'data/patches/stm32h5-47a-pac'
BASELINE_MANIFEST = 'data/patches/stm32-metapac/vendor-manifest.json'
GENERATOR_REVISION = 'caa36afd62510b0e6315ee0dccd1f9c65fbcac83'


def read_archive(raw, expected):
    safe_names(expected)
    result = {}
    with zipfile.ZipFile(io.BytesIO(raw)) as archive:
        entries = archive.infolist()
        names = [e.filename for e in entries]
        safe_names(names)
        if len(names) != len(expected) or set(names) != set(expected):
            raise ValueError('archive file inventory mismatch')
        for entry in entries:
            mode = stat.S_IFMT(entry.external_attr >> 16)
            if entry.is_dir() or mode not in (0, stat.S_IFREG):
                raise ValueError('archive contains a non-regular file')
            data = archive.read(entry)
            if sha(data) != expected[entry.filename]:
                raise ValueError('archive member hash mismatch: ' + entry.filename)
            result[entry.filename] = data
    return result


def read_package(root):
    files, package_sha = checked_package(root / PACKAGE)
    spec = json.loads(files['sources.json'])
    if (spec['schema_version'] != 1 or not spec['completed']
            or spec['generator']['revision'] != GENERATOR_REVISION
            or sorted(spec['chips']) != sorted(EXACT_CHIPS)):
        raise ValueError('invalid H5E/F PAC package identity')
    if set(spec['archives']) != {'inputs.zip', 'generated-pac.zip', 'references.zip'}:
        raise ValueError('unexpected PAC archive set')
    contents = {}
    for name, record in spec['archives'].items():
        if sha(files[name]) != record['sha256']:
            raise ValueError('PAC archive hash mismatch: ' + name)
        decoded = read_archive(files[name], record['files'])
        if name != 'references.zip': contents[name] = decoded
    chip_files = {n for n in contents['inputs.zip'] if n.startswith('chips/')}
    if chip_files != {f'chips/{chip.upper()}.json' for chip in EXACT_CHIPS}:
        raise ValueError('generator input chip inventory mismatch')
    for chip in EXACT_CHIPS:
        if json.loads(contents['inputs.zip'][f'chips/{chip.upper()}.json'])['name'] != chip.upper():
            raise ValueError('generator input chip identity mismatch')
    return files, package_sha, spec, contents


def replay(root, output):
    root, output = root.resolve(), output.resolve()
    baseline_path = root / 'vendor/stm32-metapac'
    package = root / PACKAGE
    manifest_path = root / BASELINE_MANIFEST
    tool_names = ['replay_h5_47a_pac.py', 'replay_h5_47a_hal.py', 'prepare_h5_47a_pac.py',
                  'prepare_h5_47c_pac.py', 'prepare_h5_47c_hal.py', 'h5_47c_baseline.py', 'h5_47a_integration.py']
    tools = [Path(__file__).resolve().with_name(n) for n in tool_names]
    for source in [baseline_path, package, manifest_path] + tools:
        if output.is_relative_to(source) or source.is_relative_to(output):
            raise ValueError('PAC replay output overlaps source: ' + str(source))
    if output.exists():
        raise FileExistsError('refusing to replace existing output: ' + str(output))
    tool_bytes = {p: p.read_bytes() for p in tools}
    package_files, package_sha, spec, contents = read_package(root)
    current_manifest_raw = manifest_path.read_bytes()
    baseline, manifest_raw = read_baseline(root, 'pac')
    if (manifest_raw != package_files['baseline-manifest.json']
            or sha(manifest_raw) != spec['baseline_manifest_sha256']):
        raise ValueError('PAC baseline manifest changed')
    baseline_hashes = json.loads(manifest_raw)['vendor_files_sha256']
    if {n: sha(v) for n, v in baseline.items()} != baseline_hashes:
        raise ValueError('PAC baseline source inventory/hash mismatch')
    files, composition = compose(baseline, contents['generated-pac.zip'], spec['chips'], normalized)
    if {n: sha(v) for n, v in files.items()} != spec['combined_pac_sha256']:
        raise ValueError('replayed PAC differs from verified composition')

    def check_inputs():
        if checked_package(package) != (package_files, package_sha):
            raise ValueError('PAC package changed during replay')
        if (manifest_path.read_bytes() != current_manifest_raw
                or read_baseline(root, 'pac') != (baseline, manifest_raw)):
            raise ValueError('PAC baseline changed during replay')
        if any(p.read_bytes() != raw for p, raw in tool_bytes.items()):
            raise ValueError('PAC replay tool changed during replay')

    check_inputs()
    output.mkdir(parents=True, exist_ok=False)
    receipt = dict(completed=False, sources_unchanged=False, hal_integrated=False, hil='not-run',
                   package_manifest_sha256=package_sha, baseline_manifest_sha256=sha(manifest_raw),
                   composition=composition, tools={p.name: sha(v) for p, v in tool_bytes.items()})
    with (output / 'replay.json').open('x', encoding='utf-8', newline='\n') as report:
        report.write(json.dumps(receipt, indent=2) + '\n')
        report.flush()
        for name, raw in files.items():
            path = output / 'stm32-metapac' / name
            path.parent.mkdir(parents=True, exist_ok=True)
            with path.open('xb') as handle: handle.write(raw)
        if hash_inventory(output / 'stm32-metapac') != spec['combined_pac_sha256']:
            raise ValueError('written PAC differs from replayed source')
        check_inputs()
        receipt.update(completed=True, sources_unchanged=True, files=spec['combined_pac_sha256'])
        report.seek(0)
        report.write(json.dumps(receipt, indent=2) + '\n')
        report.truncate()
    return receipt


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    result = replay(args.root, args.output)
    print(f'Replayed {len(result["files"])} PAC files from portable package; inputs unchanged')


if __name__ == '__main__': main()
