# 五类参考板的接线与默认配置

[中文配置指南](zh-CN/board-configuration.md) · [English configuration guide](en/board-configuration.md)

使用下表中的参考板，或准备把它的配置改到自己的 PCB 上时，先核对本页的时钟和接线。`reference-board` 示例根据所选 `board-*` feature（编译开关）初始化对应外设。板级代码位于 `App/src/boards/`，使用框架的 STM32 CAN/UART 适配器和仓库固定版本的 Embassy HAL（硬件抽象层）。每次只启用一个板卡 feature。

## 配置来源

参考设计来自 BSP（板级支持包）`0e106801ec26a6db0588376d59817116ca37deea`。实际硬件初始化以 [`App/src/boards/`](../App/src/boards/) 中的 Rust 模块为准；修改板级代码时，应同步更新本页。下表 HSE 是外部高速时钟，SYSCLK 是系统时钟。

| Feature | MCU / 封装 | HSE / SYSCLK |
|---|---|---|
| `board-stm32h723vg` | STM32H723VGT6 / LQFP100 | 12MHz / 540MHz |
| `board-stm32g431kb` | STM32G431KBU6 / UFQFPN32 | 12MHz / 170MHz |
| `board-stm32g474ce` | STM32G474CET6 / LQFP48，dual-bank | 12MHz / 170MHz |
| `board-stm32f407ig` | STM32F407IGH6 / UFBGA176 | 12MHz / 168MHz |
| `board-stm32f427ii` | STM32F427IIH6 / UFBGA176 | 12MHz / 168MHz |

板卡名称使用 MCU 型号，每个模块都有固定的参考接线和时钟配置。即使 PCB 使用相同 MCU，也要核对原理图、电源和晶振，再修改 Rust 配置。

这些 HSE 均使用外部晶振模式。F407 参考板的型号是 F407IG，不能按 F407VG 选择；F427 参考板的型号是 F427II，不能按 F427IG 选择。构建时会使用包含封装和 Flash 容量的完整型号。

## 初始化和默认行为

[reference_board.rs](../App/src/bin/reference_board.rs) 使用 `InitRegistry` 按九个阶段执行初始化：PreCore 检查 H723 的 DCache（数据缓存），Env 配置板级 RCC（时钟控制），Device 创建板级 `Resources`。Ready 后，程序保留这些资源，并通过 RTT 向调试探针每秒输出一次心跳日志。IRQ（中断）、DMA、SPI、CAN、PWM、USB 都会调用 HAL 初始化函数，并参与编译和链接。

所有 PWM 通道默认关闭。H723/G4 的 FDCAN 使用内部回环，F4 bxCAN 同时使用静默和回环模式，因此不会向电机总线发帧。UART 只初始化接收侧，TX 引脚保持输入；SPI 不主动传输，片选默认高电平。USB 只创建底层驱动，尚未创建或运行 `UsbDevice`；产品的 VID/PID、CDC 虚拟串口协议和连接时机需要由应用配置。G431 板没有 USB 实例，因为 PA11/PA12 已用于 CAN。

来源中已记录的串口帧格式保持原配置；其他已知波特率的串口使用 8N1（8 位数据、无校验、1 位停止位）。F407 USART1、F427 UART7 的波特率未记录，代码先保留对应外设、引脚和 DMA 资源。调用者提供 `usart::Config` 后，可通过 `into_receiver` 初始化接收器。

此入口用于正常上电复位。若从 bootloader（引导程序）跳转而来，需要另外处理它已经启动的外设。构建与验证脚本都不会烧录硬件。

## 时钟与时间资源

| 家族 | PLL 与总线 | 外设内核时钟来源 |
|---|---|---|
| H723 | PLL1: 12 / 1 × 45，P /1、Q/R /3；AHB /2 = 270MHz，四组 APB /2 = 135MHz；LDO、VOS0 | PLL2: 12 /1 ×20，P/Q /3 = 80MHz、R /2 =120MHz；FDCAN=PLL2Q，ADC=PLL2P。PLL3: 12 /1 ×27，P /3=108MHz；SPI123=PLL3P。USB=HSI48 |
| G431 | 12 /3 ×85 /2 =170MHz，AHB/APB 不分频，boost 开 | PLLQ /2 =170MHz；FDCAN=PCLK1 |
| G474 | 同 G431 CPU；PLLQ /4 =85MHz | FDCAN=PCLK1；USB=HSI48，不能使用 85MHz PLLQ |
| F407/F427 | 12 /6 ×168 /2 =168MHz，APB1 /4 =42MHz、APB2 /2 =84MHz | PLLQ /7 =48MHz供 USB；CAN=PCLK1 |

H723 保留来源配置中的 540MHz。仓库固定版本 HAL 的 `rcc/h.rs` 会根据 `SYSCFG.UR18.cpu_freq_boost` 检查最高频率；如果芯片选项位未允许该频率，初始化会拒绝这项配置。固件不会修改 option bytes（选项字节）。HAL 按实际输入频率选择合法范围。

F427 的 TIM5 用于电机 PWM，因此板卡 feature 明确选择 `time-driver-tim2`，将 TIM5 留给 PH11/PH12/PI0，避免 `time-driver-any` 自动选中它。HAL 的 `any` 只在没有明确选择时生效；指定一个 TIMx 时优先使用该定时器，同时指定多个仍会报错。其他参考板由 HAL 自动选择时间基准。G474 的 TIM2 没有已确认的 PWM 周期，因此未作为 PWM 使用。

## 引脚与已实例化资源

下表 UART 引脚按 RX / TX 排列，TX 目前保持输入。SPI 引脚按 SCK / MOSI / MISO 排列；CS 为片选，ACC 为加速度计，GYRO 为陀螺仪。IMU 是惯性测量单元。DMA 的 S 表示 Stream，Ch 表示 Channel。其他 GPIO 和完整 IRQ 名称见对应 Rust 板级模块。

| 板 | CAN | UART 接收与 DMA | SPI / IMU |
|---|---|---|---|
| H723 | FDCAN1 PD0/PD1；2 PB5/PB6；3 PD12/PD13 | USART3 PD9/PD8 921600，DMA1 S3；UART7 PE7/PE8 921600，S1；UART8 PE0/PE1 115200，S5；USART10 PE2/PE3 115200，S6 | SPI2 PB13/PC1/PC2_C，ACC CS PC0、GYRO CS PC3_C；SPI1 display TX-only PB3/PD7 |
| G431 | FDCAN1 PA11/PA12 | USART1 PB7/PB6 115200，DMA1 Ch1；USART2 PA3/PA2 10Mbps，Ch4；RS485 DE PA1 保留输入 | SPI1 PA5/PA7/PA6；ACC PB0、GYRO PA4 |
| G474 | FDCAN1 PB8/PB9；2 PB5/PB6；3 PA8/PB4 | USART1 PA10/PA9 2Mbps，DMA1 Ch2；USART2 PA3/PA2 921600，Ch3 | BMI088 SPI2 PB13/PB15/PB14，ACC PB12、GYRO PB11；encoder SPI1 PA5/PA7/PA6，CS PB1/PB2 |
| F407 | CAN1 PD0/PD1；CAN2 PB5/PB6 | USART1 PB7/PA9、DMA2 S5；未提供 Config 前保持未配置 | BMI088 SPI1 PB3/PA7/PB4，ACC PA4、GYRO PB0，MODE3 |
| F427 | CAN1 PD0/PD1；CAN2 PB12/PB13 | USART1 PB7/PA9 100000，DMA2 S2；USART6 PG9/PG14 9600，DMA2 S1；UART8 PE0/PE1 9600，DMA1 S6；UART7 PE7/PE8、DMA1 S3 保留未配置 | `bsp.h` 明确 `BSP_HAS_IMU=0`，无板载 IMU实例 |

H723 的 PC2_C/PC3_C 用作数字引脚时，使用 HAL 的 PC2/PC3 引脚对象，并接通对应的 SYSCFG 模拟开关；这与来源代码 MSP 初始化中对 PC2/PC3 的数字 GPIO 配置一致。UART、DMA、FDCAN/bxCAN 的 IRQ 名称均来自对应芯片定义，五种芯片的构建会检查这些中断绑定是否有效。

| 板 | PWM（默认禁用） | 其它实际资源 |
|---|---|---|
| H723 | TIM1 CH2 PA9 / CH3 PA10，50Hz；TIM4 CH2 PB7，周期65536µs | ADC1 IN4 PC4，16bit，PLL2P /64=1.25MHz，32.5周期采样；`read_adc()` 单次转换，DMA1 S0 供显式流配置保留。USB_OTG_HS 内置 FS，PA11/12，256字节 endpoint OUT buffer，无 ULPI |
| G431 | 原选中板未配置 PWM | 无 USB，PA8 与 RS485 DE 保留输入 |
| G474 | TIM3 CH2 PA4，50Hz | USB peripheral PA11/12；两路 encoder CS 默认高 |
| F407 | TIM1 CH1 至 CH4 PE9/PE11/PE13/PE14，500Hz；TIM4 CH3 PD14，4kHz | USB_OTG_FS PA11/12；TIM12 PB14/PB15 源周期缺失，保留输入 |
| F427 | TIM5 CH2 PH11 / CH3 PH12 / CH4 PI0，50Hz；TIM8 CH3 PI7 / CH4 PI2，50Hz | TIM4 encoder PD12/PD13，双边编码器模式，HAL 全位宽计数策略；USB_OTG_FS PA11/12 |

PWM 频率和周期来自来源配置。HAL 自行计算 PSC（预分频）和 ARR（自动重装载）组合，整数舍入可能使结果与原 C 初始化的寄存器值不同。输出通道默认关闭；应用应先设置限制到允许范围内的占空比，再启用通道。

CAN 仲裁阶段的名义速率均为 1Mbps。下列时序参数依次为 PSC、BS1、BS2、SJW，分别表示预分频、两个位时间段和重同步跳转宽度。H723 FDCAN 使用 80MHz 内核时钟，nominal（仲裁阶段）参数为 `(PSC=1, BS1=59, BS2=20, SJW=4)`，data（数据阶段）参数为 `(1,12,3,3)`，对应5Mbps。G4 使用170MHz内核时钟，nominal `(17,6,3,1)`，data `(2,10,6,1)` 对应5Mbps。F407 使用 `(3,9,4,1)`，F427 使用 `(3,10,3,1)`；来源中未记录的 SJW 采用1TQ（一个时间量子）。当前这些参数用于回环测试，连接外部总线前须由应用明确切换工作模式。

F4 两路 CAN 共用 CAN1 的过滤器 RAM。配置通过 CAN1 将分界值 split 设为14，bank0 用于 CAN1，bank14 用于 CAN2，两者都接收所有 ID 并放入 FIFO0。这样可以避免复位后过滤器默认关闭、所有接收帧被丢弃的情况。应用可以按实际需要缩小接收的 ID 范围。

## 与来源配置的差异及待补参数

H723 配置关闭 DCache，PA9/PA10 用于 TIM1 CH2/3，因此不创建 USART1。FDCAN2 数据阶段的预分频为 1，UART8 为 115200 baud。具体资源与参数以 Rust 板级模块为准。

BMI088 的 SPI 上限为10MHz。源 H723 用108MHz /8=13.5MHz，源 G4 用170MHz /16=10.625MHz，均超限；Rust 分别用6.75MHz和5.3125MHz。F407 保留84MHz /256=328125Hz、MODE3。G474 encoder SPI仍保留10.625MHz。规格依据 [Bosch BMI088 数据手册，SPI timing](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bmi088-ds001.pdf)。

BMI088 的加速度计和陀螺仪共用一条 SPI 总线，各有独立片选。总线由一处代码负责，不复制外设句柄。应用需要先安排好总线共享方式，再把两个 SPI 设备接口交给 BMI088；`SpiBusDevice` 支持借用总线。参考固件不会探测或校准 IMU。

PA15/PB3 对应 TIM2 CH1/2，来源中的 TIM2 只配置了自由计数，无法从中确定 PWM 周期。使用这些 PWM 输出前，需要自己确认周期。F407 USART1、TIM12 和 F427 UART7 缺少的参数也需要由应用提供。

H723 按来源配置保持 DCache 关闭。如果启动前已开启，程序会断言失败，以避免缓存与 DMA 看到的数据不一致。后续启用 UART DMA 或 ADC 连续采样时，缓冲区应放在 DMA1/2 可访问的 AXI SRAM `0x24000000..0x24050000`，不能放在 DTCM，也不能把 Flash 用作 DMA1/2 缓冲区。当前固件空闲时不启动这些 DMA 传输；SPI 使用阻塞调用，USB 使用片内 FIFO/PMA 缓冲区。

## 构建

仓库根目录、已激活工具链的终端：

```text
cargo build -p embodied-app --bin reference-board --release --locked --target thumbv7em-none-eabihf --features board-stm32h723vg
cargo clippy -p embodied-app --bin reference-board --locked --target thumbv7em-none-eabihf --features board-stm32h723vg -- -D warnings
```

将命令中的 feature 换成表中对应板卡的名称，不必再添加芯片 feature。产物为 `target/thumbv7em-none-eabihf/release/reference-board`。运行后会初始化资源并输出 idle 日志，控制业务需要由应用实现。烧录和使用前，检查板卡版本、电源、晶振、接线和 Flash 配置。
