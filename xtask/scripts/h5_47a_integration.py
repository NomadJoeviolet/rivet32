"""Reversible H5E/F publication layered after 47C and the G0 UCPD fix."""
import copy
import hashlib
import json
from pathlib import PurePosixPath

from generate_hal_metadata import checked_child_path, hash_bound_bytes, identified_chip, verified_package

INTEGRATION = 'data/patches/stm32h5-47a-integration/manifest.json'
KINDS = {'hal': ('embassy-stm32', 'files', 'data/patches/embassy-stm32/manifest.json'),
         'pac': ('stm32-metapac', 'vendor_files_sha256', 'data/patches/stm32-metapac/vendor-manifest.json')}


def sha(raw): return hashlib.sha256(raw).hexdigest()
def encode(value): return (json.dumps(value, indent=2) + '\n').encode()


def load(root, expected_sha=None):
    from prepare_h5_47a_pac import EXACT_CHIPS
    path = root / INTEGRATION
    digest = sha(path.read_bytes())
    if expected_sha is not None and expected_sha != digest:
        raise ValueError('H5E/F integration manifest changed')
    directory, spec = verified_package(root, {'manifest_path': INTEGRATION, 'sha256': digest})
    if (spec['schema_version'] != 1 or spec['baseline_revision'] != 'e463add8cc54375f61c6f5f83d6b589e7fc68be2'
            or spec['generator_revision'] != 'caa36afd62510b0e6315ee0dccd1f9c65fbcac83'
            or set(spec['new_chip_jsons_sha256']) != {c.upper()+'.json' for c in EXACT_CHIPS}):
        raise ValueError('H5E/F integration identity differs')
    source, source_manifest = verified_package(root, spec['source_package'])
    chips = {n.removeprefix('chips/'): h for n, h in source_manifest['files'].items() if n.startswith('chips/')}
    if chips != spec['new_chip_jsons_sha256'] or source_manifest.get('chips') != 24:
        raise ValueError('H5E/F package chip inventory differs')
    for name in chips:
        identified_chip((source/'chips'/name).read_bytes(), name)
    for key in ['hal_package', 'pac_package']:
        if spec[key] != spec['source_package']:
            verified_package(root, spec[key])
    return directory, spec, digest


def publish_manifest(baseline, spec, integration_sha, kind):
    _, inventory_key, _ = KINDS[kind]
    record = spec['baselines'][kind]
    if sha(encode(baseline)) != record['manifest_sha256']:
        raise ValueError('H5E/F publication baseline manifest differs')
    output = copy.deepcopy(baseline)
    if 'h5_47a_overlay' in output:
        raise ValueError('H5E/F overlay is already present')
    output['h5_47a_overlay'] = dict(manifest_path=INTEGRATION, sha256=integration_sha)
    output[inventory_key] = record['composed_files_sha256']
    if kind == 'hal':
        added = {n.removesuffix('.json').lower() for n in spec['new_chip_jsons_sha256']}
        if added & set(output['added_features']):
            raise ValueError('H5E/F chip feature collides with previous overlay')
        output['added_features'] = sorted(set(output['added_features']) | added)
        output['changes'] += '; exact H5E4/E5/F4/F5 DIE47A platform overlay'
        output.setdefault('layered_patches', []).append(dict(
            **spec['hal_package'], baseline_manifest_sha256=record['manifest_sha256']))
    else:
        if any(r['id'] == 'stm32h5-47a' for r in output['overlays']):
            raise ValueError('H5E/F PAC overlay identity is already present')
        output['overlays'].append(dict(id='stm32h5-47a', manifest_path=INTEGRATION, manifest_sha256=integration_sha))
        directory = PurePosixPath(spec['source_package']['manifest_path']).parent
        for name, digest in spec['new_chip_jsons_sha256'].items():
            if name in output['new_chip_jsons']:
                raise ValueError('H5E/F chip JSON collides with previous overlay')
            output['new_chip_jsons'][name] = dict(path=str(directory/'chips'/name), sha256=digest, overlay='stm32h5-47a')
    return output


def restore(files, manifest, root, kind):
    from h5_47c_baseline import restore as restore_files
    marker = manifest['h5_47a_overlay']
    if marker.get('manifest_path') != INTEGRATION or set(marker) != {'manifest_path', 'sha256'}:
        raise ValueError('unexpected H5E/F restoration manifest')
    directory, spec, digest = load(root, marker['sha256'])
    record = spec['baselines'][kind]
    raw = hash_bound_bytes(checked_child_path(directory, record['manifest_path']), record['manifest_sha256'], 'pre-47A manifest')
    previous = json.loads(raw)
    inventory_key = KINDS[kind][1]
    preimages = {name: hash_bound_bytes(checked_child_path(directory, relative), previous[inventory_key][name], 'pre-47A source')
                 for name, relative in record['preimages'].items()}
    if manifest != publish_manifest(previous, spec, digest, kind):
        raise ValueError('H5E/F published provenance differs from its composition')
    # A promoted manifest must describe the fully promoted tree, never the old
    # tree or a mixture of the two. The generic restore also validates preimages.
    if {n: sha(v) for n, v in files.items()} != record['composed_files_sha256']:
        raise ValueError('H5E/F vendor is modified or only partly published')
    restored = restore_files(files, previous[inventory_key], record['composed_files_sha256'], preimages)
    return restored, raw


def read_baseline(root, kind):
    from prepare_h5_47c_pac import inventory
    vendor, key, manifest = KINDS[kind]
    raw = (root / manifest).read_bytes()
    metadata = json.loads(raw)
    current = inventory(root / 'vendor' / vendor)
    if {n: sha(v) for n, v in current.items()} != metadata[key]:
        raise ValueError('current vendor source differs from its manifest')
    if kind == 'hal' and 'platform_fixes' in metadata:
        from hal_platform_fixes import restore as restore_platform
        current, raw = restore_platform(current, metadata, root)
        metadata = json.loads(raw)
    if kind == 'pac' and 'f7_otp_overlay' in metadata:
        from f7_otp_overlay import restore as restore_otp
        current, raw = restore_otp(current, metadata, root)
        metadata = json.loads(raw)
    from h5_rtc_overlay import MARKER as RTC_MARKER, restore as restore_rtc
    if RTC_MARKER in metadata:
        current, raw = restore_rtc(current, metadata, root, kind)
        metadata = json.loads(raw)
    if 'h5_47a_overlay' in metadata:
        return restore(current, metadata, root, kind)
    return current, raw


def apply_hal(files, manifest, root):
    from prepare_h5_47c_hal import apply, checked_package
    from vendor_embassy import new_source
    _, integration, digest = load(root)
    package = root / PurePosixPath(integration['hal_package']['manifest_path']).parent
    inputs, package_sha = checked_package(package)
    spec = json.loads(inputs['patch.json'])
    if (package_sha != integration['hal_package']['sha256']
            or sha(encode(manifest)) != spec['baseline_manifest_sha256']
            or {n: sha(v) for n, v in files.items()} != manifest['files']
            or spec['pac_files_sha256'] != integration['baselines']['pac']['composed_files_sha256']
            or sorted(spec['added_chips']) != sorted(n.removesuffix('.json').lower() for n in integration['new_chip_jsons_sha256'])):
        raise ValueError('H5E/F HAL source/baseline/PAC binding differs')
    additions = {r['path']: new_source(r, package) for r in spec['new_files']}
    if len(additions) != len(spec['new_files']):
        raise ValueError('duplicate HAL addition')
    output = apply(files, spec, additions)
    result = publish_manifest(manifest, integration, digest, 'hal')
    if {n: sha(v) for n, v in output.items()} != result['files']:
        raise ValueError('H5E/F HAL differs from integration inventory')
    return output, result


def apply_pac(files, manifest, root):
    from prepare_h5_47a_pac import compose
    from prepare_h5_47c_pac import normalized
    from replay_h5_47a_pac import read_package
    _, integration, digest = load(root)
    _, package_sha, spec, contents = read_package(root)
    if (package_sha != integration['pac_package']['sha256']
            or sha(encode(manifest)) != spec['baseline_manifest_sha256']
            or {n: sha(v) for n, v in files.items()} != manifest['vendor_files_sha256']):
        raise ValueError('H5E/F PAC source/baseline binding differs')
    output, _ = compose(files, contents['generated-pac.zip'], spec['chips'], normalized)
    result = publish_manifest(manifest, integration, digest, 'pac')
    if {n: sha(v) for n, v in output.items()} != result['vendor_files_sha256']:
        raise ValueError('H5E/F PAC differs from integration inventory')
    return output, result
