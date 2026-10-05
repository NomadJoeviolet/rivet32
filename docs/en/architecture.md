# 4. Architecture

<!-- handbook-nav -->
[Handbook index](README.md) · [简体中文](../zh-CN/architecture.md) · [Project README](../../README.en.md)

To connect an SPI sensor, App first chooses the SPI peripheral, pins and chip selects, then gives them to a driver. The device module parses the returned bytes, an algorithm processes the values, and App decides what to do with the result. Keeping these steps in separate modules lets you retain protocol and algorithm code when changing boards.

Rust calls a compilation unit a crate; it can be a library or an executable. A workspace manages related libraries and applications together. Embassy provides the STM32 HAL, time service and executor used here. The HAL controls peripherals; the executor runs async tasks when they can continue.

## From App to the registers

Arrows mean "uses/depends on." App uses framework crates, which do not depend on App. The diagram omits some third-party libraries and build-time dependencies. Solid edges indicate unconditional dependencies; dashed edges indicate dependencies enabled by Cargo features. These are component relationships, not task execution order.

```mermaid
flowchart TD
    A[App: firmware entry points and boards] --> F[embodied-framework: re-exports]
    A -. firmware .-> EX[embassy-executor]
    A -. firmware .-> ET[embassy-time]
    A -. firmware .-> ST[embodied-stm32]
    F --> C[embodied-core]
    F --> R[embodied-runtime]
    F --> AL[embodied-algorithms]
    F --> D[embodied-devices]
    F -. stm32 feature .-> ST
    R --> C
    D --> C
    D --> AL
    ST --> C
    ST --> D
    ST --> R
    ST -. hal + chip feature .-> HAL[embassy-stm32]
    ST --> EH[embedded-hal / embedded-io traits]
    ST -. usb feature .-> USB[embassy-usb]
    R -. embassy feature .-> ET
    R -. embassy feature .-> ES[embassy-sync]
    HAL --> PAC[stm32-metapac]
    PAC --> MCU[Registers]
```

Each crate's Cargo manifest, its dependency configuration file, lists the full dependencies. The [application facade](../../crates/embodied-framework/src/lib.rs) mainly re-exports modules so App can reach them through one entry point. It adds no scheduler. `embodied-stm32` accepts configured HAL instances; App selects clocks, pins, DMA memory and the supply configuration.

In the diagram, embedded-hal / embedded-io define common operation interfaces. Rust describes the operations a type must provide through a trait, for example an interface for reading and writing data. stm32-metapac supplies register access code and data for specific STM32 devices. The HAL builds peripheral drivers on top of it.

## Host and firmware build boundaries

The root [Cargo.toml](../../Cargo.toml) groups `App`, six framework crates and `xtask` into one workspace. Its `default-members` include the five framework crates that do not require a specific MCU, plus `xtask`; they exclude `App` and `embodied-stm32`. A default Cargo check at the root and a chip-specific firmware build therefore cover different code.

| Configuration entry | What it enables | What the caller still supplies |
|---|---|---|
| Default framework features | `no_std` core/runtime/algorithms/devices, re-exported by framework | A platform `critical-section` implementation when executing runtime synchronization primitives |
| `embodied-runtime/embassy` | Embassy clock and synchronization adapters | A time driver and an executor to run async tasks |
| `embodied-framework/stm32` | STM32 adapter crate and runtime's Embassy adapters | This feature alone neither selects a chip nor enables the STM32 HAL |
| `App/stm32…` | `firmware`, the corresponding STM32 HAL/chip feature, executor, time, RTT logging and panic support | Matching Rust target, bank configuration and hardware validation |
| `App/board-stm32…` | A chip, reference board module and optional dependencies required by reference boards | Matching PCB, oscillator and peripheral connections |
| `xtask` | A host command-line tool using `std` | `data/chips.json`, memory metadata, Cargo toolchain and required script environment |

A firmware build must select exactly one chip/core feature. `--all-features` enables mutually exclusive chips and boards, so it cannot replace a build matrix. `embodied-stm32/build.rs` uses the selected chip's PAC metadata to expose UART, CAN, USB and timer capabilities. Checking only the generic layer on a host does not verify these conditionally compiled paths.

## Where to put new code

| Module | Code that belongs here | Code handled elsewhere |
|---|---|---|
| App | Configure boards, start tasks, hold application state and choose control targets | Reusable framework mechanisms belong in the appropriate library |
| core | Define initialization order, integer time, synchronous notifications, connection state and communication interfaces | MCU handles and physical pins belong in board code |
| runtime | Exchange messages through queues, wait for locks, pools, semaphores or timers, and manage CAN traffic | App chooses robot actions; common HAL drivers come from upstream |
| algorithms | Calculate control outputs, filters, matrices, estimates and geometry; provide CRC, containers and finite state machines (FSM) | Callers read peripherals and schedule tasks |
| devices | Parse data, construct commands and implement behavior for motors, receivers, referee systems and IMUs | App chooses a particular PCB's SPI/UART pins |
| stm32 | Connect HAL objects to framework interfaces, check required chip capabilities and provide dedicated timers | App chooses motion targets |
| xtask | Run on the development computer to query chips, generate projects, build firmware, inspect ELF files and summarize build coverage | Does not run in the firmware |

For BMI088, App chooses the SPI instance, two chip-selects and whether the bus is shared. The STM32 layer connects these configured resources to the device SPI interface, which the BMI088 driver uses to communicate. Algorithms receive numbers without reading the IMU themselves. Protocol parsing and algorithms can therefore also run independently on the development computer.

## What happens at startup and when data arrives

Cortex-M/Embassy components provide the reset entry and runtime startup. App then uses the nine-stage initialization registry to run platform and device setup in order. Only successful completion returns Ready, which App uses to start its tasks. This depends on App following that startup order; a direct Embassy spawner call can still bypass Ready.

The following sequence follows the current single-core [minimal entry point](../../App/src/bin/minimal.rs). The registry runs `PreCore → PostCore → PreEnv → Env → PostEnv → PreDevice → Device → PostDevice → Late`. At each stage it runs the platform callback first, then that stage's hooks in registration order. The first error stops initialization, and the same registry cannot be retried.

```mermaid
sequenceDiagram
    participant Entry as Cortex-M / Embassy entry
    participant App as minimal::main
    participant Init as InitRegistry
    participant HAL as STM32 HAL
    participant Exec as Embassy executor
    participant Task as heartbeat
    Entry->>App: Start async main
    App->>Init: run(context, before_stage)
    loop Nine ordered stages
        Init->>App: before_stage(stage, context)
        opt Env stage
            App->>HAL: init(Default::default())
            HAL-->>App: Peripherals
        end
        Init->>Init: Run registered hooks for this stage
    end
    Init-->>App: Ok(Ready)
    App->>Exec: Ready.start(spawn heartbeat)
    Exec->>Task: Poll heartbeat
    loop Every scheduled second
        Task->>Task: next_tick / EmbassyClock
        Task->>Task: Log count and missed ticks via RTT
    end
```

`minimal` currently starts only an RTT heartbeat once per second; it does not initialize sensors or drive motors. Dual-core builds call [dual_core::init](../../App/src/dual_core.rs) at `Env` instead and use separate firmware images and memory layouts for each core. The [reference-board entry point](../../App/src/bin/reference_board.rs) configures board clocks at `Env`, constructs peripheral resources at `Device`, then keeps them alive while logging idle messages. It does not automatically start receive, parser or control tasks.

An async task waiting for UART or a timer stores its progress in a Future. A peripheral interrupt (IRQ) or the time service sends a wakeup, and the executor checks whether that Future can continue. This check is called poll. The driver delivers bytes or frames, a parser validates the message, and App updates state, runs algorithms and produces output. Synchronous Signal callbacks run in the caller's context without starting another task.

```mermaid
flowchart LR
    IRQ[IRQ or timer event] --> W[Wake Future]
    W --> T[App task resumes]
    T --> V[Validate bytes or frame]
    V --> Q[Bounded message queue]
    Q --> CTRL[Control task / algorithm]
    CTRL --> TX[CAN or byte-stream service]
    TX --> H[HAL peripheral]
```

The diagram shows one way to organize an application. The repository does not yet provide a generic robot controller. App chooses how to divide tasks, define messages and set the control period.

To maintain chip support, tools derive model and capability data from pinned ST sources. Generators and repeatable patches update vendor, and xtask selects exact configurations to build and inspect firmware. A catalogue entry, a linked image and a successful hardware run each need their own checks.

## How source becomes a firmware image

```mermaid
flowchart LR
    CFG[Chip feature + CPU target + bank] --> CG[Cargo and build scripts]
    META[Exact PAC metadata] --> CG
    SRC[App + crates + pinned vendor] --> CG
    CG --> RS[rustc / LLVM]
    RS --> LD[rust-lld + link.x + memory.x + defmt.x]
    LD --> ELF[ELF and optional MAP]
    ELF --> CHECK[xtask ELF inspection]
    ELF --> FLASH[Probe / programmer]
```

Cargo manages dependencies and invokes the Rust compiler. The linker then places code and data according to linker scripts and produces an ELF file. An optional MAP file lists memory allocation, and xtask inspects the ELF. CPU target means the compiler's processor target; bank means the single-bank or dual-bank Flash layout.

The [root Cargo configuration](../../.cargo/config.toml) supplies linker arguments only for bare-metal ARM builds, so development tools still compile for the host computer. Exact HAL/PAC metadata supplies the chip memory layout, and the HAL build script selects the requested bank configuration. The App build script runs on the host and adds per-core memory partitions for dual-core firmware. It does not handle single-core bank selection. Link addresses must match the actual chip; a roughly similar memory size is insufficient.

When changing chips, review the Rust toolchain file, Cargo.lock, App features, Rust board modules, support policy and generated catalogue together. Cargo features are compile-time switches. Actual clocks, pins, DMA and interrupts are configured in Rust board code.

## Which code comes from upstream

`vendor` stores pinned `embassy-stm32`, `stm32-metapac` and the source produced by applying local patches. The Embassy revision is `ae9e6f0672af84cec8e200a94c574041844396f0`. Ordinary GPIO/UART/SPI/PWM/CAN/USB drivers and the executor mainly reuse upstream implementations.

This framework provides App organization, initialization and communication interfaces, task coordination, algorithms, device protocols and STM32 adapters. Local low-level code mainly adds missing devices, fixes confirmed defects and implements exclusive compare timers. See [provenance and licenses](../../THIRD_PARTY_NOTICES.md) for sources and licensing. Historical patches include excluded devices; their status still follows the current support policy.

## Lessons from Dynamo and Warp

The source comparison uses fixed revisions: Dynamo [`519e735`](https://github.com/ai-dynamo/dynamo/tree/519e735550c1a4aac67c2fd37d4a56ed0a014653) and Warp [`b865631`](https://github.com/warpdotdev/warp/tree/b865631c9a0e46b548c7ec7dc32e228a148171d1). The table separates observable implementations from recommendations for this framework. Recommendations do not imply that a runtime redesign has been implemented.

| Observable design | Mapping and recommendation for rivet32 |
|---|---|
| Dynamo's [AsyncEngine](https://github.com/ai-dynamo/dynamo/blob/519e735550c1a4aac67c2fd37d4a56ed0a014653/lib/runtime/src/engine.rs) separates request, response and error types from execution context. Warp's [warpui entry point](https://github.com/warpdotdev/warp/blob/b865631c9a0e46b548c7ec7dc32e228a148171d1/crates/warpui/src/lib.rs) re-exports core and organizes platform, windowing and rendering modules. | Keep communication traits in `core`, pure logic in `devices/algorithms` and hardware adapters in `stm32`; App composes them. Extract new traits when multiple real implementations justify them. Prefer static generics and bounded messages over a cloud-style dynamic registry. |
| Warp's [OnCancelFuture](https://github.com/warpdotdev/warp/blob/b865631c9a0e46b548c7ec7dc32e228a148171d1/crates/warp_util/src/on_cancel.rs) distinguishes completion from Drop before completion, with [dedicated tests](https://github.com/warpdotdev/warp/blob/b865631c9a0e46b548c7ec7dc32e228a148171d1/crates/warp_util/src/on_cancel_tests.rs). Dynamo's context separately models stopping generation and terminating a request. | This framework already has [MessageLease Drop](../../crates/embodied-runtime/src/pool.rs) and [CAN worker retention/retry contracts](../../crates/embodied-runtime/src/can.rs). Next, test cancellation before first poll, after Pending, normal completion and driver failure: resources must be returned once and unsent frames retained as specified. Logical cancellation does not necessarily undo a hardware transfer. |
| Warp's [time.rs](https://github.com/warpdotdev/warp/blob/b865631c9a0e46b548c7ec7dc32e228a148171d1/crates/warpui_core/src/time.rs) exposes an advanceable time value in tests while production reads real time. | Use this framework's [Clock trait](../../crates/embodied-runtime/src/clock.rs) for a manually advanced monotonic test clock and fake buses. Check timeout boundaries, queue overflow and retries with deterministic inputs instead of real sleeps. The current [PoC](../poc.md) already uses explicit timestamps for synchronous behavior; a controllable async clock remains future work. |

The immediate additions are an executable PoC, behavior tests and explicit feature boundaries. Prioritize cancellation and fault-injection tests next, followed by on-board timing measurements. Existing `CanTxStats` and `Tick.missed` can expose overflow, driver errors and missed periods over RTT without first adding a full telemetry service.

Dynamo targets distributed inference on an operating system and Warp targets desktop applications. Their thread, allocation, network and UI runtimes are not drop-in bare-metal designs. Transfer the interface and testing principles while retaining `no_std`, fixed-capacity storage and Embassy. No code or dependencies from either project are imported here.

## Choices to make in your application

- Queues and pools have fixed capacities, which helps estimate RAM use. App must select those capacities and handle full queues or exhausted resources.
- Rust ownership makes peripheral use explicit. Shared peripherals still need an access order, and drivers and buffers must remain valid throughout their use.
- Async suits peripheral waits. When moving FreeRTOS code, revisit dependencies on forced suspension, thread termination, recursive locks and priority inheritance; the framework does not promise equivalent behavior.
- Protocols and algorithms can be checked on the host first. Electrical behavior, IRQ timing, DMA and physical buses need hardware measurement.
- Support for an MCU means there is a backend and relevant check evidence within the support scope. Check separately whether the chip has the required peripherals and can fit the selected functions together.

Start reading with the minimal App, then init/time and runtime. Follow one UART/CAN/IMU call through its STM32 adapter into HAL. The [next chapter](design-and-implementation.md) explains the implementations.

---

[Previous](board-configuration.md) · [Index](README.md) · [Next](design-and-implementation.md)
