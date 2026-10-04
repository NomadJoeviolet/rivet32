# 47C PAC 实现边界与后续集成

本补丁把冻结的 STM32H543/H553 数据准备结果落实为可编译的独立 PAC，不变更全局芯片能力表。14 个型号均使用自身封装、Flash 容量、IRQ 和 DMA 请求，不以 H563/H573 整颗别名代替。

实现链路为：精确 CMSIS/GCC 布局与位定义 → 受限 SVD 访问补充 → 独立 chiptool IR → 固定原始 stm32-metapac-gen → Raw 访问后处理 → 双重哈希重放、14 型号 ARM 编译与访问回归。规范化输出不嵌入主机内建宏；工具版本和执行路径只出现在验证报告中。

RCC/PWR/FLASH 保留能由字段证据确认的既有 metapac 方法名。PLL 数组收缩到 2，SW/SWS 收缩到 2 位；时钟源枚举由 ST HAL 真值生成。I3C 两个错误 mask 使用分别与 SVD 一致的正确 CMSIS Pos。RNG 的新寄存器按真实布局补入，访问未确认部分保持 Raw。

复用分为两类：GPIO/USART/GPDMA/FDCAN/ICACHE/DCACHE 的完整 C 布局和前缀常量一致；TIM/SPI/USB 根据已列出的严格字段差集派生新版本，并逐实例核对能力谓词。其它原生类型是精确 CMSIS 布局视图，尚需逐驱动字段语义与功能能力审查，不能自动启用原驱动。

后续 HAL 工作包括新版本 cfg、时钟/电源启动路径、外设驱动的枚举及字段接口适配、共享资源和中断绑定。独立 PAC 的常量默认为非安全窗口；完整 TrustZone 流程需要单独设计。高层 HAL 不应绕过 Raw 门控，除非调用处有具体硬件依据并明确承担 unsafe 前提。

仍未闭合的是 RM0481 可获得正文与 ES0683 所引 RM0539 的关系，以及 SVD 未提供的访问副作用/精确复位语义。未知值不会生成 reset API，未确认权限不会生成 safe modify。`register-evidence.json` 给出每个涉及寄存器的具体范围，方便下一步逐项补证据。

V3 已补齐 FDCAN 消息 RAM 的精确分配证据：锁定 ST HAL 宏与真实实例选择前缀经 H543/H553 × NS/S 四组 GCC 验证，容量均 848 字节；据此合成两实例，并逐区域核对旧 RAM IR 后复用。CRS 的完整类型、86 宏及 HAL 同步源枚举也已独立核对，复用 crs_v1，同时保留准确 IRQ75。RTC/clock 命名依据见 CLOCK-NAMES.md。UID 与温度校准的高级类型视图仍未合成；准确原始常量保留在冻结证据中。EXTI 保持准确原生三 bank 定义，不能将旧两 bank IP 整体复用。

工具要求：Python 3.10+ 标准库、Rust 1.98.1、rustfmt 和 ARM target；完整验证另用 GCC 15.2.0 重放四组 RAM C 探针。生成 PAC 或普通 Rust 固件无需 GCC。
