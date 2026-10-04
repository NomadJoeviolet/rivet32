# 生成独立 App 工程

<!-- bilingual-handbook -->
开发手册 / Developer handbook: [中文](zh-CN/README.md) · [English](en/README.md)。

需要在框架仓库之外开发自己的应用时，用本页命令生成独立工程。生成器只接受[当前支持范围](support-scope.md)内的 921 个配置。指定 G411/G414、H7R/S 或 H543/H553 时，会在创建目录前报错并说明原因。已生成的工程包含生成时的源码副本，不会随框架仓库自动更新。

在框架根目录执行（Windows 局部工具链先执行 `. .\scripts\env.ps1`）：

```text
cargo xtask new --chip stm32g474re --path ../robot-demo --name robot-app
```

目标的父目录必须存在，`robot-demo` 必须尚不存在。生成器不会覆盖已有目录，不提供 `--force`。`--name` 默认 `robot-app`，长度须为 1 至 64 字节；只能使用小写英文字母、数字、下划线和连字符，首字符必须是小写英文字母。名称 xtask 和以 embodied- 开头的名称由框架保留，不能使用。

生成的项目包括：

| 路径 | 用途 |
|---|---|
| `App/src/main.rs` | 按九个阶段执行初始化，全部成功后启动应用 |
| `App/src/tasks/mod.rs` | 每秒通过 RTT 向调试探针输出心跳日志；用户任务写在此处 |
| `App/src/boards/mod.rs` | 内部时钟最小初始化；添加实际板级引脚、时钟与外设资源 |
| `Framework/crates/` | 生成时的框架源码快照，保留原许可证 |
| `Cargo.lock`、`rust-toolchain.toml` | 锁定依赖及 Rust 1.98.1 |
| `.cargo/config.toml` | `app-build`、`app-check` 别名及链接参数 |
| `.vscode/` | Rust 编辑器扩展 rust-analyzer 的目标芯片配置 |
| `.github/workflows/ci.yml` | 自动检查格式、链接固件、执行 Clippy 静态检查，并上传 ELF 固件文件 |
| `project.json` | 记录芯片、Flash bank 分区、源码文件校验值、依赖锁文件状态，以及尚未实机验证的标记 |

框架源码随生成项目一起提交，构建时不需要原框架目录。仓库内 `vendor/` 补丁也会复制到新项目，并改为项目内的相对路径。来源清单保留公开资料的下载地址与 SHA-256 校验值；PDF 全文是可以重新下载的本地缓存，不随项目打包。生成项目使用 `* -text` 禁止 Git 自动转换换行符，以保留清单记录的原始文件字节。升级框架时，需要检查源码变更并重新验证项目。

```text
cd ../robot-demo
cargo app-build
cargo app-check
cargo clippy -p robot-app --bin firmware --locked --target thumbv7em-none-eabihf -- -D warnings
```

输出为 `target/thumbv7em-none-eabihf/release/firmware`，这是没有扩展名的 ELF 固件文件。固件通过 Rust HAL（硬件抽象层）使用默认内部时钟，并通过 RTT 向调试探针输出心跳日志，不需要连接 LED 或 UART。连接探针时，应填写实际芯片的完整料号。

生成器以框架的 `Cargo.lock` 为基础，通过 `cargo metadata --offline` 为新项目解析依赖，并保留已锁定的兼容版本。若本机缺少依赖缓存，生成会报错，并保留已生成的源码和 `generation.log`。此时先在源框架执行 `cargo fetch --locked`，再按错误提示完成目标项目的依赖锁定。生成文件后仍需执行构建命令，才能确认固件是否编译通过。

Flash 支持单 bank 或双 bank 配置的器件默认使用 `dual-bank`，也可选择 `--bank single-bank`。芯片 option bytes（选项字节）中的实际配置必须与之相同；Flash 布局固定的器件不能传入 `--bank`，否则会报错。在生成项目的 Rust 板级模块中配置实际时钟、引脚、DMA 与中断。

缺少对应 HAL 实现的器件会报错。使用外设前，还需核对所选封装是否引出相关引脚，并查看对应构建和实机验证记录。

## H7 双核成对项目

指定芯片目录中 H745/H747/H755/H757 的准确 `-cm7` 或 `-cm4` feature（编译开关），生成器会找到同一芯片的另一内核，生成两份独立 App。若缺少另一核的配置，会报错，不会改用相近型号。例如：

```text
cargo xtask new --chip stm32h745bg-cm4 --path ../dual-demo --name dual-app
cd ../dual-demo
cargo fetch --locked
cargo app-build
cargo app-check
```

`App/cm7/src/` 和 `App/cm4/src/` 分别存放各核应用，`App/shared/dual_core.rs` 处理两核共用的 HAL 启动代码。两核各自固定芯片依赖，每次 Cargo 只能构建其中一个包，不要运行 `cargo build --workspace`。`cargo app-build` 通过在电脑上运行的 Rust 启动程序调用 Python，依次构建两核，并使用独立的 `target/pair-cache/cm7`、`cm4` 缓存。产物与报告保存在 `target/pair/`。每次链接使用独立的 MAP 文件路径，避免重复构建时读取旧的内存布局报告。`cargo app-check` 只检查编译，不更新链接结果报告。

使用 Python 3.11+，与仓库开发环境保持一致。Windows 默认调用 `python`，Linux 调用 `python3`，也可把 `PYTHON` 设为解释器完整路径。单核命令 `cargo cm7-build` / `cargo cm4-build` 不比较两核的 ELF。生成器为每核提供独立 VS Code 工作区，使 rust-analyzer 只加载和检查选中的核，避免 Cargo 同时启用两组互斥的芯片 feature。

M7 使用第一 Flash bank、AXISRAM、TIM5；M4 使用第二 Flash bank、SRAM1、TIM2。容量取自准确型号的 PAC（芯片寄存器定义包）。共享状态位于 `0x38000000`，该区域标为 NOLOAD，烧录时不写入初始内容。默认使用内部 HSI 时钟和 LDO 供电模式，M7 DCache（数据缓存）关闭。两份固件必须在完整系统复位后，从指定启动地址运行；供电与复位条件见 [双核说明](dual-core.md)。初始化不会把两套代表外设使用权的 token 交给用户任务，新增引脚、DMA 或外设时必须指定唯一负责的内核。此工程不提供跨核任务调度（SMP）或跨核队列。

双核清单采用 `schema_version: 2`、`topology: dual-core-pair`，记录准确芯片型号和两份固件。校验器会检查两核标识、应用清单、时间资源和文件校验值。单核清单使用 schema 1。每次构建都应查看本次生成的报告。

## 来源与验证

生成完成后，可在源框架根目录检查未修改的项目基线：

```text
python scripts/verify_generated_project.py ../robot-demo
```

该命令检查每个文件的 SHA 校验值、按统一规则整理后的 Cargo.lock，以及完整文件清单。缺少文件、修改文件或新增参与构建的文件都会导致检查失败。它用于确认刚生成的工程；开始修改 App 后，文件自然会与生成时的记录不同。CI 在构建前把 `project.json` 复制到独立临时目录，构建后通过 `--baseline <副本路径>` 对照这份副本，避免源文件和清单同时变化后漏检。测试日志应放在项目外。项目内允许生成的运行文件仅限 `target/`、Git 元数据、Python 缓存、`.build/` 探针缓存、`generation.log` 和恢复的 `data/sources/` 资料缓存；生成器复制工程时也会排除这些缓存。

需要查阅补丁依据的数据手册时，在生成项目中执行 `python xtask/scripts/fetch_source_documents.py`。脚本先验证来源记录，再下载并核对完整 SHA 校验值；校验失败时保留已有缓存。普通固件编译不需要这些 PDF。
