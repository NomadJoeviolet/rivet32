# STM32H5E/F PAC 生成输入候选包

覆盖 24 个准确型号。`inputs.zip` 保存 321 个固定生成输入；使用未修改的
stm32-data `caa36afd62510b0e6315ee0dccd1f9c65fbcac83` 生成器和 Rust 1.98.1。
`generated-pac.zip` 保存 579 个已验证输出，供逐文件比较与组合后端重放。
归档完整文件清单、内容哈希、组合后端期望值均在 `sources.json`。

`raw_guard.py` 保留已验证生成步骤中的原函数，仍将未独立确认访问语义的
1,214 个 getter 标成 Raw，要求调用者使用 unsafe 并自行确认寄存器语义。
它从包内 `sources/generate.py` 读取原有 Raw API 定义，不执行该历史脚本。
完成此步骤后追加 Cargo 的独立 workspace 表，并恢复已锁定的许可证、NOTICE
和 Cargo.lock，即可比较最终生成字节。不得省略 Raw 访问限制。

`references.zip` 以历史相对路径保存 ST CMSIS、SVD、封装引脚 XML、来源锁、
寄存器/芯片输入和派生步骤依据；历史路径用于追溯，不是重放所需的本机路径。
芯片和寄存器生成重放以 inputs.zip 为入口；本包尚未提供全部历史转换步骤的
自动串联重建命令。上游生成器源码归档复用仓库 data/sources/stm32-gaps 中的固定版本。

本包尚未推广至主 vendor 与型号目录。最小固件 38 项通过，组合外设批次仍在运行；
编译链接不替代实机验证，HIL 为 not-run。许可证分别见包内 MIT、Apache 和 ST/DFP 文本。
