"""Recover exact pre-47C source bytes from either complete publication state.

Only files actually changed by the overlay need stored preimages. Everything
else is reused from the verified vendor tree; partial publication is rejected.
"""
import hashlib
import json
from pathlib import PurePosixPath

from generate_hal_metadata import checked_child_path, hash_bound_bytes, verified_package

INTEGRATION = 'data/patches/stm32h5-47c-integration/manifest.json'
KINDS = {'pac': ('stm32-metapac', 'vendor_files_sha256', 'data/patches/stm32-metapac/vendor-manifest.json'),
         'hal': ('embassy-stm32', 'files', 'data/patches/embassy-stm32/manifest.json')}


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def safe_names(names):
    seen = set()
    for name in names:
        path = PurePosixPath(name)
        if (not name or str(path) != name or path.is_absolute() or '..' in path.parts
                or '\\' in name or ':' in name or name.casefold() in seen):
            raise ValueError('Invalid or colliding baseline path: ' + name)
        seen.add(name.casefold())


def restore(current, old_hashes, new_hashes, preimages):
    for names in (current, old_hashes, new_hashes, preimages):
        safe_names(names)
    if not old_hashes.keys() <= new_hashes.keys():
        raise ValueError('Composition removes an existing source')
    changed = {n for n in old_hashes if old_hashes[n] != new_hashes[n]}
    if set(preimages) != changed or any(sha(raw) != old_hashes[n] for n, raw in preimages.items()):
        raise ValueError('Baseline preimage inventory/hash mismatch')
    actual = {n: sha(raw) for n, raw in current.items()}
    if actual != old_hashes and actual != new_hashes:
        raise ValueError('Vendor inventory drift or partial publication')
    restored = {n: preimages[n] if n in preimages else current[n] for n in old_hashes}
    if {n: sha(raw) for n, raw in restored.items()} != old_hashes:
        raise ValueError('Restored baseline inventory/hash mismatch')
    return restored


def read_baseline(root, kind):
    vendor_name, inventory_key, manifest_name = KINDS[kind]
    current_raw = (root / manifest_name).read_bytes()
    current_manifest = json.loads(current_raw)
    vendor = root / 'vendor' / vendor_name
    current = {}
    for path in vendor.rglob('*'):
        if path.is_symlink():
            raise ValueError('Symbolic link in vendor sources')
        if path.is_file():
            current[path.relative_to(vendor).as_posix()] = path.read_bytes()
    safe_names(current)
    if {n: sha(raw) for n, raw in current.items()} != current_manifest[inventory_key]:
        raise ValueError('Current vendor inventory/hash mismatch')
    if kind == 'hal' and 'platform_fixes' in current_manifest:
        from hal_platform_fixes import restore as restore_platform
        current, current_raw = restore_platform(current, current_manifest, root)
        current_manifest = json.loads(current_raw)
    if kind == 'pac' and 'f7_otp_overlay' in current_manifest:
        from f7_otp_overlay import restore as restore_otp
        current, current_raw = restore_otp(current, current_manifest, root)
        current_manifest = json.loads(current_raw)
    from h5_rtc_overlay import MARKER as RTC_MARKER, restore as restore_rtc
    if RTC_MARKER in current_manifest:
        current, current_raw = restore_rtc(current, current_manifest, root, kind)
        current_manifest = json.loads(current_raw)
    if 'h5_47a_overlay' in current_manifest:
        from h5_47a_integration import restore as restore_h5_47a
        current, current_raw = restore_h5_47a(current, current_manifest, root, kind)
        current_manifest = json.loads(current_raw)
    if kind == 'hal' and current_manifest.get('post_patches'):
        from hal_post_patches import restore as restore_post_patch
        current, current_raw = restore_post_patch(current, current_manifest, root)
    integration_path = root / INTEGRATION
    if not integration_path.exists():
        return current, current_raw
    directory, spec = verified_package(root, {'manifest_path': INTEGRATION, 'sha256': sha(integration_path.read_bytes())})
    record = spec['baselines'][kind]
    raw = hash_bound_bytes(checked_child_path(directory, record['manifest_path']), record['manifest_sha256'], 'baseline manifest')
    baseline_manifest = json.loads(raw)
    preimages = {name: hash_bound_bytes(checked_child_path(directory, relative), baseline_manifest[inventory_key][name], 'baseline preimage')
                 for name, relative in record['preimages'].items()}
    return restore(current, baseline_manifest[inventory_key], record['composed_files_sha256'], preimages), raw
