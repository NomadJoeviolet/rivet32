# H543/H553 CRS and EXTI source audit

Date: 2026-09-30. Read-only audit; all generated evidence is under this directory.
No PAC, generator, or vendor source was changed. No ARM build or hardware test
is claimed by this audit.

## Decision

**CRS passes the same exact-source criterion as the six previously reused IPs.**
H543 and H553 have exactly the same `CRS_TypeDef` and all 86 resolved `CRS_`
integer macros as H563. Every individual register/field offset and field width
also matches the locked `crs_v1` IR. Its `Syncsrc::{Gpio,Lse,Usb}` values
0/1/2 are independently confirmed by compiling the fixed ST HAL macros with
each exact chip header. Reusing that register IR is supported; peripheral
interrupts, pins, and other chip-level metadata remain separate inputs.

**EXTI fails that criterion and needs an exact H543/H553 definition/derivation.**
The CMSIS layouts differ, a third bank is present, and multiple existing-bank
line masks differ. The locked `exti_h5` IR cannot express the third bank and
already exposes some fields more broadly than either exact CMSIS header.
Its working GPIO0–15 subset is not evidence of complete EXTI equivalence.

## Reproducible method and source identity

Run from repository root:

```powershell
python target/h543-essential-ip-audit/audit.py
```

The script imports the existing read-only parsers from
`data/patches/stm32h5-47c/prepare.py`, redirects its build directory here,
and verifies each source against `sources.json`. It compiles actual CMSIS
`sizeof`/`offsetof` probes, including reserved members, with GCC 15.2.0.
It compiles and executes all accepted integer expressions to compare the
parser's values against C evaluation (H543: 18,605; H553: 19,002; H563:
17,947; zero mismatches). This is source-layout evidence from a host compiler,
not MMIO execution. All examined struct members have fixed-width primitive
types; no native pointer size is used to infer hardware register layout.

For each IP it then applies the existing `compare()` check to all type and
prefix-integer definitions; unsupported object macros are retained in the
machine-readable report rather than silently counted as integers. The CRS
HAL enum proof uses exact conditional preprocessing and a separate C oracle
(three constants per chip, zero mismatches). The script checks each named CRS
IR field and asserts H543/H553 IP evidence equality.

| Source | Fixed revision / SHA256 |
| --- | --- |
| [ST CMSIS H5](https://github.com/STMicroelectronics/cmsis-device-h5/tree/884b8dc78e41cbfca008363342b17f4a9e8641f7) | `884b8dc78e41cbfca008363342b17f4a9e8641f7` |
| `stm32h543xx.h` | `cf3492b9f8c38102e38ecbd0f7101029ae617c7041089cd49331550d71daad51` |
| `stm32h553xx.h` | `b0d412088d7bf0e3728e0fe0349910ca2c0aef1f42bd5a0bf422a77e3d166433` |
| `stm32h563xx.h` | `563aeb7a90e100821f740efedc564ff0a5d992469c43b8ed3f2b612b8878626b` |
| [ST H5 HAL](https://github.com/STMicroelectronics/stm32h5xx-hal-driver/tree/ba20038d938ecc31399e49d60fa6bcc8e82db5da) | `ba20038d938ecc31399e49d60fa6bcc8e82db5da` |
| [metapac data](https://github.com/embassy-rs/stm32-data-generated/tree/e463add8cc54375f61c6f5f83d6b589e7fc68be2) | `e463add8cc54375f61c6f5f83d6b589e7fc68be2` |
| `data/registers/crs_v1.json` | `aa23ad6c0be1d83babbb03fc6cde3ff5093a072a78fef5b2db1116068ae33465` |
| `data/registers/exti_h5.json` | `7c7ef915f0b1b6955ce5a58625eb4c2afa31229715bda8ed3b1d16b81e18b8c7` |

IR copies in this folder were read with `git show <commit>:<path>`, preserving
canonical Git bytes rather than hashing a Windows line-ending conversion.
`provenance.json` contains source URLs, byte sizes, hashes, and compiler
version. `comparison.json` contains the full exact difference sets;
`STM32H543.json`, `STM32H553.json`, and `STM32H563.json` contain layouts,
all prefix integer macros, interrupts, addresses, masks and IR comparisons.

## CRS evidence

All three exact types are 16 bytes: `CR` at 0, `CFGR` at 4, `ISR` at 8,
`ICR` at 12. H543 declaration: header lines 342–348; H553 ends at 351;
H563 ends at 364. Fields occupy the following masks, exactly matching IR:

| Register | CMSIS and IR field mask |
| --- | --- |
| CR | `0x00003FEF` |
| CFGR | `0xB7FFFFFF` |
| ISR | `0xFFFF870F` |
| ICR | `0x0000000F` |

This is an individual field-name/position/width comparison as well as a
mask comparison. There are no missing registers or differing named fields.
CMSIS declares all four members `__IO`; the fixed IR gives `ISR` read-only
access. Therefore C qualifiers alone do not prove bus read/write side effects.
The existing IR access policy is retained as pinned upstream information;
this audit does not infer a new write permission from `__IO`.

`CFGR.SYNCSRC` starts at bit 28 and is two bits wide. ST HAL
`stm32h5xx_hal_rcc_ex.h:1583–1592` defines GPIO=0, LSE=`SYNCSRC_0`,
and USB SOF=`SYNCSRC_1` under the actual `USB_DRD_FS` conditional selected
by all three exact headers. C oracle field values after shifting by 28 are
0/1/2. This directly supports the fixed IR's `Syncsrc::Usb` variant used by
`vendor/embassy-stm32/src/rcc/hsi48.rs:3,58–69`. Merely adding a raw integer
setter would lose that required enum API.

Per-instance data independently observed in all three exact CMSIS headers:

- CRS NS base `0x40006000`, secure alias `0x50006000`.
- `CRS_IRQn = 75` (H543 line 141; H553 line 142).
- The selected H563 baseline chip has `crs/v1/CRS`, PCLK1, APB1LENR.CRSEN,
  APB1LRSTR.CRSRST and PB3/SYNC AF10, but its CRS peripheral record lacks
  an interrupt mapping. This does not remove the exact CMSIS IRQ75.
- This audit proves the register definition and enum values. It does not
  authorize copying H563 package pins, RCC choices or a USB driver's full
  capabilities; those need the exact chip metadata already being derived.

## EXTI layout and macro differences

H543 and H553 match each other for this entire audit. Against H563:

| Property | H543 / H553 | H563 |
| --- | --- | --- |
| `sizeof(EXTI_TypeDef)` | `0xA8` | `0x98` |
| Resolved `EXTI_` integer macros | 1,134 | 1,138 |
| Added / removed / changed integer names | 152 / 156 / 6 | comparison baseline |
| Trigger/pending/security/privilege banks | 3 | 2 |
| Interrupt/event mask banks | 3 | 2 |

The common actual register offsets are unchanged. H543's declaration at
lines 687–724 adds these registers (H553 has the same layout):

| New register | Byte offset |
| --- | --- |
| RTSR3 / FTSR3 / SWIER3 | `0x40` / `0x44` / `0x48` |
| RPR3 / FPR3 | `0x4C` / `0x50` |
| SECCFGR3 / PRIVCFGR3 | `0x54` / `0x58` |
| IMR3 / EMR3 | `0xA0` / `0xA4` |

`exti_h5` has length-two arrays for these register families. Its base offsets
and strides match banks 1 and 2 but there is no bank3 accessor. For the
existing registers, the following are unions of *individual numbered CMSIS
fields*, not a claim derived from a generic IR line array:

| Register group | H543/H553 named-field mask | H563 named-field mask |
| --- | --- | --- |
| RTSR1/FTSR1/SWIER1/RPR1/FPR1 | `0x0001FFFF` | `0x0001FFFF` |
| RTSR2/FTSR2/SWIER2/RPR2/FPR2 | `0x0C264000` | `0x00244000` |
| SECCFGR1/PRIVCFGR1/IMR1 | `0xFFFFFFFF` | `0xFFFFFFFF` |
| SECCFGR2/PRIVCFGR2 | `0x2FFFFFFF` | `0x03FFFFFF` |
| IMR2 | `0x2C2FCFE1` | `0x03FFFFFF` |
| EMR1 | `0x0001FFFF` | `0xFFFFFFFF` |
| EMR2 | `0x0C264000` | `0x03FFFFFF` |
| Each EXTICR1–4 | `0x0F0F0F0F` | `0x0F0F0F0F` |
| LOCKR | `0x00000001` | `0x00000001` |

H543/H553 bank2 trigger/pending/event fields name lines 46, 49, 50, 53, 58,
59. Bank3 trigger/pending/event fields name lines 64 and 66 (mask 5);
bank3 security/privilege/interrupt fields name 64–66 (mask 7). These are
register field names, not NVIC interrupt numbers and not an assignment of
every internal line to a particular peripheral.

The six changed integer macros are three masks and their aggregate aliases:

| Names (each mask plus alias) | H543/H553 | H563 |
| --- | --- | --- |
| EXTI_EMR1_EM_Msk / EXTI_EMR1_EM | `0x0001FFFF` | `0xFFFFFFFF` |
| EXTI_EMR2_EM_Msk / EXTI_EMR2_EM | `0x0C264000` | `0x03FFFFFF` |
| EXTI_IMR2_IM_Msk / EXTI_IMR2_IM | `0x2FFFFFFF` | `0x03FFFFFF` |

The exact added/removed names and values are in `comparison.json`. In
particular, the broad H543/H553 aggregate `IMR2.IM` mask (`0x2FFFFFFF`)
is larger than its individual named bits (`0x2C2FCFE1`). A derivation should
preserve that source distinction rather than silently equating the broad
mask to independently documented per-line capability.

Two inherited/general source limitations require explicit handling:

1. CMSIS calls the struct members `SECCFGRn`/`PRIVCFGRn` but prefixes their
   integer macros `SECENRn`/`PRIVENRn`, in all three headers. The audit
   normalizes those register names explicitly before comparing masks.
2. Exact H543/H553 headers contain the unresolved alias
   `EXTI_SWIER3_SWIER64 -> EXTI_SWIER2_SWIER64_Msk`, whose target is absent
   (H543 line 9129; H553 line 9586). The correct `SWIER3_SWIER64_Pos=0`
   and `SWIER3_SWIER64_Msk=1` exist directly above it. The invalid alias
   is recorded under `unsupported_object_macros`, never counted as a
   verified integer. Source pointer macros are also retained there.

## IR accessors versus actual line and interrupt capabilities

The locked IR has a generic `LINES.LINE[32]` field per bank, and analogous
32-wide security/privilege fields. It exposes bits absent from exact CMSIS
named masks. This is already true for H563, so generic accessor availability
is not proof of line availability on either part. Moreover each IR EXTICR
field is 8 bits wide, stride 8, whereas all three CMSIS headers specify
4 bits at the same stride (e.g. H543 line 9180, H563 line 8542). The high
nibble in every EXTICR byte is not covered by the exact field masks.

Both H543/H553 have EXTI NS base `0x44002000`, secure alias `0x54002000`.
`EXTI0_IRQn` through `EXTI15_IRQn` are individual vectors 11 through 26,
matching H563 (H543 lines 80–95). IRQ metadata can preserve these exact
GPIO mappings. This does **not** make the internal line banks equivalent:
line64 is not NVIC IRQ64, and source routing, TrustZone attribution and
pin presence must be taken from exact chip/package information.

The fixed Embassy GPIO EXTI path in `src/exti/low_level.rs` uses EXTICR for
pins 0–15 and trigger/mask/pending bank0. Those field locations agree, which
explains why a restricted GPIO use can work with the older shape. Supporting
that subset does not justify publishing a complete H563 EXTI alias for
H543/H553. A native exact block or carefully derived new IR needs the third
bank and the masks above; any generic compatibility accessors should state
their permitted line/pin domain.
