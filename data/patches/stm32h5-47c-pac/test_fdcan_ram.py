import unittest
import fdcan_ram
import provenance
class FdcanRam(unittest.TestCase):
 def test_exact_layout_from_hal_macros_and_gcc_oracle(self):
  for family in ("STM32H543","STM32H553"):
   result=fdcan_ram.derive(provenance.workspace(),family)
   self.assertEqual(result["size"],848)
   self.assertEqual(result["addresses"],{"FDCANRAM1":0x4000ac00,"FDCANRAM2":0x4000af50})
   self.assertEqual(result["offsets"],[0,112,176,392,608,632])
 def test_narrow_expression_grammar_rejects_width_and_side_effects(self):
  for expression in ("((uint32_t)0x100000000ULL)","(~0U)","(0xffffffffU + 1U)","(1U << 31)","call(1)"):
   with self.assertRaises(ValueError):fdcan_ram.Constants({"bad":expression}).value("bad")
  self.assertEqual(fdcan_ram.Constants({"a":"(28U)","b":"((uint32_t)(a * 4U))"}).value("b"),112)
if __name__=="__main__":unittest.main()
