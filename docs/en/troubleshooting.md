# 8. Troubleshooting and FAQ

<!-- handbook-nav -->
[Handbook index](README.md) · [简体中文](../zh-CN/troubleshooting.md) · [Project README](../../README.en.md)

First find the failing step: tool installation, compilation on the development computer, ARM compilation and linking, programming, or operation on the board. Keep the first real error, full command, working directory and tool versions. Later errors may follow from the same problem.

## How to record the tools in use

Run these commands at the repository root in an activated terminal:

```text
rustc -Vv
cargo -V
rustup show active-toolchain
rustup target list --installed --toolchain 1.98.1
python --version
git status --short
```

For hardware issues, also record the full part number, board revision, Flash bank and TrustZone settings, probe model and the version of the debug tool in use. Run `probe-rs --version` for probe-rs or `openocd --version` for OpenOCD. For Ozone, record its version and the J-Link software version. Devices in the same STM32 family can have different packages and memory sizes, so "STM32 does not compile" is not enough to reproduce the problem. Activate a locally installed toolchain in the current terminal. VS Code and a separately opened terminal may use different environments.

## Tools are missing or a command has a syntax error

| Symptom | Common cause | Action |
|---|---|---|
| cargo/rustc not found | PATH is stale or the local environment is inactive | Reopen the terminal; activate local Rust as documented |
| Compiler is not 1.98.1 | Wrong directory or another rustup environment | Check active-toolchain, CARGO_HOME/RUSTUP_HOME and the current directory |
| `syntax error near unexpected token '&'` | PowerShell syntax entered in Git Bash | Use the commands for that shell, or run them in PowerShell |
| env.ps1 reports missing tools | The source archive omits the developer's `.tools` | Install the tools as documented to get paths for your own machine |
| Missing `link.exe` / `gcc` | The computer lacks the selected host linker | Install MSVC Build Tools/SDK, or matching MSYS2/GCC for GNU |
| Python lacks tomllib | The Python actually running is older than 3.11 | Use 3.11+ and check which interpreter the terminal finds |
| VS Code still checks old features | Its process has an old environment, or the two background check commands differ | Launch from the activated terminal and synchronize target/features/overrideCommand |

`&` is PowerShell's invocation operator. Dot-sourcing a script with `.` runs it in the current scope; calling it with `&` creates a child scope for the script. For toolchain activation, use the complete command block for your shell in the [toolchain chapter](toolchain.md).

## Code does not compile or firmware does not link

Compilation checks types and generates machine code. Linking places the compiled result at the chip's Flash/RAM addresses. In an error report, target means the compilation target and feature means a Cargo compile-time switch.

| Symptom | What to check |
|---|---|
| `can't find crate for core` | Is the selected ARM target installed for the pinned toolchain? Here, core means Rust's core library |
| Duplicate symbols / multiple MCU features | Remove `--all-features`; check whether dependencies combine mutually exclusive chip/core/time-driver features. Duplicate configuration can introduce symbols with the same name |
| Missing `memory.x` | Build an App binary with exactly one chip and the right target. memory.x supplies the chip memory layout |
| Flash bank errors | For chips with configurable banks, choose a supported single/dual-bank setting matching option bytes |
| Pin trait mismatch | Check the exact package, peripheral instance and AF (alternate-function) mapping. Correct the configuration instead of bypassing the type check with unsafe |
| Missing CAN/USB API | Confirm that the chip has the peripheral and the corresponding feature is enabled |
| excluded from support | The current policy excludes this device; historical code does not change that status |
| Bare-metal link errors in host builds | Workspace checks running on the computer should not select ARM; root configuration should not force an MCU target globally |
| `--locked` fails | Cargo sees a dependency/lockfile mismatch. Check where the dependency change came from before reviewing a lockfile update |
| Slow first macro/chip build | Large PAC register data, initial Git dependency downloads and compile-time procedural macros take time. Check whether Cargo is still making progress |

To diagnose one device, build minimal firmware first, then the required peripheral example. A linked minimal image confirms that basic compilation and linking worked. Peripheral initialization needs the corresponding build and board checks.

## Firmware downloads but the board behaves unexpectedly

| Symptom | Investigation order |
|---|---|
| No probe in `probe-rs list` | USB connection, driver/udev rules, probe firmware and tool version |
| Exact device unavailable | Check the probe-rs chip database; use a supporting CubeProgrammer version if necessary |
| Download succeeds but RTT logs are absent | Confirm that the ELF matches the firmware, then check power, reset, boot address, early panic, clocks and debug connection |
| Ozone cannot find source or places breakpoints unexpectedly | Load the ELF matching the firmware and confirm source paths still exist. Release optimization may combine statements or remove variables |
| OpenOCD cannot find configuration files | Keep the full installation directory structure and check its scripts path; specify it with -s when needed |
| GDB cannot connect | Keep OpenOCD running and use the GDB port it reports. End other debug sessions using the probe |
| A plain RTT window shows unreadable output | This project uses defmt encoding and needs a decoder. Start with probe-rs run from the [toolchain guide](toolchain.md) to read the heartbeat |
| missed keeps growing | A periodic task is running late. Check blocking operations, log volume, computation time, resource contention and time configuration |
| Partial UART data | Check baudrate, DMA/IRQ and buffer size, then whether idle is being used as a frame boundary and whether the protocol finds that boundary again after timeout |
| Incorrect SPI/IMU data | Check CPOL/CPHA, speed and CS, then ranges, initialization and the access order on a shared bus |
| No physical CAN traffic | Check whether the example still uses internal loopback, then the transceiver, filters, bitrate, termination and error state |
| USB CDC does not enumerate | Check USB clock, PHY and supply configuration, then cable, IRQ and continuous `UsbDevice::run` |
| H7 DMA errors or stale data | Check whether DMA can reach the RAM region, the buffer address and lifetime, and whether CPU DCache contents agree with RAM |
| No PWM output | Reference examples leave channels disabled. Review board configuration before enabling them |

RTT sends logs through the debug probe and needs no UART wiring. Minimal firmware does not blink an LED; use its logs to check whether it is running. CAN sent statistics mean the driver accepted frames. Physical bus ACK and peer actions need separate observation.

## Dependency downloads stall or caches grow too large

For TLS or proxy errors, check connectivity, system time, Git/Cargo proxy settings and certificates while keeping TLS validation enabled. A package-cache lock message usually means another Cargo process is using the cache. Check whether that process is still downloading or building. Starting more jobs for the same download adds waiting. Offline mode requires all needed dependencies to be cached.

A development directory includes toolchains, Cargo Git/registry downloads and target build output in addition to sources. Sources and required provenance data are below 1 GB; check other categories separately using the [storage guide](../storage.md).

`EMBODIED_CACHE_LIMIT_MIB` controls between-build pruning of designated firmware caches. It cannot limit all target directories, concurrent jobs and archives combined. Before cleanup, confirm that no build is using the cache and keep any ELF, MAP and source records still needed. Keep application code and vendor, which are source directories. See the [storage report](../storage.md) for the categories.

## Why coverage has no local records after cleanup

Coverage summarizes the reports that corresponding build commands write under `target/`. After removing those outputs, rebuild the configuration you need to produce new records. Historical test records are not included in the repository.

## Common questions when changing the project

### Is Rust HAL part of Embassy

embedded-hal is an independent interface standard that uses traits to specify driver operations. This project uses the concrete embassy-stm32 HAL, which belongs to Embassy. The HAL controls peripherals; the executor runs tasks.

### Where do I change pins

Edit the concrete pins in the Rust constructors under `App/src/boards/`, check IRQs, DMA and clocks, and build the matching board feature. A constructor is a function that creates a driver object from those settings.

### Why are register access and low-level code present

vendor stores HAL and PAC register-access code. Local patches also add missing devices and fix confirmed defects. Normal App development can use the existing HAL drivers.

### What still needs testing after compilation succeeds

Check assembly, timing, power, bus load and interrupt behavior on the board. Type checks and linking cannot observe these conditions, so hardware conclusions need corresponding hardware records.

### Can a FreeRTOS task be changed directly to async

During migration, check whether a wait blocks the whole executor, who holds resources, what state remains after cancellation, and when the task yields. Renaming functions does not resolve those differences. See [design and implementation](design-and-implementation.md) for the behavior of these interfaces.

## What to include in an issue report

Start with reproduction steps, the expected result and the observed result. Add the first error, full command, tool versions, chip/core/target/features/bank and the areas you changed. For hardware problems, include board revision, wiring and observations you can reproduce.

Logs may be trimmed to relevant portions, but keep the source commit or origin and the lockfile state. Debug with the ELF that corresponds to the running firmware to avoid mixing logs and symbols from different versions.

---

[Previous](ci-cd.md) · [Index](README.md) · [Next](README.md)
