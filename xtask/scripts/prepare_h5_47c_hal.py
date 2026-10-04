"""Replay the byte-bound H543/H553 HAL candidate into a new directory.

This is a candidate over the current local Embassy fork. It preserves existing
chip features and does not change the main vendor, catalogue, or App features.
"""
import argparse
import json
from pathlib import Path

from prepare_h5_47c_pac import inventory, safe_names, sha
from h5_47c_baseline import read_baseline
import vendor_embassy

ROOT = Path(__file__).resolve().parents[2]
PACKAGE = 'data/patches/stm32h5-47c-hal'


def checked_package(root):
    files = inventory(root)
    manifest_raw = files.pop('manifest.json')
    manifest = json.loads(manifest_raw)
    files = {p: raw for p, raw in files.items() if '__pycache__' not in Path(p).parts}
    if {name: sha(raw) for name, raw in files.items()} != manifest['files']:
        raise ValueError('candidate package inventory/hash mismatch: ' + str(root))
    return files, sha(manifest_raw)


def read_inputs(root):
    package = root / PACKAGE
    files, package_sha = checked_package(package)
    spec = json.loads(files['patch.json'])
    if spec['schema_version'] != 1 or spec['base_revision'] != vendor_embassy.REV:
        raise ValueError('candidate baseline revision/schema mismatch')
    if spec['baseline_manifest_path'] != 'data/patches/embassy-stm32/manifest.json':
        raise ValueError('unexpected baseline manifest path')
    baseline, baseline_raw = read_baseline(root, 'hal')
    if sha(baseline_raw) != spec['baseline_manifest_sha256']:
        raise ValueError('candidate HAL baseline manifest changed')
    if {name: sha(raw) for name, raw in baseline.items()} != json.loads(baseline_raw)['files']:
        raise ValueError('candidate HAL baseline files changed')
    if spec['pac_manifest_path'] != 'data/patches/stm32h5-47c-pac/manifest.json':
        raise ValueError('unexpected PAC manifest path')
    _, pac_sha = checked_package(root / 'data/patches/stm32h5-47c-pac')
    if pac_sha != spec['pac_manifest_sha256']:
        raise ValueError('candidate exact PAC package changed')
    additions = {}
    for record in spec['new_files']:
        if record['path'] in additions:
            raise ValueError('duplicate added source')
        additions[record['path']] = vendor_embassy.new_source(record, package)
    receipt = dict(package_manifest_sha256=package_sha, baseline_manifest_sha256=sha(baseline_raw),
                   pac_manifest_sha256=pac_sha, added_chips=spec['added_chips'],
                   generator_sha256=sha(Path(__file__).read_bytes()), hal_integrated=False, hil='not-run')
    return baseline, spec, additions, receipt


def apply(baseline, spec, additions):
    safe_names(baseline)
    safe_names(additions)
    result = dict(baseline)
    seen = set()
    for record in spec['files']:
        name = record['path']
        if name not in baseline or name in seen:
            raise ValueError('missing or duplicate baseline patch file')
        seen.add(name)
        result[name] = vendor_embassy.patch_source(baseline[name], record)
    if set(additions) != {record['path'] for record in spec['new_files']}:
        raise ValueError('added source inventory mismatch')
    for record in spec['new_files']:
        name = record['path']
        if name.casefold() in {n.casefold() for n in result}:
            raise ValueError('added source collision')
        if sha(additions[name]) != record['sha256']:
            raise ValueError('added source checksum mismatch')
        result[name] = additions[name]
    if {name: sha(raw) for name, raw in result.items()} != spec['candidate_files_sha256']:
        raise ValueError('candidate final inventory/hash mismatch')
    safe_names(result)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=ROOT)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    root, output = args.root.resolve(), args.output.resolve()
    if output.exists():
        raise ValueError('refusing to overwrite an existing candidate')
    report = output.with_name(output.name + '-replay.json')
    if report.exists():
        raise ValueError('refusing to overwrite existing replay evidence')
    for source in [root / 'vendor', root / 'data/patches']:
        if output.is_relative_to(source) or source.is_relative_to(output):
            raise ValueError('candidate output overlaps source tree')
    baseline, spec, additions, receipt = read_inputs(root)
    result = apply(baseline, spec, additions)
    # Refuse source drift during derivation, before creating the output.
    if read_inputs(root) != (baseline, spec, additions, receipt):
        raise ValueError('candidate inputs changed during replay')
    # Acquire both output names exclusively after input verification; an early
    # exists() check alone must never authorize overwriting a concurrent writer.
    output.mkdir(parents=True, exist_ok=False)
    with report.open('x', encoding='utf-8', newline='\n') as report_file:
        receipt['completed'] = False
        report_file.write(json.dumps(receipt, indent=2) + '\n')
        report_file.flush()
        for name, raw in result.items():
            path = output / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(raw)
        if inventory(output) != result:
            raise ValueError('written HAL differs from candidate')
        # Evidence remains outside the source directory, preserving its bytes.
        receipt['files'] = {name: sha(raw) for name, raw in sorted(result.items())}
        receipt['completed'] = True
        report_file.seek(0)
        report_file.write(json.dumps(receipt, indent=2) + '\n')
        report_file.truncate()
    print(f'Replayed {len(result)} HAL files and {len(spec["added_chips"])} exact chip features')


if __name__ == '__main__':
    main()
