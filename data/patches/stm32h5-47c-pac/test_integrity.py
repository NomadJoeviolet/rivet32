import json
from pathlib import Path
import shutil
import tempfile
import unittest
import generate
import provenance
class Integrity(unittest.TestCase):
 def test_second_writer_is_rejected_without_touching_owner(self):
  with tempfile.TemporaryDirectory() as name:
   d=Path(name)
   with generate.output_lock(d):
    owner=(d/"generation.lock").read_bytes()
    with self.assertRaisesRegex(RuntimeError,"no artifact was read"):
     with generate.output_lock(d):self.fail("second writer acquired lease")
    self.assertEqual((d/"generation.lock").read_bytes(),owner)
   self.assertFalse((d/"generation.lock").exists())
 def test_interrupt_preserves_output_lock(self):
  with tempfile.TemporaryDirectory() as name:
   d=Path(name)
   with self.assertRaises(KeyboardInterrupt):
    with generate.output_lock(d):raise KeyboardInterrupt()
   self.assertTrue((d/"generation.lock").exists())
 def test_unlocked_source_rejected(self):
  with tempfile.TemporaryDirectory() as name:
   d=Path(name);shutil.copytree(provenance.HERE/"sources",d/"sources");shutil.copyfile(provenance.HERE/"sources.json",d/"sources.json")
   (d/"sources/unlocked.json").write_text("{}")
   with self.assertRaisesRegex(ValueError,"Source inventory"):provenance.verify(provenance.workspace(),d)
 def test_missing_frozen_dependency_rejected(self):
  with tempfile.TemporaryDirectory() as name:
   d=Path(name);lock=json.loads((provenance.HERE/"sources.json").read_text(encoding="utf-8"));lock["frozen_inputs"].pop("evidence.json")
   (d/"sources.json").write_text(json.dumps(lock),encoding="utf-8")
   with self.assertRaisesRegex(ValueError,"Frozen input inventory"):provenance.verify(provenance.workspace(),d)
 def test_stale_chip_input_rejected_before_generation(self):
  with tempfile.TemporaryDirectory() as name:
   d=Path(name);(d/"build/data/chips").mkdir(parents=True);(d/"build/data/chips/FAKE.json").write_text("{}")
   with self.assertRaisesRegex(ValueError,"Unexpected chip"):generate.build_inputs(provenance.workspace(),d)
if __name__=="__main__":unittest.main()
