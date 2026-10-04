"""Evict reconstructible Cargo outputs while the caller owns its cache lock."""
from pathlib import Path
import os
import shutil
import stat

DEFAULT_LIMIT_MIB = 2048
TARGETS = {'thumbv6m-none-eabi', 'thumbv7m-none-eabi', 'thumbv7em-none-eabi',
           'thumbv7em-none-eabihf', 'thumbv8m.main-none-eabi', 'thumbv8m.main-none-eabihf'}


def limit_bytes(value=None):
    if value is None:
        value = str(DEFAULT_LIMIT_MIB)
    if not isinstance(value, str) or not value.isascii() or not value.isdecimal():
        raise ValueError('cache limit must be a positive MiB integer')
    mib = int(value)
    if not 0 < mib <= (2**64 - 1) // (1024 * 1024):
        raise ValueError('cache limit is zero or overflows a byte count')
    return mib * 1024 * 1024


def _native(path):
    """Use Windows extended paths for Cargo's deeply nested build outputs."""
    path = Path(path)
    if os.name != 'nt':
        return path
    value = str(path.absolute())
    if value.startswith('\\\\?\\'):
        return Path(value)
    if value.startswith('\\\\'):
        return Path('\\\\?\\UNC\\' + value[2:])
    return Path('\\\\?\\' + value)


def _plain(path):
    meta = _native(path).lstat()
    if stat.S_ISLNK(meta.st_mode) or getattr(meta, 'st_file_attributes', 0) & 0x400:
        raise ValueError('cache contains a link or reparse point: ' + str(path))
    if not (stat.S_ISDIR(meta.st_mode) or stat.S_ISREG(meta.st_mode)):
        raise ValueError('cache contains a non-regular entry: ' + str(path))
    return meta


def _size(path):
    meta = _plain(path)
    if stat.S_ISREG(meta.st_mode):
        return meta.st_size
    return sum(_size(child) for child in _native(path).iterdir())


def prune_locked(base, cache, target, budget):
    """Only call while holding the same lock as the Cargo/cache writer.

    Eligible paths are output/cache/<target> or output/dual-cache/cm[47].
    Reports, source inventories, lock files, and final ELF/MAPs are outside
    those build trees. The threshold is checked before a subsequent build.
    """
    base = Path(base).resolve(strict=True)
    cache = Path(cache).absolute()
    if target not in TARGETS or not isinstance(budget, int) or budget <= 0:
        raise ValueError('invalid cache target or byte budget')
    try:
        relative = cache.relative_to(base)
    except ValueError as error:
        raise ValueError('cache escaped selected output directory') from error
    allowed = {('cache', target), ('dual-cache', 'cm7'), ('dual-cache', 'cm4')}
    if relative.parts not in allowed:
        raise ValueError('not a reserved constructor cache path')
    current = base
    for part in relative.parts:
        current = current / part
        try:
            meta = _plain(current)
        except FileNotFoundError:
            continue
        if not stat.S_ISDIR(meta.st_mode):
            raise ValueError('cache component is not a directory')
    if not cache.exists():
        return dict(path=str(cache), limit_bytes=budget, bytes_before=0, bytes_after=0, removed_bytes=0)
    cache = cache.resolve(strict=True)
    if not cache.is_relative_to(base):
        raise ValueError('resolved cache escaped selected output directory')
    before = _size(cache)
    if before > budget:
        for name in ['release', target]:
            child = cache / name
            if not child.exists():
                continue
            if not stat.S_ISDIR(_plain(child).st_mode):
                raise ValueError('unexpected non-directory Cargo output')
            resolved = child.resolve(strict=True)
            if resolved.parent != cache:
                raise ValueError('Cargo output escaped selected constructor cache')
            shutil.rmtree(_native(resolved))
    after = _size(cache)
    if after > before:
        raise ValueError('cache grew while exclusively locked')
    return dict(path=str(cache), limit_bytes=budget, bytes_before=before, bytes_after=after,
                removed_bytes=before - after)
