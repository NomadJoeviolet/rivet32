"""Replay local evidence twice, run regressions, and record exact output hashes."""
import contextlib
import hashlib
import io
import json
from pathlib import Path
import subprocess
import sys

import prepare

HERE = Path(__file__).resolve().parent


def output_hashes():
    paths = prepare.expected_output_paths(HERE)
    return {p.relative_to(HERE).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}


def main():
    prepare.write_json(HERE / "validation.json", {"completed": False})
    log = io.StringIO()
    with contextlib.redirect_stdout(log):
        prepare.produce()
        first = output_hashes()
        prepare.produce()
        second = output_hashes()
    if first != second:
        raise RuntimeError("Evidence generation is not deterministic")
    tests = subprocess.run([sys.executable, "-m", "unittest", "discover", "-s", str(HERE),
                            "-p", "test_preparation.py", "-v"], capture_output=True, text=True)
    (HERE / ".build" / "validation.log").write_text(log.getvalue() + tests.stdout + tests.stderr, encoding="utf-8")
    if tests.returncode:
        raise RuntimeError(tests.stdout + tests.stderr)
    source = json.loads((HERE / "sources.json").read_text(encoding="utf-8"))
    scripts = {name: hashlib.sha256((HERE / name).read_bytes()).hexdigest()
               for name in ("prepare.py", "verify.py", "test_preparation.py", "capture_sources.py")}
    documentation = {name: hashlib.sha256((HERE / name).read_bytes()).hexdigest()
                     for name in ("README.md", "PLAN.md", "official-documents.json", "integration-decisions.json", "generator-inputs.json", "integer-audit.json")}
    result = {"completed": True, "tests_returncode": tests.returncode, "test_output": tests.stderr.strip(),
              "deterministic_replays": 2, "sources": len(source["files"]),
              "sources_lock_sha256": hashlib.sha256((HERE / "sources.json").read_bytes()).hexdigest(),
              "scripts": scripts, "documentation": documentation, "outputs": second,
              "gcc": subprocess.check_output(["gcc", "--version"], text=True).splitlines()[0],
              "python": sys.version.split()[0], "pac_generated": False, "hal_integrated": False}
    prepare.write_json(HERE / "validation.json", result)
    print(f"Verified {len(second)} deterministic outputs and {len(source['files'])} pinned sources; regressions passed")


if __name__ == "__main__":
    main()
