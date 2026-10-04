# STM32F723RC / STM32F723RE 独立 VQFPN68 PAC 补丁

这两个 feature 对应 ST 官方 XML 的准确型号与 68 引脚封装，分别有独立 chip JSON、PAC、metadata 和 device.x。没有将 RC/RE 重定向到 VC/VE/F722。仅复用经核对相同的 F723 寄存器定义与内部元数据，重新构建全部封装位置、GPIO 和外设 AF。

目前证明的是准确型号的数据生成与软件编译。旧 DS11853 Rev9 没有完整 68 引脚订货表；ST 后续更正预告要求补入此封装，但尚未取得包含完整温度等级、电气和订货选项的新版数据手册。不能据此承诺当前可采购、所有工作条件或上板运行。

## 固定来源

`sources.json` 记录下载 URL、固定 revision 与 SHA-256；`sources/` 保留原 XML、准确 CMSIS 头文件及各自原许可证。

| 来源 | 固定版本 | 用途 |
|---|---|---|
| ST STM32_open_pin_data | `7d1f1514ed5583ec5007ad91236b4e1d377295b1` | RCVx、REVx、对照 V(C-E)Tx/Yx、GPIO F72x AF XML |
| ST cmsis-device-f7 | `2352e888e821aa0f4fe549bd5ea81d29c67a3222` | 准确 `stm32f723xx.h` 的寄存器地址、IRQ 和 RCC 位 |
| stm32-data-generated | `e463add8cc54375f61c6f5f83d6b589e7fc68be2` | 同容量 F723VC/VE 基线 JSON 与共享寄存器 JSON |
| stm32-data generator | `caa36afd62510b0e6315ee0dccd1f9c65fbcac83` | 原始未修改的 `stm32-metapac-gen` |

官方源：[RCVx XML](https://github.com/STMicroelectronics/STM32_open_pin_data/blob/7d1f1514ed5583ec5007ad91236b4e1d377295b1/mcu/STM32F723RCVx.xml)、[REVx XML](https://github.com/STMicroelectronics/STM32_open_pin_data/blob/7d1f1514ed5583ec5007ad91236b4e1d377295b1/mcu/STM32F723REVx.xml)、[F72x AF 数据](https://github.com/STMicroelectronics/STM32_open_pin_data/blob/7d1f1514ed5583ec5007ad91236b4e1d377295b1/mcu/IP/GPIO-STM32F72x_gpio_v1_0_Modes.xml)、[准确 F723 CMSIS](https://github.com/STMicroelectronics/cmsis-device-f7/blob/2352e888e821aa0f4fe549bd5ea81d29c67a3222/Include/stm32f723xx.h)。

## 逐项处理与真实差异

每颗芯片均为 DIE452、Cortex-M7，封装 68 个位置、51 个 GPIO。RC 为 256 KiB flash，RE 为 512 KiB flash；各有 64 KiB DTCM、192 KiB SRAM。flash 分区和 erase/write 属性沿用相同容量 F723VC/VE 的固定数据，未由型号字母推算地址。

`comparison.json` 保存完整实例/version 对照、封装电源脚、GPIO/AF 增删、63 个保留外设的 CMSIS 地址、85 个 IRQ、102 个 RCC gate/reset 掩码。全部保留实例的非 pin 元数据与对应基线结构相等；RCC 掩码同时与原 `rcc_f7.json` 的 bit_offset/bit_size 核对。

R XML 与 V XML 不是完整 IP 集合相等：

- R XML 省略 FMC、LPTIM1、SPI4、UART7、UART8，补丁按原生成器的 XML 实例选择规则移除这五项。DS11853 Rev9 Table2 的 R 列也列出无 FMC/低功耗定时器、3 个 SPI、4 USART + 2 UART，但该旧表尚未涵盖新的 F723 68 引脚订货信息，因此以准确 R XML 为直接证据。
- R XML 错列 AES。准确 F723 CMSIS 没有 AES 实例或基地址，固定生成器本来也会跳过无法解析地址的 XML IP。补丁不从 F733 借 AES。
- 当前 F72x XML 给 F723 列出 12 个 ULPI 信号，但 ST DS11853 Rev9 第 80 页 Table10 脚注 4 明确说明 F723 不提供这些信号。第 45 页描述其内部 HS PHY。补丁过滤这 12 项，不生成相应 HAL pin trait。
- 固定 VC/VE JSON 多出 `PC1 / RTC_TS`，当前 ST XML 与 DS11853 Rev9 第 63 页 PC1 行均只有 TAMP3，没有 TS。补丁不把该基线 pin 带入 RC/RE。过滤 ULPI 后，参考封装重建与固定基线仅剩这一条经过显式审核的差异；任何其它差异都会中止生成。

来源：[DS11853 官方入口](https://www.st.com/resource/en/datasheet/stm32f723ie.pdf)、[ST 预告更正表](https://community.st.com/stm32-mcus-60/stm32-mcu-datasheets-expected-preliminary-updates-127810)。Rev9 原文可通过 `python xtask/scripts/fetch_source_documents.py` 恢复到被 Git 忽略的 `data/sources/stm32-gaps/stm32f723-rev9.pdf`。从先前调查记录找回的 [Chipdip 镜像](https://static.chipdip.ru/lib/694/DOC030694347.pdf) 已重新下载核对：6,678,029 字节，与 sources.json 固定 SHA-256 完全一致，页脚保留 Arrow 标记。ST 官方入口当前返回的同版 PDF 字节不同，不能替换固定证据。PDF 版权归 ST，源码仓库与独立项目仅附来源 URL、SHA 和恢复脚本，不打包全文；完整来源验证仍要求缓存存在且哈希正确。

## 电源与启动边界

准确 XML 给出 VCAP_1 位于 30、REXTPHYHS 位于 35、VDDPHYHS 位于 36、V12_PHYHS 位于 37；其余 VDD/VSS/VDDA/VSSA/VBAT 位置见 comparison.json。保留这些独立电源/模拟脚，不把它们算成 GPIO。PD3 是位置 58，可用 SPI2 SCK / I2S CK AF5；这是小封装真实 pin map 的一项检查点。

这里没有 H5 的 Q/SMPS package 选择。F7 HAL 的默认内部时钟和电压初始化所访问的寄存器与同 die F723 基线一致；正确供电、去耦、VCAP 与 HS PHY 外部电路仍必须由板级设计满足。正式 68 引脚供电图、额定工作条件和实际上电均未验证。尤其不能将 PAC/最小 ELF 通过解读为 USB HS 硬件已经可用；此补丁没有新增 USBPHYC 驱动或改动上游 F7 USB HAL。

## 重现与组合清单

从已包含 LI 补丁的 vendor 状态运行：

```powershell
. .\scripts\env.ps1
python xtask/scripts/prepare_f723_patch.py --materialize --write-manifest
python -m unittest discover -s xtask/scripts -p test_f723_patch.py -v
cargo check --manifest-path vendor/stm32-metapac/Cargo.toml --target thumbv7em-none-eabihf --features rt,metadata,stm32f723rc --target-dir target/f723-pac-check --locked --offline
cargo check --manifest-path vendor/stm32-metapac/Cargo.toml --target thumbv7em-none-eabihf --features rt,metadata,stm32f723re --target-dir target/f723-pac-check --locked --offline
```

原生成器由固定 ZIP SHA 校验后按原 Cargo.lock 构建；共享 register/peripheral Rust 模块与原 baseline 比较全部 token，只允许格式化差异。两个芯片目录和 `metadata_f723_r_*` 由真实生成器产生。

旧 `data/patches/stm32h5-li/vendor-manifest.json` 保持原样，作为 LI 阶段证据；不再把它当作组合 vendor 的当前清单。新组合清单位于 `data/patches/stm32-metapac/vendor-manifest.json`，schema_version 2：

- `vendor_files_sha256`：当前完整 vendor 文件集合。
- `baseline_chip_git_blobs` / `baseline_pac_git_blobs`：原固定 Git 对象的独立证明。
- `overlays`：LI 与 F723 的 manifest 路径和 SHA。
- `new_chip_jsons`：四个准确芯片输入，各带项目根相对 path、SHA、overlay id。

只更新组合所需的 Cargo.toml、all_chips.rs、NOTICE，并添加 F723 的七个 PAC/metadata 文件。其它已有 LI/baseline 文件、许可证均保持原 SHA。再次生成前先核对现有完整 inventory 和所有内容 SHA，拒绝吞掉未知本地修改。

不要用旧 LI 的 materialize 命令覆盖已经组合的 vendor；如从空 vendor 重建，先完整重现 LI，再执行 F723。HAL 依赖和全型号目录由根集成步骤接入，HAL/最小 ELF 的证据另记于框架构建报告。

## 本次验证结果

2026-09-30，Rust 1.98.1：

- 初次运行 6 个 artifact 检验全部失败，失败原因分别为准确 R JSON、独立 PAC feature 和组合清单尚不存在。
- 完成后 F723 的 11 个测试通过，包含准确 pin/容量、AES/ULPI 修正、CMSIS 地址/RCC/IRQ、未知 die/IP、源文件篡改、组合 inventory 多文件和内容篡改拒绝。
- 原 LI 的 8 个测试继续通过；所有非共享组合文件仍符合原 LI manifest SHA。
- 上述 RC、RE 两个 `cargo check --locked --offline` 均退出 0，开启 `rt` 与 `metadata`，目标为 `thumbv7em-none-eabihf`。
- 原固定生成器运行两次，10 个新增/修改文件的 overlay 清单完全相同。
- 完整组合清单包含 6869 个 vendor 文件、1616 个原始 chip Git blob 和四个本地 JSON 输入。overlay → sources/comparison 的 SHA 链核对通过。

这些结果不包含上板、烧录、USB PHY、电气或正式订货状态验证。

## 框架接入后的实际链接

根集成已执行 `cargo xtask build --chip stm32f723rc` 和 `stm32f723re`，两者均通过 compile/link/ELF 校验；独立报告在 `target/reports/<chip>/result.json`。

另有实际小封装引脚案例 `App/src/bin/f723_r_pins.rs`：SPI2 使用 PD3 SCK、PC3 MOSI、PC2 MISO，250 kHz。可在测试台把 PC3 接到 PC2 做回环；本次只完成链接和严格 Clippy，未连接实物。

```text
cargo build -p embodied-app --bin f723-r-pin-check --release --locked --target thumbv7em-none-eabihf --features stm32f723rc,f723-r-pin-check
cargo clippy -p embodied-app --bin f723-r-pin-check --locked --target thumbv7em-none-eabihf --features stm32f723rc,f723-r-pin-check -- -D warnings
```

RE 将 feature 替换为 `stm32f723re`。两个 release ELF 已分别保存在 `target/f723-r-pins/<chip>/firmware.elf`，RC SHA-256 为 `9b205ed76f1461f5eff0316bfa1c12d90e7853ebae5567d8580bb9aaac85ab7f`，RE 为 `8da7e001a5ff348ebd57f3066a16c4d7feebc7ee12cde930c1325df5e128435f`。CI 的 `package-pins` job 会逐芯片重复这两个检查。
