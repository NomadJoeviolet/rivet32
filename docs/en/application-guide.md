# 2. Application development guide

<!-- handbook-nav -->
[Handbook index](README.md) · [简体中文](../zh-CN/application-guide.md) · [Project README](../../README.en.md)

Build and run the minimal heartbeat to check your tools and debug connection, then add your peripherals and tasks. The heartbeat needs no additional LED or serial wiring. Complete [toolchain setup](toolchain.md) first. Unless stated otherwise, run commands at the repository root in a terminal where Rust is available. `text` blocks work in PowerShell, Git Bash and Linux Bash.

## Obtain a complete project

Clone the Git URL supplied by your team or extract the source archive. Enter the directory containing the root `Cargo.toml`, the project configuration file read by Cargo, Rust's build tool.

Keep App, crates, vendor, data, lockfiles and configuration together. Copying only App loses the framework and local patches. Use the project generator below when you need a separately maintained application.

## Select the actual device and build configuration

Determine the full part number, package and core from the schematic and MCU marking, then query:

```text
cargo xtask list-chips --chip stm32h723vg
cargo xtask list-chips --family G4
```

The output lists the compilation target and backend, the underlying drivers for that chip. You can select from 895 models and 921 chip/core configurations. The 57 G411/G414, H7R/S and H543/H553 models are excluded. Their original data remains in the chip catalogue; use the [support policy](../../data/support-policy.json) to determine which configurations are available.

Cargo uses features as build configuration switches; the chip selection is one such switch. Select one chip/core per firmware image. H7 `-cm7` and `-cm4` configurations correspond to separate firmware images and need separate builds.

## First firmware: H723 heartbeat

```text
cargo build -p embodied-app --bin minimal --release --locked --target thumbv7em-none-eabihf --features stm32h723vg
```

This command builds the minimal firmware for STM32H723VG. Look up the matching configuration before changing chips. A successful build produces `target/thumbv7em-none-eabihf/release/minimal`, an ELF file containing the program and debug information.

The entry point is [minimal.rs](../../App/src/bin/minimal.rs). It stores initialized resources in a Context structure and uses an initialization registry to run the startup stages in order. In Env, the hardware-environment stage, it calls the HAL (the chip drivers) with internal-clock defaults. Completing every stage successfully produces Ready, which starts the heartbeat task. The task reports a count and missed periods once per second.

The example needs no external crystal, LED or serial connection. It sends logs through RTT, which you read using a debug probe. Follow the [toolchain chapter](toolchain.md) for flashing and reading logs. The first build downloads dependencies and can take time. After the ELF is built, flash it to the matching chip and check for the once-per-second heartbeat to confirm that the program runs on the board.

## Chip features and Flash banks

Flash banks describe how the chip's Flash is divided. The build must match the layout selected by its option bytes. For a generic G474RE minimal image on hardware configured for dual-bank operation:

```text
cargo build -p embodied-app --bin minimal --release --locked --target thumbv7em-none-eabihf --features stm32g474re,dual-bank
```

If option bytes select single-bank, select that layout in the build too. Features change the linker and driver configuration; they do not change the chip's option bytes. xtask is the repository's command tool. It looks up the target for a chip and defaults to dual-bank on devices with configurable layouts:

```text
cargo xtask build --chip stm32g474re --bank dual-bank
```

The command reports the build result and ELF inspection records. Find the firmware at the path printed in the terminal: xtask uses different build directories from direct Cargo commands. The tool uses the project's memory data to determine which bank layouts a chip supports.

## Where application code belongs

```text
App/
  Cargo.toml             binaries, features and application dependencies
  src/bin/               firmware entry points
  src/lib.rs             application-level shared modules
  src/tasks/             application task implementations
  src/boards/            Rust clock, IRQ, pin and resource configuration
```

`src/bin/` contains two firmware entry points: the minimal heartbeat and the reference board. `src/tasks/` currently contains only a README. After adding a Rust module, declare it in its parent and use it from an entry point so it is compiled and run. Put board initialization in `src/boards/`; see the [reference-board document](../reference-boards.md) for wiring.

Keep application behavior in App, reusable protocols in devices, and pure computation in algorithms. Framework crates do not depend on App.

For another firmware entry point, declare `[[bin]]`, its entry path and suitable `required-features` in App's Cargo configuration, with `test = false` and `bench = false` as in existing firmware. Do not change the generated chip-feature table to implement application behavior.

## Adding a task

1. Follow the existing heartbeat: mark an async function with `#[embassy_executor::task]` and place it in an application module. An async function can let other ready tasks run while it waits for a timer or peripheral.
2. Decide resource ownership: for example, only the receive task gets the UART receiver, and the transmit task gets its sender.
3. Construct peripherals during initialization and store them in Context. At startup, use `Option::take` to move a peripheral to its task. This leaves its previous location empty, so it cannot be taken twice.
4. Start tasks through `ready.start(...)`. Handle failures when spawning (submitting tasks to the executor) and while tasks run. The example's `unwrap` triggers a panic on error. Your application should define how to report initialization failures and stop affected functions.
5. Add waiting points to loops. Long computations delay other tasks. `.await` only yields when the operation being awaited is unfinished and returns Pending.

Prefer fixed-capacity queues for messages between tasks. When tasks share a peripheral, define the access order; an async mutex lets them use it one at a time. A HAL peripheral token represents the right to use that peripheral. Do not duplicate it or initialize the same peripheral in multiple tasks. See [best practices](best-practices.md).

## Using framework modules

Applications normally import `embodied_framework::{core, runtime, algorithms, devices, stm32}`. The `stm32` module is feature-gated; generic algorithms and protocols can be tested on the host.

| Requirement | Entry point | Application decisions |
|---|---|---|
| Stable periodic work | `core::time::Periodic`, `runtime::embassy::next_tick` | Period and business response to missed deadlines |
| Inter-task messages | `runtime::queue::SharedQueue` | Capacity, timeout and overflow handling |
| Reusable message storage | `runtime::pool::MessagePool` | Type, object count and lifetime |
| CAN transmit/receive | `runtime::can` + `stm32::can` | Bitrate, filtering, recovery and protocol routing |
| IMU | `devices::imu` + `stm32::bus` | Device, ranges, SPI mode, calibration and bus sharing |
| Control/filtering | `algorithms` | Sample units, parameters, saturation and invalid-input policy |

Do not refresh connection state on invalid packets; observe the connection only after packet validation succeeds. Motor protocols encode and decode data without knowing mechanical limits or motion goals. The dual-arm referee message explicitly selects `DoubleArm14`; its version is not inferred from payload length.

## Reference boards

These five board features select configurations in the repository, including each board's chip, clocks and peripheral resources:

| Feature | MCU | Description |
|---|---|---|
| `board-stm32h723vg` | H723VG | Reference resource configuration |
| `board-stm32g431kb` | G431KB | Reference resource configuration |
| `board-stm32g474ce` | G474CE | Reference resource configuration, includes dual-bank |
| `board-stm32f407ig` | F407IG | Reference resource configuration |
| `board-stm32f427ii` | F427II | Reference resource configuration, explicitly selects TIM2 as its timebase |

For example, build the G474 reference board:

```text
cargo build -p embodied-app --bin reference-board --release --locked --target thumbv7em-none-eabihf --features board-stm32g474ce
```

A successful build produces `target/thumbv7em-none-eabihf/release/reference-board`. This configuration requires G474CE and a 12 MHz HSE (external high-speed clock). The earlier generic minimal build uses G474RE, a different model, so do not apply this board configuration to it. The reference image initializes and retains resources and periodically reports idle status. Add your own tasks for robot control. Read the [board chapter](board-configuration.md) for wiring and limitations.

## Generate a standalone application

Run from the framework root. The parent directory must exist and the destination must not:

```text
cargo xtask new --chip stm32g474re --path ../robot-demo --name robot-app
```

The generator copies App, the framework and vendor sources, and adds a lockfile, editor settings, CI checks and a `project.json` record of their origins. The generated project contains its own sources and does not depend on the original repository's absolute path. Review source differences when upgrading the framework.

Enter the generated directory and run:

```text
cd ../robot-demo
cargo app-build
cargo app-check
cargo clippy -p robot-app --bin firmware --locked --target thumbv7em-none-eabihf -- -D warnings
```

Expected ELF: `target/thumbv7em-none-eabihf/release/firmware` inside the generated project. Defaults use an internal clock and RTT heartbeat; add the actual board configuration. Names start with a lowercase letter and otherwise use lowercase letters, digits, underscores or hyphens. Existing directories are never overwritten. See the [project generator guide](../project-generator.md) for length and reserved-name restrictions.

Specify the H7 core when generating a dual-core project, then follow the [dual-core guide](../dual-core.md) to build both firmware images and check their shared memory regions.

## Daily iteration order

After changing pure algorithms or parsers, run compilation and static checks on your computer, then build App for the target chip. After changing pins, clocks or DMA, check the board documentation and ELF before testing the affected hardware. Review the lockfile, board feature and Rust configuration together. Record the exact MCU, bank and probe, along with the compilation checks and board tests completed.

---

[Previous](toolchain.md) · [Index](README.md) · [Next](board-configuration.md)
