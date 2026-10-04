# STM32H543/H553（DIE47C）后端准备

本目录提供STM32H543/H553准确型号的来源证据、差分与可重复测试。此阶段交付PAC输入，尚未生成可用PAC或HAL；H563仅作为逐项差分对照，不作为新芯片alias。

## 工具与已交付状态

准备/重放是普通Rust固件构建之外的额外流程：需要Python 3.10+和可在PATH执行的GCC；仅Python标准库，无第三方Python包。本机实际Python 3.13、GCC 15.2.0。GCC编译运行主机端C布局探针，无需ARM GCC或烧录硬件。重新捕获源需要完整项目只读缓存与首次HTTPS下载；已保存输入的prepare/verify不需要网络。

已完成14型号/29封装精确输入、45份带哈希ST源文件、寄存器C布局探针、IRQ/AF/DMA/RCC提取和H563逐值差分。`README.md`记录可重放命令，`validation.json`记录实际回归及确定性输出哈希。生成器11份对照文件仅保存URL、SHA与抽取事实，不新增复制原文；未下载PDF全文。真实PAC生成和HAL集成仍未完成。

## 精确范围

H543：CE、CG、RE、RG、UG、VE、VG、ZE、ZG；H553：CG、RG、UG、VG、ZG，共14个基础型号、29份 ST 封装 XML。非安全与安全地址分别保存，Flash 容量来自精确封装 XML；不会把 CMSIS `FLASH_SIZE_DEFAULT` 当作全部型号容量。

## 实施步骤

1. 固定并校验 ST pin-data `7d1f1514ed5583ec5007ad91236b4e1d377295b1`、CMSIS H5 `884b8dc78e41cbfca008363342b17f4a9e8641f7`。提取29份封装 XML、准确 GPIO AF 文件、H543/H553/H563头文件及许可证，所有输入有 SHA-256 和官方来源 URL。
2. web 核验 ST DS15168、DS15167、RM0481 和 ES0683。下载可取得的原文，并记录版本及不能取得的内容，不把旧 RM0481 Rev4 冒作覆盖 H543/553 的 Rev5。
3. 先写失败测试，再实现安全的 C 常量解析、精确 CMSIS IRQ/地址/寄存器布局和位域提取。用主机 C 编译器的 `sizeof`/`offsetof` 独立验证布局，保留真实保留区和数组。
4. 以每个型号的全部封装重建 pin/AF/IP 集合，以各自头文件重建芯片地址、IRQ、内存、RCC gate/reset/mux 字段。根据 ST HAL 条件编译后的定义提取 DMA request；不得由相近型号编号推测。
5. 与固定和当前生成器比较 RCC/PWR/FLASH/IRQ/memory/AF/DMA；标注可复用块、新版本、缺少枚举语义或需人工核对内容。输出 JSON 和可被后续 PAC 生成工作消费的寄存器输入，保留完整判定证据。
6. 验证14型号全集、29封装、两种Flash容量、304KiB SRAM与2KiB backup、差异外设、AF合法性、IRQ空洞、DMA请求以及确定性重放。只有足以满足 stm32-data schema 的芯片才生成可用 chip JSON，否则明确保持 input schema，禁止伪装成已经可构建的 PAC/HAL。

## 当前官方核验

- [ST 产品页](https://www.st.com/en/microcontrollers-microprocessors/stm32h543-h553.html)：250MHz、最多1MiB双bank Flash、304KiB SRAM。
- [ST 文档页](https://www.st.com/en/microcontrollers-microprocessors/stm32h543-h553/documentation.html)：DS15168（H543）、DS15167（H553）、RM0481（包含新组）、ES0683。
- [DS15167 Rev1](https://www.st.com/resource/en/datasheet/stm32h553cg.pdf)、[DS15168 Rev1](https://www.st.com/resource/en/datasheet/stm32h543cg.pdf)：2026年7月。已通过 web 读取；表1分别列出5个和9个基础型号。
- [ES0683](https://www.st.com/resource/en/errata_sheet/es0683-stm32h543xx-stm32h553xx-device-errata-stmicroelectronics.pdf)：新组专用勘误。

产品列表中的部分短描述（例如 H543UG 被写成512KiB/crypto）与精确型号 DS/XML 不一致；实现以 DS 表格、CMSIS 和封装 XML 交叉校验，不采用产品列表的复制描述。
