# 2. App 开发指南

<!-- handbook-nav -->
[手册目录](README.md) · [English](../en/application-guide.md) · [项目首页](../../README.md)

先编译并运行最小心跳，确认工具和调试连接可用，再添加自己的外设和任务。心跳不需要额外连接 LED 或串口。开始前请完成[工具链安装](toolchain.md)。本章命令默认在仓库根目录、已能使用 Rust 的终端执行；`text` 代码块适用于 PowerShell、Git Bash 和 Linux Bash。

## 获取并确认项目完整性

使用团队提供的 Git 仓库地址克隆，或解压源码包，然后进入包含根 `Cargo.toml` 的目录。`Cargo.toml` 是 Cargo 构建工具读取的项目配置文件。

保留 `App`、`crates`、`vendor`、`data`、锁文件和配置。仅复制 App 会丢失框架和本地补丁。需要可以单独维护的新项目时，使用本章的工程生成器。

## 确定真实器件与构建配置

先从原理图和 MCU 丝印确定完整料号、封装和内核，再查询：

```text
cargo xtask list-chips --chip stm32h723vg
cargo xtask list-chips --family G4
```

输出列出对应的 target（编译目标）和后端（该芯片使用的底层驱动）。当前可选择 895 个型号、921 个芯片/内核配置；G411/G414、H7R/S、H543/H553 共 57 个型号已排除。芯片目录中仍保留这些型号的原始数据，实际可用范围以[支持策略](../../data/support-policy.json)为准。

Cargo 用 feature 选择编译配置，芯片型号就是其中一种开关。一个固件只选一个芯片/内核。H7 双核的 `-cm7` 和 `-cm4` 对应两份独立固件，需要分别编译。

## 第一个固件：H723 心跳

```text
cargo build -p embodied-app --bin minimal --release --locked --target thumbv7em-none-eabihf --features stm32h723vg
```

这条命令为 STM32H723VG 构建最小固件；换芯片时先查询对应配置。编译成功后生成 `target/thumbv7em-none-eabihf/release/minimal`，这是包含程序和调试信息的 ELF 文件。

入口是 [minimal.rs](../../App/src/bin/minimal.rs)。它用 Context 结构体暂存初始化得到的资源，并让初始化注册表按顺序执行各阶段。在 Env（硬件环境初始化阶段）调用 HAL，也就是芯片驱动，启用默认内部时钟。全部阶段成功后会得到 Ready，再由它启动心跳任务，每秒输出一次计数和错过的周期数。

这个示例不需要外部晶振、LED 或串口连接。日志使用 RTT，通过调试探针读取；烧录和读取步骤见[工具链章节](toolchain.md)。首次构建需要下载依赖，可能较慢。生成 ELF 后再烧录到匹配的芯片，看到每秒一次的心跳，才能确认程序已在板上运行。

## chip feature 与 Flash bank

Flash bank 是芯片的闪存分区方式，构建配置必须与芯片的选项字节（option bytes）一致。例如，实际 G474RE 硬件使用 dual-bank（双区）布局时，构建通用最小固件：

```text
cargo build -p embodied-app --bin minimal --release --locked --target thumbv7em-none-eabihf --features stm32g474re,dual-bank
```

若芯片 option bytes 选择 single-bank（单区），构建时也要选择相应布局。feature 只改变链接和驱动配置，不会修改芯片的 option bytes。`xtask` 是仓库自带的命令工具，可以根据芯片查出 target；对可配置布局的器件，它默认选择 dual-bank：

```text
cargo xtask build --chip stm32g474re --bank dual-bank
```

命令完成后会输出构建结果和 ELF 检查记录，按终端给出的路径查找固件。xtask 使用的构建目录与直接运行 Cargo 时不同。工具根据项目的内存数据判断芯片支持哪些 bank 布局。

## App 中的代码放在哪里

```text
App/
  Cargo.toml             binaries, features and application dependencies
  src/bin/               firmware entry points
  src/lib.rs             application-level shared modules
  src/tasks/             application task implementations
  src/boards/            Rust clock, IRQ, pin and resource configuration
```

`src/bin/` 现有最小心跳和参考板两个固件入口。`src/tasks/` 目前只有说明文件；新增 Rust 模块后，需要在父模块中声明，并从入口使用，才会参与编译和运行。板级初始化放在 `src/boards/`，接线说明见[参考板文档](../reference-boards.md)。

应用行为留在 App；可复用协议放 devices，纯计算放 algorithms。框架不依赖 App。

新增固件入口时，应在 App 的 Cargo 配置文件中声明 `[[bin]]`、入口路径和适当的 `required-features`，并像现有固件一样设置 `test = false`、`bench = false`。不要修改自动生成的芯片 feature 表来解决业务需求。

## 新增任务的步骤

1. 参考现有 `heartbeat` 定义新任务：用 `#[embassy_executor::task]` 标记一个 async 函数，并放入应用模块。async 函数可以在等待定时器或外设时让其他就绪任务运行。
2. 确定任务拥有哪些资源：例如 UART 的接收端只交给接收任务，发送端交给发送任务。
3. 在初始化阶段创建外设并保存在 Context。启动时使用 `Option::take` 取出外设交给任务，原位置随即变为空，避免同一外设被取走两次。
4. 通过 `ready.start(...)` 启动任务，并处理 spawn（提交任务给执行器）失败和运行错误。示例中的 `unwrap` 会在遇到错误时触发 panic；正式应用应规定初始化失败后如何记录原因、停止相关功能。
5. 在循环中安排等待点。长时间计算会拖延其他任务；`.await` 只有在所等操作尚未完成、返回 Pending 时才会让出执行权。

任务间传递数据优先使用固定容量消息队列。多个任务共享一个外设时，要规定访问顺序；异步互斥锁可以让它们依次使用外设。HAL 的外设 token 表示使用该外设的唯一凭据，不要复制它，也不要在多个任务中重复初始化同一外设。细节见[最佳实践](best-practices.md)。

## 使用框架模块

App 通常通过 `embodied_framework::{core, runtime, algorithms, devices, stm32}` 导入功能。`stm32` 由 feature 启用；通用算法和协议可在主机上测试。

| 应用需要 | 入口 | 应用仍需决定 |
|---|---|---|
| 稳定周期 | `core::time::Periodic`、`runtime::embassy::next_tick` | 周期、错过截止时间后的业务处理 |
| 任务间消息 | `runtime::queue::SharedQueue` | 容量、超时和满队列处理 |
| 可复用消息存储 | `runtime::pool::MessagePool` | 类型、对象数量及生命周期 |
| CAN 发送/接收 | `runtime::can` + `stm32::can` | 总线速率、过滤、恢复和协议分发 |
| IMU | `devices::imu` + `stm32::bus` | 型号、量程、SPI 模式、校准和总线共享 |
| 控制/滤波 | `algorithms` | 采样单位、参数、限幅、异常输入策略 |

无效报文不能刷新连接状态；明确报文校验成功后再调用连接监测。电机协议负责编码和解析，不自动知道机械限位或动作目标。双臂裁判消息使用显式 `DoubleArm14` 配置，不按负载长度猜测版本。

## 使用参考板

下面五个 board feature 选择仓库中的板级配置，包含该板的芯片、时钟和外设资源：

| feature | 芯片 | 说明 |
|---|---|---|
| `board-stm32h723vg` | H723VG | 参考资源配置 |
| `board-stm32g431kb` | G431KB | 参考资源配置 |
| `board-stm32g474ce` | G474CE | 参考资源配置，包含 dual-bank |
| `board-stm32f407ig` | F407IG | 参考资源配置 |
| `board-stm32f427ii` | F427II | 参考资源配置，明确使用 TIM2 时间基准 |

例如 G474 参考板固件：

```text
cargo build -p embodied-app --bin reference-board --release --locked --target thumbv7em-none-eabihf --features board-stm32g474ce
```

编译成功后生成 `target/thumbv7em-none-eabihf/release/reference-board`。该配置对应 G474CE，要求板上有 12 MHz HSE（外部高速时钟）。前面通用最小固件使用的 G474RE 是另一个型号，不能直接套用这份板级配置。参考固件初始化并保留资源，周期打印 idle 状态；机器人控制任务需要自行添加。详细接线与限制见[板级章节](board-configuration.md)。

## 生成独立应用工程

在仓库根目录运行；父目录必须存在，目标目录必须不存在：

```text
cargo xtask new --chip stm32g474re --path ../robot-demo --name robot-app
```

生成器会复制 App、框架和 vendor 源码，并加入锁文件、编辑器配置、CI 自动检查流程及 `project.json` 来源清单。新工程包含自己的源码副本，不依赖原仓库的绝对路径。以后升级框架时，需要检查新旧源码的差异。

然后进入生成目录，执行：

```text
cd ../robot-demo
cargo app-build
cargo app-check
cargo clippy -p robot-app --bin firmware --locked --target thumbv7em-none-eabihf -- -D warnings
```

预期 ELF 为生成工程下的 `target/thumbv7em-none-eabihf/release/firmware`。生成器默认内部时钟和 RTT 心跳；根据真实板卡补充配置。名字首字符必须为小写字母，其余使用小写字母、数字、下划线或连字符；已有目录不会被覆盖。长度和保留名称限制见[工程生成器说明](../project-generator.md)。

生成双核工程时，要明确指定 H7 的内核，再按[双核说明](../dual-core.md)分别构建两份固件并检查共享内存区域。

## 日常迭代顺序

修改纯算法或解析器后，先在电脑上运行编译和静态检查，再构建目标芯片的 App。改变引脚、时钟或 DMA 后，先核对板级资料和 ELF，再测试对应硬件。评审时一起检查锁文件、board feature 和 Rust 配置，并记录准确 MCU、bank、探针，以及已完成的编译检查和上板测试。

---

[上一章](toolchain.md) · [目录](README.md) · [下一章](board-configuration.md)
