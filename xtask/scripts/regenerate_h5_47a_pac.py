"""Regenerate the exact 24-part H5E/F PAC from its portable fixed IR archive."""
import argparse
from concurrent.futures import ThreadPoolExecutor
import importlib.util
import json
from pathlib import Path
import subprocess

from prepare_h5_47c_hal import checked_package
from prepare_h5_47c_pac import inventory, sha
from replay_h5_47a_pac import PACKAGE, read_package


def load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def finalize(pac, package, spec, expected):
    guard = load_module('h5_47a_raw_guard', package / 'raw_guard.py')
    products = json.loads((package / 'raw-evidence.json').read_bytes())
    count = guard.guard_raw(pac, products)
    if count != spec['raw_getters']:
        raise ValueError('regenerated Raw getter count differs from verified PAC')
    manifest = pac / 'Cargo.toml'
    manifest.write_bytes(manifest.read_bytes() + b'\n[workspace]\n')
    for name in ['Cargo.lock', 'LICENSE-MIT', 'LICENSE-APACHE', 'ST-LICENSE', 'DFP-LICENSE', 'NOTICE']:
        (pac / name).write_bytes(expected[name])
    actual = inventory(pac)
    if actual != expected:
        changed = sorted(n for n in actual.keys() | expected.keys() if actual.get(n) != expected.get(n))
        raise ValueError('regenerated PAC differs from verified output: ' + ', '.join(changed))
    return count


def regenerate(root, output, rebuild_generator=False):
    root, output = root.resolve(), output.resolve()
    package = root / PACKAGE
    provenance_path = root / 'data/patches/stm32h5-47c-pac/provenance.py'
    tools = [Path(__file__).resolve().with_name(n) for n in (
        'regenerate_h5_47a_pac.py', 'replay_h5_47a_pac.py', 'replay_h5_47a_hal.py',
        'prepare_h5_47a_pac.py', 'prepare_h5_47c_pac.py', 'prepare_h5_47c_hal.py', 'h5_47c_baseline.py')]
    tools += [provenance_path]
    for source in [root / 'data', root / 'vendor'] + tools:
        if output.is_relative_to(source) or source.is_relative_to(output):
            raise ValueError('regeneration output overlaps source: ' + str(source))
    if output.exists():
        raise FileExistsError('refusing to replace existing regeneration output')
    tool_bytes = {p: p.read_bytes() for p in tools}
    package_files, package_sha, spec, contents = read_package(root)
    if spec['rustfmt'] != dict(toolchain='1.98.1', edition='2024', line_endings='LF'):
        raise ValueError('unexpected PAC formatting toolchain')
    provenance = load_module('h5_47a_generator_provenance', provenance_path)
    generator_inputs = provenance.verify(root)
    binary, generator_record = provenance.generator(root, rebuild=rebuild_generator)
    if (generator_record['revision'] != spec['generator']['revision']
            or generator_record['archive_sha256'] != spec['generator']['archive_sha256']):
        raise ValueError('generator source revision/archive differs from H5E/F lock')
    binary_sha = sha(binary.read_bytes())

    def check_inputs():
        if checked_package(package) != (package_files, package_sha):
            raise ValueError('portable PAC inputs changed during regeneration')
        if any(p.read_bytes() != raw for p, raw in tool_bytes.items()):
            raise ValueError('regeneration tools changed')
        if provenance.verify(root) != generator_inputs or sha(binary.read_bytes()) != binary_sha:
            raise ValueError('generator or its pinned provenance changed')

    check_inputs()
    output.mkdir(parents=True, exist_ok=False)
    report = dict(completed=False, sources_unchanged=False, hal_integrated=False, hil='not-run',
                  chips=spec['chips'], package_manifest_sha256=package_sha, generator=generator_record,
                  generator_sha256=binary_sha, tools={str(p): sha(raw) for p, raw in tool_bytes.items()})
    with (output / 'generation.json').open('x', encoding='utf-8', newline='\n') as receipt:
        def save():
            receipt.seek(0)
            receipt.write(json.dumps(report, indent=2) + '\n')
            receipt.truncate()
            receipt.flush()
        save()
        try:
            for name, raw in contents['inputs.zip'].items():
                path = output / 'build/data' / name
                path.parent.mkdir(parents=True, exist_ok=True)
                with path.open('xb') as handle: handle.write(raw)
            with (output / 'generator.log').open('xb') as log:
                subprocess.run([str(binary)], cwd=output, stdout=log, stderr=subprocess.STDOUT, check=True)
            pac = output / 'build/stm32-metapac'
            def format_file(path):
                formatted = subprocess.run(['rustfmt', '+1.98.1', '--edition', '2024', '--emit', 'stdout'],
                                           input=path.read_bytes(), capture_output=True, check=True)
                path.write_bytes(formatted.stdout.replace(b'\r\n', b'\n'))
            with ThreadPoolExecutor(max_workers=4) as pool:
                list(pool.map(format_file, sorted(pac.rglob('*.rs'))))
            raw_count = finalize(pac, package, spec, contents['generated-pac.zip'])
            if inventory(output / 'build/data') != contents['inputs.zip']:
                raise ValueError('generator modified its fixed inputs')
            check_inputs()
            report.update(completed=True, sources_unchanged=True, raw_getters=raw_count,
                          pac_sha256=spec['archives']['generated-pac.zip']['files'],
                          data_sha256=spec['archives']['inputs.zip']['files'])
        except Exception as error:
            report['error'] = repr(error)
            raise
        finally:
            save()
    return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--rebuild-generator', action='store_true')
    args = parser.parse_args()
    report = regenerate(args.root, args.output, args.rebuild_generator)
    print(f'Regenerated {len(report["pac_sha256"])} exact PAC files; {report["raw_getters"]} guarded getters; inputs unchanged')


if __name__ == '__main__': main()
