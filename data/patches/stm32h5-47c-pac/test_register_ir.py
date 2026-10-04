"""Golden constraints from GCC-checked CMSIS and pinned DFP SVD."""
import unittest
from pathlib import Path
import register_ir
ROOT = register_ir.provenance.workspace()
class RegisterTests(unittest.TestCase):
 @classmethod
 def setUpClass(cls):
  cls.products = register_ir.build(ROOT)
 def ir(self, kind, family="STM32H543"):
  return self.products[family]["registers"][kind]
 def test_rcc_two_plls_and_no_crypto_in_543(self):
  d=self.ir("rcc"); items={x["name"]:x for x in d["block/RCC"]["items"]}
  self.assertEqual(items["PLLCFGR"]["array"], {"len":2,"stride":4})
  self.assertEqual(next(x for x in d["fieldset/CR"]["fields"] if x["name"]=="PLLON")["array"]["len"],2)
  self.assertNotIn("SAESEN",[x["name"] for x in d["fieldset/AHB2ENR"]["fields"]])
 def test_mux_sources_are_exact(self):
  d=self.ir("rcc")
  self.assertEqual({x["name"]:x["value"] for x in d["enum/SPI1SEL"]["variants"]},{"PLL1_Q":0,"PLL2_P":1,"I2S_CKIN":3,"PER":4})
  self.assertEqual({x["name"]:x["value"] for x in d["enum/I3C2SEL"]["variants"]},{"PCLK3":0,"PLL2_R":1,"HSI":2})
  self.assertFalse(any("PLL3" in x["name"] for k,v in d.items() if k.startswith("enum/") for x in v["variants"]))
 def test_two_bit_system_clock_enum_matches_st_hal(self):
  d=self.ir("rcc")
  self.assertEqual(d["enum/SW"]["bit_size"],2)
  self.assertEqual({x["name"]:x["value"] for x in d["enum/SW"]["variants"]},{"HSI":0,"CSI":1,"HSE":2,"PLL1_P":3})
  for name in ("SW","SWS"):
   f=next(f for f in d["fieldset/CFGR"]["fields"] if f["name"]==name)
   self.assertEqual((f["bit_size"],f["enum"]),(2,"SW"))
 def test_rtc_and_bus_clock_api_names_preserve_exact_values(self):
  for family in register_ir.FAMILIES:
   d=self.ir("rcc",family)
   def values(name):return {x["name"]:x["value"] for x in d["enum/"+name]["variants"]}
   self.assertEqual(values("RTCSEL"),{"DISABLE":0,"LSE":1,"LSI":2,"HSE_DIV_RTCPRE":3})
   self.assertEqual(values("ADCDACSEL")["HCLK2"],0)
   self.assertEqual(values("ADCDACSEL")["SYS"],1)
   self.assertEqual(values("OCTOSPI1SEL")["HCLK4"],0)
   self.assertEqual(values("ETHPTPCLKSEL")["HCLK1"],0)
 def test_power_changed_positions(self):
  d=self.ir("pwr"); f={x["name"]:x for x in d["fieldset/PMCR"]["fields"]}
  self.assertEqual(f["SRAM2_48SO"]["bit_offset"],26)
  self.assertEqual(f["SRAM1SO"]["bit_offset"],27)
  self.assertNotIn("SMPSEN",[x["name"] for x in d["fieldset/SCCR"]["fields"]])
 def test_flash_exact_members_and_widths(self):
  d=self.ir("flash"); names={x["name"] for x in d["block/FLASH"]["items"]}
  self.assertIn("NSEPOCHR_PRG",names);self.assertNotIn("SECBB1R3",names)
  for fs,field,w in [("NSCR","SNB",6),("SECWM","SECWM_STRT",6),("WRP","WRPSG",16)]:
   self.assertEqual(next(x["bit_size"] for x in d["fieldset/"+fs]["fields"] if x["name"]==field),w)
 def test_rng_has_real_new_registers(self):
  d=self.ir("rng");items={x["name"]:x for x in d["block/RNG"]["items"]}
  self.assertEqual(items["HTSR"]["byte_offset"],32);self.assertEqual(items["HTSR"]["array"]["len"],2)
  self.assertEqual(items["NSMR"]["byte_offset"],48)
  self.assertEqual(next(x["bit_size"] for x in d["fieldset/CR"]["fields"] if x["name"]=="RNG_CONFIG1"),8)
  self.assertIn("RNG.NSMR",self.products["STM32H543"]["raw_registers"]["rng"])
 def test_i3c_documented_mask_corrections(self):
  d=self.ir("i3c");f={x["name"]:x for x in d["fieldset/TIMINGR2"]["fields"]}
  self.assertEqual(f["STALLS"]["bit_offset"],5);self.assertEqual(f["STALLL"]["bit_offset"],6)
 def test_every_cmsis_member_is_addressed(self):
  for family,p in self.products.items():
   self.assertEqual(p["coverage"]["unmapped_members"],[],family)
 def test_svd_unknown_access_fails_closed(self):
  self.assertEqual(register_ir.access_policy(None),"Raw")
  self.assertEqual(register_ir.access_policy("read-only"),"Read")
if __name__=="__main__":unittest.main()
