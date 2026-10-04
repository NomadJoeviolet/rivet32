"""DIE47C evidence checks; no shared workspace mutation."""
import hashlib
import importlib.util
import json
from pathlib import Path
import unittest
import tempfile
import re

HERE = Path(__file__).resolve().parent
PARTS = {"STM32H543" + suffix for suffix in ("CE", "CG", "RE", "RG", "UG", "VE", "VG", "ZE", "ZG")} | {
    "STM32H553" + suffix for suffix in ("CG", "RG", "UG", "VG", "ZG")}


class Evidence(unittest.TestCase):
    def catalog(self):
        path = HERE / "evidence.json"
        self.assertTrue(path.exists(), "generate the exact H543/H553 evidence, not a H563 alias")
        return json.loads(path.read_text(encoding="utf-8"))

    def test_all_fourteen_parts_and_twenty_nine_packages(self):
        catalog = self.catalog()
        self.assertEqual({p["name"] for p in catalog["chips"]}, PARTS)
        self.assertEqual(sum(len(p["packages"]) for p in catalog["chips"]), 29)
        self.assertTrue(all(p["die"] == "DIE47C" for p in catalog["chips"]))

    def test_accurate_flash_and_sram_not_h563_memory(self):
        for part in self.catalog()["chips"]:
            self.assertEqual(part["flash_bytes"], 512 * 1024 if part["name"].endswith("E") else 1024 * 1024)
            self.assertEqual([(m["name"], m["address_ns"], m["size"]) for m in part["ram"]], [
                ("SRAM1", 0x20000000, 128 * 1024), ("SRAM2", 0x20020000, 80 * 1024),
                ("SRAM3", 0x20034000, 96 * 1024), ("BKPSRAM", 0x40036400, 2 * 1024)])

    def test_real_irq_numbers_and_security_difference(self):
        families = self.catalog()["families"]
        for name in ("STM32H543", "STM32H553"):
            irqs = families[name]["interrupts"]
            self.assertEqual(irqs["ADC3"], 149)
            self.assertEqual(irqs["PLAY1"], 147)
            self.assertEqual(irqs["I3C2_EV"], 131)
            self.assertEqual(irqs["PKA"], 118)
        self.assertNotIn("AES", families["STM32H543"]["interrupts"])
        self.assertEqual(families["STM32H553"]["interrupts"]["AES"], 116)
        self.assertEqual(families["STM32H553"]["interrupts"]["SAES"], 36)
        self.assertEqual(families["STM32H553"]["interrupts"]["OTFDEC1"], 115)

    def test_package_af_is_bonded_and_not_copied_from_h563(self):
        for chip in self.catalog()["chips"]:
            for package in chip["packages"]:
                pads = {p["gpio"] for p in package["pins"] if p.get("gpio")}
                self.assertTrue(package["alternate_functions"])
                for af in package["alternate_functions"]:
                    self.assertIn(af["pin"], pads)
                    self.assertIn(af["af"], range(16))
        ug = next(p for p in self.catalog()["chips"] if p["name"] == "STM32H543UG")
        self.assertEqual([p["package"] for p in ug["packages"]], ["WLCSP63"])

    def test_source_inputs_have_hashes_and_explicit_urls(self):
        source = HERE / "sources.json"
        self.assertTrue(source.exists())
        for name, item in json.loads(source.read_text(encoding="utf-8"))["files"].items():
            self.assertTrue(item["url"].startswith("https://"))
            self.assertEqual(hashlib.sha256((HERE / "sources" / name).read_bytes()).hexdigest(), item["sha256"])

    def test_new_rcc_and_flash_are_not_marked_reusable(self):
        for name in ("STM32H543", "STM32H553"):
            compatibility = self.catalog()["families"][name]["vs_h563"]
            self.assertFalse(compatibility["RCC"]["identical"])
            self.assertFalse(compatibility["FLASH"]["identical"])
            self.assertTrue(compatibility["GPIO"]["identical"])

    def test_all_exposed_pointer_types_have_measured_layouts(self):
        for family, evidence in self.catalog()["families"].items():
            registers = json.loads((HERE / "register-inputs" / (family + ".json")).read_text())
            self.assertFalse({p["ctype"] for p in evidence["pointers"].values()} - registers["types"].keys())

    def test_integer_facts_exclude_host_preprocessor_builtins(self):
        for family in ("STM32H543", "STM32H553"):
            registers = json.loads((HERE / "register-inputs" / (family + ".json")).read_text())
            header = (HERE / "sources" / (family.lower() + "xx.h")).read_text()
            declared = set(re.findall(r"^\s*#\s*define\s+(\w+)", header, re.M))
            self.assertLessEqual(set(registers["numeric_macros"]), declared)
            self.assertFalse({"WIN32", "WIN64", "_WIN32", "_WIN64", "linux", "unix"} & registers["numeric_macros"].keys())
            self.assertEqual(registers["numeric_oracle"]["verified"], len(registers["numeric_macros"]))

    def test_dma_exact_requests_and_upstream_validator_conflict(self):
        for evidence in self.catalog()["families"].values():
            requests = evidence["dma_requests"]
            for dma in ("GPDMA1", "GPDMA2"):
                self.assertEqual(requests[dma + "_REQUEST_ADC3"], 142)
                self.assertEqual(requests[dma + "_REQUEST_I3C2_RS"], 139)
                self.assertNotIn(dma + "_REQUEST_SPI5_RX", requests)
            self.assertFalse(evidence["dma_adc3_passes_st_hal_validator"])

    def test_removed_pll3_and_new_clock_mux_evidence(self):
        for evidence in self.catalog()["families"].values():
            clocks = evidence["rcc_inputs"]
            self.assertIn("RCC_I3C2CLKSOURCE_PCLK3", clocks["source_constants"])
            self.assertFalse(any("PLL3" in key for key in clocks["source_constants"]))
            self.assertIn("RCC_CCIPR3_PLAY1SEL", clocks["fields"])
            self.assertNotIn("RCC_PLL3CFGR_PLL3SRC", clocks["fields"])

    def test_integration_scope_and_rng_v4_mismatch_are_supported(self):
        decisions = json.loads((HERE / "integration-decisions.json").read_text())
        self.assertFalse(decisions["generated_chip_json"])
        self.assertEqual({part for parts in decisions["headers"].values() for part in parts}, PARTS)
        for family in ("STM32H543", "STM32H553"):
            registers = json.loads((HERE / "register-inputs" / (family + ".json")).read_text())
            oscillators = [f for f in registers["bitfields"] if f.startswith("RNG_NSCR_EN_OSC")]
            self.assertEqual(len(oscillators), 6)
            rcc = registers["types"]["RCC_TypeDef"]
            self.assertFalse(any(m["name"].startswith("PLL3") for m in rcc["members"]))
        generator = json.loads((HERE / "generator-inputs.json").read_text())
        self.assertEqual(generator["facts"]["rng_v4_NSCR_EN_OSC_array_length"], 3)
        for family, evidence in self.catalog()["families"].items():
            self.assertEqual("CCB_NS" in evidence["pointers"], family == "STM32H553")


class Constants(unittest.TestCase):
    def parser(self):
        path = HERE / "prepare.py"
        self.assertTrue(path.exists(), "provide a safe CMSIS constant parser")
        spec = importlib.util.spec_from_file_location("prepare47c", path)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        return module.Constants

    def test_typedef_parser_does_not_cross_unrelated_structs(self):
        self.parser()
        spec = importlib.util.spec_from_file_location("prepare47c", HERE / "prepare.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        parsed = module.register_typedefs('''
          typedef struct { uint32_t Size; } Config_t;
          typedef struct { __IO uint32_t CR; uint32_t RESERVED[2]; } RCC_TypeDef;
          typedef struct { Callback Function; } Runtime_TypeDef;
        ''')
        self.assertEqual(list(parsed), ["RCC_TypeDef"])
        self.assertEqual([f["name"] for f in parsed["RCC_TypeDef"]], ["CR", "RESERVED"])

    def test_c_integer_suffix_alias_shift_and_cast(self):
        constants = self.parser()({"BASE": "(0x40000000UL)", "REG": "(BASE + 0x34000UL)",
                                   "POS": "(7U)", "MASK": "(0x1FUL << POS)", "LITERAL": "((uint32_t)0xF80U)"})
        self.assertEqual(constants.value("REG"), 0x40034000)
        self.assertEqual(constants.value("MASK"), 0xF80)
        self.assertEqual(constants.value("LITERAL"), 0xF80)

    def test_width_sensitive_expressions_are_rejected(self):
        for value in ("((uint32_t)0x100000000ULL)", "(~0U)", "(0xFFFFFFFFU + 1U)",
                      "((uint32_t)(0xFFFFFFFFULL + 1))", "(-1)", "((uint8_t)256)"):
            with self.subTest(value=value), self.assertRaises(ValueError):
                self.parser()({"X": value}).value("X")

    def test_unknown_function_and_cycle_fail_closed(self):
        constants = self.parser()({"CALL": "__import__('os')", "CYCLE": "CYCLE", "UNKNOWN": "missing + 1"})
        for name in ("CALL", "CYCLE", "UNKNOWN"):
            with self.subTest(name=name), self.assertRaises(ValueError):
                constants.value(name)


class Integrity(unittest.TestCase):
    def module(self, name):
        spec = importlib.util.spec_from_file_location(name + "47c", HERE / (name + ".py"))
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        return module

    def test_unlocked_input_is_rejected(self):
        with tempfile.TemporaryDirectory(prefix="h5-source-inventory-") as directory:
            folder = Path(directory)
            (folder / "unlocked.xml").write_text("unlocked")
            with self.assertRaisesRegex(ValueError, "inventory"):
                self.module("prepare").verify_source_inventory(folder, {"files": {}})

    def test_stale_output_is_rejected(self):
        with tempfile.TemporaryDirectory(prefix="h5-output-inventory-") as directory:
            folder = Path(directory)
            (folder / "chip-inputs").mkdir()
            (folder / "chip-inputs/EXTRA.json").write_text("{}")
            with self.assertRaisesRegex(ValueError, "Unexpected"):
                self.module("prepare").expected_output_paths(folder)

    def test_explicit_workspace_precedes_ancestor_discovery(self):
        explicit = HERE.parents[1]
        self.assertEqual(self.module("capture_sources").resolve_workspace(Path("C:/unrelated/review"), explicit), explicit)


if __name__ == "__main__":
    unittest.main()
