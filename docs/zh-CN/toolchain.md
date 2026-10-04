# 1. 工具链安装与使用

<!-- handbook-nav -->
[手册目录](README.md) · [English](../en/toolchain.md) · [项目首页](../../README.md)

先装好 Rust，编译一个最小程序，再选择烧录工具把它写进 STM32。本章以 H723VG 为例，使用项目固定的 Rust 1.98.1 和 Edition 2024。版本写在 [rust-toolchain.toml](../../rust-toolchain.toml) 中。

第一次使用，按自己的系统阅读 Windows 或 Linux 安装步骤；当前目录已经有 `.tools/` 时，可以直接阅读局部工具链一节。编译成功后，从 probe-rs、Ozone、OpenOCD 中选一条烧录调试路线即可。STM32CubeProgrammer 也可用于单独烧录。

## 编译时需要选什么

| 配置 | 示例 | 决定什么 |
|---|---|---|
| 主机 host | `x86_64-pc-windows-msvc` | 运行 Rust 工具的电脑系统；Windows、Linux 的安装包不同 |
| MCU target | `thumbv7em-none-eabihf` | 为哪类 ARM 内核编译；H723 和 G474 都使用这一项 |
| 芯片 feature | `stm32h723vg` | 选择具体 STM32 型号，决定可用外设、引脚和内存 |

在固件编译命令中，`--target` 和 `--features` 要一起指定。可以把它们理解为“使用哪类 CPU”和“使用哪颗芯片”。项目里的构建辅助程序还要在电脑上运行，所以仓库没有把全部编译都设为 ARM。

## 需要安装哪些工具

| 工具 | 本项目用途 | 何时需要 |
|---|---|---|
| rustup | 安装和选择 Rust 工具链及 target | 安装、维护环境 |
| rustc / Cargo | rustc 是编译器；平时用 cargo 命令下载依赖和编译项目 | 日常开发，固定 1.98.1 |
| LLVM / rust-lld | 生成机器码，把编译结果合成固件文件 | 随 Rust 工具链提供，无需另装 |
| rustfmt / Clippy | 整理代码格式、检查常见代码问题 | 提交前和自动检查时 |
| Git | 获取仓库和固定 revision 的依赖 | 首次获取、依赖更新 |
| Python 3.11+ | 运行项目提供的生成和检查脚本 | 使用这些脚本时；直接用 Cargo 编译单核固件不需要 Python |
| MSVC Build Tools 或主机 GCC | 生成在电脑上运行的构建辅助程序 | 与所选 host 匹配 |
| VS Code + rust-analyzer | 编辑、补全、跳转、编译诊断 | 推荐开发环境 |
| probe-rs / Ozone / OpenOCD / CubeProgrammer | 通过 ST-LINK、J-Link 等探针写入固件；调试功能见后文 | 准备上板时选择一种 |
| LLVM tools / cargo-binutils | 尺寸分析、导出 BIN/HEX | 分析或发布时 |
| ARM GDB | 连接 OpenOCD，设置断点并查看程序状态 | 选择 OpenOCD 源码调试时 |

普通固件编译由 Rust 工具链完成，不需要安装 Keil 或 IAR。后文使用的 ARM GDB 负责调试，不参与 Rust 编译。从源码安装 probe-rs 等工具时，可能额外需要 CMake，按对应工具的安装说明处理。

## Windows 全新环境：推荐 MSVC host

1. 安装 [Git for Windows](https://git-scm.com/downloads/win)、[Python](https://www.python.org/downloads/windows/) 和 [VS Code](https://code.visualstudio.com/)。Python 选择 3.11 或以上，并确认 `python --version` 能执行。
2. 按 [rustup 的 MSVC 前置要求](https://rust-lang.github.io/rustup/installation/windows-msvc.html)安装 Visual Studio Build Tools 的 C++ 桌面工具和 Windows SDK。
3. 从 [Rust 官方安装页](https://rust-lang.org/tools/install/)运行适合电脑架构的 rustup 安装器。普通 x64 Windows 选择 MSVC host。已有 GNU/MSYS2 工具链的开发者可保留 GNU host，但必须提供匹配的主机链接器与运行库，参见 [Windows ABI 说明](https://rust-lang.github.io/rustup/installation/windows.html)。
4. 重新打开 PowerShell，在已获取并解压或克隆的仓库根目录执行：

```powershell
rustup toolchain install 1.98.1 --profile minimal --component rustfmt --component clippy
rustup target add --toolchain 1.98.1 thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabihf thumbv8m.main-none-eabihf
rustup component add rust-src --toolchain 1.98.1
rustc -Vv
cargo -V
rustup show active-toolchain
rustup target list --installed --toolchain 1.98.1
```

看到 `rustc 1.98.1`，并在已安装列表中看到四种 ARM target，说明 Rust 安装完成。进入仓库后，rustup 会按项目文件选择这个版本，无需修改全局默认版本。首次安装工具和下载依赖需要联网。

## 当前目录已有局部工具链

`.tools/` 是当前电脑上的局部安装目录，被 Git 忽略，也不包含在源码交付包中。别人下载仓库后仍需安装环境。只有目录中已经存在工具链时，才使用下面的激活命令。

[环境脚本](../../scripts/env.ps1)让当前终端从 `.tools/` 查找 Rust 和依赖，并关闭自动安装。它只设置 `PATH`、`CARGO_HOME`、`RUSTUP_HOME` 等进程环境变量，不会下载工具，也不修改系统环境。

PowerShell，仓库根目录，每次新开终端执行：

```powershell
. .\scripts\env.ps1
rustc -Vv
cargo -V
```

命令最前面的点和空格不能省略：它把设置加载到当前 PowerShell 中。执行后应能看到 Rust 和 Cargo 的版本。如果提示没有局部工具链，按 Windows 首次安装一节操作。

Git Bash，仓库根目录，确认 `.tools/cargo/bin/cargo.exe` 已存在后执行：

```bash
export RUSTUP_HOME="$(pwd -W)/.tools/rustup"
export CARGO_HOME="$(pwd -W)/.tools/cargo"
export RUSTUP_AUTO_INSTALL=0
export PATH="$PWD/.tools/cargo/bin:$PATH"
rustc -Vv
cargo -V
```

每次新开 Git Bash，都要重新设置这组变量。已经安装全局 rustup 的用户可直接运行 Cargo。PowerShell 的 `& .\script.ps1` 不能直接粘贴进 Git Bash；从 Bash 启动 PowerShell 子进程，也不会把环境设置带回原来的 Bash。

## Linux：Ubuntu / Debian

以下命令适用于 Ubuntu 24.04、Debian 12 等系统，在 Bash 中执行，工作目录不限。先安装系统工具，再从 [rustup 官方入口](https://rust-lang.org/tools/install/)安装 Rust：

```bash
sudo apt-get update
sudo apt-get install -y build-essential git curl python3 python-is-python3
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/embodied-rustup.sh
sh /tmp/embodied-rustup.sh --profile minimal --default-toolchain 1.98.1
. "$HOME/.cargo/env"
rustup component add rustfmt clippy rust-src --toolchain 1.98.1
rustup target add --toolchain 1.98.1 thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabihf thumbv8m.main-none-eabihf
rustc -Vv
python3 --version
```

`python-is-python3` 让后面的 `python` 命令调用 Python 3；也可以一直使用 `python3`。编译前进入仓库根目录。使用 WSL 时，在 WSL 内安装 Linux 工具链；连接探针还需要把 USB 设备转交给 WSL。

## 配置编辑器

先激活准备使用的工具链，再从同一终端进入仓库根目录：

```text
code --install-extension rust-lang.rust-analyzer
code stm32h723.code-workspace
```

打开后，按 `Ctrl+Shift+B` 可编译 H723 最小固件。编辑通用算法或协议时，可以打开 [framework.code-workspace](../../framework.code-workspace)。rust-analyzer 提供补全和错误提示，见 [VS Code Rust 支持](https://code.visualstudio.com/docs/languages/rust)。

换芯片时，修改工作区中的 target 和 feature，也要同步修改构建任务、`cargo.buildScripts.overrideCommand`、`check.overrideCommand`。后两项控制编辑器后台执行的命令，遗漏它们会让错误提示仍按旧芯片生成。

已有 VS Code 进程可能沿用启动时的环境。使用局部工具链时，关闭相关实例后从激活的终端启动；需要独立实例可采用官方支持的 `--user-data-dir`，例如在仓库根目录运行 `code --user-data-dir .tools/vscode-profile stm32h723.code-workspace`，该实例需单独安装扩展。参见 [命令行说明](https://code.visualstudio.com/docs/configure/command-line)。

## 编译第一个固件

在仓库根目录运行以下命令。PowerShell、Git Bash 和 Linux Bash 的写法相同，前提是 Rust 已安装或已激活。首次只想确认能生成固件，可以先执行最后一条 `cargo build`；前面的命令用于格式检查、代码检查和查询芯片：

```text
cargo fmt --all --check
cargo check --workspace --locked
cargo clippy --workspace --lib --bins --locked -- -D warnings
cargo xtask list-chips --chip stm32h723vg
cargo build -p embodied-app --bin minimal --release --locked --target thumbv7em-none-eabihf --features stm32h723vg
```

最后一条命令成功后，应生成 `target/thumbv7em-none-eabihf/release/minimal`。这是 ELF 固件文件，包含程序和调试信息，文件名没有扩展名。

命令里的 `-p embodied-app` 选择 App 软件包，`--bin minimal` 选择最小心跳入口，`--release` 使用发布编译设置。`--target` 选择 ARM 内核类型，`--features stm32h723vg` 选择芯片。`--locked` 要求沿用 `Cargo.lock` 中的依赖版本。仅在本机已经缓存全部依赖时使用 `--offline`。

| target | 当前范围内对应内核 / 系列 |
|---|---|
| `thumbv6m-none-eabi` | Cortex-M0/M0+；F0、G0 |
| `thumbv7m-none-eabi` | Cortex-M3；F1、F2 |
| `thumbv7em-none-eabihf` | Cortex-M4/M7；F3、F4、F7、G4、H7 |
| `thumbv8m.main-none-eabihf` | Cortex-M33；H5 |

换成其他 MCU 时，先查 [芯片目录](../../data/chips.json)和[支持策略](../../data/support-policy.json)。每个固件只选择一个芯片或内核，不要使用 `--all-features`。部分器件还要选择 Flash 的单区或双区布局，例如 G474RE；操作见[App 指南](application-guide.md)。

## 选择烧录和调试工具

先选一条适合现有探针的路线，不需要把所有工具都装上。烧录前连接 SWDIO、SWCLK、GND 和探针要求的参考电压，按板卡和探针需要连接 NRST。选择准确芯片型号。下文的 H723 示例只用于 H723VG 单核固件。

| 想做什么 | 可以使用 | 还需要什么 |
|---|---|---|
| 烧录后直接看本项目的心跳日志 | [probe-rs](#probe-rs烧录并看心跳) | 支持的探针，例如 ST-LINK |
| 在图形界面里断点、单步和查看变量 | [Ozone](#ozone用图形界面调试) | 通常使用 J-Link 或 J-Trace |
| 用命令行烧录，再通过 GDB 调试 | [OpenOCD](#openocd命令行烧录) | 匹配的探针配置；源码调试另装 ARM GDB |
| 使用 ST 的图形烧录界面 | STM32CubeProgrammer | ST-LINK 等受支持接口 |

这些工具的芯片支持范围各自独立。查不到准确器件时，先检查工具版本和设备列表。Ozone、OpenOCD 的步骤已按官方文档核对，本项目尚未完成这两条路线的实机测试。

### probe-rs：烧录并看心跳

从 [probe-rs 官方安装页](https://probe.rs/docs/getting-started/installation/)下载对应系统的预编译工具，可以省去在本机编译工具的时间和缓存。也可使用下面的源码安装方式。Linux 用户先按该页面安装 `pkg-config`、`libudev-dev`、`cmake` 和 `git`。在 Rust 已激活的终端中执行，工作目录不限：

```text
cargo install probe-rs-tools --locked
probe-rs --version
probe-rs list
probe-rs chip list
```

看到版本号后，再看 `list` 是否列出已插入的探针。`chip list` 用来核对工具所用的芯片名称；此时还没有写入 MCU。Linux 普通用户访问探针前，需要按[探针配置说明](https://probe.rs/docs/getting-started/probe-setup/)设置 udev 规则。Windows 的驱动要求也以该说明和探针厂商资料为准。

如果习惯图形烧录界面，可以安装 [STM32CubeProgrammer](https://www.st.com/en/development-tools/stm32cubeprog.html)，通过 ST-LINK/SWD 连接后打开刚生成的 ELF，执行下载和校验。它也支持 HEX、BIN；BIN 的起始地址需要另行指定。

### 用 probe-rs 启动固件

确认硬件是 H723VG，并且 `probe-rs chip list` 包含对应名称。在仓库根目录执行以下命令，它会写入 Flash、启动程序并打开日志：

```text
probe-rs run --chip STM32H723VGTx target/thumbv7em-none-eabihf/release/minimal
```

程序启动后，预期每秒出现一条 `App heartbeat=... missed=...`。RTT 通过调试接口传递日志，不需要连接串口或 LED。若没有输出，先按[排错章节](troubleshooting.md)检查探针、复位和 ELF 是否匹配。

换用 CubeProgrammer、Ozone 或 OpenOCD 时，继续使用同一份 ELF，并核对芯片和 Flash 布局。仓库没有针对所有型号的统一调试配置。双核固件需要分别选择内核和镜像，见[双核说明](../dual-core.md)。

### Ozone：用图形界面调试

Ozone 可以直接读取 Rust 编译出来的 ELF，并显示 Rust 源码、调用栈、变量和寄存器，见 [SEGGER 的 Rust 支持说明](https://www.segger.com/news/pr-240927-ozone-support-rust/)。下面采用 J-Link/SWD 连接。

1. 从 [SEGGER 下载页](https://www.segger.com/downloads/jlink/)安装适合操作系统的 Ozone 和 J-Link Software and Documentation Pack。打开 Ozone，确认能找到所连接的 J-Link。
2. 先用前面的 Cargo 命令生成固件。Ozone 负责下载和调试，修改 Rust 源码后仍需用 Cargo 重新编译。
3. 在 Ozone 新建项目，选择实际 MCU 对应的器件和 SWD 接口。示例使用 H723VG；若设备列表没有对应型号，先核对安装版本。
4. 把 `target/thumbv7em-none-eabihf/release/minimal` 选为程序文件。它没有 `.elf` 扩展名，必要时切换文件选择器的过滤条件。将调试项目保存为 `.jdebug`，以后可直接打开。
5. 开始下载调试。打开 `App/src/bin/minimal.rs`，在可执行语句上设置断点，然后运行。停下后可以单步、查看变量、内存和寄存器。完成后继续运行或结束会话。

操作界面以 [Ozone 入门说明](https://www.segger.com/products/development-tools/ozone-j-link-debugger/technology/getting-started-with-ozone/)为准。源码调试要加载与板上程序一致的 ELF；BIN/HEX 不包含同样的调试信息。[根 Cargo 配置](../../Cargo.toml)已为 release 保留 `debug = 2`，同时开启优化，因此部分变量仍可能显示为已优化掉。

### OpenOCD：命令行烧录

OpenOCD 通过配置文件选择探针和芯片。本节示例使用 ST-LINK 和单核 H723VG。换成其他芯片时，使用对应的 target 配置文件，并确认 Flash 分区和复位方式。

Windows 可以从 [OpenOCD 官方获取页面](https://openocd.org/pages/getting-openocd.html)进入其 Windows 发行包下载链接，或使用该页列出的 [xPack OpenOCD](https://xpack-dev-tools.github.io/openocd-xpack/docs/install/)。下载与电脑架构匹配的压缩包，保留完整目录结构解压，将其中 `bin` 目录加入用户 `PATH`，再新开终端。不要只复制 `openocd.exe`，它还需要配套的脚本和库。

Ubuntu/Debian 用户在 Bash 中执行，工作目录不限：

```bash
sudo apt-get install -y openocd gdb-multiarch
openocd --version
gdb-multiarch --version
```

这里一并安装了稍后调试需要的 GDB。Windows 用户若需要源码调试，从 [Arm GNU Toolchain 下载页](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads)选择适合主机、面向 `arm-none-eabi` 的工具包，将 `bin` 加入 `PATH`。这个工具包提供 ARM GDB，Rust 固件仍由 Cargo 编译。PowerShell 自检命令如下，工作目录不限：

```powershell
openocd --version
arm-none-eabi-gdb --version
```

确认版本号能显示后，在仓库根目录运行下面的命令。它会擦写 ELF 覆盖的 Flash 区域、校验内容，然后复位运行并退出：

```text
openocd -f interface/stlink.cfg -f target/stm32h7x.cfg -c "adapter speed 1000" -c "program target/thumbv7em-none-eabihf/release/minimal verify reset exit"
```

`interface/stlink.cfg` 选择探针，`target/stm32h7x.cfg` 选择 H7 芯片配置，`adapter speed 1000` 将调试时钟设为 1 MHz。预期终端报告写入和校验成功；这条命令本身不显示 App 心跳。命令格式见 [OpenOCD 烧录说明](https://openocd.org/doc/html/Flash-Programming.html)。如果提示找不到 `.cfg`，检查安装包里的 `scripts` 目录，必要时使用 `-s` 指定这个目录；不同发行包的安装位置不同。

### OpenOCD + GDB：设置断点和单步

打开两个终端，都进入仓库根目录。第一个终端启动 OpenOCD 并保持运行：

```text
openocd -f interface/stlink.cfg -f target/stm32h7x.cfg -c "adapter speed 1000"
```

看到 GDB 服务监听端口的提示后，在第二个终端打开 ELF。Windows 使用：

```text
arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/minimal
```

安装了上面的 Linux 软件包时，使用：

```text
gdb-multiarch target/thumbv7em-none-eabihf/release/minimal
```

接下来这些命令输入到 GDB 的 `(gdb)` 提示符中，不是输入到 PowerShell 或 Bash。示例使用默认第一个目标端口 3333；若 OpenOCD 输出不同端口，以实际输出为准：

```gdb
target extended-remote localhost:3333
monitor reset halt
load
monitor reset halt
info registers
continue
```

`load` 会写入固件。`monitor reset halt` 让 MCU 复位后暂停；`info registers` 查看寄存器，`continue` 继续运行。要调试源码，先在 GDB 中暂停程序，用 `break 文件名:行号` 给实际可执行行设置断点，再继续运行。断点停下后使用 `next`、`step` 和 `info locals` 单步或查看局部变量。连接和下载流程见 [OpenOCD 与 GDB 说明](https://openocd.org/doc/html/GDB-and-OpenOCD.html)。

只有烧录需求时，运行上一节的单条 OpenOCD 命令即可。GDB 调试时保留 ELF，并注意优化可能改变源码与机器指令的对应关系。H7 双核需要为两个内核分别配置调试目标，不能直接沿用本节单核命令。

### 已经能烧录，为什么看不到日志

本项目用 RTT 传输日志，用 `defmt` 对日志进行编码。RTT 负责传递字节，`defmt` 解码器再结合对应 ELF，把这些字节还原成文字。普通 RTT 文本窗口不能直接把它们当作字符串显示，见 [defmt 文档](https://defmt.ferrous-systems.com/)。

Ozone 和 OpenOCD 可以正常烧录、调试程序，但本章没有配置它们的 `defmt` 解码流程。首次确认心跳最方便的方式是使用前面的 `probe-rs run`。换工具前先结束原调试会话，让新工具能够连接探针。若以后改成普通 RTT 文本日志，需要同时修改应用的日志输出方式。

## 查看尺寸并导出 BIN / HEX

需要查看固件大小，或烧录软件要求 BIN/HEX 时，再安装 [cargo-binutils](https://github.com/rust-embedded/cargo-binutils)。在仓库根目录、已激活 Rust 的终端执行：

```text
rustup component add llvm-tools --toolchain 1.98.1
cargo install cargo-binutils --locked
rust-size target/thumbv7em-none-eabihf/release/minimal
rust-objcopy -O binary target/thumbv7em-none-eabihf/release/minimal minimal.bin
rust-objcopy -O ihex target/thumbv7em-none-eabihf/release/minimal minimal.hex
```

调试时保留 ELF，它包含函数名、源码位置等信息。HEX 自带烧录地址；BIN 只有数据，烧录时需要填写对应 ELF 的 Flash 起始地址。切换 MCU、Flash 分区或双核镜像后，重新确认这个地址。

## 哪一步失败，就从哪一步排查

先确认 `rustc -Vv` 显示 1.98.1，再确认最小固件能生成 ELF。若编译失败，检查主机链接器、ARM target 和依赖下载；若编辑器报错但终端正常，检查二者是否使用同一工具链。

生成 ELF 后再连接硬件。烧录失败通常先查探针识别、型号和接线；烧录成功但无日志，再查程序是否运行以及日志解码方式。详细处理见[排错章节](troubleshooting.md)。

---

[上一章](README.md) · [目录](README.md) · [下一章](application-guide.md)
