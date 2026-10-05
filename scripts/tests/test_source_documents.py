"""Exercise document retrieval against a local HTTP server, without the mirror."""
import hashlib
import http.server
import importlib.util
import io
from pathlib import Path
import sys
import tempfile
import threading
import unittest
from unittest.mock import patch

SCRIPTS = Path(__file__).resolve().parents[2] / "xtask/scripts"
sys.path.insert(0, str(SCRIPTS))
spec = importlib.util.spec_from_file_location("fetch_source_documents", SCRIPTS / "fetch_source_documents.py")
fetch = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fetch)

DOCUMENT = b"%PDF-1.4\nfixed test document\n"
EXPECTED = hashlib.sha256(DOCUMENT).hexdigest()


class DocumentFetchTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        self.responses = []
        self.requests = 0
        owner = self

        class Handler(http.server.BaseHTTPRequestHandler):
            def do_GET(self):
                owner.requests += 1
                status, body, length = owner.responses[min(owner.requests - 1, len(owner.responses) - 1)]
                self.send_response(status)
                self.send_header("Content-Type", "application/pdf" if body.startswith(b"%PDF") else "text/html")
                self.send_header("Content-Length", str(length if length is not None else len(body)))
                self.end_headers()
                self.wfile.write(body)
                self.close_connection = True

            def log_message(self, *args):
                pass

        self.server = http.server.HTTPServer(("127.0.0.1", 0), Handler)
        self.thread = threading.Thread(target=self.server.serve_forever, kwargs={"poll_interval": 0.01})
        self.thread.start()
        self.addCleanup(self.stop_server)
        self.record = {
            "url": f"http://127.0.0.1:{self.server.server_port}/document.pdf",
            "cache_path": "data/sources/document.pdf",
            "sha256": EXPECTED,
        }
        self.path = self.root / self.record["cache_path"]
        self.addCleanup(patch.stopall)
        patch("time.sleep").start()
        self.errors = patch("sys.stderr", new_callable=io.StringIO).start()
        # Keep local fault-injection traffic independent of developer proxy settings.
        direct = fetch.urllib.request.build_opener(fetch.urllib.request.ProxyHandler({}))
        patch.object(fetch.urllib.request, "urlopen", direct.open).start()

    def stop_server(self):
        self.server.shutdown()
        self.server.server_close()
        self.thread.join()

    def check_installed(self, attempts):
        self.assertEqual(fetch.fetch_document(self.root, self.record), "downloaded")
        self.assertEqual(self.path.read_bytes(), DOCUMENT)
        self.assertEqual(self.requests, attempts)
        self.assertEqual(list(self.path.parent.glob(".document-*.tmp")), [])

    def test_valid_cache_needs_no_network(self):
        self.path.parent.mkdir(parents=True)
        self.path.write_bytes(DOCUMENT)
        self.assertEqual(fetch.fetch_document(self.root, self.record), "cached")
        self.assertEqual(self.requests, 0)

    def test_html_response_then_pinned_document(self):
        self.responses = [(200, b"<html>try again</html>", None), (200, DOCUMENT, None)]
        self.check_installed(2)
        self.assertIn("text/html", self.errors.getvalue())
        self.assertIn(EXPECTED, self.errors.getvalue())

    def test_invalid_cache_is_replaced_only_by_matching_document(self):
        self.path.parent.mkdir(parents=True)
        self.path.write_bytes(b"invalid cache entry")
        self.responses = [(200, DOCUMENT, None)]
        self.check_installed(1)

    def test_http_service_failure_then_pinned_document(self):
        self.responses = [(503, b"unavailable", None), (200, DOCUMENT, None)]
        self.check_installed(2)

    def test_truncated_response_then_pinned_document(self):
        self.responses = [(200, DOCUMENT[:5], len(DOCUMENT)), (200, DOCUMENT, None)]
        self.check_installed(2)

    def test_wrong_document_never_replaces_existing_file(self):
        self.path.parent.mkdir(parents=True)
        self.path.write_bytes(b"previous cache entry")
        self.responses = [(200, b"%PDF-different revision", None)]
        with self.assertRaisesRegex(ValueError, "SHA-256") as caught:
            fetch.fetch_document(self.root, self.record)
        self.assertEqual(self.requests, 3)
        self.assertIn(EXPECTED, str(caught.exception))
        self.assertEqual(self.path.read_bytes(), b"previous cache entry")
        self.assertEqual(list(self.path.parent.glob(".document-*.tmp")), [])

    def test_missing_document_fails_without_retry(self):
        self.responses = [(404, b"missing", None)]
        with self.assertRaises(fetch.urllib.error.HTTPError):
            fetch.fetch_document(self.root, self.record)
        self.assertEqual(self.requests, 1)
        self.assertFalse(self.path.exists())


if __name__ == "__main__":
    unittest.main()
