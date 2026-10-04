# 3. Rust 板级配置

<!-- handbook-nav -->
[手册目录](README.md) · [English](../en/board-configuration.md) · [项目首页](../../README.md)

板级配置描述这块板使用的芯片、时钟、引脚和外设。本项目把这些配置写在 App 的 Rust 代码中，由 `embassy-stm32` 驱动初始化 MCU。修改前先准备对应板版本的原理图、芯片数据手册和参考手册。

## 从规划到实际寄存器

```mermaid
flowchart LR
    S[Schematic and device manuals] --> R[Rust board configuration]
    R --> H[embassy-stm32 HAL]
    H --> P[stm32-metapac]
    P --> M[MCU registers]
```

| 层级 | 作用 | 本项目怎样使用 |
|---|---|---|
| STM32 C HAL | ST 的 C 驱动库 | 标准 Rust 固件不链接这套驱动 |
| `embedded-hal` | Rust 设备和总线的通用接口，用 trait 定义 | 让设备驱动可用于不同 MCU；它不属于 Embassy，也不会初始化芯片 |
| `embassy-stm32` | 具体 STM32 Rust HAL | 访问硬件，配置时钟、GPIO、DMA、外设和中断配合 |
| `stm32-metapac` | 类型化寄存器访问与芯片元数据 | 为 HAL 提供准确型号的底层定义 |
| `embassy-executor` | 调度异步任务 | 中断或定时事件唤醒任务后，检查任务能否继续执行 |

HAL 是操作硬件的驱动层，应用通过构造器创建外设，再调用方法读写。PAC 是寄存器访问库；直接使用它时，需要自行处理寄存器状态、外设独占和中断同步。trait 是 Rust 用来约定类型应提供哪些操作的接口。

应用从 `embodied_framework::stm32::hal` 访问驱动，这里导出仓库固定版本的 Embassy HAL。

## 一张板的配置清单

开始前记录完整芯片料号、封装和板版本，以及供电方式、HSE/LSE 类型与频率、Flash bank/TrustZone 状态、调试接口和外设接线。同名开发板的不同版本也可能需要不同配置。

直接配置 Rust 板卡的步骤：

1. 根据原理图与数据手册确认准确芯片、封装和引脚，在 App feature 中选择对应型号。
2. 为每个外设记录实例、TX/RX 或 SCK/MOSI/MISO、CS/使能脚、GPIO 初始状态和上拉下拉。
3. 核对时钟树：系统时钟、总线分频、外设工作时钟（kernel clock）、USB 48 MHz 来源，以及相关电源和电压配置。
4. 核对中断（IRQ）和 DMA 请求、通道，检查它们是否与时间服务、PWM 或硬件比较事件占用同一资源。
5. 在 Rust `clock_config()` 中填写 `hal::Config`；在 Env（硬件环境初始化阶段）调用一次 `hal::init`，在 Device（设备初始化阶段）创建外设。
6. 将外设所有权分配给应用任务，再依次验证构建、接线和实际传输。

`p.PA2` 这样的值称为 token，代表使用 PA2 引脚的唯一凭据。把它传给外设构造器后，这个外设就持有该引脚。编译器会根据构造器的 trait 约束检查引脚复用是否受支持；焊接、外部电路、电压和板版本仍需要对照实物确认。

## 参考 G474 板的具体映射

[STM32G474CE 配置](../../App/src/boards/stm32g474ce.rs)对应 `STM32G474CET6`，其时钟定义在 [common.rs](../../App/src/boards/common.rs)。12 MHz HSE 经 PLL 的 `/3 ×85 /2` 得到 170 MHz SYSCLK；USB 单独选择 HSI48。这里的频率属于该板配置，不是所有 G4 的默认值。

| 功能 | 该配置使用的资源 |
|---|---|
| FDCAN1 / 2 / 3 | PB8/PB9、PB5/PB6、PA8/PB4；默认内部回环 |
| USART1 RX | PA10、DMA1_CH2，2,000,000 baud |
| USART2 RX | PA3、DMA1_CH3，921,600 baud |
| IMU SPI2 | PB13/PB15/PB14；PB12、PB11 两个 CS；5,312,500 Hz |
| 编码器 SPI1 | PA5/PA7/PA6；PB1、PB2 两个 CS；10,625,000 Hz |
| 舵机 PWM | TIM3 CH2 / PA4，50 Hz；默认输出关闭 |
| USB | PA12/PA11，USB_LP IRQ |

使用这些资源前，逐项核对自己 PCB 的接线、供电和晶振。参考固件只完成资源初始化，传感器校准和控制任务需要在应用中补充。

## GPIO、IRQ 和 DMA 应怎样阅读

以下是 GPIO 构造方式的局部示意，`p` 来自一次 HAL 初始化；使用前导入 HAL 的 `gpio::{Output, Level, Speed}`，并确认板上 PB0 可用：

```rust
Output::new(p.PB0, Level::Low, Speed::Low)
```

PB0 创建后为低电平。UART/SPI/CAN 的构造还需要外设实例、引脚、DMA，以及通过 `bind_interrupts!` 指定的中断处理关系。复用一个 UART 构造器时，一起检查 TX/RX 引脚、时钟、DMA 和中断绑定。

DMA 缓冲区要在传输期间一直有效，所在内存也必须能被该 DMA 控制器访问。`'static` 只说明存活时间，不能证明 DMA 能访问这个地址。H723 的 DMA1/2 需要使用可达的 SRAM；如果启用了数据缓存 DCache，还要通过缓存维护或内存保护单元 MPU 的配置，保证 CPU 和 DMA 看到一致的数据。不同 H7 型号的内存布局需要分别核对。

## 时间资源与特殊器件

- Embassy 的 time driver（时间驱动）会占用一个定时器，这个定时器不能再分配给 PWM 或硬件比较事件。F427 参考板选择 TIM2，其他系列需要查看各自配置。
- H7 双核需要分别分配外设、IRQ、Flash/RAM 和时间资源。当前配置中，CM7 使用 TIM5，CM4 使用 TIM2。详见[双核说明](../dual-core.md)。
- H5 当前默认配置关闭 TrustZone；没有提供任意安全域/非安全域切分的完整产品模板。
- 小容量芯片只启用需要的功能。支持一种 MCU，不等于它有 CAN/USB，也不等于能同时容纳所有模块。

## 新增板卡的最小工作

在 App 中保存板版本和引脚说明，再参照现有 board 模块编写 `clock_config()`。用 `Resources` 结构体保存创建好的外设，字段类型使用 HAL 对应的具体类型。

随后在应用的 Cargo 配置文件中添加 board feature，指定准确芯片、bank 布局和时间驱动，并在板选择模块中加入这块板。使用该 feature 构建参考入口或自己的入口，一次只选择一块板。

如果框架已有该芯片的底层支持，新增板卡通常只需修改 App。新增尚未支持的芯片则需要检查 PAC、内存、中断、时钟和封装数据；使用相近芯片的别名无法保证这些配置正确。

固件通过 `cortex-m-rt` 启动，由 Rust HAL 统一初始化时钟、DMA 和 IRQ。每个硬件资源只交给一个所有者，避免重复初始化。

---

[上一章](application-guide.md) · [目录](README.md) · [下一章](architecture.md)
