"""Replay or restore the exact H5 RTC candidate into a fresh source directory."""
from pathlib import Path, PurePosixPath
import argparse
import hashlib
import io
import json
import stat
import zipfile


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def safe_names(names):
    seen = set()
    for name in names:
        path = PurePosixPath(name)
        if (not name or str(path) != name or path.is_absolute() or '..' in path.parts
                or '\\' in name or ':' in name or name.casefold() in seen):
            raise ValueError('Invalid or colliding relative source path: ' + name)
        seen.add(name.casefold())


def inventory(root):
    result = {}
    if not root.is_dir() or root.is_symlink():
        raise ValueError('Expected a real source directory: ' + str(root))
    for path in sorted(root.rglob('*')):
        if path.is_symlink():
            raise ValueError('Source symlink: ' + str(path))
        if path.is_file():
            result[path.relative_to(root).as_posix()] = path.read_bytes()
    safe_names(result)
    return result


def hashes(files):
    return {name: sha(raw) for name, raw in sorted(files.items())}


def read_archive(raw, expected):
    safe_names(expected)
    decoded = {}
    with zipfile.ZipFile(io.BytesIO(raw)) as archive:
        entries = archive.infolist()
        safe_names(entry.filename for entry in entries)
        if len(entries) != len(expected) or {e.filename for e in entries} != set(expected):
            raise ValueError('Archive inventory mismatch')
        for entry in entries:
            mode = stat.S_IFMT(entry.external_attr >> 16)
            if entry.is_dir() or mode not in (0, stat.S_IFREG):
                raise ValueError('Non-regular archive member')
            content = archive.read(entry)
            if sha(content) != expected[entry.filename]:
                raise ValueError('Archive member hash mismatch: ' + entry.filename)
            decoded[entry.filename] = content
    return decoded


def load(package):
    contents = inventory(package)
    manifest_raw = contents.pop('manifest.json')
    manifest = json.loads(manifest_raw)
    if manifest.get('schema_version') != 1 or not manifest.get('completed') or hashes(contents) != manifest['files']:
        raise ValueError('Portable package inventory or identity differs')
    spec = json.loads(contents['overlay.json'])
    if (spec['schema_version'] != 1 or len(spec['chips']) != 38 or len(set(spec['chips'])) != 38
            or set(spec['components']) != {'embassy-stm32', 'stm32-metapac'}):
        raise ValueError('Unexpected RTC overlay scope')
    decoded = {}
    for name, record in spec['archives'].items():
        if sha(contents[name]) != record['sha256']:
            raise ValueError('Archive hash differs: ' + name)
        decoded[name] = read_archive(contents[name], record['files'])
    if set(decoded) != {'changes.zip', 'generator-inputs.zip', 'references.zip'}:
        raise ValueError('Unexpected portable archive set')
    return spec, decoded, manifest_raw


def compose(inputs, spec, changes, restore=False):
    output = {}
    if set(inputs) != set(spec['components']):
        raise ValueError('Expected both HAL and PAC sources')
    expected_changes = set()
    for component, record in spec['components'].items():
        before, after = record['before'], record['after']
        safe_names(before)
        safe_names(after)
        if not before.keys() <= after.keys():
            raise ValueError('Overlay removes an existing source')
        modified = {n for n in before if before[n] != after[n]}
        added = set(after) - set(before)
        for name in modified:
            expected_changes.add('before/' + component + '/' + name)
        for name in modified | added:
            expected_changes.add('after/' + component + '/' + name)
        if hashes(inputs[component]) != (after if restore else before):
            raise ValueError('Source inventory differs or is partly published: ' + component)
        for name in modified:
            if sha(changes['before/' + component + '/' + name]) != before[name]:
                raise ValueError('Preimage differs: ' + name)
        for name in modified | added:
            if sha(changes['after/' + component + '/' + name]) != after[name]:
                raise ValueError('Postimage differs: ' + name)
        if restore:
            output[component] = {n: changes['before/' + component + '/' + n] if n in modified else inputs[component][n] for n in before}
        else:
            output[component] = {**inputs[component], **{n: changes['after/' + component + '/' + n] for n in modified | added}}
        if hashes(output[component]) != (before if restore else after):
            raise ValueError('Composed source inventory differs')
    if set(changes) != expected_changes:
        raise ValueError('Unused or missing patch contents')
    return output


def replay(package, sources, output, restore=False):
    package, sources, output = package.resolve(), sources.resolve(), output.resolve()
    for source in [package, sources]:
        if output.is_relative_to(source) or source.is_relative_to(output):
            raise ValueError('Output overlaps input: ' + str(source))
    if output.exists():
        raise FileExistsError('Refusing to replace output: ' + str(output))
    original_package = inventory(package)
    spec, archives, manifest_raw = load(package)
    inputs = {name: inventory(sources / name) for name in spec['components']}
    result = compose(inputs, spec, archives['changes.zip'], restore)
    output.mkdir(parents=True, exist_ok=False)
    receipt = dict(completed=False, integrated=False, hil='not-run', restore=restore,
                   package_sha256=sha(manifest_raw), components={})
    def save():
        (output / 'replay.json').write_text(json.dumps(receipt, indent=2) + '\n', encoding='utf-8')
    save()
    try:
        for component, files in result.items():
            for name, raw in files.items():
                path = output / component / name
                path.parent.mkdir(parents=True, exist_ok=True)
                with path.open('xb') as handle:
                    handle.write(raw)
            if inventory(output / component) != files:
                raise ValueError('Written output differs: ' + component)
            receipt['components'][component] = hashes(files)
        if inventory(package) != original_package:
            raise ValueError('Package changed during replay')
        for name, files in inputs.items():
            if inventory(sources / name) != files:
                raise ValueError('Sources changed during replay: ' + name)
        receipt.update(completed=True, sources_unchanged=True)
        save()
        return receipt
    except BaseException as error:
        receipt['error'] = repr(error)
        save()
        raise


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--package', type=Path, default=Path(__file__).resolve().parent)
    parser.add_argument('--sources', type=Path, required=True, help='Directory containing embassy-stm32 and stm32-metapac')
    parser.add_argument('--output', type=Path, required=True, help='Fresh directory outside source/package trees')
    parser.add_argument('--restore', action='store_true', help='Restore the exact pre-RTC source baseline into the fresh output')
    args = parser.parse_args()
    receipt = replay(args.package, args.sources, args.output, args.restore)
    print('Replayed', {name: len(files) for name, files in receipt['components'].items()}, 'source files; hardware not run')
