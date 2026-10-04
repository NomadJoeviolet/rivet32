# H563LI/H573LI Implementation Plan

> **For agentic workers:** Use superpowers:executing-plans; track the checks below. Root explicitly authorized direct implementation in the shared checkout, no commit, no extra approval.

**Goal:** Produce real H563LI/H573LI PAC data and a reproducible vendor overlay, preserving the fixed PAC baseline.

**Architecture:** Validate exact ST LI/ZI IP identity and fixed inputs; rebuild LI package pins and peripheral AF from ST LI/GPIO XML. Run the unmodified baseline stm32-metapac generator against the two new JSON records; overlay its output on the complete original PAC without changing other chips.

**Tech Stack:** Python stdlib, fixed stm32-data Rust generator, existing Rust toolchain.

**Spec:** Root task and `docs/stm32-gaps.md` sections 6/8.

## Global Constraints

- Do not change root Cargo, data/generated, App or embodied-stm32.
- No ZI feature alias or copied ZI pin list. Same-die register/IRQ/RCC/memory reuse requires machine-readable checks.
- Keep ST and upstream licenses; preserve every existing PAC feature.
- Root owns final HAL/link integration; report PAC compile evidence separately from HIL.

## Review Focus

- New PH2–15/PI0–11 pins must expose real AF, not just package labels.
- Changing source die/IP version or source bytes must fail, not silently reuse registers.
- SMPS-only Q package must appear in machine-readable constraints.
- Generated shared metadata filenames must not overwrite existing chips.
- Materialization must be deterministic and preserve all original PAC sources/features.

## Tasks

- [x] Add output tests and run them against absent LI artifacts to observe expected failure.
- [x] Add pinned ST inputs/license/provenance and rebuild exact package/core/peripheral pins with AF.
- [x] Validate unchanged die-level records and explicit LI vs ZI pin/AF differences; record supply requirements.
- [x] Run baseline upstream PAC generator; overlay standalone new chip/metadata files and two features onto complete baseline vendor.
- [ ] Run tests, two standalone PAC target checks, regenerate and compare artifacts; deliver exact root integration instructions.

## Execution notes

Baseline generated revision: e463add8cc54375f61c6f5f83d6b589e7fc68be2; generator source: caa36afd62510b0e6315ee0dccd1f9c65fbcac83. Registers are shared through existing versioned files, not a ZI chip alias.
