# App 应用层 / Application layer

[中文指南](../docs/zh-CN/application-guide.md) · [English guide](../docs/en/application-guide.md)

从 `src/bin/` 中的固件入口开始写应用。这里现有最小心跳和参考板两个入口；`src/tasks/` 目前只有说明文件，供你添加业务任务。时钟、引脚、DMA 和中断配置放在 `src/boards/`，由板级代码创建外设，再交给任务使用。框架 crate（Rust 包）不依赖 App。

Start your application from the firmware entry points in `src/bin/`. The two existing entries are the minimal heartbeat and reference-board firmware. `src/tasks/` currently contains only a README and is ready for your application tasks. Put clock, pin, DMA and interrupt configuration in `src/boards/`, where board code constructs peripherals before handing them to tasks. Framework crates (Rust packages) do not depend on App.

在仓库根目录、已激活 Rust 的终端构建最小心跳 / Build the minimal heartbeat at the repository root with Rust activated:

```text
cargo build -p embodied-app --bin minimal --release --locked --target thumbv7em-none-eabihf --features stm32h723vg
```

编译成功后生成固件和调试信息文件 / A successful build produces the firmware and debug information: `target/thumbv7em-none-eabihf/release/minimal` (ELF).

`minimal` 使用内部时钟，烧录到匹配芯片并正常启动后，每秒通过 RTT 输出一次心跳；RTT 日志由调试探针读取。`reference-board` 创建参考板的外设资源，需要用一个准确的 `board-*` feature（编译配置开关）选择板卡。这个入口保留资源并输出 idle 状态。开发自己的应用时，再把每个资源交给一个任务，任务间通过队列或显式同步通信。

`minimal` uses internal clocks. Once flashed to the matching chip and started, it prints a heartbeat once per second through RTT, which carries logs through the debug probe. `reference-board` constructs peripherals for the board selected by one exact `board-*` feature (a build configuration switch). This entry retains those resources and prints idle status. In your application, give each resource to one task and communicate through queues or explicit synchronization.

[工具链 / Toolchain](../docs/zh-CN/toolchain.md) · [参考板 / Boards](../docs/reference-boards.md) · [双核 / Dual core](../docs/dual-core.md) · [生成独立项目 / Generate a project](../docs/project-generator.md)
