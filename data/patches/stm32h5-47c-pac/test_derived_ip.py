import copy
from pathlib import Path
import unittest

import derived_ip
import provenance

ROOT = provenance.workspace()


class DerivedIp(unittest.TestCase):
    def test_timer_blocks_follow_exact_instance_width_channels_and_break_capabilities(self):
        for family in ("STM32H543", "STM32H553"):
            result = derived_ip.derive(ROOT, family)
            timers = {name: spec["block"] for name, spec in result["instances"].items() if name.startswith("TIM")}
            self.assertEqual(timers, dict(TIM1="TIM_ADV", TIM2="TIM_GP32", TIM3="TIM_GP16", TIM4="TIM_GP16", TIM5="TIM_GP32", TIM6="TIM_BASIC", TIM7="TIM_BASIC", TIM8="TIM_ADV", TIM12="TIM_2CH", TIM15="TIM_2CH_CMP"))
            self.assertTrue(all(result["instances"][name]["version"] == "h5_47c" for name in timers))

    def test_spi_has_exact_new_drds_field_and_only_four_instances(self):
        result = derived_ip.derive(ROOT, "STM32H543")
        fields = result["registers"]["spi"]["fieldset/CFG1"]["fields"]
        drds = [field for field in fields if field["name"] == "DRDS"]
        self.assertEqual(len(drds), 1)
        self.assertEqual((drds[0]["bit_offset"], drds[0]["bit_size"]), (24, 1))
        self.assertEqual({n for n in result["instances"] if n.startswith("SPI")}, {"SPI1", "SPI2", "SPI3", "SPI4"})

    def test_usb_pma_is_exact_2048_bytes_and_removed_bits_are_not_exposed(self):
        result = derived_ip.derive(ROOT, "STM32H553")
        self.assertEqual(result["usb_ram_address"], 0x40016400)
        self.assertEqual(result["usb_ram_bytes"], 2048)
        self.assertEqual(result["instances"]["USB"]["kind"], "usb")
        for ir in result["registers"].values():
            self.assertFalse(any(field["name"] in {"RTCPREEN", "L1XACT"} for key,value in ir.items() if key.startswith("fieldset/") for field in value["fields"]))

    def test_unreviewed_register_difference_and_ambiguous_capability_macros_are_rejected(self):
        facts = derived_ip.load_inputs(ROOT, "STM32H543")
        bad = copy.deepcopy(facts)
        bad["family_evidence"]["vs_h563"]["TIM"]["macros"]["changed"]["TIM_CR1_CEN"] = {"old": 1, "new": 2}
        with self.assertRaises(ValueError):
            derived_ip.derive_inputs(bad)
        with self.assertRaises(ValueError):
            derived_ip.members("#define IS_TIM_CC1_INSTANCE(X) ((X)==TIM1_NS) && (opaque(X))", "IS_TIM_CC1_INSTANCE", "TIM")


if __name__ == "__main__":
    unittest.main()
