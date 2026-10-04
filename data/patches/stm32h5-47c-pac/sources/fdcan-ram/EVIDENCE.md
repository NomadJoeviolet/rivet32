# H543/H553 FDCAN message RAM evidence

Read-only investigation, 2026-09-30. No vendor, generator or locked-source directory was changed. The ST HAL files were absent locally and downloaded from the already selected HAL commit into this target directory.

**Result:** both exact device headers and the fixed ST H5 HAL support synthesizing non-secure `FDCANRAM1 = 0x4000ac00` and `FDCANRAM2 = 0x4000af50`. Each controller's declared message RAM allocation is **848 bytes (0x350)**. The two half-open ranges are `[0x4000ac00, 0x4000af50)` and `[0x4000af50, 0x4000b2a0)`. This conclusion does not borrow H563 addresses or assert a hardware test.

## Fixed primary sources

HAL commit: `ba20038d938ecc31399e49d60fa6bcc8e82db5da`.

- [stm32h5xx_hal_fdcan.c](https://github.com/STMicroelectronics/stm32h5xx-hal-driver/blob/ba20038d938ecc31399e49d60fa6bcc8e82db5da/Src/stm32h5xx_hal_fdcan.c), 122981 bytes, SHA-256 `a430429a327511cf96203dcebd45a1ab8491d1f09ecb19027333b718c3f35b53`.
- [stm32h5xx_hal_fdcan.h](https://github.com/STMicroelectronics/stm32h5xx-hal-driver/blob/ba20038d938ecc31399e49d60fa6bcc8e82db5da/Inc/stm32h5xx_hal_fdcan.h), 79977 bytes, SHA-256 `c1a5f502a56f11dbc3f263c018f40a4b8d3b3208166b3c93aaacacc5d8ed9a94`.

Exact CMSIS commit, already locked in `data/patches/stm32h5-47c/sources.json`: `884b8dc78e41cbfca008363342b17f4a9e8641f7`.

- [stm32h543xx.h](https://github.com/STMicroelectronics/cmsis-device-h5/blob/884b8dc78e41cbfca008363342b17f4a9e8641f7/Include/stm32h543xx.h), SHA-256 `cf3492b9f8c38102e38ecbd0f7101029ae617c7041089cd49331550d71daad51`.
- [stm32h553xx.h](https://github.com/STMicroelectronics/cmsis-device-h5/blob/884b8dc78e41cbfca008363342b17f4a9e8641f7/Include/stm32h553xx.h), SHA-256 `b0d412088d7bf0e3728e0fe0349910ca2c0aef1f42bd5a0bf422a77e3d166433`.

## Layout derivation and applicability

The HAL C source has outer guards `defined(FDCAN1)` at line 170 and `HAL_FDCAN_MODULE_ENABLED` at line 181. Its layout definitions at lines 211–235 have no H543/H553 exclusion or family-specific alternative:

| Region | Count | Element bytes | Offset | Region bytes |
|---|---:|---:|---:|---:|
| Standard filters | 28 | 4 | 0x000 | 112 |
| Extended filters | 8 | 8 | 0x070 | 64 |
| RX FIFO0 | 3 | 72 | 0x0b0 | 216 |
| RX FIFO1 | 3 | 72 | 0x188 | 216 |
| TX event FIFO | 3 | 8 | 0x260 | 24 |
| TX FIFO/queue | 3 | 72 | 0x278 | 216 |
| Total (`SRAMCAN_SIZE`) | | | | 848 = 0x350 |

`FDCAN_CalcultateRamBlockAddresses` at lines 3425–3441 starts from `SRAMCAN_BASE`. Under `#if defined(FDCAN2)`, it adds `SRAMCAN_SIZE` exactly when the instance equals `FDCAN2`. A separate `#if defined(FDCAN3)` branch adds twice the size for a third controller. Both exact H543/H553 headers define FDCAN1 and FDCAN2, and neither defines FDCAN3. Lines 3444–3465 assign each region's start from this selected base; lines 3468–3471 clear exactly one `SRAMCAN_SIZE` allocation. Filter-count settings do not compact these fixed regions.

For H543, CMSIS lines 1796/1812 establish APB1 non-secure base `0x40000000`; lines 1849–1852 define both controller register bases and `SRAMCAN_BASE_NS = APB1PERIPH_BASE_NS + 0xac00`. Lines 3214–3222 select the non-secure aliases. For H553, corresponding lines are 1871/1887, 1924–1927 and 3369–3377. Thus the RAM derivation uses each family's own exact header.

The secure alias branches are selected by `defined(__ARM_FEATURE_CMSE) && (__ARM_FEATURE_CMSE == 3U)` (H543 line 2600, H553 line 2731). They use `PERIPH_BASE_S = 0x50000000`, yielding RAM bases `0x5000ac00` and `0x5000af50`. These are address aliases, not two additional controller allocations. PAC generation using non-secure peripheral addresses should consistently use the non-secure RAM bases. This source review does not configure GTZC/TrustZone permissions or claim access is allowed from every security state.

## Native arithmetic/selection check

`python target/h543-fdcan-ram/verify_layout.py` verified the HAL SHA and each exact CMSIS SHA against its existing lock. It extracted the original SRAMCAN macros and original instance-selection function prefix, together with the exact CMSIS base/pointer aliases. GCC compiled and ran all four combinations (H543/H553 × non-secure/secure) with `-std=c11 -Wall -Wextra -Werror`; all expected addresses, offsets and the 848-byte total matched.

This is not a compilation of the whole HAL or a peripheral simulation. The minimal handle type supports the pointer comparison; execution stops before the first MMIO access or RAM-clear loop. `verified-layout.json` records commands, hashes and numeric results; `source-lock.json` records the downloaded official URLs, revisions and hashes. UID and calibration constants were outside this review.
