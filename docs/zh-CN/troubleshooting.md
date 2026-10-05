# 8. 排错与常见问题

<!-- handbook-nav -->
[手册目录](README.md) · [English](../en/troubleshooting.md) · [项目首页](../../README.md)

先看问题发生在哪一步：工具是否装好，代码能否在电脑上编译，能否为 ARM 编译和链接，固件是否下载成功，以及板上是否正常运行。保留第一条实际错误、完整命令、工作目录和工具版本；后面的错误可能只是同一问题引起的。

## 怎样记录当前使用的工具

在仓库根目录、已激活环境的终端运行：

```text
rustc -Vv
cargo -V
rustup show active-toolchain
rustup target list --installed --toolchain 1.98.1
python --version
git status --short
```

硬件问题还需要完整料号和板版本、Flash bank 与 TrustZone 配置、探针型号，以及所用调试工具的版本。使用 probe-rs 时运行 `probe-rs --version`；使用 OpenOCD 时运行 `openocd --version`；使用 Ozone 时记录其版本和 J-Link 软件版本。例如，同系列芯片的封装和内存大小可能不同，仅提供“STM32 编译不了”无法复现问题。局部安装的工具链需要在当前终端激活；VS Code 和另外打开的终端可能使用不同环境。

## 找不到工具或命令报语法错误

| 现象 | 常见原因 | 处理方法 |
|---|---|---|
| 找不到 cargo/rustc | PATH 未刷新，或局部环境未激活 | 重开终端；局部安装按工具链章节激活 |
| 版本不是 1.98.1 | 不在仓库目录，或使用了另一套 rustup 环境 | 检查 active-toolchain、CARGO_HOME/RUSTUP_HOME 和当前目录 |
| `syntax error near unexpected token '&'` | 在 Git Bash 中输入了 PowerShell 命令 | 使用对应 shell 的命令，或打开 PowerShell 执行 |
| 局部 env.ps1 报工具不存在 | 源码包不包含开发者本机的 `.tools` | 按工具链章节重新安装，生成自己的工具路径 |
| 找不到 `link.exe` / `gcc` | 缺少电脑上使用的链接器 | MSVC 环境安装 Build Tools/SDK；GNU 环境安装匹配的 MSYS2/GCC |
| Python 不支持 tomllib | 实际运行的 Python 低于 3.11 | 使用 3.11+，并核对终端找到的是哪个解释器 |
| VS Code 仍按旧 feature 检查 | 进程保留旧环境，或两个后台检查命令配置不一致 | 从激活后的终端启动 VS Code，同步工作区 target/features/overrideCommand |

`&` 是 PowerShell 的调用运算符。用 `.` 点加载脚本会在当前作用域执行它，而用 `&` 调用脚本会为脚本创建子作用域。激活工具链时，应使用[工具链章节](toolchain.md)对应 shell 的完整命令块。

## 代码不能编译或固件不能链接

编译检查类型和生成机器代码，链接则把编译结果放到芯片的 Flash/RAM 地址中。报错中的 target 是编译目标，feature 是 Cargo 的编译时开关。

| 现象 | 应检查的内容 |
|---|---|
| `can't find crate for core` | 固定工具链是否已安装所选 ARM target；这里的 core 是 Rust 核心库 |
| duplicate symbols / 多个 MCU feature | 去掉 `--all-features`，检查各依赖是否一起启用了互斥 chip/core 或 time-driver；同名符号可能来自重复配置 |
| 找不到 `memory.x` | 是否正在构建 App binary，是否选择了一个准确芯片并使用正确 target；memory.x 提供芯片内存布局 |
| Flash bank 相关错误 | 对支持配置 bank 的 MCU，选择与 option bytes 一致的 single/dual-bank |
| 引脚 trait 不匹配 | 核对准确封装、外设实例和 AF（引脚复用）映射；类型检查发现的冲突应通过改正配置解决，不能用 unsafe 绕过 |
| 没有 CAN/USB 接口 | 所选芯片是否实际具备该外设，以及对应 feature 是否开启 |
| excluded from support | 当前支持策略已排除此型号；历史源码仍存在不改变其支持状态 |
| 主机编译出现裸机链接错误 | 电脑上运行的 workspace 检查不应指定 ARM target；根配置也不应统一强制 MCU target |
| `--locked` 失败 | Cargo 认为依赖和锁文件不一致；先核对依赖变更来源，再评审锁文件更新 |
| 首次编译宏或芯片数据很慢 | 大型 PAC 寄存器数据、初次 Git 依赖下载和编译时运行的过程宏需要时间，查看 Cargo 是否仍有进度 |

定位某个型号的问题时，先构建最小固件，再构建所需外设的示例。最小固件能链接，说明这一基础配置完成了编译链接；外设初始化还需要对应的构建和板上检查。

## 固件能下载，板上却没有预期行为

| 现象 | 排查顺序 |
|---|---|
| `probe-rs list` 看不到探针 | USB 连接、驱动/udev 规则、探针固件、工具版本 |
| 找不到准确芯片 | 核对 probe-rs 芯片数据库；必要时使用支持该料号的 CubeProgrammer |
| 能下载但没有 RTT 日志 | 当前 ELF 是否与固件一致，接着检查供电、复位、启动地址、早期 panic、时钟和探针连接 |
| Ozone 找不到源码或断点位置不对 | 载入与当前固件一致的 ELF，确认源码路径仍有效；release 优化可能合并语句或移除变量 |
| OpenOCD 找不到配置文件 | 保留安装包的完整目录结构，检查 scripts 路径；必要时用 -s 指定它 |
| GDB 连接失败 | 确认 OpenOCD 仍在运行，并使用它报告的 GDB 端口；先结束占用探针的其他调试会话 |
| 普通 RTT 窗口显示乱码 | 本项目使用 defmt 编码，需要配套解码；先按[工具链手册](toolchain.md)用 probe-rs run 查看心跳 |
| `missed` 持续增长 | 周期任务未按时执行；检查阻塞操作、日志量、算法耗时、资源争用和时间配置 |
| UART 只收到部分数据 | 检查波特率、DMA/IRQ 和缓冲大小，再检查是否把线路空闲（idle）当作一帧结束，以及超时后能否重新找到协议帧边界 |
| SPI/IMU 数据异常 | 核对 CPOL/CPHA、频率和 CS，再查量程、初始化及共享总线的访问顺序 |
| CAN 没有真实总线数据 | 确认示例是否仍用内部回环，再查收发器、滤波、速率、终端电阻和错误状态 |
| USB CDC 不枚举 | 检查 USB 时钟、PHY 和供电配置，然后检查线缆、IRQ 及 `UsbDevice::run` 是否持续运行 |
| H7 DMA 错误或数据过期 | 检查 DMA 能否访问该 RAM 区域、缓冲地址和有效时间，以及 CPU 的 DCache 与 RAM 中的数据是否一致 |
| PWM 无输出 | 参考示例默认关闭通道；核对板级配置后再显式启用 |

RTT 通过调试探针传日志，不需要 UART 接线。最小固件没有点灯行为，应通过它的日志判断运行状态。`sent` 只说明 CAN 驱动接受了帧；是否收到物理总线 ACK、对端是否执行，还要另外观察。

## 下载依赖卡住或缓存太大

若 `fetch_source_documents.py` 报 `Downloaded document SHA-256 differs`，说明参考资料的下载内容与固定版本不一致，此时还没有开始外设编译。日志会列出预期和实际哈希、字节数、HTTP 状态及内容类型。脚本最多尝试 3 次；若仍失败，检查镜像站是否返回错误页或更换了文件，不要直接修改预期哈希，也不要关闭校验。

TLS 或代理报错时，检查网络、系统时间、Git/Cargo 代理和证书环境，保留 TLS 校验。出现 `Blocking waiting for file lock on package cache`，通常是另一个 Cargo 进程正在使用包缓存。先查看那个进程是否还在下载或构建；为同一次下载再开多个任务，只会增加等待。离线模式需要所需依赖已完整缓存。

开发目录除了源码，还包含工具链、Cargo 的 Git/registry 下载内容和 target 编译结果。源码及必要来源数据低于 1 GB，其余占用需分别查看，见[存储管理](../storage.md)。

`EMBODIED_CACHE_LIMIT_MIB` 控制指定固件缓存在两次构建之间的回收，不能限制所有 target、并发作业和归档的总占用。清理前先确认没有构建正在使用缓存，并保存仍需使用的 ELF、MAP 和来源记录。用户应用和 vendor 是源码，要保留。各项说明见[存储报告](../storage.md)。

## 清理后为何看不到 coverage 本地记录

coverage 汇总的是相关构建命令写在 `target/` 下的报告。删除这些输出后，需要重新构建所需配置才会有新记录；仓库不附带历史测试记录。

## 修改工程时的常见问题

### Rust HAL 属于 Embassy 吗

`embedded-hal` 是独立的接口标准，用 trait 规定驱动应提供哪些操作。本项目使用的具体 STM32 HAL 是 `embassy-stm32`，属于 Embassy。HAL 驱动外设，执行器负责运行任务。

### 修改引脚应改哪里

在 `App/src/boards/` 的 Rust 构造器中修改具体引脚，同时核对 IRQ、DMA 和时钟，再编译对应板卡 feature。构造器是用这些配置创建驱动对象的函数。

### 为什么仓库中有寄存器与底层代码

vendor 保存 HAL 和 PAC 寄存器访问代码，本地还包含补充缺失型号、修复已确认问题的补丁。普通 App 开发使用现有 HAL 驱动即可。

### 编译通过后还需要测什么

上板检查焊接、时序、电源、总线负载和中断实际表现。类型检查和链接无法观察这些条件，硬件结论需要对应的实机记录。

### 能否把现有 FreeRTOS 任务直接改成 async

迁移时，要检查等待是否阻塞整个执行器、资源由谁持有、取消操作后留下什么状态，以及任务何时让出执行。函数名的替换无法解决这些差异。具体行为见[设计原理](design-and-implementation.md)。

## 问题报告应包含什么

先写如何复现、期望结果和实际结果，再附第一条错误、完整命令、工具版本，以及 chip/core/target/feature/bank 和修改过的内容。硬件问题还需板版本、接线和能重复观察到的现象。

日志可以只保留相关部分，但要保留源码提交或来源、锁文件状态。调试时使用与运行固件对应的 ELF，避免把不同版本的日志和符号文件混在一起。

---

[上一章](ci-cd.md) · [目录](README.md) · [下一章](README.md)
