"""Apply and reverse the exact RTC/low-power delta after the published H5 layers."""
import copy
import hashlib
import json

from generate_hal_metadata import checked_child_path, hash_bound_bytes, verified_package
from h5_47c_baseline import safe_names

PACKAGE = 'data/patches/stm32h5-rtc-low-power/manifest.json'
MARKER = 'h5_rtc_low_power_overlay'
KINDS = {'hal': ('embassy-stm32', 'files'), 'pac': ('stm32-metapac', 'vendor_files_sha256')}


def sha(raw): return hashlib.sha256(raw).hexdigest()
def encode(value): return (json.dumps(value, indent=2) + '\n').encode()
def hashes(files): return {name:sha(raw) for name,raw in sorted(files.items())}


def load(root, expected_sha=None):
    from replay_h5_47a_pac import read_archive

    digest = sha((root / PACKAGE).read_bytes())
    if expected_sha is not None and digest != expected_sha:
        raise ValueError('RTC overlay package changed')
    directory, package = verified_package(root, dict(manifest_path=PACKAGE, sha256=digest))
    spec = json.loads((directory / 'overlay.json').read_bytes())
    families = {'STM32H543':9, 'STM32H553':5, 'STM32H5E4':8, 'STM32H5E5':8, 'STM32H5F4':4, 'STM32H5F5':4}
    if (spec.get('schema_version') != 1 or package.get('chips') != 38
            or len(spec['chips']) != 38 or len(set(spec['chips'])) != 38
            or {family:sum(chip.startswith(family) for chip in spec['chips']) for family in families} != families
            or spec['embassy_revision'] != 'ae9e6f0672af84cec8e200a94c574041844396f0'
            or spec['pac_revision'] != 'e463add8cc54375f61c6f5f83d6b589e7fc68be2'
            or spec['generator_revision'] != 'caa36afd62510b0e6315ee0dccd1f9c65fbcac83'
            or set(spec['components']) != {v[0] for v in KINDS.values()}):
        raise ValueError('RTC overlay scope or pinned revisions differ')
    archive = spec['archives']['changes.zip']
    raw = hash_bound_bytes(directory / 'changes.zip', archive['sha256'], 'RTC source delta')
    changes = read_archive(raw, archive['files'])
    expected_changes, baselines = set(), {}
    for component, key in KINDS.values():
        record = spec['components'][component]
        before, after = record['before'], record['after']
        safe_names(before)
        safe_names(after)
        if not before.keys() <= after.keys() or before.get('Cargo.toml') != after.get('Cargo.toml'):
            raise ValueError('RTC overlay removes sources or changes feature definitions')
        modified = sorted(name for name in before if before[name] != after[name])
        added = sorted(after.keys() - before.keys())
        if modified != record['modified'] or added != record['added']:
            raise ValueError('RTC changed source inventory differs')
        baseline_raw = hash_bound_bytes(checked_child_path(directory, record['baseline_manifest']),
                                       record['baseline_manifest_sha256'], 'RTC baseline manifest')
        baseline = json.loads(baseline_raw)
        if baseline.get(key) != before or MARKER in baseline:
            raise ValueError('RTC baseline inventory or prior publication differs')
        baselines[component] = baseline_raw
        for prefix,names,expected in [('before',modified,before),('after',modified+added,after)]:
            for name in names:
                member = prefix + '/' + component + '/' + name
                expected_changes.add(member)
                if member not in changes or sha(changes[member]) != expected[name]:
                    raise ValueError('RTC source delta member differs: ' + member)
    if set(changes) != expected_changes:
        raise ValueError('Unexpected RTC source delta member')
    return spec, changes, baselines, digest


def publish_manifest(baseline, record, digest, kind):
    key = KINDS[kind][1]
    if MARKER in baseline or sha(encode(baseline)) != record['baseline_manifest_sha256']:
        raise ValueError('RTC publication baseline manifest differs')
    result = copy.deepcopy(baseline)
    result[key] = record['after']
    result[MARKER] = dict(manifest_path=PACKAGE, sha256=digest)
    return result


def apply(files, manifest, root, kind):
    component, key = KINDS[kind]
    spec, changes, baselines, digest = load(root)
    record = spec['components'][component]
    if (MARKER in manifest or encode(manifest) != baselines[component]
            or hashes(files) != record['before'] or manifest[key] != record['before']):
        raise ValueError('RTC source or manifest differs from its exact baseline')
    result = dict(files)
    for name in record['modified'] + record['added']:
        result[name] = changes['after/' + component + '/' + name]
    if hashes(result) != record['after']:
        raise ValueError('RTC composed inventory differs')
    return result, publish_manifest(manifest, record, digest, kind)


def validate_manifest(manifest, root, kind):
    component, key = KINDS[kind]
    marker = manifest.get(MARKER)
    if not isinstance(marker,dict) or set(marker) != {'manifest_path','sha256'} or marker['manifest_path'] != PACKAGE:
        raise ValueError('Unexpected RTC restoration marker')
    spec, changes, baselines, digest = load(root,marker['sha256'])
    record = spec['components'][component]
    baseline = json.loads(baselines[component])
    if manifest != publish_manifest(baseline,record,digest,kind):
        raise ValueError('RTC published provenance differs')
    return record, changes, baselines[component]


def restore(files, manifest, root, kind):
    component, key = KINDS[kind]
    record, changes, baseline_raw = validate_manifest(manifest, root, kind)
    if hashes(files) != record['after']:
        raise ValueError('RTC vendor is changed or only partly published')
    result = {name:changes['before/' + component + '/' + name] if name in record['modified'] else files[name]
              for name in record['before']}
    if hashes(result) != record['before']:
        raise ValueError('RTC restored baseline differs')
    return result, baseline_raw
