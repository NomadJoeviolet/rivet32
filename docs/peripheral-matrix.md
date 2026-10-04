# 外设构建工具 / Peripheral build tools

[中文 CI 指南](zh-CN/ci-cd.md) · [English CI guide](en/ci-cd.md)

需要检查某个型号的外设驱动是否能构建，或批量检查多个型号时，使用 `cargo xtask peripherals`。它从仓库固定版本的 PAC（芯片寄存器定义包）中读取准确型号、内核和封装，选取可用资源，为 GPIO、UART、CAN/FDCAN、USB CDC 虚拟串口、SPI、PWM 和定时器分别生成固件，再检查链接结果。它补充了最小心跳固件的批量构建检查，不会烧录设备；通信和时序仍需上板测试。

## 工具与命令

按 [工具链说明](zh-CN/toolchain.md) 安装 Rust 1.98.1、对应型号的 ARM 编译目标、用于电脑端工具的链接器、Git 和 Python 3.11+。Windows 使用仓库内工具链时，先运行 `. .\scripts\env.ps1`。首次运行先下载指定版本的依赖和来源资料；外设构建工具运行时使用离线模式和独立缓存：

```text
cargo fetch --locked
python xtask/scripts/fetch_source_documents.py
cargo xtask peripherals --chip stm32f103c8
cargo xtask peripherals --family F0 --limit 2
cargo xtask peripherals --shard 0/32
```

`--chip` 后填写芯片目录中的完整 feature 名称，不能同时使用 `--family` 或 `--shard`。`--family` 可写 `F0` 或 `STM32F0`。分片用于把批量构建拆成多份：工具先按家族筛选、按 feature 排序，再按从零开始的 `index/count` 分配，最后应用 `--limit` 数量限制。缺少芯片实现的条目也会保留并报错。不传选择参数时，工具会处理整个支持目录。

只想查看将使用的外设实例、引脚及其复用功能、DMA 和中断时，加上 `--plan-only`。这个模式不编译固件：

```text
cargo xtask peripherals --chip stm32f103c8 --plan-only
```

默认选择芯片 JSON 中按名称排序的第一个实际封装。也可用 `--package` 指定 JSON 中的准确 ST 封装订货模式名称，但必须同时指定 `--chip`。报告会记录所选名称和封装。某个封装检查通过后，其他封装仍需分别检查。用 `--output` 设置报告、临时项目和缓存的保存目录。

## 检查范围

每类外设使用单独的测试固件，因此小容量 MCU 无需同时容纳所有功能。生成固件时，工具会检查封装是否引出所需引脚、AF（引脚复用功能）或 F1 重映射配置是否匹配、UART 的 DMA 通道是否冲突，以及是否有对应中断绑定。它也会避开 Embassy 时间驱动已经使用的定时器。

生成的代码会调用框架适配接口和 HAL（硬件抽象层）的外设初始化函数。UART 使用 DMA，SPI 使用总线与片选适配器，CAN 使用对应控制器和框架帧适配器，USB 创建 CDC ACM 虚拟串口，PWM 创建通道适配器，定时器创建框架的比较事件驱动。链接后检查 ARM ELF 固件文件、向量表、内存区间，以及初始化函数是否实际进入了固件。报告会保留 ELF/MAP 文件、链接布局、依赖锁文件、执行命令和来源文件的 SHA 校验值；MAP 文件用于查看链接后的内存分布。

双核 H7 分别生成两核固件，CM7/CM4 分别为时间驱动保留 TIM5/TIM2，并检查各核私有内存和共享的 NOLOAD 区域（烧录时不写入初始内容）。每个生成配置都在 `board_requirements` 中记录板卡要求。晶振、供电及缓存设置必须与实际板卡核对，不能直接套用。

型号选择都按 `data/support-policy.json` 中的支持范围执行。缺少芯片实现或外设初始化方案时会报错。要确认当前源码能否构建，需查看本次运行结果。

## 报告

默认输出为 `target/peripheral-probes/`：

- `reports/<feature>/result.json`：该型号七类外设的检查状态和依据。
- `reports/<feature>/<category>/`：具体构建日志、ELF/MAP 和检查结果。
- `projects/<feature>/`：实际编译的 Cargo 项目、锁文件和源代码。
- `source-snapshots/`：构建使用的 HAL、PAC、补丁来源、框架、检查器和完整 `xtask/scripts` 的源文件清单及 SHA 校验值。
- `summary-*.json`：整批状态、使用的来源记录和各子报告的 SHA 校验值；分片号写入文件名。

工具在读取芯片目录前记录整批构建使用的文件及校验值，之后不会为下一个型号重新记录。芯片目录和补丁清单都从已校验的文件内容中读取；预检后、每个型号构建前后及整批结束时，都会与最初记录比较。若源码、芯片目录或来源资料发生变化，该批选中型号的报告都会标为 `failed`，命令以非零退出码结束，已链接的 ELF 会保留以便排查。需要更新这些文件时，先结束当前批次，更新后重新运行。

| 类别状态 | 含义 |
|---|---|
| `planned` | 已选好资源并生成初始化方案，尚未构建 |
| `constructor-linked` | 外设初始化函数已链接到固件，并通过产物检查 |
| `not-present` | 该芯片、该内核的数据中没有此硬件控制器 |
| `package-unavailable` | 控制器存在，但所选封装没有初始化方案所需的引脚 |
| `unsupported` | 硬件存在，但当前缺少驱动或初始化方案 |
| `blocked` | 缺少准确型号的芯片实现、双核方案，或存在未解决的运行问题 |
| `failed` | 构建、链接、产物或来源校验失败 |

七类外设必须全部为 `constructor-linked`，或有依据说明硬件不存在的 `not-present`，整次检查才会记录 `constructor-matrix-passed`。其他情况会保留问题报告，并以非零退出码结束。`hil` 表示实板测试，始终记录为 `not-run`，需要另外上板验证。

共享缓存使用进程锁，避免多个构建同时修改。`EMBODIED_CACHE_LIMIT_MIB` 默认 2048；超过限制后，工具会在下一次构建前持锁清理已有产物副本的 Cargo 输出，保留报告和锁。详见 [缓存规则](zh-CN/ci-cd.md)。如果构建中断后留下锁文件，确认相关 Cargo/rustc 子进程全部结束后才能清理。

全型号工作流的 `all-peripherals` 作业分为 32 片，最多同时执行 8 片。每片上传报告、源工程和产物，不上传编译缓存。是否已在远端运行，需要查看 GitHub Actions 的实际执行记录。
