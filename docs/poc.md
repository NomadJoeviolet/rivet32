# 可复现 PoC / Reproducible PoC

[文档目录 / Documentation](README.md) · [中文首页](../README.md) · [English home](../README.en.md)

本页把验证分为主机运行、固件链接、独立工程和实机心跳四层。主机 PoC 使用模拟 CAN 帧和显式时间，不需要开发板；它会实际执行框架公共 API，并用断言检查结果。工具准备见[工具链手册](zh-CN/toolchain.md)，模块关系与启动时序见[架构设计](zh-CN/architecture.md)。

This guide separates host execution, firmware linking, standalone project generation and on-board heartbeat verification. The host PoC executes public framework APIs with synthetic CAN frames and explicit timestamps, checking results with assertions. It needs no board. See the [toolchain guide](en/toolchain.md) and [architecture](en/architecture.md) for setup and component boundaries.

## 1. 主机运行 / Host execution

在仓库根目录执行，使用 [rust-toolchain.toml](../rust-toolchain.toml) 固定的版本；不要传 MCU target 或芯片 feature。首次获取依赖需要联网，已有完整缓存时可加 `--offline`。Python 文档检查使用 Python 3.11+；Windows 中可把 `python3` 换为对应的 Python 命令。

Run from the repository root with the pinned toolchain, without an MCU target or chip feature. The initial dependency fetch needs network access; use `--offline` once the complete dependency cache is available. The documentation check uses Python 3.11+; on Windows, use the equivalent Python command if `python3` is unavailable.

```text
cargo fetch --locked
cargo fmt --all --check
python3 scripts/check_docs.py
cargo check --workspace --locked
cargo check -p embodied-stm32 --features usb --locked
cargo clippy --workspace --lib --bins --locked -- -D warnings
cargo test -p embodied-framework --test host_contracts --locked
cargo run -p embodied-framework --example host_poc --locked
```

[host_poc.rs](../crates/embodied-framework/examples/host_poc.rs) 执行以下链路。PID 的 20 是这个模拟例子的原始命令值，不是经过真实电机标定的电流目标。

[host_poc.rs](../crates/embodied-framework/examples/host_poc.rs) executes the following pipeline. Its PID output of 20 is a raw command value for this synthetic example, not a calibrated motor current target.

```mermaid
flowchart LR
    INIT[Nine init stages] --> READY[Ready.start]
    READY --> RX[Synthetic CAN feedback]
    RX --> Q[Bounded Queue]
    Q --> DJI[DJI M3508 decoder]
    DJI --> PID[PID: target 70 RPM, feedback 60 RPM]
    PID --> TX[Encode command: CAN ID 0x200]
```

PoC 还检查队列满与空、非法编码器值、倒退时间戳、超时连接状态，以及迟到 tick 跳过周期后保持原有相位。成功时退出码为 0，并输出：

The PoC also checks full/empty queues, invalid encoder values, backwards timestamps, connection expiry and skipped periodic deadlines without phase drift. Success means exit code 0 and this output:

```text
host PoC passed: 9 init stages, bounded CAN queue, DJI validation, PID output=20, missed ticks=2
```

[host_contracts.rs](../crates/embodied-framework/tests/host_contracts.rs) 另有三个行为测试：初始化失败后不返回 Ready 且禁止重试；拒绝的反馈不覆盖上次状态或刷新在线时间；周期计算溢出时不改变调度状态。这两个执行命令也在 CI 的 Linux/Windows host job 中运行。

[host_contracts.rs](../crates/embodied-framework/tests/host_contracts.rs) adds three behavior tests: initialization failure returns no Ready and prevents retries; rejected feedback preserves the last accepted state and freshness; periodic overflow leaves the schedule unchanged. Both execution commands run in the Linux/Windows CI host jobs.

这条主机链路使用普通 `Queue`，不验证 `SharedQueue`、ISR、异步等待/取消、Embassy 执行器或真实总线。`cargo check --workspace` 的默认 feature 也不包含具体芯片的 HAL 路径，需要下一步交叉编译。

The host pipeline uses the owned `Queue`. It does not exercise `SharedQueue`, interrupts, async waiting/cancellation, the Embassy executor or a physical bus. Default-feature workspace checks do not cover chip-specific HAL paths; cross-compile those separately.

运行控制有独立的异步测试与示例，覆盖协作停止、中止、保留帧恢复和可控 deadline。它们使用假驱动和主机时钟；源码依据、接口语义和预期输出见[运行控制说明](runtime-control.md)。

Lifecycle control has separate async tests and a demo covering graceful stop, abort, retained-frame recovery and controlled deadlines. They use fake drivers and a host test clock; see [runtime control](runtime-control.md) for source evidence, contracts and expected output.

```text
cargo test -p embodied-runtime --tests --locked
cargo run -p embodied-runtime --example controlled_can --locked
```

## 2. H723 最小固件 / H723 minimal firmware

```text
cargo xtask build --chip stm32h723vg
```

该命令选择 `thumbv7em-none-eabihf` 和对应芯片 feature，执行编译、链接及 ELF 检查。相比单独 `cargo check`，它还验证 ARM ELF、向量表和内存区域。查看本次生成的结果：

This selects `thumbv7em-none-eabihf` and the matching chip feature, then compiles, links and inspects the ARM ELF, vector table and memory regions. Inspect the newly generated outputs:

| 路径 / Path | 用途 / Purpose |
|---|---|
| `target/reports/stm32h723vg/result.json` | 编译、链接、ELF 状态与工具链信息 / Compilation, link and ELF status with toolchain evidence |
| `target/artifacts/stm32h723vg/minimal.elf` | 可供调试探针加载的固件 / Firmware for the debug probe |
| `target/artifacts/stm32h723vg/minimal.map` | 链接布局 / Link layout |

要求 `compile.status`、`link.status` 和 `elf_validation.status` 均为 `passed`；报告的 `hardware_validation` 仍为 `not-run`。这只覆盖最小心跳和所选芯片，不代表目录中所有芯片、外设构造或双核配置均已验证。

Require `compile.status`, `link.status` and `elf_validation.status` to be `passed`. The report still marks `hardware_validation` as `not-run`. This covers the selected chip's minimal heartbeat, not every catalogue entry, peripheral constructor or dual-core configuration.

## 3. 生成独立工程 / Generate a standalone project

下面验证生成器、框架源码快照和独立固件构建。先创建 `target` 父目录（前面的 Cargo 命令已创建）；`target/review-generated` 必须尚不存在。重复运行时选择新的空路径。

This validates the generator, framework snapshot and independent firmware build. The parent `target` directory must exist (the preceding Cargo commands create it), and `target/review-generated` must not already exist. Choose a new destination when repeating this step.

```text
cargo fetch --locked
cargo xtask new --chip stm32g474re --path target/review-generated --name review-poc
python3 scripts/verify_generated_project.py target/review-generated
cd target/review-generated
cargo app-build
cargo app-check
cd ../..
python3 scripts/verify_generated_project.py target/review-generated
```

生成的 ELF 位于 `target/review-generated/target/thumbv7em-none-eabihf/release/firmware`。前后两次快照检查确认生成清单与源码匹配；该工程采用 G474RE 通用模板和内部时钟。更多参数与双核流程见[工程生成说明](project-generator.md)。

The ELF is `target/review-generated/target/thumbv7em-none-eabihf/release/firmware`. The before/after snapshot checks verify source files against the generated inventory. This uses the generic G474RE template and internal clocks; see [project generation](project-generator.md) for options and the dual-core workflow.

## 4. 板上验证 / On-board verification

连接匹配的 STM32H723VG 板卡和受支持探针后，按[烧录步骤](zh-CN/toolchain.md#用-probe-rs-启动固件)加载第二步的 ELF，并观察多次连续的 `App heartbeat=... missed=...` RTT 日志。记录 MCU、板卡、探针、固件及日志后，才可认定这块板上的心跳运行通过。

With a matching STM32H723VG board and supported probe, follow the [flashing guide](en/toolchain.md) using the ELF from step 2. Observe repeated `App heartbeat=... missed=...` RTT messages and record the MCU, board, probe, firmware and logs before claiming an on-board heartbeat pass.

本页前三步无需硬件，第四步需要实际设备。主机 PoC 与固件链接成功，均不能代替上板运行，也不能证明电机控制或传感器数据链路已经通过实机验证。

The first three steps need no hardware; the fourth does. Host execution and firmware linking do not establish an on-board pass or validate a real motor/sensor pipeline.
