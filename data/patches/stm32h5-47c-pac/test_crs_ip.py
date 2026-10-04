from copy import deepcopy
import unittest
import crs_ip
import provenance

class CrsProof(unittest.TestCase):
 def test_both_exact_families(self):
  for family in ("STM32H543","STM32H553"):
   self.assertEqual(crs_ip.derive(provenance.workspace(),family)["spec"],{"kind":"crs","version":"v1","block":"CRS"})
 def test_changed_macro_and_layout_fail_closed(self):
  folder=crs_ip.HERE/"sources/crs-audit"
  a=crs_ip.load(folder/"STM32H543.json");b=crs_ip.load(folder/"STM32H563.json")
  ir=crs_ip.load(crs_ip.HERE/"sources/registers/crs_v1.json")
  for changed in ("macro","layout"):
   bad=deepcopy(a)
   if changed=="macro":bad["ip"]["CRS"]["numeric_macros"]["CRS_CR_CEN_Pos"]+=1
   else:bad["ip"]["CRS"]["type"]["members"][0]["offset"]=4
   with self.assertRaises(ValueError):crs_ip.check_evidence(bad,b,ir)
 def test_sync_enum_and_ir_width_fail_closed(self):
  folder=crs_ip.HERE/"sources/crs-audit"
  a=crs_ip.load(folder/"STM32H543.json");b=crs_ip.load(folder/"STM32H563.json")
  ir=crs_ip.load(crs_ip.HERE/"sources/registers/crs_v1.json")
  bad=deepcopy(ir);bad["enum/SYNCSRC"]["variants"][0]["value"]=3
  with self.assertRaises(ValueError):crs_ip.check_evidence(a,b,bad)
  bad=deepcopy(ir);bad["fieldset/CR"]["fields"][0]["bit_size"]+=1
  with self.assertRaises(ValueError):crs_ip.check_evidence(a,b,bad)

if __name__=="__main__":unittest.main()
