# 47C RCC 枚举 API 名称对照

本次只规范公开名称，不改变 mux 位宽、位置、编码值或真实频率。精确硬件输入仍来自 `../stm32h5-47c` 中经过 GCC 真值核对的 ST 常量。补充源为固定 HAL `ba20038d938ecc31399e49d60fa6bcc8e82db5da` 的 `sources/clock-names/stm32h5xx_hal_rcc.c` 和 `stm32h5xx_hal_rcc_ex.c`，SHA 见 `sources.json`。

| 精确 mux | ST 常量后缀及编码 | PAC 名称 | 依据 |
| --- | --- | --- | --- |
| RTCSEL | NO_CLK / 0 | DISABLE | 固定 rcc.h 第 437 行明确 no clock；沿用旧 PAC 的同值名称 |
| RTCSEL | HSE_DIVx / 3 | HSE_DIV_RTCPRE | rcc.h 第 440 行以及第 5140–5164 行通过 BDCR.RTCPRE 配置 HSE 分频；沿用旧 PAC 名称 |
| ADCDACSEL | HCLK / 0 | HCLK2 | rcc_ex.c 第 4481–4483 行调用 GetHCLKFreq；准确 CMSIS 的 ADCEN、DAC1EN、ADC3EN 位于 AHB2ENR |
| ADCDACSEL | SYSCLK / 1 | SYS | rcc_ex.c 第 4485–4487 行调用 GetSysClockFreq；统一系统时钟 API 名称 |
| OCTOSPI1SEL | HCLK / 0 | HCLK4 | rcc_ex.c 第 5625–5627 行调用 GetHCLKFreq；准确 CMSIS 的 OCTOSPI1EN 位于 AHB4ENR |
| ETHPTPCLKSEL | HCLK / 0 | HCLK1 | rcc_ex.c 第 6123–6125 行以 GetHCLKFreq 为输入；准确 CMSIS 的 ETHEN 位于 AHB1ENR |

HCLK1/2/4 表示各总线视图。固定 ST `HAL_RCC_GetHCLKFreq`（rcc.c 第 1491–1497 行）仅通过 CFGR2.HPRE 对系统时钟分频，未在这些 API 名称中额外引入分频器。生成时分别检查精确 RCC 的 enable 字段；未进行全局字符串 `HCLK` 替换。ADCDAC 与 OctoSPI 名称还与固定旧 H5 IR 的同一来源编码一致。

ETHPTP 的这个枚举描述 mux 输入。最终 ETHPTP 输出仍要除以独立 ETHPTPDIV（rcc_ex.c 第 6118–6125 行）；本补丁不宣称名称规范化已实现 HAL 的 PTP 频率计算。RTC 枚举值 0 是实际禁用编码，不是保留值。
