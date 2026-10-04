"""Compose the verified private H5E/F HAL with all existing chip features."""
import argparse
import json
from pathlib import Path
import tomllib

from prepare_h5_47a_pac import EXACT_CHIPS
from prepare_h5_47c_pac import inventory, safe_names, sha

CHANGED = {'build.rs', 'src/exti/mod.rs', 'src/lib.rs', 'src/rcc/bd.rs',
           'src/rcc/h.rs', 'src/rcc/mco.rs', 'src/rcc/mod.rs',
           'src/usb/mod.rs', 'src/usb/otg.rs'}
ADDED = {'src/exti/h5_47a.rs', 'src/exti/h5_47a_regs.rs',
         'src/rcc/h5_47a_usb.rs', 'src/usb/h5_47a.rs', 'upstream_cfg_inventory.rs'}


def is_chip(name):
    # `stm32-hrtim` is an optional dependency, not a chip selection.
    return name.startswith('stm32') and not name.startswith('stm32-')


def compose(base, candidate):
    safe_names(base)
    safe_names(candidate)
    if base.keys() - candidate.keys():
        raise ValueError('candidate removed baseline sources')
    if candidate.keys() - base.keys() != ADDED | {'Cargo.lock'}:
        raise ValueError('unexpected added candidate sources')
    changed = {n for n in base if base[n] != candidate[n]}
    if changed != CHANGED | {'Cargo.toml'}:
        raise ValueError('unexpected candidate source modifications')
    old = tomllib.loads(base['Cargo.toml'].decode())
    new = tomllib.loads(candidate['Cargo.toml'].decode())
    features = new['features']
    chips = {n: features[n] for n in features if is_chip(n)}
    if chips != {c: ['stm32-metapac/' + c] for c in EXACT_CHIPS}:
        raise ValueError('candidate features must select exact H5E/F PAC chips')
    if set(EXACT_CHIPS) & old['features'].keys():
        raise ValueError('new chip collides with baseline features')
    old_common = dict(old, features={n: v for n, v in old['features'].items() if not is_chip(n)})
    new_common = dict(new, features={n: v for n, v in features.items() if not is_chip(n)})
    patch = new_common.pop('patch', {})
    if set(patch) != {'https://github.com/embassy-rs/stm32-data-generated'}:
        raise ValueError('unexpected private dependency patch')
    pac = patch['https://github.com/embassy-rs/stm32-data-generated']
    if set(pac) != {'stm32-metapac'} or set(pac['stm32-metapac']) != {'path'}:
        raise ValueError('unexpected private PAC patch')
    if old_common != new_common:
        raise ValueError('candidate modifies dependencies or non-chip settings')
    cargo = base['Cargo.toml'].decode()
    if cargo.count('[features]\n') != 1:
        raise ValueError('unexpected baseline features table')
    cargo = cargo.replace('[features]\n', '[features]\n' + ''.join(
        f'{c} = ["stm32-metapac/{c}"]\n' for c in EXACT_CHIPS), 1)
    result = {n: v for n, v in candidate.items() if n != 'Cargo.lock'}
    result['Cargo.toml'] = cargo.encode()
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument('--candidate', type=Path, required=True)
    parser.add_argument('--preparation', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    root, candidate_path, output = args.root.resolve(), args.candidate.resolve(), args.output.resolve()
    baseline_path = root / 'vendor/embassy-stm32'
    manifest_path = root / 'data/patches/embassy-stm32/manifest.json'
    provenance = [manifest_path.resolve(), args.preparation.resolve()]
    tools = [Path(__file__).resolve().with_name(n) for n in
             ['prepare_h5_47a_hal.py', 'prepare_h5_47a_pac.py', 'prepare_h5_47c_pac.py']]
    for source in [baseline_path, candidate_path] + provenance + tools:
        if output.is_relative_to(source) or source.is_relative_to(output):
            raise ValueError('output overlaps composition inputs')
    before = {p: p.read_bytes() for p in provenance + tools}
    base, candidate = inventory(baseline_path), inventory(candidate_path)
    if {n: sha(v) for n, v in base.items()} != json.loads(before[manifest_path.resolve()])['files']:
        raise ValueError('main HAL differs from its source manifest')
    preparation = json.loads(before[args.preparation.resolve()])
    if not preparation['completed'] or {n: sha(v) for n, v in candidate.items()} != preparation['files']:
        raise ValueError('candidate HAL differs from its preparation receipt')
    files = compose(base, candidate)

    def check_inputs():
        if inventory(baseline_path) != base or inventory(candidate_path) != candidate:
            raise ValueError('HAL sources changed during composition')
        if any(p.read_bytes() != raw for p, raw in before.items()):
            raise ValueError('HAL provenance or composition tool changed')

    check_inputs()
    output.mkdir(parents=True, exist_ok=False)
    receipt = dict(completed=False, hal_integrated=False, hil='not-run',
                   added_chips=list(EXACT_CHIPS), changed_existing=sorted(CHANGED | {'Cargo.toml'}),
                   added_sources=sorted(ADDED),
                   sources={str(p): sha(raw) for p, raw in before.items()})
    with (output / 'composition.json').open('x', encoding='utf-8', newline='\n') as report:
        report.write(json.dumps(receipt, indent=2) + '\n')
        report.flush()
        hal = output / 'embassy-stm32'
        for name, raw in files.items():
            path = hal / name
            path.parent.mkdir(parents=True, exist_ok=True)
            with path.open('xb') as handle:
                handle.write(raw)
        if inventory(hal) != files:
            raise ValueError('written HAL differs from composed source')
        check_inputs()
        receipt.update(completed=True, sources_unchanged=True,
                       files={n: sha(v) for n, v in sorted(files.items())})
        report.seek(0)
        report.write(json.dumps(receipt, indent=2) + '\n')
        report.truncate()
    print(f'Composed {len(files)} HAL files; retained existing features and added 24 exact H5E/F parts')


if __name__ == '__main__':
    main()
