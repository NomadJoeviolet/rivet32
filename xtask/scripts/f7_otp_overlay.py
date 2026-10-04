"""Reversible correction of existing F745/F746/F756 OTP metadata."""
import copy
import hashlib
import json
from pathlib import PurePosixPath
import re

from generate_hal_metadata import hash_bound_bytes, identified_chip, verified_package

PACKAGE = 'data/patches/stm32f7-otp/manifest.json'
MARKER = 'f7_otp_overlay'
EXACT = frozenset('stm32f745ie stm32f745ig stm32f745ve stm32f745vg stm32f745ze stm32f745zg stm32f746be stm32f746bg stm32f746ie stm32f746ig stm32f746ne stm32f746ng stm32f746ve stm32f746vg stm32f746ze stm32f746zg stm32f756bg stm32f756ig stm32f756ng stm32f756vg stm32f756zg'.split())


def sha(raw): return hashlib.sha256(raw).hexdigest()
def encode(value): return (json.dumps(value, indent=2) + '\n').encode()
def hashes(files): return {name:sha(raw) for name, raw in sorted(files.items())}


def check_input_change(before, after, name):
    original = identified_chip(before, name)
    expected = copy.deepcopy(original)
    if len(expected.get('memory', [])) != 1:
        raise ValueError('Unexpected F7 memory layouts')
    regions = [r for r in expected['memory'][0] if r['name'] == 'OTP']
    if len(regions) != 1 or regions[0] != dict(name='OTP', kind='flash', address=0x08fff000, size=2048,
                                             settings=dict(erase_size=0, write_size=16, erase_value=255)):
        raise ValueError('Unexpected original OTP data window')
    regions[0].update(address=0x1ff0f000, size=1024)
    if identified_chip(after, name) != expected:
        raise ValueError('Correction changes data outside the exact OTP user window')


def load(root, expected_sha=None):
    from replay_h5_47a_pac import read_archive
    digest = sha((root / PACKAGE).read_bytes())
    if expected_sha is not None and digest != expected_sha:
        raise ValueError('F7 OTP package changed')
    directory, package = verified_package(root, dict(manifest_path=PACKAGE, sha256=digest))
    spec = json.loads((directory / 'overlay.json').read_bytes())
    names = {chip.upper() + '.json' for chip in EXACT}
    if (package.get('chips') != 21 or spec.get('schema_version') != 1 or set(spec['chips']) != names
            or spec['baseline_revision'] != 'e463add8cc54375f61c6f5f83d6b589e7fc68be2'
            or spec['generator_revision'] != 'caa36afd62510b0e6315ee0dccd1f9c65fbcac83'):
        raise ValueError('F7 OTP scope or fixed revisions differ')
    baseline_raw = hash_bound_bytes(directory / 'baseline.json', spec['baseline_manifest_sha256'], 'OTP baseline')
    baseline = json.loads(baseline_raw)
    if (MARKER in baseline or 'corrected_chip_jsons' in baseline
            or baseline.get('baseline_revision') != spec['baseline_revision']
            or baseline.get('generator_revision') != spec['generator_revision']
            or names & baseline['new_chip_jsons'].keys()):
        raise ValueError('OTP baseline already contains corrections or new-chip collisions')
    record = spec['changes_archive']
    raw = hash_bound_bytes(directory / 'changes.zip', record['sha256'], 'OTP source delta')
    changes = read_archive(raw, record['files'])
    expected_members = set()
    for name, item in spec['chips'].items():
        before = hash_bound_bytes(directory / 'before/chips' / name, item['before_sha256'], 'original F7 input')
        after = hash_bound_bytes(directory / 'chips' / name, item['after_sha256'], 'corrected F7 input')
        check_input_change(before, after, name)
        path = f'src/chips/{name[:-5].lower()}/metadata.rs'
        if item['metadata_path'] != path or item['metadata_before_sha256'] != baseline['vendor_files_sha256'].get(path):
            raise ValueError('F7 metadata source identity differs')
        for prefix, key in [('before', 'metadata_before_sha256'), ('after', 'metadata_after_sha256')]:
            member = prefix + '/' + path
            expected_members.add(member)
            if member not in changes or sha(changes[member]) != item[key]:
                raise ValueError('F7 metadata delta differs: ' + member)
        pattern = rb'(name: "OTP",\s+kind: MemoryRegionKind::Flash,\s+address: )0x8fff000(,\s+size: )2048(,)'
        expected, count = re.subn(pattern, rb'\g<1>0x1ff0f000\g<2>1024\g<3>', changes['before/' + path])
        if count != 1 or expected != changes['after/' + path]:
            raise ValueError('F7 PAC delta changes more than the exact OTP address/size')
    if set(changes) != expected_members:
        raise ValueError('Unexpected OTP archive member')
    for prefix in ['chips/', 'before/chips/']:
        if {n.removeprefix(prefix) for n in package['files'] if n.startswith(prefix)} != names:
            raise ValueError('Unexpected or incomplete OTP chip input inventory')
    return spec, changes, baseline_raw, digest


def publish_manifest(baseline, spec, digest):
    if sha(encode(baseline)) != spec['baseline_manifest_sha256']:
        raise ValueError('OTP baseline manifest differs')
    result = copy.deepcopy(baseline)
    result[MARKER] = dict(manifest_path=PACKAGE, sha256=digest)
    result['corrected_chip_jsons'] = {}
    for name, item in sorted(spec['chips'].items()):
        result['vendor_files_sha256'][item['metadata_path']] = item['metadata_after_sha256']
        result['corrected_chip_jsons'][name] = dict(
            path=str(PurePosixPath(PACKAGE).parent / 'chips' / name), sha256=item['after_sha256'],
            baseline_sha256=item['before_sha256'], overlay='stm32f7-otp')
    return result


def apply(files, manifest, root):
    spec, changes, raw, digest = load(root)
    if encode(manifest) != raw or hashes(files) != manifest['vendor_files_sha256']:
        raise ValueError('OTP source or manifest differs from the complete baseline')
    result = dict(files)
    for item in spec['chips'].values(): result[item['metadata_path']] = changes['after/' + item['metadata_path']]
    return result, publish_manifest(manifest, spec, digest)


def validate_manifest(manifest, root):
    marker = manifest.get(MARKER)
    if not isinstance(marker, dict) or set(marker) != {'manifest_path', 'sha256'} or marker['manifest_path'] != PACKAGE:
        raise ValueError('Unexpected OTP publication marker')
    spec, changes, raw, digest = load(root, marker['sha256'])
    if manifest != publish_manifest(json.loads(raw), spec, digest):
        raise ValueError('OTP published provenance differs')
    return spec, changes, raw


def restore(files, manifest, root):
    spec, changes, raw = validate_manifest(manifest, root)
    if hashes(files) != manifest['vendor_files_sha256']:
        raise ValueError('OTP vendor is modified or partly published')
    result = dict(files)
    for item in spec['chips'].values(): result[item['metadata_path']] = changes['before/' + item['metadata_path']]
    if hashes(result) != json.loads(raw)['vendor_files_sha256']:
        raise ValueError('OTP baseline restoration differs')
    return result, raw
