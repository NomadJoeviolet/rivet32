# 1. Toolchain installation and use

<!-- handbook-nav -->
[Handbook index](README.md) · [简体中文](../zh-CN/toolchain.md) · [Project README](../../README.en.md)

Install Rust, build a small program, then use a programming tool to write it to the STM32. This chapter uses the H723VG and the project's fixed Rust 1.98.1 compiler with Edition 2024. The version is recorded in [rust-toolchain.toml](../../rust-toolchain.toml).

On a new computer, follow the Windows or Linux installation steps. If this checkout already has a toolchain in `.tools/`, use the local-toolchain section instead. After building, choose one programming and debugging route: probe-rs, Ozone or OpenOCD. STM32CubeProgrammer is another option for programming the device.

## What to select when building

| Selection | Example | What it controls |
|---|---|---|
| Host | `x86_64-pc-windows-msvc` | The computer running Rust; Windows and Linux need different installations |
| MCU target | `thumbv7em-none-eabihf` | The ARM core type to build for; both H723 and G474 use this target |
| Chip feature | `stm32h723vg` | The exact STM32 device, including its peripherals, pins and memory |

Specify both `--target` and `--features` when building firmware. Think of them as the CPU type and the chip model. Build helpers also need to run on your computer, so the workspace does not set ARM as the target for every package.

## Which tools to install

| Tool | Role in this project | When required |
|---|---|---|
| rustup | Install/select toolchains and targets | Environment setup and maintenance |
| rustc / Cargo | rustc is the compiler; use cargo commands to download dependencies and build the project | Daily development, fixed at 1.98.1 |
| LLVM / rust-lld | Generate machine code and combine it into a firmware file | Included with Rust; no separate installation |
| rustfmt / Clippy | Format code and check for common coding mistakes | Before submitting changes and in automated checks |
| Git | Obtain the repository and revision-pinned dependencies | Initial checkout and dependency changes |
| Python 3.11+ | Run the project's generation and checking scripts | When using those scripts; direct single-core Cargo builds do not need Python |
| MSVC Build Tools or host GCC | Build helper programs that run on your computer | Must match the selected host |
| VS Code + rust-analyzer | Editing, completion, navigation and diagnostics | Recommended development environment |
| probe-rs / Ozone / OpenOCD / CubeProgrammer | Write firmware through a probe such as ST-LINK or J-Link; debugging options are described below | Choose one when you are ready to use hardware |
| LLVM tools / cargo-binutils | Size inspection and BIN/HEX conversion | Analysis or release preparation |
| ARM GDB | Connect to OpenOCD, set breakpoints and inspect program state | When debugging source code through OpenOCD |

The Rust toolchain builds the firmware; Keil and IAR are not required. ARM GDB, used later in this chapter, is a debugger and does not compile Rust. Building tools such as probe-rs from source may require CMake; follow that tool's installation instructions.

## Fresh Windows setup: MSVC host recommended

1. Install [Git for Windows](https://git-scm.com/downloads/win), [Python](https://www.python.org/downloads/windows/) and [VS Code](https://code.visualstudio.com/). Choose Python 3.11 or newer and check `python --version`.
2. Install the Visual Studio C++ desktop tools and Windows SDK following [rustup's MSVC prerequisites](https://rust-lang.github.io/rustup/installation/windows-msvc.html).
3. Run the architecture-appropriate installer from the [official Rust installation page](https://rust-lang.org/tools/install/). Use the MSVC host on a typical x64 Windows machine. Existing GNU/MSYS2 users can retain their GNU host with a matching linker and runtime; see the [Windows ABI guidance](https://rust-lang.github.io/rustup/installation/windows.html).
4. Open a new PowerShell terminal. From the root of your cloned or extracted repository, run:

```powershell
rustup toolchain install 1.98.1 --profile minimal --component rustfmt --component clippy
rustup target add --toolchain 1.98.1 thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabihf thumbv8m.main-none-eabihf
rustup component add rust-src --toolchain 1.98.1
rustc -Vv
cargo -V
rustup show active-toolchain
rustup target list --installed --toolchain 1.98.1
```

Look for `rustc 1.98.1` and the four ARM targets in the installed list. Once you enter the repository, rustup selects the version named by the project, without changing your global default. The first installation and dependency download need internet access.

## An existing project-local toolchain

`.tools/` holds a local installation on this computer. Git ignores it and source archives do not include it, so another user still needs to install their environment after downloading the repository. Use these activation commands only when the toolchain is already present.

The [environment script](../../scripts/env.ps1) makes this terminal find Rust and dependencies under `.tools/` and disables automatic installation. It sets process variables such as `PATH`, `CARGO_HOME` and `RUSTUP_HOME`; it does not download tools or change system settings.

PowerShell, repository root, once per new terminal:

```powershell
. .\scripts\env.ps1
rustc -Vv
cargo -V
```

Keep the leading dot and space: they load the settings into the current PowerShell session. You should then see the Rust and Cargo versions. If the script reports that the local toolchain is missing, follow the fresh Windows installation steps.

Git Bash, repository root, after confirming `.tools/cargo/bin/cargo.exe` exists:

```bash
export RUSTUP_HOME="$(pwd -W)/.tools/rustup"
export CARGO_HOME="$(pwd -W)/.tools/cargo"
export RUSTUP_AUTO_INSTALL=0
export PATH="$PWD/.tools/cargo/bin:$PATH"
rustc -Vv
cargo -V
```

Repeat these settings in each new Git Bash terminal. If you already use a global rustup installation, run Cargo directly. PowerShell's `& .\script.ps1` syntax cannot be pasted into Git Bash. Starting a PowerShell child process from Bash does not pass the environment settings back to Bash.

## Linux: Ubuntu / Debian

These commands are for Ubuntu 24.04, Debian 12 and similar systems. Run them in Bash from any directory. Install the system tools, then install Rust through the [official rustup entry point](https://rust-lang.org/tools/install/):

```bash
sudo apt-get update
sudo apt-get install -y build-essential git curl python3 python-is-python3
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/embodied-rustup.sh
sh /tmp/embodied-rustup.sh --profile minimal --default-toolchain 1.98.1
. "$HOME/.cargo/env"
rustup component add rustfmt clippy rust-src --toolchain 1.98.1
rustup target add --toolchain 1.98.1 thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabihf thumbv8m.main-none-eabihf
rustc -Vv
python3 --version
```

`python-is-python3` makes the later `python` commands use Python 3; using `python3` throughout also works. Enter the repository root before building. With WSL, install the Linux toolchain inside WSL and arrange USB forwarding before connecting a probe.

## Configure the editor

Activate the intended toolchain, enter the repository root in the same terminal, then run:

```text
code --install-extension rust-lang.rust-analyzer
code stm32h723.code-workspace
```

Once the workspace opens, `Ctrl+Shift+B` builds the H723 minimal firmware. Use [framework.code-workspace](../../framework.code-workspace) when working on general algorithms or protocols. rust-analyzer supplies completion and diagnostics; see [VS Code Rust support](https://code.visualstudio.com/docs/languages/rust).

When changing chips, update the workspace target and feature, the build task, `cargo.buildScripts.overrideCommand` and `check.overrideCommand`. The last two settings control commands run in the background; if they keep the old chip, editor diagnostics will still use it.

An existing VS Code process may retain its original environment. With local Rust, close the relevant instances and launch from the activated terminal. For an independent instance, the supported `--user-data-dir` option can be used: run `code --user-data-dir .tools/vscode-profile stm32h723.code-workspace` at the repository root, and install the extension in that profile. See the [command-line documentation](https://code.visualstudio.com/docs/configure/command-line).

## Build the first firmware

Run from the repository root in PowerShell, Git Bash or Linux Bash, with Rust installed or activated. To check that you can produce firmware, start with the last `cargo build` command. The earlier commands check formatting and code, and look up the chip:

```text
cargo fmt --all --check
cargo check --workspace --locked
cargo clippy --workspace --lib --bins --locked -- -D warnings
cargo xtask list-chips --chip stm32h723vg
cargo build -p embodied-app --bin minimal --release --locked --target thumbv7em-none-eabihf --features stm32h723vg
```

The build should produce `target/thumbv7em-none-eabihf/release/minimal`. This is an ELF firmware file containing the program and debug information, even though its name has no extension.

`-p embodied-app` selects the App package, `--bin minimal` selects the heartbeat entry point, and `--release` uses the release build settings. `--target` selects the ARM core type; `--features stm32h723vg` selects the chip. `--locked` requires the dependency versions in `Cargo.lock`. Use `--offline` only when the computer already has every dependency cached.

| Target | Cores / families in the current scope |
|---|---|
| `thumbv6m-none-eabi` | Cortex-M0/M0+; F0, G0 |
| `thumbv7m-none-eabi` | Cortex-M3; F1, F2 |
| `thumbv7em-none-eabihf` | Cortex-M4/M7; F3, F4, F7, G4, H7 |
| `thumbv8m.main-none-eabihf` | Cortex-M33; H5 |

For another MCU, check the [chip catalogue](../../data/chips.json) and [support policy](../../data/support-policy.json). Select one chip or core per firmware and do not use `--all-features`. Some devices, such as G474RE, also require the correct single-bank or dual-bank Flash layout; see the [application guide](application-guide.md).

## Choose a programming and debugging tool

Choose a route that suits your probe; you do not need every tool. Before programming, connect SWDIO, SWCLK, GND and the reference voltage required by the probe. Connect NRST as required by the board and probe, and select the exact chip. The H723 examples below are for a single-core H723VG firmware.

| What you want to do | Tool | What else you need |
|---|---|---|
| Program the board and read this project's heartbeat logs | [probe-rs](#probe-rs-program-the-board-and-read-the-heartbeat) | A supported probe, such as ST-LINK |
| Use a graphical debugger for breakpoints, stepping and variables | [Ozone](#ozone-debug-through-a-graphical-interface) | Usually J-Link or J-Trace |
| Program from the command line and debug through GDB | [OpenOCD](#openocd-program-from-the-command-line) | A matching probe configuration; ARM GDB for source debugging |
| Use ST's graphical programming application | STM32CubeProgrammer | ST-LINK or another supported interface |

Each tool has its own supported-device list. If the exact device is missing, check the tool version and device list first. The Ozone and OpenOCD instructions below have been checked against official documentation; this project has not yet tested these routes on hardware.

### probe-rs: program the board and read the heartbeat

Download a prebuilt package for your system from the [official probe-rs installation page](https://probe.rs/docs/getting-started/installation/) to avoid compiling the tool and creating its build cache locally. The source installation below is another option. On Linux, first install `pkg-config`, `libudev-dev`, `cmake` and `git` as described on that page. Run in a terminal with Rust activated, from any directory:

```text
cargo install probe-rs-tools --locked
probe-rs --version
probe-rs list
probe-rs chip list
```

After checking the version, use `list` to confirm that the connected probe appears. `chip list` shows the device names the tool accepts. No MCU has been programmed yet. On Linux, configure udev rules using the [probe setup instructions](https://probe.rs/docs/getting-started/probe-setup/). For Windows driver requirements, follow those instructions and the probe vendor's documentation.

If you prefer a graphical programming application, install [STM32CubeProgrammer](https://www.st.com/en/development-tools/stm32cubeprog.html). Connect through ST-LINK/SWD, open the ELF just built, and download and verify it. HEX and BIN are also supported; a BIN file needs a separately specified start address.

### Start the firmware with probe-rs

Confirm that the board uses H723VG and that `probe-rs chip list` contains its device name. Run from the repository root. This writes Flash, starts the program and opens the log output:

```text
probe-rs run --chip STM32H723VGTx target/thumbv7em-none-eabihf/release/minimal
```

After startup, expect one `App heartbeat=... missed=...` line per second. RTT carries logs over the debug connection, so no UART or LED connection is needed. If nothing appears, check the probe, reset and matching ELF as described in [troubleshooting](troubleshooting.md).

Use the same ELF when switching to CubeProgrammer, Ozone or OpenOCD, and check the chip and Flash layout. The repository has no single debug configuration that covers every model. Dual-core firmware needs separate core and image selections; see the [dual-core guide](../dual-core.md).

### Ozone: debug through a graphical interface

Ozone reads Rust ELF files and can display Rust source, call stacks, variables and registers; see [SEGGER's Rust support announcement](https://www.segger.com/news/pr-240927-ozone-support-rust/). These steps use J-Link over SWD.

1. Install Ozone and the J-Link Software and Documentation Pack for your operating system from [SEGGER's downloads](https://www.segger.com/downloads/jlink/). Open Ozone and confirm that it can find your J-Link.
2. Build the firmware with the earlier Cargo command. Ozone downloads and debugs the result; rebuild with Cargo after changing Rust source.
3. Create an Ozone project, select the device matching your MCU, and choose SWD. This example uses H723VG. If it is missing from the device list, check your installed version.
4. Select `target/thumbv7em-none-eabihf/release/minimal` as the program file. It has no `.elf` extension, so change the file-dialog filter if needed. Save the debug project as a `.jdebug` file for later use.
5. Start a download and debug session. Open `App/src/bin/minimal.rs`, place a breakpoint on an executable statement and run. When execution stops, step through the code or inspect variables, memory and registers. Resume execution or end the session when finished.

See [Ozone's getting-started guide](https://www.segger.com/products/development-tools/ozone-j-link-debugger/technology/getting-started-with-ozone/) for the interface. Source debugging needs the ELF that matches the program on the board; BIN/HEX files lack the same debug information. The [root Cargo configuration](../../Cargo.toml) retains `debug = 2` in release builds and also enables optimization, so some variables may still appear as optimized out.

### OpenOCD: program from the command line

OpenOCD uses configuration files to select the probe and chip. This example uses ST-LINK and a single-core H723VG. For another chip, choose its target configuration and check the Flash layout and reset method.

On Windows, follow the Windows release links on the [OpenOCD download page](https://openocd.org/pages/getting-openocd.html), or use the listed [xPack OpenOCD distribution](https://xpack-dev-tools.github.io/openocd-xpack/docs/install/). Download an archive matching your computer, extract the whole directory, add its `bin` directory to your user `PATH`, and open a new terminal. Keep the supplied scripts and libraries alongside the executable.

On Ubuntu/Debian, run in Bash from any directory:

```bash
sudo apt-get install -y openocd gdb-multiarch
openocd --version
gdb-multiarch --version
```

This also installs GDB for the debugging steps below. On Windows, install the host-appropriate `arm-none-eabi` package from the [Arm GNU Toolchain downloads](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads) and add its `bin` directory to `PATH` if you need source debugging. That package provides ARM GDB; Cargo still builds the Rust firmware. Check the tools in PowerShell, from any directory:

```powershell
openocd --version
arm-none-eabi-gdb --version
```

Once both commands display version information, run the following from the repository root. It erases and writes the Flash covered by the ELF, verifies it, resets the MCU to run, and exits:

```text
openocd -f interface/stlink.cfg -f target/stm32h7x.cfg -c "adapter speed 1000" -c "program target/thumbv7em-none-eabihf/release/minimal verify reset exit"
```

`interface/stlink.cfg` selects the probe, `target/stm32h7x.cfg` selects the H7 configuration, and `adapter speed 1000` sets a 1 MHz debug clock. Expect messages reporting successful programming and verification; this command does not display the App heartbeat. See the [OpenOCD programming guide](https://openocd.org/doc/html/Flash-Programming.html) for the command syntax. If a `.cfg` file cannot be found, locate the installation's `scripts` directory and use `-s` to specify it when necessary. Its location varies by distribution.

### OpenOCD + GDB: breakpoints and stepping

Open two terminals at the repository root. Start OpenOCD in the first and leave it running:

```text
openocd -f interface/stlink.cfg -f target/stm32h7x.cfg -c "adapter speed 1000"
```

After OpenOCD reports its GDB listening port, open the ELF in the second terminal. On Windows:

```text
arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/minimal
```

With the Linux packages installed above, use:

```text
gdb-multiarch target/thumbv7em-none-eabihf/release/minimal
```

Enter the following at GDB's `(gdb)` prompt, not in PowerShell or Bash. Port 3333 is the default for the first target; use the port reported by OpenOCD if it differs:

```gdb
target extended-remote localhost:3333
monitor reset halt
load
monitor reset halt
info registers
continue
```

`load` writes the firmware. `monitor reset halt` resets and pauses the MCU; `info registers` displays registers, and `continue` resumes execution. To debug source, interrupt the program in GDB, use `break filename:line` on an executable line, then resume. After hitting the breakpoint, use `next`, `step` and `info locals` to step or inspect local variables. See the [OpenOCD and GDB guide](https://openocd.org/doc/html/GDB-and-OpenOCD.html) for connection and download details.

For programming alone, use the single OpenOCD command in the previous section. Keep the ELF for GDB debugging; optimization can change how source lines map to instructions. H7 dual-core devices need separate debug targets for each core, so the single-core commands here cannot be reused unchanged.

### Programming works, but logs are missing

This project transports logs through RTT and encodes them with `defmt`. RTT carries bytes; a `defmt` decoder uses the matching ELF to turn those bytes into text. A plain RTT text window cannot display them as ordinary strings. See the [defmt documentation](https://defmt.ferrous-systems.com/).

Ozone and OpenOCD can program and debug the firmware, but this chapter does not configure `defmt` decoding through them. For the first heartbeat check, use the earlier `probe-rs run` command. End the previous debug session before switching tools so the new tool can connect to the probe. Switching to plain RTT text later also requires changing the application's logging output.

## Inspect size and export BIN / HEX

Install [cargo-binutils](https://github.com/rust-embedded/cargo-binutils) when you need to inspect firmware size or your programmer needs BIN/HEX files. Run from the repository root with Rust activated:

```text
rustup component add llvm-tools --toolchain 1.98.1
cargo install cargo-binutils --locked
rust-size target/thumbv7em-none-eabihf/release/minimal
rust-objcopy -O binary target/thumbv7em-none-eabihf/release/minimal minimal.bin
rust-objcopy -O ihex target/thumbv7em-none-eabihf/release/minimal minimal.hex
```

Keep the ELF for debugging: it contains function names and source locations. HEX includes programming addresses. BIN contains only data, so supply the Flash start address used by that ELF. Check the address again after changing MCU, Flash layout or dual-core image.

## Find the step that failed

First confirm that `rustc -Vv` reports 1.98.1, then check that the minimal firmware produces an ELF. For build failures, check the host linker, ARM target and dependency downloads. If the terminal works but the editor reports errors, check that they use the same toolchain.

Connect hardware after producing an ELF. For programming failures, start with probe detection, chip selection and wiring. If programming succeeds but logs are missing, check whether the program runs and how the logs are decoded. The [troubleshooting chapter](troubleshooting.md) covers these cases.

---

[Previous](README.md) · [Index](README.md) · [Next](application-guide.md)
