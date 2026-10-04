import io
import json
from pathlib import Path
import tempfile
import unittest
import zipfile
import provenance
import publish

class Snapshot(unittest.TestCase):
 def test_old_snapshot_is_complete_and_deterministic(self):
  with tempfile.TemporaryDirectory() as directory:
   path=Path(directory);raw=b"source\n";(path/"a.rs").write_bytes(raw)
   manifest={"completed":True,"files":{"a.rs":provenance.digest(raw)}}
   (path/"manifest.json").write_text(json.dumps(manifest),encoding="utf-8")
   a,receipt=publish.archive_existing(path,"v2");b,_=publish.archive_existing(path,"v2")
   self.assertEqual(a,b);self.assertEqual(receipt["files"],2)
   with zipfile.ZipFile(io.BytesIO(a)) as z:self.assertEqual(z.read("a.rs"),raw)
 def test_changed_or_extra_previous_file_is_rejected(self):
  with tempfile.TemporaryDirectory() as directory:
   path=Path(directory);(path/"a.rs").write_bytes(b"source")
   (path/"manifest.json").write_text(json.dumps({"completed":True,"files":{"a.rs":provenance.digest(b"source")}}),encoding="utf-8")
   (path/"a.rs").write_bytes(b"changed")
   with self.assertRaises(ValueError):publish.archive_existing(path,"v2")
   (path/"a.rs").write_bytes(b"source");(path/"untracked.rs").write_bytes(b"extra")
   with self.assertRaises(ValueError):publish.archive_existing(path,"v2")

if __name__=="__main__":unittest.main()
