"""Explicitly restore byte-pinned audit documents; stdlib only, no PDF bundling.

Run from a clean checkout or generated project before verifying full source
evidence. Source/overlay manifests are validated before any network request.
Correct existing files require no network. Failed retrieval never replaces a
previous cache entry; only a completed, matching download is installed.
"""
import hashlib
import os
from pathlib import Path
import tempfile
import urllib.request

from generate_hal_metadata import ROOT, checked_child_path, verified_source_documents


def fetch_document(root, record):
    path = checked_child_path(root, record["cache_path"])
    if not path.is_relative_to((root / "data/sources").resolve()):
        raise ValueError("Document cache must be inside data/sources")
    expected = record["sha256"]
    if path.is_file() and hashlib.sha256(path.read_bytes()).hexdigest() == expected:
        return "cached"
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = None
    try:
        request = urllib.request.Request(record["url"], headers={"User-Agent": "embodied-framework-source-audit"})
        with urllib.request.urlopen(request, timeout=60) as response:
            with tempfile.NamedTemporaryFile(dir=path.parent, prefix=".document-", suffix=".tmp", delete=False) as stream:
                temporary = Path(stream.name)
                digest = hashlib.sha256()
                while chunk := response.read(1024 * 1024):
                    stream.write(chunk)
                    digest.update(chunk)
                stream.flush()
                os.fsync(stream.fileno())
        if digest.hexdigest() != expected:
            raise ValueError(f"Downloaded document SHA-256 differs: {record['url']}")
        os.replace(temporary, path)
        temporary = None
        return "downloaded"
    finally:
        if temporary is not None:
            temporary.unlink(missing_ok=True)


def main():
    for record in verified_source_documents(ROOT):
        status = fetch_document(ROOT, record)
        print(f"{status}: {record['cache_path']} SHA-256 {record['sha256']}")


if __name__ == "__main__":
    main()
