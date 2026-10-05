"""Explicitly restore byte-pinned audit documents; stdlib only, no PDF bundling.

Run from a clean checkout or generated project before verifying full source
evidence. Source/overlay manifests are validated before any network request.
Correct existing files require no network. Failed retrieval never replaces a
previous cache entry; only a completed, matching download is installed.
"""
import hashlib
import http.client
import os
from pathlib import Path
import sys
import tempfile
import time
import urllib.error
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
    for attempt in range(1, 4):
        temporary = None
        try:
            request = urllib.request.Request(record["url"], headers={"User-Agent": "embodied-framework-source-audit"})
            with urllib.request.urlopen(request, timeout=60) as response:
                content_type = response.headers.get("Content-Type", "unknown")
                status = response.status
                size = 0
                with tempfile.NamedTemporaryFile(dir=path.parent, prefix=".document-", suffix=".tmp", delete=False) as stream:
                    temporary = Path(stream.name)
                    digest = hashlib.sha256()
                    while chunk := response.read(1024 * 1024):
                        stream.write(chunk)
                        digest.update(chunk)
                        size += len(chunk)
                    stream.flush()
                    os.fsync(stream.fileno())
            actual = digest.hexdigest()
            if actual != expected:
                raise ValueError(
                    f"Downloaded document SHA-256 differs: {record['url']}; "
                    f"expected={expected}, actual={actual}, bytes={size}, "
                    f"HTTP={status}, Content-Type={content_type}"
                )
            os.replace(temporary, path)
            temporary = None
            return "downloaded"
        except (urllib.error.URLError, TimeoutError, ConnectionError, http.client.HTTPException, ValueError) as error:
            # Retry transient responses, including HTTP 200 error pages. Every
            # attempt must match the original digest; persistent changes fail.
            if isinstance(error, urllib.error.HTTPError):
                if error.code not in {408, 429, 500, 502, 503, 504}:
                    raise
            if attempt == 3:
                raise
            print(f"Document attempt {attempt}/3 failed: {error}; retrying", file=sys.stderr, flush=True)
        finally:
            if temporary is not None:
                temporary.unlink(missing_ok=True)
        time.sleep(2 ** (attempt - 1))


def main():
    for record in verified_source_documents(ROOT):
        status = fetch_document(ROOT, record)
        print(f"{status}: {record['cache_path']} SHA-256 {record['sha256']}")


if __name__ == "__main__":
    main()
