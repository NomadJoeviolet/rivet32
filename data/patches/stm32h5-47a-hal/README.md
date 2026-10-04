# STM32H5E/F HAL 组合补丁候选

本包包含 H5E4/E5/F4/F5 共 24 个准确型号的 Embassy HAL 增量。
复用现成外设驱动，补齐时钟、电源、EXTI 和 USB 平台差异。
10 个既有文件的改动保存为基线哈希约束的文本替换，另有 5 个新增源码。
原有全部 1,715 项 feature 保留；没有复制整份 HAL，也不依赖原机器的 target 路径。

`baseline-manifest.json` 固定前一版完整后端；`patch.json` 绑定前后完整文件清单、
准确芯片选择及配套组合 PAC。底层应用补丁函数沿用 `prepare_h5_47c_hal.apply`。
上游许可证保留在重放后的 HAL；新增文件沿用对应 Embassy 源码许可。

24 个新型号与 14 个旧系列对照的最小固件已完成 release 链接及 ELF 检查。
全部七类构造器和 USB FS/HS 的组合复验仍在执行；不据此宣称已完成全部后端支持。
RTC 配置改变时会重置备份域，丢失 RTC 日历和备份寄存器；配置相同则保留。
H5E/F 的 STOP/warm PHY 行为与实机收发未验证，H543/H553 的既有 RTC 限制未改动。
本包尚未写入主 vendor、型号目录或 App feature。
