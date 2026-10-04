# 开发手册

[中文](../zh-CN/README.md) · [English](../en/README.md) · [Home](../../README.md)

这份手册面向熟悉 STM32、刚开始使用 Rust 的开发者。第一次使用时，先完成工具链章节中的安装和最小固件编译，再读 App 开发指南，把自己的任务加进应用。需要使用外部晶振或外设时，继续读板级配置。

架构和实现章节用于了解框架内部；最佳使用示范解释两个现有固件入口。维护自动构建和发布时再读 CI/CD。

1. [工具链安装与使用](toolchain.md)
2. [App 开发指南](application-guide.md)
3. [Rust 板级配置](board-configuration.md)
4. [架构设计](architecture.md)
5. [设计原理与实现](design-and-implementation.md)
6. [最佳使用示范](best-practices.md)
7. [CI/CD 设计](ci-cd.md)
8. [排错与常见问题](troubleshooting.md)

## 阅读约定

运行命令前，先确认说明中的终端和工作目录。标为 `text` 的命令适用于已能使用 Rust 的 PowerShell、Git Bash 和 Linux Bash。

完整示例附有固件入口的源码链接；局部代码片段需要放回相应模块才能使用。中英文手册使用相同的型号、参数和代码。编译成功后，还需要烧录到匹配的板卡，检查日志和外设行为。

[Support](../support-scope.md) · [Devices](../devices.md) · [Algorithms](../algorithms.md) · [Dual core](../dual-core.md) · [Project generation](../project-generator.md) · [Storage](../storage.md)
