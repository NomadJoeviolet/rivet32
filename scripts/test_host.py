"""Repeat native runtime/framework tests and retain verifiable results (stdlib only)."""

import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import signal
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]
SUMMARY = re.compile(
    r"^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; "
    r"(\d+) measured; (\d+) filtered out;", re.MULTILINE
)


def positive(value):
    number = int(value)
    if number < 1:
        raise argparse.ArgumentTypeError("must be positive")
    return number


def source_digest():
    # Include first-party Rust sources/manifests and the runner. The lockfile
    # records external versions; this is not a digest of the dependency cache.
    paths = {ROOT / name for name in (
        "Cargo.toml", "Cargo.lock", "rust-toolchain.toml", ".cargo/config.toml",
        "scripts/test_host.py",
    )}
    paths.update(p for p in (ROOT / "crates").rglob("*")
                 if p.is_file() and (p.suffix == ".rs" or p.name == "Cargo.toml"))
    digest = hashlib.sha256()
    for path in sorted(paths):
        digest.update(path.relative_to(ROOT).as_posix().encode() + b"\0")
        digest.update(hashlib.sha256(path.read_bytes()).digest())
    return digest.hexdigest()


def terminate_tree(process):
    """Stop the invocation's process group and bound every cleanup wait."""
    errors = []
    try:
        if os.name == "nt":
            # Killing cargo alone leaves rustc/linker children running. Do not
            # capture pipes here either: inherited pipe handles can block EOF.
            result = subprocess.run(
                ["taskkill", "/PID", str(process.pid), "/T", "/F"],
                stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=5)
            if result.returncode:
                errors.append(f"taskkill exited {result.returncode}")
        else:
            os.killpg(process.pid, signal.SIGKILL)
    except ProcessLookupError:
        pass  # The group already exited.
    except (OSError, subprocess.TimeoutExpired) as error:
        errors.append(f"process-tree cleanup: {error}")
    # Fall back to terminating the immediate process even if tree cleanup failed.
    try:
        if process.poll() is None:
            process.kill()
        process.wait(timeout=5)
    except (OSError, subprocess.TimeoutExpired) as error:
        errors.append(f"process reap: {error}")
    return "\n".join(errors)


def invoke(command, log, timeout):
    env = os.environ.copy()
    env["CARGO_TERM_COLOR"] = "never"
    started = time.monotonic()
    group = ({"creationflags": subprocess.CREATE_NEW_PROCESS_GROUP} if os.name == "nt"
             else {"start_new_session": True})
    # Regular files avoid waiting for EOF from pipe handles inherited by a
    # grandchild. Text reads still normalize Windows CRLF for libtest parsing.
    with tempfile.TemporaryFile(mode="w+", encoding="utf-8", errors="replace") as stdout, \
            tempfile.TemporaryFile(mode="w+", encoding="utf-8", errors="replace") as stderr:
        process = subprocess.Popen(command, cwd=ROOT, env=env, stdout=stdout, stderr=stderr, **group)
        failure = None
        note = ""
        try:
            process.wait(timeout=timeout)
        except BaseException as error:
            # Also clean up on Ctrl-C; re-raise it after preserving partial logs.
            failure = error
            note = (f"TIMEOUT after {timeout}s" if isinstance(error, subprocess.TimeoutExpired)
                    else f"INTERRUPTED: {type(error).__name__}")
            cleanup = terminate_tree(process)
            if cleanup:
                note += "\n" + cleanup
        finally:
            stdout.seek(0)
            stderr.seek(0)
            output = stdout.read()
            log.write_text(output + "\n" + stderr.read() + ("\n" + note + "\n" if note else ""),
                           encoding="utf-8")
        if isinstance(failure, subprocess.TimeoutExpired):
            raise RuntimeError(f"timeout after {timeout}s; see {log.name}") from failure
        if failure is not None:
            raise failure
        if process.returncode:
            raise RuntimeError(f"exit {process.returncode}; see {log.name}")
        return output, time.monotonic() - started


def test_counts(output, discovered):
    matches = SUMMARY.findall(output)
    if len(matches) != 1:
        raise RuntimeError("missing or ambiguous libtest result")
    state, *values = matches[0]
    counts = dict(zip(("passed", "failed", "ignored", "measured", "filtered"), map(int, values)))
    if (state != "ok" or counts["failed"] or counts["measured"] or counts["filtered"]
            or counts["passed"] + counts["ignored"] != discovered):
        raise RuntimeError(f"unexpected test result: {counts}, discovered={discovered}")
    return counts


def discover(profile, args, folder, host):
    command = ["cargo", "test", "-p", "embodied-runtime", "-p", "embodied-framework",
               "--tests", "--no-run", "--message-format=json", "--locked", "--target", host]
    if profile == "release":
        command.append("--release")
    if args.offline:
        command.append("--offline")
    output, _ = invoke(command, folder / f"{profile}-build.log", args.build_timeout)
    artifacts = {}
    for line in output.splitlines():
        if not line.startswith("{"):
            continue
        message = json.loads(line)
        if (message.get("reason") == "compiler-artifact" and message.get("executable")
                and message.get("profile", {}).get("test")):
            artifacts[message["executable"]] = message["target"]
    suites = []
    for index, (executable, target) in enumerate(sorted(artifacts.items())):
        output, _ = invoke([executable, "--list", "--format", "terse"],
                           folder / f"{profile}-{index}-list.log", args.timeout)
        cases = re.findall(r"^(.+): test$", output, re.MULTILINE)
        suites.append({"name": target["name"], "kind": target["kind"],
                       "executable": executable, "cases": cases})
    if not suites or not sum(len(suite["cases"]) for suite in suites):
        raise RuntimeError(f"no test cases discovered for {profile}")
    return command, suites


def save_report(report, folder):
    (folder / "summary.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    passed = sum(run.get("passed", 0) for run in report["runs"])
    ignored = sum(run.get("ignored", 0) for run in report["runs"])
    lines = ["# Native host test campaign", "",
             f"Status: **{report['status']}**. Recorded passing test executions: **{passed}**; "
             f"ignored executions: {ignored}.", "",
             f"- Commit: `{report.get('commit', 'unknown')}`; dirty at start: `{report.get('dirty', 'unknown')}`",
             f"- Host: `{report['platform']}`",
             f"- Started (UTC): {report['started_at']}; elapsed: {report['elapsed_seconds']:.2f}s",
             f"- Source SHA-256: `{report.get('source_before', 'unknown')}`",
             f"- Sources unchanged at completion: `{report.get('sources_unchanged', False)}`",
             f"- Requested rounds per profile: {report['rounds']}; test threads rotate: {report['test_threads']}",
             "", "| Profile | Suite | Cases | Completed executions | Passed | Ignored |",
             "|---|---|---:|---:|---:|---:|"]
    for profile, item in report["profiles"].items():
        for suite in item["suites"]:
            runs = [run for run in report["runs"] if run["profile"] == profile
                    and run["suite"] == suite["name"] and run["status"] == "passed"]
            lines.append(f"| {profile} | {suite['name']} | {len(suite['cases'])} | {len(runs)} | "
                         f"{sum(run['passed'] for run in runs)} | {sum(run['ignored'] for run in runs)} |")
    lines += ["", "```text", report.get("rustc", "unavailable").strip(), "```", "",
              "Source digest covers first-party crates/*.rs, crate manifests, workspace manifest/lockfile, "
              "toolchain, Cargo config and this runner. Full commands, discovered cases, thread counts, "
              "durations and per-executable results are in summary.json; raw output is in *.log.", "",
              "These native fake-driver/manual-clock tests do not validate MCU interrupt behavior, "
              "the Embassy executor, physical CAN traffic or real-time deadlines. Repetition is not "
              "exhaustive interleaving exploration or a branch-coverage measurement."]
    if report.get("error"):
        lines += ["", f"Failure: {report['error']}"]
    (folder / "summary.md").write_text("\n".join(lines) + "\n", encoding="utf-8")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rounds", type=positive, default=10, help="rounds per profile (default: 10)")
    parser.add_argument("--profiles", nargs="+", choices=("debug", "release"), default=["debug", "release"])
    parser.add_argument("--test-threads", nargs="+", type=positive, default=[1, 4, 16])
    parser.add_argument("--timeout", type=positive, default=30, help="seconds per test executable")
    parser.add_argument("--build-timeout", type=positive, default=600)
    parser.add_argument("--offline", action="store_true")
    parser.add_argument("--report-dir", type=Path)
    args = parser.parse_args()
    if len(set(args.profiles)) != len(args.profiles):
        parser.error("profiles must be unique")
    stamp = datetime.now(timezone.utc)
    folder = args.report_dir or ROOT / "target/test-reports" / stamp.strftime("%Y%m%dT%H%M%S.%fZ")
    if not folder.is_absolute():
        folder = ROOT / folder
    # Never overwrite earlier evidence, including partial/failed campaigns.
    folder.mkdir(parents=True, exist_ok=False)
    started = time.monotonic()
    report = {"status": "failed", "started_at": stamp.isoformat(), "platform": platform.platform(),
              "rounds": args.rounds, "test_threads": args.test_threads,
              "profiles": {}, "runs": [], "options": {**vars(args), "report_dir": str(folder)}}
    try:
        report["source_before"] = source_digest()
        report["commit"], _ = invoke(["git", "rev-parse", "HEAD"], folder / "commit.log", 30)
        report["commit"] = report["commit"].strip()
        status, _ = invoke(["git", "status", "--porcelain"], folder / "git-status.log", 30)
        report["dirty"] = bool(status.strip())
        report["rustc"], _ = invoke(["rustc", "-Vv"], folder / "rustc.log", 30)
        host = re.search(r"^host: (\S+)$", report["rustc"], re.MULTILINE).group(1)
        for profile in args.profiles:
            print(f"Building and discovering {profile} tests...", flush=True)
            command, suites = discover(profile, args, folder, host)
            report["profiles"][profile] = {"build_command": command, "suites": suites, "completed_rounds": 0}
            for round_number in range(1, args.rounds + 1):
                threads = args.test_threads[(round_number - 1) % len(args.test_threads)]
                for index, suite in enumerate(suites):
                    log = folder / f"{profile}-{round_number:04d}-{index}-{suite['name']}.log"
                    command = [suite["executable"], "--test-threads", str(threads)]
                    run = {"profile": profile, "round": round_number, "suite": suite["name"],
                           "test_threads": threads, "command": command, "log": log.name, "status": "failed"}
                    report["runs"].append(run)
                    output, elapsed = invoke(command, log, args.timeout)
                    run["elapsed_seconds"] = elapsed
                    run.update(test_counts(output, len(suite["cases"])))
                    run["status"] = "passed"
                report["profiles"][profile]["completed_rounds"] = round_number
                if round_number % 10 == 0 or round_number == args.rounds:
                    print(f"{profile}: {round_number}/{args.rounds} rounds passed", flush=True)
        report["status"] = "passed"
    except (OSError, ValueError, RuntimeError, AttributeError, KeyboardInterrupt) as error:
        report["error"] = str(error) or type(error).__name__
    finally:
        report["elapsed_seconds"] = time.monotonic() - started
        try:
            report["source_after"] = source_digest()
            report["sources_unchanged"] = report["source_after"] == report.get("source_before")
            if not report["sources_unchanged"]:
                report["status"] = "failed"
                report["error"] = report.get("error", "") + " Source inputs changed during campaign."
        except OSError as error:
            report["status"] = "failed"
            report["error"] = report.get("error", "") + f" Cannot verify source inputs: {error}"
        save_report(report, folder)
    print(f"{report['status']}: {folder / 'summary.md'}", flush=True)
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    sys.exit(main())
