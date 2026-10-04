# H563LI / H573LI exact-package PAC patch

This patch adds independent `stm32h563li` and `stm32h573li` features on the fixed `stm32-metapac` baseline. Neither feature forwards to a ZI chip. It preserves all existing baseline chip features and register modules.

## Inputs and generation

- PAC/data baseline: `embassy-rs/stm32-data-generated` commit `e463add8cc54375f61c6f5f83d6b589e7fc68be2`, tag `stm32-data-caa36afd62510b0e6315ee0dccd1f9c65fbcac83`.
- Unmodified PAC generator: `embassy-rs/stm32-data` commit `caa36afd62510b0e6315ee0dccd1f9c65fbcac83`, including its `Cargo.lock` and fixed chiptool revision.
- ST package/GPIO XML: `STMicroelectronics/STM32_open_pin_data` commit `7d1f1514ed5583ec5007ad91236b4e1d377295b1`.
- `sources.json` contains URLs and SHA-256 for every retained source XML, GPIO AF XML, upstream analog correction and license, plus the generator archive checksum. Source mismatch fails generation.

`prepare_h5_li_patch.py` rebuilds LI package positions, core GPIO names and peripheral pin/AF assignments from the **LI** XML plus `GPIO-STM32H56x_gpio_v1_0_Modes.xml`. It follows the fixed generator's signal normalization, I2S-to-SPI merge, CTS fallback and H5 OPAMP correction. The ZI pin arrays are used only as a validation reference: reconstructing both ZITx and ZITxQ source packages must reproduce every fixed-baseline ZI peripheral pin record exactly before LI generation proceeds.

This union matters. Non-Q ZI uses positions for regulator supply that Q can expose as PB11/PE1; the Q ZI XML omits ETH. The combined ZI IP instance/version set equals LI, as does the non-Q ZI IP set. Matching die (`DIE484`), core, flash and SRAM are checked. After reconstruction, removing pin/package/name/document fields leaves exactly identical die-level metadata: register versions and addresses, memory banks, RCC gates/muxes/resets, IRQs and DMA data. H573's AES/SAES/OTFDEC differences remain intact.

Each LI package has **225 ball positions and 140 distinct GPIO names**, including 26 GPIOs beyond the baseline ZI union: PH2–PH15 and PI0–PI11. Specific source-backed examples are ball D15 = PI1, SPI2_SCK/I2S_CK AF5; PH11 = I3C1_SCL AF5. These pins are present in the generated metadata and are usable by HAL trait generation. `comparison.json` records the complete source IP comparison, new GPIOs, pin/AF differences, die-level digest and supply assumptions.

The two rebuilt JSONs are passed through the original Rust `stm32-metapac-gen`. Every resulting shared peripheral and register module must match the original version after whitespace normalization; no register implementation is replaced. Only the generated LI chip directories and their uniquely named `metadata_li_*` files are added to the full vendor crate. Original Cargo features and `ALL_CHIPS` are extended with the two actual names.

## Reproduce

From the framework root, with Python 3 and the project Rust environment:

```powershell
. .\scripts\env.ps1
python xtask/scripts/prepare_h5_li_patch.py --materialize --write-manifest
python -m unittest discover -s xtask/scripts -p test_h5_li_patch.py -v
```

The script normally locates the fixed original PAC in `.tools/cargo/git/checkouts`. Pass `--pac-source <checkout>` when using another cache. That checkout's HEAD must equal the fixed revision. If starting without a cached original checkout, obtain the fixed upstream tag in a separate source-cache directory first; a local patched vendor directory is not a replacement for the original Git source. The generator archive is fetched from its fixed official codeload URL when absent, checked against its recorded SHA, extracted under `target/h5-li-generator-source`, and built with `cargo +1.98.1 ... --locked`. No generated framework catalog is modified.

For inspection without materializing the vendor, omit `--materialize`; this performs all source and pin reconstruction checks and writes only the two JSONs/comparison. `--generator-binary` is a development shortcut for an already-built fixed generator; the normal reproducible path is `--materialize`.

PAC checks, separately from framework/HAL linking:

```powershell
cargo +1.98.1 check --manifest-path vendor/stm32-metapac/Cargo.toml --target thumbv8m.main-none-eabihf --features rt,metadata,stm32h563li --target-dir target/h5-li-pac-check --offline --locked
cargo +1.98.1 check --manifest-path vendor/stm32-metapac/Cargo.toml --target thumbv8m.main-none-eabihf --features rt,metadata,stm32h573li --target-dir target/h5-li-pac-check --offline --locked
```

The complete `vendor-manifest.json` exposes `vendor_files_sha256` keyed relative to `vendor/stm32-metapac`; `baseline_chip_git_blobs` and `baseline_pac_git_blobs` retain the independent original Git blob IDs; `new_chip_jsons_sha256` keys are `STM32H563LI.json` and `STM32H573LI.json`, relative to this patch's `chips` directory. Git blob verification uses `sha1(b"blob " + decimal_length + b"\0" + raw_bytes)`. The manifest also binds `sources.json` and `comparison.json` by SHA-256. `vendor-overlay.json` lists the much smaller set of added/modified vendor files.

## Framework integration

The parent project should patch the original Git source:

```toml
[patch."https://github.com/embassy-rs/stm32-data-generated"]
stm32-metapac = { path = "vendor/stm32-metapac" }
```

Its pinned Embassy HAL needs actual forwarding features:

```toml
stm32h563li = ["stm32-metapac/stm32h563li"]
stm32h573li = ["stm32-metapac/stm32h573li"]
```

Use the exact LI JSONs for memory/catalog generation. Preserve the original 65-upstream-gap result and record two local patches independently. Root owns these Cargo/catalog/adapter changes and final HAL/minimal firmware linking; this generator does not edit them.

## SMPS and startup boundary

The `HxQ` TFBGA225 package exposes VDDSMPS (R13), VLXSMPS (R12), VSSSMPS (P12) and VCAP balls. H563LI's official ordering page identifies the package as SMPS. A package-aware board configuration should retain this constraint; no code should turn it into an H7-style LDO/SMPS software selection. [ST H563LI ordering information](https://www.st.com/en/microcontrollers-microprocessors/stm32h563li.html)

ST **AN5930 Rev2 §3, page 5** states that H5's package determines the regulator in hardware, and that supplying VDDSMPS enables the SMPS after reset. Both regulator types support VOS0–VOS3; VOS0 supports 250 MHz. Thus a correctly powered LI board starting from normal reset has a documented internal-regulator path without an explicit H7 `SupplyConfig` call. The recommended SMPS external circuit and voltage constraints remain board requirements. [ST AN5930](https://www.st.com/resource/en/application_note/an5930-guidelines-for-power-management-on-stm32h5-mcus-stmicroelectronics.pdf)

The ST H5 HAL's `HAL_PWREx_ConfigSupply` accepts only `PWR_EXTERNAL_SOURCE_SUPPLY` (bypass); it is not an LDO-versus-SMPS selector. The fixed PAC's SCCR descriptions likewise identify LDOEN/SMPSEN as hardware-selected fields. [ST HAL implementation](https://github.com/STMicroelectronics/stm32h5xx-hal-driver/blob/ba20038d938ecc31399e49d60fa6bcc8e82db5da/Src/stm32h5xx_hal_pwr_ex.c#L97), [ST HAL header](https://github.com/STMicroelectronics/stm32h5xx-hal-driver/blob/ba20038d938ecc31399e49d60fa6bcc8e82db5da/Inc/stm32h5xx_hal_pwr_ex.h#L87), [fixed PWR register description](https://github.com/embassy-rs/stm32-data/blob/caa36afd62510b0e6315ee0dccd1f9c65fbcac83/data/registers/pwr_h5.yaml#L215)

Pinned Embassy `rcc/h.rs` defines `supply_config` only for H7 power variants (line 238). Its default H5 path selects HSI/Div1 (64 MHz) and VOS Scale0 (lines 269–297); it writes VOSCR then waits for VOSRDY (lines 417–427) before clock setup. That is consistent with the documented cold-reset H5 supply model. It neither configures nor repairs SCCR BYPASS. [Pinned Embassy initialization](https://github.com/embassy-rs/embassy/blob/ae9e6f0672af84cec8e200a94c574041844396f0/embassy-stm32/src/rcc/h.rs#L417)

This supports a **cold-reset, correctly wired and powered internal-SMPS assumption**, not a hardware result. Bootloader handoff with BYPASS or altered power/clock state, externally supplied VCORE, missing/incorrect SMPS components, board rail sequencing, option bytes and low-power transitions are not validated here. Do not label a successful link as startup/HIL success, and do not write an arbitrary H7 supply configuration to H5 registers.

## Validation and licensing

The initial tests failed because LI JSON/PAC/comparison artifacts were absent. After implementation, eight tests pass: exact package/GPIO counts, new AF and I2S merging, H573 crypto distinction, pin containment, SMPS/source comparison, independent PAC artifacts, rejection of unknown die/IP and rejection of modified source bytes. Both LI PACs pass `rt,metadata` compilation for `thumbv8m.main-none-eabihf`.

A byte comparison of all 6,849 original PAC files found changes only in `Cargo.toml` and `src/all_chips.rs`; new chip and shared metadata files have separate names. The original baseline's `MIT OR Apache-2.0` declaration and every original source/README notice remain. Its snapshot did not contain standalone license files; MIT/Apache text from the pinned Embassy project and full ST BSD-3-Clause terms are retained with provenance and `NOTICE`. The source XML's 2026 ST copyright notices are preserved.

No firmware has been flashed and no electrical or startup behavior has been tested on LI hardware in this task.
