# @NAME@

本项目用于 `@CHIP@`。Rust 编译目标为 `@TARGET@`，编译器为 Rust 1.98.1 stable。先按下面的命令构建心跳固件，再添加自己的板级配置和任务。

在 `App/src/tasks/` 编写应用任务，在 `App/src/boards/` 配置板级时钟、引脚和外设资源。`App/src/main.rs` 按九个阶段执行初始化，全部成功后才启动心跳。默认使用内部时钟，不需要 LED 或串口接线。

```text
rustup toolchain install 1.98.1 --profile minimal --component rustfmt --component clippy
rustup target add --toolchain 1.98.1 @TARGET@
cargo app-build
cargo clippy -p @NAME@ --bin firmware --locked --target @TARGET@ -- -D warnings
```

构建产物为 `target/@TARGET@/release/firmware`，这是带调试符号的 ELF 固件文件。初次生成时，生成器根据框架的依赖锁文件离线解析项目，并保存 `Cargo.lock`。如果生成报告提示缺少依赖缓存，先在源框架执行 `cargo fetch --locked`，再按报错提示完成当前项目的依赖锁定。

若 `project.json` 的 `bank` 非空，烧录前核对芯片 option bytes（选项字节）中的 Flash 分区是否与之相同。H5 默认要求关闭 TrustZone 安全隔离功能。构建、CI（持续集成）和生成命令都不会烧录芯片或改变选项字节。

用 VS Code 打开本目录，安装推荐的 Rust 代码分析扩展 rust-analyzer。`.vscode/settings.json` 已指定编译目标；根目录的 Cargo 配置不强制使用 MCU 目标，便于单独检查通用框架库。GitHub Actions 配置位于 `.github/workflows/ci.yml`，推送仓库后会检查格式、编译固件、运行 Clippy 静态检查，并上传 ELF 文件。

接入实际硬件时，在 `App/src/boards/` 中配置时钟、引脚、DMA 和中断，由 Rust HAL（硬件抽象层）完成初始化。电脑端链接器、下载工具和 RTT 调试日志的说明见 `docs/zh-CN/toolchain.md`。阅读该文档时，当前项目的构建命令使用上面的 `cargo app-build`；文档中源框架的 xtask 和最小示例构建命令不适用于本项目。

编译通过后，还需要在实际板卡上检查时钟、引脚、电源和外设行为。

## 框架源码与来源资料

`Framework/` 保存生成时的框架源码副本，随项目一起提交，并保留 LICENSE 和 THIRD_PARTY_NOTICES。构建不需要原框架目录。`project.json` 记录完整芯片型号、Flash bank 分区方式，以及生成时各文件的校验值。升级框架时，需要检查源码变更并重新验证项目。

`data/patches/` 包含芯片补丁的来源清单、原始数据和 SHA-256 校验值。项目不包含受版权保护的数据手册全文。需要查阅原文时，运行 `python xtask/scripts/fetch_source_documents.py`，脚本会按记录的地址下载文件，核对 SHA 校验值，并存入 Git 忽略的 `data/sources/` 缓存。下载内容不匹配时会报错，已有缓存保持不变。

项目的 `.gitattributes` 使用 `* -text` 关闭 Git 自动换行转换，让上游文件与 `project.json` 中记录的校验值保持一致。Git 仍可正常显示文本差异。修改文件后，它的校验值会与生成时的记录不同。
