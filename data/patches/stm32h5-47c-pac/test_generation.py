import json
from pathlib import Path
import tempfile
import unittest
import generate
import provenance


class Generation(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp=tempfile.TemporaryDirectory()
        cls.path=Path(cls.temp.name)
        generate.build_inputs(provenance.workspace(),cls.path)
        cls.chips=[json.loads(p.read_text(encoding="utf-8")) for p in sorted((cls.path/"build/data/chips").glob("*.json"))]

    @classmethod
    def tearDownClass(cls):cls.temp.cleanup()

    def test_exact_fourteen_parts_and_no_duplicate_eth_token(self):
        self.assertEqual(len(self.chips),14)
        for chip in self.chips:
            names=[p["name"] for p in chip["cores"][0]["peripherals"]]
            self.assertEqual(len(names),len(set(names)))
            self.assertIn("ETH",names);self.assertNotIn("ETH_MAC",names)

    def test_verified_driver_versions_and_usb_ram(self):
        for chip in self.chips:
            peris={p["name"]:p for p in chip["cores"][0]["peripherals"]}
            for name,kind in (("TIM2","timer"),("SPI1","spi"),("USB","usb")):
                self.assertEqual((peris[name]["registers"]["kind"],peris[name]["registers"]["version"]),(kind,"h5_47c"))
            self.assertEqual(peris["TIM2"]["registers"]["block"],"TIM_GP32")
            self.assertEqual(peris["USBRAM"]["address"],0x40016400)

    def test_exact_fdcan_ram_regions_for_every_part(self):
        for chip in self.chips:
            peris={p["name"]:p for p in chip["cores"][0]["peripherals"]}
            for number,base in ((1,0x4000ac00),(2,0x4000af50)):
                ram=peris["FDCANRAM"+str(number)]
                self.assertEqual(ram["address"],base)
                self.assertEqual(ram["registers"],{"kind":"fdcanram","version":"v1","block":"FDCANRAM"})
            self.assertNotIn("FDCANRAM3",peris)

    def test_crs_register_reuse_keeps_exact_chip_irq(self):
        for chip in self.chips:
            crs=next(p for p in chip["cores"][0]["peripherals"] if p["name"]=="CRS")
            self.assertEqual(crs["registers"],{"kind":"crs","version":"v1","block":"CRS"})
            self.assertEqual(crs["address"],0x40006000)
            self.assertEqual(crs["interrupts"],[{"signal":"GLOBAL","interrupt":"CRS"}])

    def test_each_selected_register_block_exists_in_one_version_per_kind(self):
        for chip in self.chips:
            kinds={}
            for p in chip["cores"][0]["peripherals"]:
                s=p["registers"]
                self.assertEqual(kinds.setdefault(s["kind"],s["version"]),s["version"])
                d=json.loads((self.path/"build/data/registers"/(s["kind"]+"_"+s["version"]+".json")).read_text(encoding="utf-8"))
                self.assertIn("block/"+s["block"],d)


if __name__=="__main__":unittest.main()
