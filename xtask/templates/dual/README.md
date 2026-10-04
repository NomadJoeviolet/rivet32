# @NAME@

This project builds separate CM7 and CM4 firmware for @PART@. Start with the build commands below, then add each core's tasks and board setup in `App/cm7/src` and `App/cm4/src`. Both programs use the startup code in `App/shared/dual_core.rs` and the framework source copy in `Framework/`, including its licenses. You do not need the original framework checkout.

Install Rust 1.98.1 and its `@TARGET@` compilation target. Use Python 3.11+ to match the repository's development environment. Set `PYTHON` to the interpreter's full path if needed, then run:

```text
cargo fetch --locked
cargo app-build
```

To check the format of both App packages and their build tool, run `cargo fmt --check -p @NAME@-cm7 -p @NAME@-cm4 -p @NAME@-builder`. Avoid `cargo fmt --all` here: it also follows local path dependencies into the pinned PAC sources in `Framework/vendor/`.

The build compiles each package separately into `target/pair-cache/cm7` and `target/pair-cache/cm4`. It checks each ELF executable firmware file, then compares the two shared-memory layouts. Firmware images, map files describing their memory layout, linker scripts, logs and `result.json` are copied into `target/pair/`.

Use `cargo app-check` to check compilation without linking or updating the pair's build report. To build one image, use `cargo cm7-build` or `cargo cm4-build`. These commands use separate caches and do not compare the two images. Do not run `cargo build --workspace` or build both App packages in one Cargo command: the two chip features cannot be enabled together.

In VS Code, open `cm7.code-workspace` or `cm4.code-workspace` to select a core. Each workspace uses the same single-package command for build-script analysis and diagnostics. Opening the project directory directly selects CM7.

CM7 uses BANK_1 at `0x08000000`, AXISRAM and TIM5. CM4 uses BANK_2 at `0x08100000`, SRAM1 and TIM2. Capacities come from each exact PAC (the package defining the device's registers and memory), including the gap between banks on G-capacity parts. Shared HAL (hardware abstraction layer) state reserves the first 4 KiB at `0x38000000`. This area is aligned and marked NOLOAD, so programming does not write initial data there. The linker checks the memory partitions, and each image sets its own VTOR interrupt vector table address.

Startup consumes the HAL peripheral tokens, which represent permission to use each peripheral. Before adding a pin, DMA channel or peripheral, assign it to one core. Private framework queues cannot be used for IPC, or communication between cores.

Program both images without erasing the other core's bank, then perform a full system reset. Enable BCM7/BCM4, set BOOT0=0, use `0x08000000` as the CM7 boot address and `0x08100000` as the CM4 boot address, and disable bank swap. CM7 uses the internal HSI oscillator and default LDO regulator mode; confirm that LDO mode matches the board's supply wiring.

Keep D-cache (data cache) disabled unless you add and review an MPU (memory protection unit) configuration that makes shared memory non-cacheable. Holding one core, restarting a single core or using incompatible HAL configurations is unsupported. See `docs/dual-core.md` for the full startup procedure and resource requirements. In this generated project, use `cargo app-build` in place of that document's source-framework build commands.

`project.json` records both packages and the checksum of every source file when the project was generated. Editing application code will make those files differ from that record. During the two builds, the pair runner checks that project metadata and `Cargo.lock` remain unchanged, and records the checksum of each linked ELF and map file. `.gitattributes` disables automatic newline conversion so Git preserves the recorded bytes.

The generated project omits `.build`, target and Python caches. It includes data source manifests, licenses and patch records, but excludes copyrighted PDFs. To read those documents, run `python xtask/scripts/fetch_source_documents.py`. It downloads them from the recorded URLs, checks their SHA checksums and stores them in the ignored `data/sources/` cache.

GitHub Actions builds and checks the pair, runs Clippy static checks on each core and uploads `target/pair/`. Passing confirms linking and ELF checks. Use a board to verify startup, both heartbeats, power and clock behavior, and programming through a debugger.
