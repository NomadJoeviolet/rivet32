"""Regenerate the exact F7 OTP metadata using the pinned, unchanged PAC generator."""
import argparse
import importlib.util
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--workspace', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    root, out = args.workspace.resolve(), args.output.resolve()
    if out.exists() or not out.is_relative_to(root / 'target'):
        raise ValueError('Use a fresh output directory inside workspace target')
    sys.path.insert(0, str(root / 'xtask/scripts'))
    from f7_otp_overlay import PACKAGE, load, sha
    from prepare_h5_47c_pac import normalized
    spec, changes, _, digest = load(root)
    package = root / Path(PACKAGE).parent
    out.mkdir(parents=True)
    for prefix, names in [('chips', spec['chips']), ('registers', spec['register_inputs'])]:
        for name in names:
            source = package / ('chips' if prefix == 'chips' else 'sources/registers') / name
            dest = out / 'build/data' / prefix / name
            dest.parent.mkdir(parents=True, exist_ok=True)
            if prefix == 'registers' and sha(source.read_bytes()) != spec['register_inputs'][name]:
                raise ValueError('Register input differs: ' + name)
            shutil.copyfile(source, dest)
    source = root / 'data/patches/stm32h5-47c-pac/provenance.py'
    module_spec = importlib.util.spec_from_file_location('f7_generator_provenance', source)
    provenance = importlib.util.module_from_spec(module_spec)
    module_spec.loader.exec_module(provenance)
    generator, provenance_record = provenance.generator(root)
    with (out / 'generator.log').open('wb') as stream:
        subprocess.run([str(generator)], cwd=out, stdout=stream, stderr=subprocess.STDOUT, check=True)
    generated = out / 'build/stm32-metapac'
    include = re.compile(rb'include!\("\.\./(metadata_\d+\.rs)"\);')
    for item in spec['chips'].values():
        path = item['metadata_path']
        expected = changes['after/' + path]
        actual = (generated / path).read_bytes()
        expected_ref, actual_ref = include.search(expected), include.search(actual)
        if not expected_ref or not actual_ref:
            raise ValueError('Generated shared metadata reference missing')
        before_shared = root / 'vendor/stm32-metapac/src/chips' / expected_ref[1].decode()
        actual_shared = generated / 'src/chips' / actual_ref[1].decode()
        if normalized(before_shared.read_bytes()).strip() != normalized(actual_shared.read_bytes()).strip():
            raise ValueError('Shared peripheral metadata changed')
        actual = actual.replace(actual_ref[0], expected_ref[0], 1)
        if normalized(actual).strip() != normalized(expected).strip():
            raise ValueError('Generated OTP metadata differs: ' + path)
    result = dict(completed=True, chips=len(spec['chips']), package_sha256=digest, generator=provenance_record,
                  generated_files={p.relative_to(generated).as_posix():sha(p.read_bytes())
                                   for p in generated.rglob('*') if p.is_file()})
    (out / 'validation.json').write_text(json.dumps(result, indent=2) + '\n', encoding='utf-8')
    print('Regenerated and compared 21 exact OTP metadata corrections', flush=True)


if __name__ == '__main__':
    main()
