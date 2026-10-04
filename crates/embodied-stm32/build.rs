use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    for capability in [
        "stm32_has_usart",
        "stm32_has_usb",
        "stm32_has_timer",
        "stm32_can_bxcan",
        "stm32_can_fdcan",
    ] {
        println!("cargo:rustc-check-cfg=cfg({capability})");
    }
    let chips: Vec<_> = env::vars()
        .filter(|(key, _)| key.starts_with("CARGO_FEATURE_STM32"))
        .map(|(key, _)| key)
        .collect();
    if env::var_os("CARGO_FEATURE_HAL").is_some() {
        assert_eq!(chips.len(), 1, "select exactly one STM32 chip/core feature");
        assert_eq!(
            env::var("CARGO_CFG_TARGET_OS").as_deref(),
            Ok("none"),
            "STM32 firmware requires an explicit thumb*-none-* target"
        );
    }
    #[cfg(feature = "hal")]
    capabilities();
}

#[cfg(feature = "hal")]
fn capabilities() {
    // Use the exact metadata selected by Embassy's chip feature, including any
    // workspace patch. Never infer peripheral presence from a family name.
    let mut enabled = std::collections::BTreeSet::new();
    for peripheral in stm32_metapac::metadata::METADATA.peripherals {
        let Some(registers) = peripheral.registers.as_ref() else {
            continue;
        };
        match registers.kind {
            "usart" => {
                enabled.insert("stm32_has_usart");
            }
            "usb" | "otg" => {
                enabled.insert("stm32_has_usb");
            }
            "timer" => {
                enabled.insert("stm32_has_timer");
            }
            "can" if registers.version == "bxcan" => {
                enabled.insert("stm32_can_bxcan");
            }
            "can" if registers.version.starts_with("fdcan_") => {
                enabled.insert("stm32_can_fdcan");
            }
            _ => {}
        }
    }
    for capability in enabled {
        println!("cargo:rustc-cfg={capability}");
    }
}
