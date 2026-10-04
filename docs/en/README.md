# Developer handbook

[中文](../zh-CN/README.md) · [English](../en/README.md) · [Home](../../README.en.md)

This handbook is for STM32 developers who are new to Rust. Start by installing the tools and building the minimal firmware in the toolchain chapter. Then use the application guide to add your own tasks. Read board configuration when you need an external crystal or peripherals.

The architecture and implementation chapters explain the framework internals. Best practices walks through the two existing firmware entry points. Read CI/CD when maintaining automated builds and releases.

1. [Toolchain installation and use](toolchain.md)
2. [Application development](application-guide.md)
3. [Rust board configuration](board-configuration.md)
4. [Architecture](architecture.md)
5. [Design and implementation](design-and-implementation.md)
6. [Best practices](best-practices.md)
7. [CI/CD design](ci-cd.md)
8. [Troubleshooting](troubleshooting.md)

## Reading conventions

Before running a command, check its shell and working directory. Blocks marked `text` work in PowerShell, Git Bash and Linux Bash once Rust is available in that terminal.

Complete examples link to their firmware entry-point source. Partial snippets need their surrounding module to work. Both languages use the same models, parameters and code. After a successful build, flash the matching board and check its logs and peripheral behavior.

[Support](../support-scope.md) · [Devices](../devices.md) · [Algorithms](../algorithms.md) · [Dual core](../dual-core.md) · [Project generation](../project-generator.md) · [Storage](../storage.md)
