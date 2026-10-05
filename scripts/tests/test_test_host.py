"""Regression tests for the host campaign runner (Python stdlib only)."""

import importlib.util
import json
from pathlib import Path
import sys
import tempfile
import time
import unittest
from unittest import mock


SPEC = importlib.util.spec_from_file_location(
    "host_campaign", Path(__file__).resolve().parents[1] / "test_host.py")
runner = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(runner)

SUCCESS = "test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;"


class HostCampaignTests(unittest.TestCase):
    def test_success_normalizes_crlf_and_retains_both_output_streams(self):
        with tempfile.TemporaryDirectory() as directory:
            log = Path(directory) / "success.log"
            code = (f"import sys;sys.stdout.buffer.write({(SUCCESS + chr(13) + chr(10)).encode()!r});"
                    "print('diagnostic',file=sys.stderr)")
            output, elapsed = runner.invoke([sys.executable, "-c", code], log, 10)
            self.assertEqual(output, SUCCESS + "\n")
            self.assertEqual(runner.test_counts(output, 2)["passed"], 2)
            self.assertGreaterEqual(elapsed, 0)
            self.assertIn("diagnostic", log.read_text(encoding="utf-8"))

    def test_crashed_process_cannot_pass_by_printing_a_success_summary(self):
        with tempfile.TemporaryDirectory() as directory:
            log = Path(directory) / "crash.log"
            code = f"import sys;print({SUCCESS!r});print('failed',file=sys.stderr);sys.exit(3)"
            with self.assertRaisesRegex(RuntimeError, "exit 3"):
                runner.invoke([sys.executable, "-c", code], log, 10)
            output = log.read_text(encoding="utf-8")
            self.assertIn(SUCCESS, output)
            self.assertIn("failed", output)

    def test_missing_ambiguous_failed_and_incomplete_counts_cannot_pass(self):
        invalid = [
            ("", 2),
            (SUCCESS + "\n" + SUCCESS, 2),
            (SUCCESS, 3),
            (SUCCESS.replace("ok.", "FAILED."), 2),
            (SUCCESS.replace("0 failed", "1 failed"), 2),
            (SUCCESS.replace("0 measured", "1 measured"), 2),
            (SUCCESS.replace("0 filtered out", "1 filtered out"), 2),
        ]
        for output, discovered in invalid:
            with self.subTest(output=output, discovered=discovered):
                with self.assertRaises(RuntimeError):
                    runner.test_counts(output, discovered)

    def test_timeout_terminates_descendants_and_retains_partial_logs(self):
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            ready = folder / "ready.json"
            survived = folder / "survived"
            log = folder / "timeout.log"
            # This is a real parent -> child -> grandchild tree. The grandchild
            # writes evidence after the timeout if process-tree cleanup fails.
            grandchild = (
                "import json,pathlib,time;"
                f"pathlib.Path({str(ready)!r}).write_text(json.dumps(time.monotonic()+3));"
                "print('grandchild started',flush=True);time.sleep(3);"
                f"pathlib.Path({str(survived)!r}).write_text('survived timeout')")
            child = ("import subprocess,sys,time;"
                     f"subprocess.Popen([sys.executable,'-c',{grandchild!r}]);"
                     "time.sleep(10)")
            parent = ("import subprocess,sys,time;"
                      "print('parent started',flush=True);"
                      "print('parent diagnostic',file=sys.stderr,flush=True);"
                      f"subprocess.Popen([sys.executable,'-c',{child!r}]);"
                      "time.sleep(10)")
            started = time.monotonic()
            with self.assertRaisesRegex(RuntimeError, "timeout after 2s"):
                runner.invoke([sys.executable, "-c", parent], log, 2)
            self.assertLess(time.monotonic() - started, 8)
            output = log.read_text(encoding="utf-8")
            self.assertIn("parent started", output)
            self.assertIn("parent diagnostic", output)
            self.assertIn("grandchild started", output)
            self.assertIn("TIMEOUT after 2s", output)
            deadline = json.loads(ready.read_text(encoding="utf-8"))
            time.sleep(max(0, deadline - time.monotonic()) + 0.25)
            self.assertFalse(survived.exists(), "a grandchild survived the timeout")

    def test_keyboard_interrupt_reaps_the_process_and_preserves_a_log(self):
        real_popen = runner.subprocess.Popen
        interrupted = []

        def interrupt_first_wait(command, **kwargs):
            process = real_popen(command, **kwargs)
            # Windows tree cleanup invokes taskkill with Popen too; let that
            # command run normally and interrupt only the test invocation.
            if command[0] == sys.executable:
                real_wait = process.wait

                def wait(timeout=None):
                    process.wait = real_wait
                    raise KeyboardInterrupt

                process.wait = wait
                interrupted.append(process)
            return process

        with tempfile.TemporaryDirectory() as directory:
            log = Path(directory) / "interrupt.log"
            with mock.patch.object(runner.subprocess, "Popen", side_effect=interrupt_first_wait):
                with self.assertRaises(KeyboardInterrupt):
                    runner.invoke([sys.executable, "-c", "import time;time.sleep(10)"], log, 10)
            self.assertEqual(len(interrupted), 1)
            self.assertIsNotNone(interrupted[0].poll())
            self.assertIn("INTERRUPTED: KeyboardInterrupt", log.read_text(encoding="utf-8"))


if __name__ == "__main__":
    unittest.main()
