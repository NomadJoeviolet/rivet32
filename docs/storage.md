# 存储管理 / Storage

[中文手册](zh-CN/README.md) · [English handbook](en/README.md)

需要腾出磁盘空间或打包源码时，先按下表区分源码和缓存，再选择清理命令。

Use this page when freeing disk space or packaging sources. Identify source files and caches in the table before choosing a cleanup command.

| 目录 / Directory | 处理方式 / Policy |
|---|---|
| `App/`, `crates/`, `vendor/`, `data/patches/` | 构建与补丁来源，保留 / Build and patch sources: retain |
| `docs/`, `scripts/`, `xtask/`, `.github/workflows/` | 文档和开发工具，保留 / Documentation and development tools: retain |
| `.tools/` | 本机 Rust 工具链和 Cargo 下载依赖；不打入源码包 / Local toolchain and downloaded dependencies; exclude from source bundles |
| `target/` | 可重建编译缓存和构建产物；仅在构建进程停止后清理 / Rebuildable output; remove only when builds are stopped |
| `.cache/` | 可恢复来源缓存，维护脚本可能重新下载 / Recoverable source cache; maintenance scripts may restore it |
| `dist/` | 按需生成发布包，不保留阶段验证归档 / Release packages generated on demand; no phase-specific archives |

源代码和用于核对来源的数据合计低于 1 GB；本机工具链与下载依赖另占空间。缓存大小限制只适用于指定的构建目录，整个仓库仍可能超过该大小。保留正在使用的源码、依赖和许可证；不能仅凭文件夹大小删除 vendor 或补丁数据。

Sources and the data needed to check their origins total less than 1 GB. Local tools and downloaded dependencies use additional space. Cache size limits apply to specific build directories; the whole checkout can be larger. Keep active sources, dependencies and licenses. Do not delete vendor or patch data just because a directory is large.

[clean-delivery.ps1](../scripts/clean-delivery.ps1) 只清理脚本中列出的旧测试目录和交付文件。先用 `-WhatIf` 查看待删除内容，确认本仓库的 Cargo/rustc 进程已停止后再执行。日常构建结束后，可用 `cargo clean` 清理 Cargo 默认输出目录。

[clean-delivery.ps1](../scripts/clean-delivery.ps1) removes only the old test directories and delivery files listed in the script. Preview the files with `-WhatIf`, then run it after this checkout's Cargo/rustc processes have stopped. Use `cargo clean` to clear Cargo's default output directory after ordinary builds.
