# 用户任务 / User tasks

这个目录目前只有说明文件。先阅读下面链接的最小心跳入口，再在这里添加自己的异步任务。异步任务在等待定时器或外设时，可以让其他就绪任务运行。

新增文件后，在 `App/src/lib.rs` 或固件入口声明对应模块，Rust 才会编译它。使用 `#[embassy_executor::task]` 定义任务，并在初始化成功后的 `ready.start(...)` 中启动。把每个外设交给负责使用它的任务，周期等待使用 `Periodic` 与 `next_tick`。

This directory currently contains only this README. Read the minimal heartbeat linked below, then add your own async tasks here. An async task can let other ready tasks run while it waits for a timer or peripheral.

After adding a file, declare its module in `App/src/lib.rs` or the firmware entry point so Rust includes it in the build. Define tasks with `#[embassy_executor::task]` and start them inside `ready.start(...)` after initialization succeeds. Give each peripheral to the task that uses it. Use `Periodic` and `next_tick` for periodic waits.

[最小完整入口 / Complete minimal entry](../bin/minimal.rs) · [使用示范 / Best practices](../../../docs/zh-CN/best-practices.md)
