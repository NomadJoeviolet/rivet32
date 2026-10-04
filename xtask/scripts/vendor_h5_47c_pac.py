"""Reproduce the complete PAC with the exact H543/H553 and H5E/F overlays."""
import argparse
import json
from pathlib import Path

from generate_hal_metadata import verified_package, verify_overlay_inputs
from h5_47c_baseline import INTEGRATION, read_baseline
from prepare_h5_47c_pac import compose, inventory, normalized, sha

ROOT = Path(__file__).resolve().parents[2]


def derive(root=ROOT):
    baseline, baseline_raw = read_baseline(root, 'pac')
    integration_path = root / INTEGRATION
    integration_sha = sha(integration_path.read_bytes())
    _, spec = verified_package(root, {'manifest_path': INTEGRATION, 'sha256': integration_sha})
    package_dir, package = verified_package(root, spec['source_package'])
    generated = {name.removeprefix('generated-pac/'): (package_dir / name).read_bytes()
                 for name in package['files'] if name.startswith('generated-pac/')}
    names = sorted(name.removesuffix('.json').lower() for name in spec['new_chip_jsons_sha256'])
    files, evidence = compose(baseline, generated, names, normalized)
    hashes = {name: sha(raw) for name, raw in sorted(files.items())}
    if hashes != spec['baselines']['pac']['composed_files_sha256']:
        raise ValueError('PAC replay differs from the verified composition')
    manifest = json.loads(baseline_raw)
    identifier = 'stm32h5-47c'
    if any(overlay['id'] == identifier for overlay in manifest['overlays']):
        raise ValueError('47C overlay already present in baseline')
    manifest['overlays'].append({'id': identifier, 'manifest_path': INTEGRATION, 'manifest_sha256': integration_sha})
    for filename, digest in spec['new_chip_jsons_sha256'].items():
        if filename in manifest['new_chip_jsons']:
            raise ValueError('New chip identity collides with baseline')
        manifest['new_chip_jsons'][filename] = {
            'path': (package_dir / 'chips' / filename).relative_to(root).as_posix(),
            'sha256': digest, 'overlay': identifier}
    manifest['vendor_files_sha256'] = hashes
    verify_overlay_inputs(manifest, root)
    from h5_47a_integration import INTEGRATION as H5_47A, apply_pac
    if (root / H5_47A).exists():
        files, manifest = apply_pac(files, manifest, root)
        verify_overlay_inputs(manifest, root)
    from h5_rtc_overlay import PACKAGE as RTC_PACKAGE, apply as apply_rtc
    if (root / RTC_PACKAGE).exists():
        files, manifest = apply_rtc(files, manifest, root, 'pac')
        verify_overlay_inputs(manifest, root)
    from f7_otp_overlay import PACKAGE as OTP_PACKAGE, apply as apply_otp
    if (root / OTP_PACKAGE).exists():
        files, manifest = apply_otp(files, manifest, root)
        verify_overlay_inputs(manifest, root)
    return files, manifest


def generate(check=False):
    files, manifest = derive()
    vendor = ROOT / 'vendor/stm32-metapac'
    current = inventory(vendor)
    if current.keys() - files.keys():
        raise ValueError('Unreviewed PAC files would remain after publication')
    manifest_path = ROOT / 'data/patches/stm32-metapac/vendor-manifest.json'
    raw = (json.dumps(manifest, indent=2) + '\n').encode()
    if check:
        if current != files or manifest_path.read_bytes() != raw:
            raise ValueError('Main PAC source/manifest differs from full replay')
    else:
        for name, content in files.items():
            if current.get(name) != content:
                path = vendor / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(content)
        manifest_path.write_bytes(raw)
    print(f'{"Verified" if check else "Vendored"} {len(files)} PAC files; {len(manifest["new_chip_jsons"])} exact local inputs')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    generate(parser.parse_args().check)
