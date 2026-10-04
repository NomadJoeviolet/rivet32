"""Replay the portable H5E/F HAL patch against its exact baseline and PAC.

Writes a new candidate directory only. Main vendor files and catalogue support
are unchanged. No historical validation workspace is needed for this replay.
"""
import argparse
import hashlib
import json
from pathlib import Path

from prepare_h5_47a_pac import EXACT_CHIPS
from prepare_h5_47c_hal import apply, checked_package
from prepare_h5_47c_pac import inventory, safe_names, sha
import vendor_embassy
from h5_47a_integration import read_baseline

PACKAGE = 'data/patches/stm32h5-47a-hal'
BASELINE_MANIFEST = 'data/patches/embassy-stm32/manifest.json'


def hash_inventory(root):
    """Hash the full PAC without holding another complete copy in memory."""
    result = {}
    for path in sorted(root.rglob('*')):
        if path.is_symlink():
            raise ValueError('symbolic link in source: ' + str(path))
        if path.is_file():
            with path.open('rb') as handle:
                result[path.relative_to(root).as_posix()] = hashlib.file_digest(handle, 'sha256').hexdigest()
    safe_names(result)
    return result


def replay(root, pac, output):
    root, pac, output = root.resolve(), pac.resolve(), output.resolve()
    package, baseline_path = root / PACKAGE, root / 'vendor/embassy-stm32'
    manifest_path = root / BASELINE_MANIFEST
    tools = [Path(__file__).resolve().with_name(name) for name in (
        'replay_h5_47a_hal.py', 'prepare_h5_47a_pac.py', 'prepare_h5_47c_pac.py',
        'prepare_h5_47c_hal.py', 'vendor_embassy.py', 'h5_47c_baseline.py', 'h5_47a_integration.py')]
    for source in [package, baseline_path, pac, manifest_path] + tools:
        if output.is_relative_to(source) or source.is_relative_to(output):
            raise ValueError('replay output overlaps source: ' + str(source))
    if output.exists():
        raise FileExistsError('refusing to replace existing output: ' + str(output))
    tool_bytes = {p: p.read_bytes() for p in tools}
    package_files, package_sha = checked_package(package)
    spec = json.loads(package_files['patch.json'])
    if spec['schema_version'] != 1 or spec['base_revision'] != vendor_embassy.REV:
        raise ValueError('unsupported HAL patch schema or baseline revision')
    if sorted(spec['added_chips']) != sorted(EXACT_CHIPS):
        raise ValueError('HAL patch must select all 24 exact H5E/F chips')
    if spec.get('baseline_manifest_path', BASELINE_MANIFEST) != BASELINE_MANIFEST:
        raise ValueError('unexpected HAL baseline manifest path')
    current_manifest_raw = manifest_path.read_bytes()
    baseline, manifest_raw = read_baseline(root, 'hal')
    if (manifest_raw != package_files['baseline-manifest.json']
            or sha(manifest_raw) != spec['baseline_manifest_sha256']):
        raise ValueError('HAL baseline manifest changed')
    if {n: sha(v) for n, v in baseline.items()} != json.loads(manifest_raw)['files']:
        raise ValueError('HAL baseline source inventory/hash mismatch')
    if not spec['pac_files_sha256'] or hash_inventory(pac) != spec['pac_files_sha256']:
        raise ValueError('paired PAC inventory/hash mismatch')
    additions = {}
    for record in spec['new_files']:
        if record['path'] in additions:
            raise ValueError('duplicate added HAL source')
        additions[record['path']] = vendor_embassy.new_source(record, package)
    files = apply(baseline, spec, additions)

    def check_inputs():
        if checked_package(package) != (package_files, package_sha):
            raise ValueError('HAL patch package changed during replay')
        if (manifest_path.read_bytes() != current_manifest_raw
                or read_baseline(root, 'hal') != (baseline, manifest_raw)):
            raise ValueError('HAL baseline changed during replay')
        if hash_inventory(pac) != spec['pac_files_sha256']:
            raise ValueError('paired PAC changed during replay')
        if any(p.read_bytes() != raw for p, raw in tool_bytes.items()):
            raise ValueError('replay tool changed during replay')

    check_inputs()
    output.mkdir(parents=True, exist_ok=False)
    receipt = dict(completed=False, sources_unchanged=False, hal_integrated=False,
                   hil='not-run', added_chips=list(EXACT_CHIPS),
                   package_manifest_sha256=package_sha,
                   baseline_manifest_sha256=sha(manifest_raw),
                   pac_files_sha256=spec['pac_files_sha256'],
                   tools={p.name: sha(raw) for p, raw in tool_bytes.items()})
    with (output / 'replay.json').open('x', encoding='utf-8', newline='\n') as report:
        report.write(json.dumps(receipt, indent=2) + '\n')
        report.flush()
        hal = output / 'embassy-stm32'
        for name, raw in files.items():
            path = hal / name
            path.parent.mkdir(parents=True, exist_ok=True)
            with path.open('xb') as handle:
                handle.write(raw)
        if inventory(hal) != files:
            raise ValueError('written HAL differs from replayed source')
        check_inputs()
        receipt.update(completed=True, sources_unchanged=True,
                       files={n: sha(v) for n, v in sorted(files.items())})
        report.seek(0)
        report.write(json.dumps(receipt, indent=2) + '\n')
        report.truncate()
    return receipt


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument('--pac', type=Path, required=True, help='Exact combined stm32-metapac source directory')
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    result = replay(args.root, args.pac, args.output)
    print(f'Replayed {len(result["files"])} HAL files; verified paired PAC and unchanged inputs')


if __name__ == '__main__':
    main()
