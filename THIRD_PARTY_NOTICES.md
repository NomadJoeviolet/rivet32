# 第三方来源与许可证 / Third-party notices

本项目使用 [MIT 许可证](LICENSE)，保留 Copyright (c) 2025 姜欢桐 及本项目贡献者署名。部分通用算法和设备行为参考了其他 MIT 授权实现，相关作者的权利和署名仍需保留。交付内容和运行依赖中不包含旧 C/C++ 对照工程。

This project uses the [MIT license](LICENSE) and retains Copyright (c) 2025 姜欢桐 and project contributor attribution. Some general algorithms and device behavior derive from other MIT-licensed implementations; retain those authors' rights and attribution. The delivered project and its runtime dependencies do not include the former C/C++ comparison projects.

| 来源 / Source | 固定版本 / Revision | 用途 / Use |
|---|---|---|
| [Embassy](https://github.com/embassy-rs/embassy) | `ae9e6f0672af84cec8e200a94c574041844396f0` | Executor, time and STM32 HAL; original MIT/Apache licenses |
| [STM32 data](https://github.com/embassy-rs/stm32-data-generated) | `stm32-data-caa36afd62510b0e6315ee0dccd1f9c65fbcac83` | PAC and metadata; retained vendor licenses and notices |
| [ST open pin data](https://github.com/STMicroelectronics/STM32_open_pin_data) | `7d1f1514ed5583ec5007ad91236b4e1d377295b1` | Model/package metadata; [ST license](data/ST-LICENSE.txt) |
| [BSP pack](https://gitee.com/BITRM2026/bsp-pack) | `0e106801ec26a6db0588376d59817116ca37deea` | Board and IMU reference; original author attribution retained |

`Cargo.lock` 记录 Rust 依赖的版本和校验值，每个依赖使用自己的许可证。发布或复制项目时，保留 `vendor/` 和 `data/patches/` 中的许可证、作者声明和来源记录。项目根目录的 MIT 许可证不会替代 ST 数据和第三方依赖的许可证。

`Cargo.lock` records Rust dependency versions and checksums. Each dependency has its own license. When copying or distributing the project, retain the licenses, author notices and source records in `vendor/` and `data/patches/`. The root MIT license does not replace the licenses for ST data or third-party dependencies.

## 本地芯片补丁 / Local chip patches

`data/patches/stm32h5-47c*` 保存 H543/H553 共 14 个型号的 PAC/HAL 补丁来源和许可证。这些型号当前已被排除，保留记录用于追溯补丁来源，当前支持范围见[支持说明](docs/support-scope.md)。

G071EB/G081EB 的 UCPD 条件编译修复位于 `data/patches/embassy-stm32/ucpd-port-cfg.json`。该修复在已有补丁之后应用，记录原始来源、修改前文件和两个官方封装 XML 的哈希，详见 [G0 修复记录](docs/support-scope.md)。哈希用于检查文件内容是否与记录一致。

`vendor/embassy-stm32` 使用固定版本的 HAL，并保留 MIT/Apache-2.0 许可证。本地修改包括依赖路径、H563LI/H573LI 和 F723RC/RE 四个芯片 feature，以及双核启动、DMA 优先级和时间驱动的资源选择。`data/patches/embassy-stm32/dual-core.json` 记录该组修改前后的 SHA 校验值。

`vendor/stm32-metapac` 保留固定版本的 PAC，并加入从 ST 数据生成的 H563LI/H573LI、F723RC/RE 数据。许可证和来源说明保存在该目录的 LICENSE/NOTICE/ST-LICENSE 文件中。重建这些内容的脚本是 `xtask/scripts/vendor_embassy.py`、`prepare_h5_li_patch.py`、`prepare_f723_patch.py`；当前 PAC 文件清单见 `data/patches/stm32-metapac/vendor-manifest.json`。生成独立 App 项目时，会一起复制许可证和补丁来源。

F723 补丁引用 ST DS11853 Rev 9（2022 年 7 月，226 页）。PDF 末页标明 © 2022 STMicroelectronics、保留所有权利，没有声明开放文档许可证。本项目的 MIT、ST pin-data 和 CMSIS 许可证不适用于这份 PDF。仓库和独立 App 快照只记录 URL、SHA-256 与使用页码，不分发全文。

需要核对原文时，运行 `python xtask/scripts/fetch_source_documents.py`。它从已核对的 [Chipdip 镜像](https://static.chipdip.ru/lib/694/DOC030694347.pdf) 恢复本地文件，大小为 6,678,029 字节，SHA-256 为 `25e8b094853890df20d8c7cb2e7fb37b780d34fbe88422aa65a113db9f908f73`。该版本带 Arrow 页脚，保留 ST 原声明。[ST 官方入口](https://www.st.com/resource/en/datasheet/stm32f723ie.pdf) 返回的同版手册字节不同，因此来源校验仍使用记录中的文件。详细记录见 `data/patches/stm32f723-r/sources.json`。
