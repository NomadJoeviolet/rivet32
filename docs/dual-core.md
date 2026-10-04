# Independent STM32H7 CM7 and CM4 images

Read this page before building or programming an H745/H747/H755/H757 project that uses both cores. The `minimal` application builds the catalogue's `-cm7` and `-cm4` features into two separate firmware programs. Each program has its own interrupt vector table, reset handler, stack, static data, task executor and Embassy timebase. The cores run independently, an arrangement called asymmetric multiprocessing. The framework does not schedule tasks across cores (SMP), decide which core may use a peripheral, or provide a shared task queue.

## Memory and resource policy

`App/build.rs` reads the selected device's PAC, the package that defines its registers and memory. It uses the version fixed by the repository and rejects missing or overlapping memory regions. It writes `memory.x`, `dual-shared.x` and `dual-core-layout.json` to Cargo's `OUT_DIR`. Memory capacities come from that exact device's data. On a `G`-capacity device, the two physical Flash banks have an address gap; the linker keeps that gap instead of treating the banks as one contiguous megabyte.

| Resource | CM7 primary | CM4 secondary |
| --- | --- | --- |
| Program and `.data` load bytes | `BANK_1`, `0x08000000`, selected bank capacity | `BANK_2`, `0x08100000`, selected bank capacity |
| Stack, `.data`, `.bss`, `.uninit`, executor, RTT debug logs | `AXISRAM`, `0x24000000`, metadata capacity | `SRAM1`, `0x30000000`, metadata capacity |
| HAL (hardware abstraction layer) `SharedData` | Fixed address `0x38000000`, first 4 KiB of SRAM4 reserved | Same address and layout |
| Embassy `time-driver-any` | TIM5, including its interrupt (IRQ) | TIM2, including its IRQ |
| RCC/PWR system initialization | `hal::init_primary` | Reads published clocks via `hal::init_secondary` |
| Startup HSEM (hardware semaphore) | Logical HAL channel 1, hardware semaphore index 0 | Polls and acknowledges its own core's index-0 notification |
| Other pins, timers, DMA, communication peripherals | Unassigned | Unassigned |

The shared memory section is aligned to 32 bytes and marked `NOLOAD`, so programming does not write initial data there. Neither image's `.bss` clearing or `.data` copying writes it during startup. The primary core initializes the HAL fields that the secondary reads. The linker checks the Flash and RAM partitions, the fixed shared symbol address and the maximum shared size. It places the shared section after `.got`, after the Cortex-M runtime has calculated each core's private heap and stack limits. Placing it directly after `.uninit` would move those limits into SRAM4.

The minimal startup takes and drops the HAL peripheral tokens, which represent permission to use each peripheral. It does not pass duplicate tokens to application tasks. Before adding peripherals to either program, make a board-level table assigning each peripheral, DMA channel and pin to one core. Both cores can access many of the same registers, even when their Rust programs are built separately.

The current `critical-section-single-core` implementation only protects against interrupts on the local core. Framework queues and mutexes must remain in that core's private RAM; they cannot be used for IPC (communication between cores). Shared HAL clock and configuration data is read-only after startup. Reinitializing RCC, changing clocks for low-power modes and restarting one core while the other runs are unsupported.

## Boot and cache conditions

Program both images without erasing the other core's bank, then perform a complete system reset. Enable both cores (`BCM7=1`, `BCM4=1`), set `BOOT0=0`, use `0x08000000` as the CM7 boot address and `0x08100000` as the CM4 boot address, and disable Flash bank swap. The firmware does not change option bytes or release a core held by a debugger. If one core is held or stopped, the other may wait indefinitely for startup. Resetting only one core through the debugger is unsupported. Cortex-M runtime's `set-vtor` feature installs each image's own interrupt vector table before enabling interrupts.

ST describes the boot addresses, separate boot option bits, shared SRAM4 access and the application's responsibility to avoid peripheral conflicts in [AN5557](https://www.st.com/resource/en/application_note/an5557-stm32h745755-and-stm32h747757-lines-dualcore-architecture-stmicroelectronics.pdf), sections 2.2, 2.3 and 3.2. This project assigns memory within those hardware resources as shown above.

Keep CM7 D-cache (data cache) disabled in this example. Startup checks that it is off, and the code does not turn it on. Before enabling it in a board project, use the MPU (memory protection unit) to mark shared memory as non-cacheable, or implement and review a complete cache-maintenance protocol. A memory barrier alone does not flush cache lines. Both cores must use the same HAL revision, configuration features and `SharedData` ABI, meaning the same binary layout. The pair checker compares shared addresses and sizes; you must still build both images from the same source and configuration.

The primary core uses the HAL's default internal HSI oscillator and LDO regulator mode. Confirm that LDO mode matches the board's supply wiring, including on H747 discovery and evaluation boards. The example does not configure an external crystal, LED pin or board connector.

## Build and check both images

On Windows, run `. .\scripts\env.ps1` first if using the repository's local Rust 1.98.1 installation. Build each core separately. Enabling both chip features in one Cargo command is invalid.

The repository's `minimal` binary and generated dual-core projects use the startup procedure described above. Passing either exact core feature to `cargo xtask new` creates separate `App/cm7` and `App/cm4` packages. In a generated project, use `cargo app-build` to build the packages separately and compare the resulting images. Do not build both App packages in one Cargo command. The pair check requires Python; see [project generation](project-generator.md) for commands. In the source repository, use:

```powershell
cargo xtask build --chip stm32h747xi-cm7
cargo xtask build --chip stm32h747xi-cm4
python scripts/check_dual_core.py stm32h747xi
# Recheck already-built, hash-matched xtask artifacts without invoking Cargo:
python scripts/check_dual_core.py --inspect-only stm32h747xi
# Optional full catalogue pair build (52 independent images):
python scripts/check_dual_core.py --all
```

`xtask` uses the App build script's `memory.x` for dual-core features and reports an error if it is unavailable. It never falls back to the HAL's whole-chip layout. ELF checks therefore use each image's actual memory partition; ELF is the executable firmware file format.

The generated `dual-core-layout.json` contains `schema_version`, `chip`, `core`, `flash`, `ram`, `shared` (each region has integer `origin`/`length` values), and `time_driver`. Tools reading this file obtain its path from the matching `embodied-app` Cargo message's `build-script-executed.out_dir`. The file also records boot requirements and `hil: not_run`, indicating that hardware testing has not run.

The pair checker stores build files in `target/dual-core/cache` and holds an exclusive lock while compiling, copying outputs and validating ELF files. It saves each ELF and map file, Cargo logs, generated linker scripts and layout, and `result.json` under `target/dual-core/<feature>/`. Map files describe the linked memory layout. The checker verifies ARM32 executable headers; each core's vector table, reset handler and stack; every PT_LOAD segment's virtual and physical address range; the shared `NOBITS` section's placement; the time-driver IRQ; and both images' shared address and size. PT_LOAD identifies loadable segments, while NOBITS marks a section with no initial bytes stored in the ELF. A passing result is `link_and_elf_passed`; `hil` stays `not_run` until separate hardware testing.

The checker refuses to use a cache with an existing lock. After `KeyboardInterrupt` or `SystemExit`, it keeps `target/dual-core/cache/.dual-core.lock`: stopping Cargo on Windows does not guarantee that its rustc child processes have stopped. Before manually removing a stale lock, check that the recorded owner PID and all its Cargo/rustc child processes have exited. If validation finishes with an ordinary failure, the checker releases only its own lock.

`--inspect-only` works with existing outputs whose three xtask stages all passed. It compares the copied ELF and map files with the SHA-256 checksums in the report and rejects a report that changes while being copied. It writes additional ELF check results under `target/dual-core/inspected/<feature>/`, without rebuilding or changing the original outputs.


The catalogue contains 26 dual-core parts (52 core features). After these software checks pass, use a board to verify startup, simultaneous heartbeats, power, reset and cache behavior.
