#[cfg(feature = "firmware")]
mod build_support;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(embodied_dual_core)");
    println!("cargo:rustc-check-cfg=cfg(embodied_core_cm7)");
    println!("cargo:rustc-check-cfg=cfg(embodied_core_cm4)");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=build_support.rs");
    #[cfg(feature = "firmware")]
    firmware();
}

#[cfg(feature = "firmware")]
fn firmware() {
    use build_support::{Core, DualLayout, Region, validate_time_driver};
    use std::{env, fs, path::PathBuf};
    let chips: Vec<_> = env::vars()
        .filter_map(|(key, _)| {
            key.strip_prefix("CARGO_FEATURE_STM32")
                .map(|suffix| format!("stm32{}", suffix.to_ascii_lowercase().replace('_', "-")))
        })
        .collect();
    assert_eq!(
        chips.len(),
        1,
        "firmware requires exactly one STM32 chip feature"
    );
    let chip = &chips[0];
    let Some(core) = Core::from_chip(chip).expect("valid dual-core feature") else {
        return;
    };
    let explicit_timers: Vec<_> = env::vars()
        .filter_map(|(key, _)| {
            key.strip_prefix("CARGO_FEATURE_TIME_DRIVER_")
                .map(str::to_owned)
        })
        .filter(|name| name != "ANY")
        .collect();
    validate_time_driver(
        core,
        &explicit_timers
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
    )
    .expect("distinct per-core time driver ownership");
    let metadata = &stm32_metapac::metadata::METADATA;
    assert_eq!(
        metadata.memory.len(),
        1,
        "dual-core memory must have one explicit PAC bank layout"
    );
    let regions: Vec<_> = metadata.memory[0]
        .iter()
        .map(|r| Region {
            name: r.name,
            origin: r.address,
            length: r.size,
        })
        .collect();
    let layout =
        DualLayout::from_regions(core, &regions).expect("valid disjoint dual-core memory policy");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out.join("memory.x"), layout.memory_script()).unwrap();
    let shared_path = out.join("dual-shared.x");
    fs::write(&shared_path, layout.shared_script()).unwrap();
    let (core_name, timer) = match core {
        Core::Cm7 => ("cm7", "TIM5"),
        Core::Cm4 => ("cm4", "TIM2"),
    };
    fs::write(out.join("dual-core-layout.json"), format!(
        "{{\n  \"schema_version\": 1,\n  \"chip\": \"{chip}\",\n  \"core\": \"{core_name}\",\n  \"flash\": {{\"origin\": {}, \"length\": {}}},\n  \"ram\": {{\"origin\": {}, \"length\": {}}},\n  \"shared\": {{\"origin\": {}, \"length\": {}}},\n  \"time_driver\": \"{timer}\",\n  \"boot\": \"both cores enabled; BOOT0=0; bank swap disabled; full system reset\",\n  \"hil\": \"not_run\"\n}}\n",
        layout.flash.origin, layout.flash.length, layout.ram.origin, layout.ram.length, layout.shared.origin, layout.shared.length,
    )).unwrap();
    println!("cargo:rustc-cfg=embodied_dual_core");
    println!(
        "cargo:rustc-cfg={}",
        match core {
            Core::Cm7 => "embodied_core_cm7",
            Core::Cm4 => "embodied_core_cm4",
        }
    );
    // Cargo puts this package's native search path before its dependencies'.
    // The absolute extra script asserts the chosen memory.x so a changed search
    // order fails linking instead of silently loading the HAL's whole-chip map.
    println!("cargo:rustc-link-search={}", out.display());
    println!(
        "cargo:rustc-link-arg-bin=minimal=-T{}",
        shared_path.display()
    );
    println!(
        "cargo:warning=dual-core {chip}: separate image; flash=0x{:08x}+{} private_ram=0x{:08x}+{} shared=0x{:08x}+{}",
        layout.flash.origin,
        layout.flash.length,
        layout.ram.origin,
        layout.ram.length,
        layout.shared.origin,
        layout.shared.length
    );
}
