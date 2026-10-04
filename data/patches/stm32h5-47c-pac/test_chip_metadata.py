import copy
import json
from pathlib import Path
import unittest

from chip_metadata import chip_scaffold, peripheral_dma, peripheral_pins

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "data/patches/stm32h5-47c/evidence.json").is_file())
DATA = json.loads((ROOT / "data/patches/stm32h5-47c/evidence.json").read_text(encoding="utf-8"))
CHIPS = {c["name"]: c for c in DATA["chips"]}


class ChipMetadata(unittest.TestCase):
    def test_all_fourteen_chips_have_exact_memory_and_physical_packages(self):
        self.assertEqual(len(CHIPS), 14)
        for chip in CHIPS.values():
            with self.subTest(chip=chip["name"]):
                out = chip_scaffold(chip, DATA["families"][chip["family"]])
                self.assertEqual(out["name"], chip["name"])
                self.assertEqual(out["device_id"], 0x47C)
                memory = {m["name"]: m for m in out["memory"][0]}
                flash = chip["flash_bytes"]
                self.assertEqual(memory["BANK_1"]["size"], flash // 2)
                self.assertEqual(memory["BANK_2"]["address"], 0x08000000 + flash // 2)
                self.assertEqual(memory["BANK_2"]["size"], flash // 2)
                self.assertEqual(memory["BANK_1"]["settings"]["write_size"], 16)
                self.assertEqual(memory["BANK_1"]["settings"]["erase_size"], 8192)
                self.assertEqual([(memory[x]["address"], memory[x]["size"]) for x in ("SRAM1", "SRAM2", "SRAM3")],
                                 [(0x20000000, 128*1024), (0x20020000, 80*1024), (0x20034000, 96*1024)])
                self.assertEqual(memory["BKPSRAM"]["size"], 2048)
                self.assertEqual(memory["OTP"]["settings"]["write_size"], 2)
                self.assertEqual(memory["OTP"]["address"], 0x08FFF000)
                self.assertEqual(len(out["packages"]), len(chip["packages"]))
                for physical, source in zip(out["packages"], sorted(chip["packages"], key=lambda p:p["refname"])):
                    self.assertEqual(physical["name"], source["refname"])
                    self.assertEqual(physical["package"], source["package"])
                    self.assertEqual(len(physical["pins"]), len(source["pins"]))
                self.assertFalse(any("FMC" in name or "SDRAM" in name for name in memory))

    def test_interrupt_numbers_preserve_gaps_and_security_capabilities(self):
        for name, expected in (("STM32H543CG", 113), ("STM32H553CG", 116)):
            chip=CHIPS[name]
            core=chip_scaffold(chip, DATA["families"][chip["family"]])["cores"][0]
            irqs={i["name"]:i["number"] for i in core["interrupts"]}
            self.assertEqual(len(irqs), expected)
            self.assertEqual(irqs["ADC3"],149)
            self.assertEqual(irqs["I3C2_ER"],132)
            self.assertEqual(core["nvic_priority_bits"],4)
            self.assertEqual(core["name"],"cm33")
            self.assertEqual("SAES" in irqs,name.startswith("STM32H553"))

    def test_all_sixteen_gpdma_channels_and_exact_two_dimensional_capabilities(self):
        chip=CHIPS["STM32H543CE"]
        channels=chip_scaffold(chip, DATA["families"][chip["family"]])["cores"][0]["dma_channels"]
        self.assertEqual(len(channels),16)
        self.assertEqual({c["name"] for c in channels if c["supports_2d"]},
                         {f"GPDMA{controller}_CH{channel}" for controller in (1,2) for channel in (6,7)})

    def test_dma_routes_preserve_unnamed_requests_and_adc3_above_st_hal_range(self):
        routes=peripheral_dma(DATA["families"]["STM32H543"])
        self.assertEqual(routes["ADC3"],[dict(signal="ADC3",dma=f"GPDMA{i}",request=142) for i in (1,2)])
        self.assertEqual(routes["OCTOSPI1"],[dict(signal="OCTOSPI1",dma=f"GPDMA{i}",request=57) for i in (1,2)])
        self.assertEqual({r["request"] for r in routes["I3C2"]},{136,137,138,139})
        self.assertNotIn("AES",routes)
        crypto=peripheral_dma(DATA["families"]["STM32H553"])
        self.assertIn("AES",crypto)
        self.assertEqual(sum(map(len,routes.values())),198)
        self.assertEqual(sum(map(len,crypto.values())),206)

    def test_only_bonded_pins_are_exposed_and_alt_functions_are_exact(self):
        for chip in CHIPS.values():
            pins=peripheral_pins(chip)
            bonded={p["gpio"] for package in chip["packages"] for p in package["pins"] if p["gpio"]}
            self.assertTrue(all(p["pin"] in bonded for values in pins.values() for p in values))
            for values in pins.values():
                self.assertEqual(len(values),len({(v["pin"],v["signal"],v.get("af")) for v in values}))
        small=peripheral_pins(CHIPS["STM32H543CE"])
        self.assertIn(dict(pin="PA9",signal="TX",af=7),small["USART1"])
        self.assertTrue(any(p["signal"].startswith("I2S_") for p in small["SPI1"]))
        self.assertTrue(any(p["signal"].startswith("INP") and "af" not in p for p in small["ADC1"]))
        self.assertFalse(any(p["pin"].startswith("PI") for values in small.values() for p in values))

    def test_fail_closed_on_mixed_die_capacity_missing_channels_or_conflicting_af(self):
        chip=copy.deepcopy(CHIPS["STM32H543CE"])
        family=copy.deepcopy(DATA["families"]["STM32H543"])
        chip["packages"][0]["flash_bytes"] *= 2
        with self.assertRaises(ValueError): chip_scaffold(chip,family)
        chip=copy.deepcopy(CHIPS["STM32H543CE"])
        chip["die"]="DIE484"
        with self.assertRaises(ValueError): chip_scaffold(chip,family)
        chip=copy.deepcopy(CHIPS["STM32H543CE"])
        del family["pointers"]["GPDMA1_Channel7_NS"]
        with self.assertRaises(ValueError): chip_scaffold(chip,family)
        row=next(x for x in chip["packages"][0]["alternate_functions"] if x["signal"]=="USART1_TX")
        chip["packages"][0]["alternate_functions"].append(dict(row,af=15))
        with self.assertRaises(ValueError): peripheral_pins(chip)


if __name__=="__main__": unittest.main()
