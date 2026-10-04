# STM32H543/H553 HAL 候选补丁

本目录保存 H543/H553 的 14 个精确型号适配。它基于当前固定版本的 Embassy 及已有本地修复，保留原有芯片 feature，复用现成 GPIO、USART、GPDMA、FDCAN、SPI、USB 和定时器实现。相对于该基线共修改 15 个文件、新增 2 个 EXTI 文件；新文件与所有替换片段均由 SHA 绑定。

**这是可重放的候选后端，尚未接入主工作区的 vendor、型号目录和 App feature。** 它与独立的 [47C PAC 包](../stm32h5-47c-pac/README.md) 配套。普通 App 开发仍使用主框架的已支持型号，不需要运行这里的维护命令。

## 重放

需要 Python 3.11+、Rust 1.98.1 和 rustfmt。先从框架根目录激活局部工具链，再生成合并 PAC 和 HAL。输出目录必须不存在，命令拒绝覆盖源文件或既有结果：

```powershell
. ./scripts/env.ps1
python xtask/scripts/prepare_h5_47c_pac.py --output target/h5-47c-replay
python xtask/scripts/prepare_h5_47c_hal.py --output target/h5-47c-replay/embassy-stm32
python -m unittest discover -s xtask/scripts -p 'test_h5_47c_*.py'
```

`target/h5-47c-replay/stm32-metapac` 包含原有完整 PAC 和新增 14 型号；`embassy-stm32` 包含完整原有 HAL feature 和新增型号。合并 PAC 为独立生成的共享元数据增加命名空间，对已有 IP 模块保持原始字节。仅在经过固定 rustfmt 处理后完全相同的情况下接受格式差异。

在独立 Cargo 工程中使用这两份输出，需要将 Embassy 与 stm32-data-generated 的 Cargo patch 分别指向上述目录，并显式选择一个准确芯片，例如 `embassy-stm32/stm32h543ce`。主工作区尚没有对应的 App feature，不能直接把候选 HAL 的 feature 当作主 App 已支持的 feature。维护者的构建证据与后续接入状态见 [实施状态](../../../docs/STATUS.md)。

`patch.json` 绑定当前 HAL 基线清单、完整 47C PAC 包清单、修改前后文件 SHA、唯一替换片段和最终完整文件清单。脚本会验证包内源码及全部基线文件。局部修改、来源缺失、大小写路径冲突或错误版本均会停止重放。重新生成到另一空目录时，源码应逐字节一致；构建日志和重放凭据放在源目录之外。

## 接入内容与边界

- 准确的 47C RCC/PWR、两颗 PLL、Flash 等待周期、GPIO EXTI 和外设版本选择；不伪造 PLL3，也不把 H543/H553 映射为旧 H563。
- 只为已经核对并接入的外设产生 HAL 驱动能力。其它 PAC 中存在的硬件不会被错误标记为不存在，但仍不提供未经验证的高级驱动。
- PLL 输入、VCO 模式和输出上限检查采用准确数据手册；LSE/LSI 频率单独发布，供独立外设时钟选择使用。
- EXTI GPIO 0–15 使用准确的寄存器布局，保留类型化引脚所有权、IRQ 唤醒和取消语义。其它 EXTI 线路没有借用旧 H5 布局。
- 默认目标是 TrustZone 关闭后的非安全地址窗口、冷启动内部时钟；未覆盖安全域启动、动态电压/时钟切换、低功耗恢复或自定义 bootloader 遗留状态。

已保留的 RTC 时钟源切换仍受备份域行为影响，相关 SRAM2/备份域复位条件尚未闭合；不能把重新编译当作解决此问题。USB 使用 HSI48 的构造案例没有启用 SOF/CRS 校准，不宣称频率精度、枚举或通信已通过。全部 HIL 状态为未运行。

现有隔离证据包括 14 个最小 App、14×7 个实际外设构造器，以及 PLL/LSE/LSI 的 38 项主机回归。后续把候选与完整旧型号集合合并后，还须分别进行编译和回归验证。编译链接成功不代替 IRQ、DMA、USB、CAN 或时钟的实机测试。
