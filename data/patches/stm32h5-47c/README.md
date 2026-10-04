# STM32H543/H553：准确 PAC 输入准备

本目录提供14个真实型号的可重放源数据、寄存器实测布局和差分，尚未生成 `stm32-data` schema 的可用芯片 JSON、PAC 或 Embassy HAL。`chip-inputs` 是明确标记的输入格式，不能复制进全局芯片目录当作后端支持。没有修改 vendor、Cargo manifest 或全局能力表。

## 重放与文件

这是一套额外的后端证据准备工具，与普通 Rust 固件构建分开。离线重放需要 Python 3.10+ 和 GCC，且 `python`、`gcc` 可在 PATH 中执行；仅使用 Python 标准库，不需要 pip 或任何第三方 Python 包。本轮实际工具为 Python 3.13、GCC 15.2.0。GCC会编译并运行主机端 C `sizeof`/`offsetof` 探针，不需要 ARM GCC、Rust 编译器或硬件。已保存离线重放所需的ST源码：

```powershell
python data/patches/stm32h5-47c/prepare.py
python -m unittest discover -s data/patches/stm32h5-47c -p test_preparation.py
python data/patches/stm32h5-47c/verify.py
```

`verify.py` 先将报告设为未完成，连续生成两次并比对17个输出的 SHA-256，然后执行回归；成功才写 `validation.json`。`.build` 是可丢弃的 C 探针和日志。输入实际文件集合必须与45项源锁完全一致；缺项、未锁定文件或额外生成JSON都拒绝。寄存器 `sizeof`、每个字段及保留区的 `offsetof` 和大小由 GCC 编译 ST 原始 typedef 实测。所有暴露的安全/非安全外设指针必须具有测得的类型布局。

常量解释器只产生非负32位候选，**不实现完整C整数语义**。仅允许不改变数值的`uintN_t`字面量转换；拒绝补码`~`、负数、溢出中间值、一般类型转换、除法、函数调用、解引用和未解析标识符。支持的整数运算/别名也必须再经过实际GCC求值：所有导出的CMSIS整数、DMA请求与RCC时钟源常量逐一对照C真值，不一致则停止生成。未接受的对象宏保存在`unsupported_object_macros`，不能把它们当作不存在的硬件。

`integer-audit.json`记录收窄语法后的审计：H543的18,605个、H553的19,002个整数候选全数与GCC一致。旧实现接受的4个USB宽度敏感常量也单独与GCC核对，一致但现在主动排除；另外删除7项主机预处理器内建宏。原已接受器件常量没有发现数值错误。该结论只覆盖固定来源，宽度敏感的反例仍须拒绝。

| 文件 | 内容 |
|---|---|
| `sources.json`、`sources/` | 固定 commit、原始文件 URL、SHA-256、字节数；pin XML 另校验 Git blob SHA-1；ST 许可证原文 |
| `capture_sources.py` | 从项目只读缓存复取29个 XML、GPIO AF、CMSIS；从固定 ST HAL commit 下载相关头文件 |
| `generator-inputs.json` | 11份生成器比较来源的固定URL、SHA-256及抽取事实；不额外复制未查到许可声明的生成器源码/YAML |
| `evidence.json` | 全14型号/29封装、各自 IP version/引脚/AF、IRQ、地址、DMA、RCC 和 H563 差分 |
| `chip-inputs/*.json` | 每个准确型号的独立输入，链接其家族寄存器证据 |
| `register-inputs/*.json` | H543/H553 各自原始位域、整数宏、60/64个实测类型布局；没有臆造 reset value 或访问副作用 |
| `official-documents.json`、`integer-audit.json` | 官方文档版本/页码/获取边界与冲突；C整数真值和旧候选审计 |
| `integration-decisions.json` | 机器可读的版本复用/新建判断、优先匹配规则和未闭合项 |

来源许可分别为pin XML及ST HAL的BSD-3-Clause、CMSIS的Apache-2.0；三份原始许可证随源保留，具体映射见`sources.json.licenses`。器件数值宏严格限制为锁定ST头文件声明的名称，不包含Windows/Linux预处理器内建宏；17个规范化输出使用固定LF，工具版本仅出现在验证报告。

若需要重新捕获来源，在完整项目中运行（依赖已有只读 `data/sources` 缓存；首次获取ST HAL源码需要HTTPS网络，仍不需要第三方Python库）：

```powershell
python data/patches/stm32h5-47c/capture_sources.py --workspace .
# 可选重新请求官方 PDF；失败状态保留在 sources.json：
python data/patches/stm32h5-47c/capture_sources.py --workspace . --fetch-documents
```

原始 PDF 在本环境直接下载返回 HTTP 567，未保存伪 PDF 或伪哈希。数据手册和勘误通过 web 可读取；新 RM0481 的 web 读取因22,139,766字节超限失败。可选下载成功时仅写入项目`data/sources/stm32h5-47c-documents`本地缓存，本patch目录只记录URL/路径/哈希，不放PDF全文供独立项目再次分发。当前生成不依赖不可取得的 PDF。

## 精确型号与内存

H543 为 CE、CG、RE、RG、UG、VE、VG、ZE、ZG；H553 为 CG、RG、UG、VG、ZG。29份封装 XML 都为 `DIE47C`，E为512KiB Flash、G为1MiB。每颗型号读取其 XML 容量，不能套用 CMSIS 的1MiB `FLASH_SIZE_DEFAULT`。UG是 WLCSP63。

| 存储区 | 非安全地址 | 大小 |
|---|---:|---:|
| SRAM1 | `0x20000000` | 128KiB |
| SRAM2 | `0x20020000` | 80KiB |
| SRAM3 | `0x20034000` | 96KiB |
| Backup SRAM | `0x40036400` | 2KiB |

安全别名同样直接取各自 CMSIS 宏并保存。常规 SRAM 合计304KiB，与29份 XML逐份交叉验证。Flash擦除单位宏为8192字节；保护寄存器数量与 sector-index 位宽必须采用新版本。完整 bank/保护/高循环区模型留待 RM0481 审核。

## 可复用与必须改变的 IP

差分数字是数值宏的新增/删除/变化（包括 `_Pos`、`_Msk`、别名），不是物理寄存器数量。以前 `stm32-gaps.md` 的原始表达式文本差分计数与本次解析后整数计数略有不同；本目录保留每个差异的原值、新值和真实 typedef。

| 块 | 证据与后续处理 |
|---|---|
| RCC | XML `STM32H5_rcc_v1_2`；H543宏+93/-382，H553+120/-382。`PLL3CFGR/DIVR/FRACR` 不存在。新增 ADC3、I3C2、PLAY 和 Ethernet mux/divider，必须建立47C版本及对应 HAL 初始化。`rcc_inputs`保存203个 ST HAL 编码后的时钟源常量、全部 RCC 位域/布局；它们尚不是规范化 clock tree。 |
| PWR | 布局相同，语义不同：`SRAM1SO`由bit26到27，`SRAM2_48SO`由25到26，新增 SRAM2两个16KiB分段，移除 `SCCR_SMPSEN`。不能沿用旧寄存器位定义。 |
| FLASH | 移除两bank各自第三/第四个 SECBB/PRIVBB 寄存器，SNB位宽、secure watermark、WRP mask缩小。需独立47C映射和容量规则。 |
| GPDMA | DMA类型布局和数值宏与H563一致，可复用 `gpdma_v1` 的寄存器基础；每家族请求表重新提取，分别198/206项，两个控制器各8通道。ADC3=142；I3C2 RX/TX/TC/RS=136/137/138/139。 |
| GPIO/USART/FDCAN/ICACHE/DCACHE | 比较范围内的类型布局及数值宏完全一致；可复用已有寄存器块，外设实例与封装AF仍使用本目录准确输入。ICACHE还需补匹配H543/H553的规则。 |
| RNG | 新HTCR[4]、HTSR[2]、NSMR，旧v3不适用。现有 `rng_v4.yaml` 布局接近，但 `NSCR.EN_OSC`数组只有3项，新CMSIS有6项，不能直接宣称完整兼容；需扩充独立版本/参数变体并审核枚举。 |
| I3C | 新MISR、额外83个宏、2项改值；需扩展版本，两个真实实例。 |
| OCTOSPI/XSPI | CMSIS将 `OCTOSPI_TypeDef` 显式别名到 `XSPI_TypeDef`；本工具测量二者。位域+24/-3/改3，XML为 `octospi1_v6_3_Cube`，旧mapper只处理v5_1，需新规则/字段。 |
| GTZC/RAMCFG/HASH/PKA/ADC/SPI/TIM/USB | 完整差分均已保存；有字段增删或大小改变，不能仅依据family名称复用。RAMCFG有WPR3，HASH digest数组扩展。按实际实例审查已有版本或新版本。 |
| PLAY/CCB/crypto | PLAY为新块。H543与H553均有PKA IRQ118；只有H553有AES、SAES、OTFDEC1、CCB指针。H543虽然存在 `RCC_*CCB*`字段宏，不能据此暴露CCB实例。 |

H543/H553分别有113/116个非负IRQ定义，保留149之前的所有空洞，不把向量按数量顺序重编号。ADC3=149、PLAY1=147、PLAY1_S=148、I3C2_EV/ER=131/132；H553另有AES=116、SAES=36、OTFDEC1=115。对H563删掉的21个IRQ也完整列出。

AF只采用准确 `GPIO-STM32H5(4-5)3x_gpio_v1_0_Modes.xml`，与每个封装真实焊盘及其声明 signal 取交集。模拟输入、系统功能等不需要AF的信号仍保留在 `pins[].signals`；不能把整个家族AF集合绑定到小封装。

## 上游源矛盾和剩余步骤

ST HAL commit `ba20038d...` 的 `stm32h5xx_hal_dma.h` 对这两家族同时定义 ADC3请求142、I3C2请求末尾139，但 `IS_DMA_REQUEST` 在有I3C2/无ADF1时以139作为上限。C预处理器实测 `IS_DMA_REQUEST(GPDMA1_REQUEST_ADC3)` 为false，回归固定此事实。后续Rust DMA路由应保留142，不能复制该错误范围断言。

ST官网个别型号短描述与精确DS/XML不一致；例如H543UG列表描述512KiB，准确XML为1MiB。另 ES0683 Rev1首页写 RM0539，而官方H543/H553文档页和系列页链接 RM0481 Rev5（2026-07-07）。这里保留冲突，未取得 RM0539，未将旧 RM0481 Rev4 当作新器件规范。

后续可从这些输入继续：将两组准确 header mapping 加入生成器，为 `STM32H5[45]3.*` 建立优先于宽泛H5的内存/RCC/PWR/FLASH规则；按照位域与官方最新RM补寄存器YAML、reset/access语义、mux枚举和IRQ信号映射；更新DMA路由和所有实例/AF；生成真实14颗JSON/PAC，再实现/验证Embassy时钟与flash路径。现有公开pin仓库没有新的RCC/NVIC/GPDMA Cube模式XML，需取得对应新CubeDB或以已审核的官方数据独立建立这些元数据。

当前只声明离线证据提取与回归验证通过。PAC生成、14型号ARM编译、HAL链接和硬件验收均未执行，也未把这些型号改成supported。
