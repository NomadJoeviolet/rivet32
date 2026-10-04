"""Replay the H7R/S memory and H5E5/F5 PHY fixes after the existing HAL layers."""
import copy
import hashlib
import json

PATCH = 'data/patches/embassy-stm32/platform-fixes.json'
MARKER = 'platform_fixes'
REVISION = 'ae9e6f0672af84cec8e200a94c574041844396f0'
PATHS = ['build.rs', 'src/rcc/h.rs', 'src/rcc/h5_47a_usb.rs', 'src/rcc/mod.rs']


def encode(value): return (json.dumps(value, indent=2) + '\n').encode()
def sha(raw): return hashlib.sha256(raw).hexdigest()
def inventory(files): return {name: sha(raw) for name, raw in sorted(files.items())}


def load(root):
    raw = (root / PATCH).read_bytes()
    spec = json.loads(raw)
    baseline = spec['baseline_manifest']
    if (spec.get('schema_version') != 1 or spec.get('base_revision') != REVISION
            or baseline.get('revision') != REVISION or MARKER in baseline
            or sha(encode(baseline)) != spec.get('baseline_manifest_sha256')
            or [record.get('path') for record in spec['files']] != PATHS
            or spec.get('new_files')):
        raise ValueError('HAL platform patch baseline, revision or four-file scope differs')
    for record in spec['files']:
        if record['upstream_sha256'] != baseline['files'].get(record['path']):
            raise ValueError('HAL platform patch preimage differs from the baseline manifest')
    return spec, sha(raw)


def apply(files, manifest, root):
    from vendor_embassy import patch_source

    spec, patch_sha = load(root)
    if (encode(manifest) != encode(spec['baseline_manifest'])
            or inventory(files) != manifest['files']):
        raise ValueError('HAL platform patch requires its complete, unchanged source baseline')
    result = dict(files)
    for record in spec['files']:
        result[record['path']] = patch_source(result[record['path']], record)
    published = copy.deepcopy(manifest)
    published['files'] = inventory(result)
    published[MARKER] = dict(path=PATCH, sha256=patch_sha)
    return result, published


def restore(files, manifest, root):
    spec, patch_sha = load(root)
    if (manifest.get(MARKER) != dict(path=PATCH, sha256=patch_sha)
            or inventory(files) != manifest['files']):
        raise ValueError('HAL platform patch source or provenance differs')
    original = dict(files)
    for record in reversed(spec['files']):
        name = record['path']
        if sha(original[name]) != record['patched_sha256']:
            raise ValueError('HAL platform patch is mixed or modified: ' + name)
        text = original[name].decode('utf-8')
        for replacement in reversed(record['replacements']):
            if text.count(replacement['new']) != 1:
                raise ValueError('HAL platform reverse hunk is ambiguous: ' + name)
            text = text.replace(replacement['new'], replacement['old'])
        original[name] = text.encode('utf-8')
        if sha(original[name]) != record['upstream_sha256']:
            raise ValueError('HAL platform restored source differs: ' + name)
    if inventory(original) != spec['baseline_manifest']['files']:
        raise ValueError('HAL platform restored inventory differs')
    rebuilt, published = apply(original, spec['baseline_manifest'], root)
    if rebuilt != files or published != manifest:
        raise ValueError('HAL platform reverse replay differs from the published composition')
    return original, encode(spec['baseline_manifest'])
