# STM32H543/H553（DIE47C）独立 PAC

本目录提供 14 个精确型号的寄存器输入、可重复生成工具和独立 `stm32-metapac` 代码。它不修改工作区的 vendor、芯片支持目录或 HAL。PAC 编译通过不代表整个 Embassy 后端已经支持这些芯片。

输入沿用冻结的 `../stm32h5-47c`：29 个封装 XML、GPIO AF、GCC 核对的 CMSIS 布局和常量。新增补充来源是固定提交的 ST/Keil DFP SVD；FLASH 地址/保护组、RNG 字段和 I3C 掩码的已知错误均由明确规则修正，不能把该 SVD 当成整颗器件的权威定义。

## 重放

在框架根目录运行：

```powershell
. ./scripts/env.ps1
python data/patches/stm32h5-47c-pac/verify.py
```

默认输出在 `target/h5-47c-pac-generation`。验证会生成两次并对比全部 PAC 源文件 SHA，随后对 14 个型号逐一执行 `thumbv8m.main-none-eabihf` 的 `pac + metadata + rt` 编译，运行实际生成代码的字段测试及 Raw 编译约束回归。每一步日志、命令和最终产物哈希都进入该目录的 `validation.json`。完整验证通过才写 `completed: true`。

只生成可用代码：

```powershell
python data/patches/stm32h5-47c-pac/generate.py
```

`generate.py --rebuild-generator` 从固定 SHA 的原始源码归档重新构建生成器。默认可复用已经验证过 SHA 的本机生成器；如果不存在则执行固定源码构建。工作区同一输出目录由互斥锁保护，第二个写入者直接失败。中断会保留锁；仅当原进程及所有 Cargo、rustc、生成器、rustfmt 子进程已退出，才可手工移除旧锁。

## 工具

普通 Rust 固件构建不需要下面的数据准备工具。此 PAC 重放需要 Python 3.10 或更新版本、Rust 1.98.1、rustfmt 和 `thumbv8m.main-none-eabihf` target；Python 仅使用标准库，无第三方库和 pip 依赖。构建原始生成器需要其锁定的 Cargo 依赖缓存或网络。`capture_sources.py` 是一次性来源捕获工具，另外需要 Git 和既有固定缓存；正常重放不调用它。

完整 `verify.py` 另外需要可在 PATH 调用的 GCC（本次 15.2.0），会在忽略的 target 目录重新编译、执行四组 FDCAN 分配真值探针。它们使用准确 H543/H553 头文件和原始 ST HAL 分配函数的纯算术前缀，覆盖 NS/S 两地址窗口，不读写 MMIO。单独 `generate.py` 和普通 Rust 固件构建不需要 GCC；此前 CMSIS 全布局与常量、CRS 的 GCC 证据也保留在锁定来源中。

## 可用接口与限制

RCC、PWR、FLASH 使用真正的独立 47C 寄存器版本。仅有两颗 PLL，没有伪造 PLL3、SMPSEN 或 H543 不存在的加密实例。RCC mux 的数值来自锁定 ST HAL 常量；SW/SWS 是精确的 2 位字段。

GPIO、USART、GPDMA、FDCAN、ICACHE、DCACHE 的 IP 复用需要同时通过 C 类型布局和完整前缀数值定义一致性检查。TIM、SPI、USB 派生为独立 `h5_47c` 版本：TIM 按精确 CMSIS 实例能力宏选择 16/32 位、通道数和刹车功能块；SPI 增加真实 CFG1.DRDS bit 24；USB 删除位不重新暴露，并按 CMSIS 地址提供 2048 字节 PMA。其它 CMSIS 原生视图使用独立版本；原生族类型可能是 CMSIS 的公共布局，不构成对每个实例的高级驱动能力声明。HAL 必须根据精确实例能力接入，不能仅凭相似名称启用旧驱动。

访问方式不能仅从 `__IO` 推断。缺少相符 SVD 显式访问依据的寄存器返回 `Reg<T, Raw>`，元数据也写为 `Access::Raw`；这些对象没有安全 `read`、`write`、`modify`、`write_value` 或 `reset`，只提供调用者承担硬件语义的 `unsafe read_unchecked/write_unchecked`。例如 RNG 新增 HTSR/NSMR、PWR SCCR、FLASH epoch 编程寄存器属于这一范围。寄存器值结构的 `Default(0)` 仅是普通值构造，不声称硬件复位值；PAC 不生成复位 API。

当前器件常量选择 CMSIS 的非安全地址窗口。安全地址别名仍保留在冻结来源中，本次不声明完整 TrustZone 启动流程已完成。所有数据都可供继续生成安全域视图，但不能由非安全地址常量推断安全域权限。

`api-diff.json` 记录相对于旧 H5 IP 的方法/字段/枚举差异；`register-evidence.json` 记录访问策略与 SVD 复位值出处（仅证据、不生成 reset API）。原始生成器版本、源码归档 SHA、实际执行文件 SHA 和构建方式见 `sources.json`、`generation.json`。

来源中的 SVD 带 ST 2026 Apache-2.0 许可，按原始字节压缩保存，并同时锁定压缩与解压 SHA。未将版权手册 PDF 放入本目录；RM0481 正文和 ES0683 引用 RM0539 的矛盾仍需后续准确资料解决。

## V3 补充来源及接口

`fdcan-ram.json` 记录准确 ST HAL SRAMCAN 宏计算的六个区域：28 个标准过滤器、8 个扩展过滤器、各 3 个 RX FIFO 项、3 个 TX event 项、3 个 TX buffer 项，合计 848 字节。原始实例选择代码和四组 GCC 结果共同证明 NS RAM1=`0x4000ac00`、RAM2=`0x4000af50`；只有上述几何布局与旧 `fdcanram_v1` 完全一致才允许复用。C 常量解释器仅接受非负且全部中间值不超过 INT32_MAX 的加法、乘法、名称、U 后缀及 uint32_t cast；位运算、移位、宽整数、负数和其它 C 语义直接拒绝。新 GCC 真值重放进一步核对实际分配分支。

CRS 复用 `crs_v1`：严格验证准确 C 类型、86 个整数宏、每个字段、HAL 的 GPIO/LSE/USB 同步枚举，与单独 GCC 审计结果一致；保留精确 CRS IRQ75，未复制旧芯片的中断或封装记录。`crs-proof.json` 绑定完整来源和结论。EXTI 没有套用旧 `exti_h5`：准确原生寄存器仍保留三个 bank；旧版本仅两 bank 且 EXTICR 位宽更宽，差集在锁定 `sources/crs-audit` 证据中。

RTC 及几个总线来源枚举的规范名称只改变 API，不改硬件值。逐 mux 对照、ST 源文件行号和 ETHPTPDIV 边界见 [CLOCK-NAMES.md](CLOCK-NAMES.md)。

`history/v2/source-snapshot.zip` 保存替换前全部 V2 源文件和 manifest；`receipt.json` 记录压缩包及原 manifest SHA。发布先核对原清单全部文件，再归档、更新，并仅移除旧清单中不再生成的路径。正式目录的全部当前文件由新 `manifest.json` 绑定。归档不含版权手册或构建二进制。
