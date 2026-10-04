# STM32F745/F746/F756 OTP 元数据修正

修正 21 个已有型号的 OTP 用户区地址和容量：起始地址为 `0x1FF0F000`，大小为 1,024 字节。依据固定版本 ST CMSIS 的三个准确型号头文件；`sources.json` 保存 URL 和 SHA。锁定字节和保留区不计入普通 OTP 用户区，不执行任何硬件写入。

保留原有 PAC feature、寄存器接口、HAL 驱动、应用 Flash/RAM、时钟和引脚数据。该包修正已有芯片，不增加新芯片支持数。`before/chips` 和 `changes.zip` 保存准确前像；补丁应用和恢复均要求完整来源匹配，拒绝部分发布和本地漂移。

从工程根目录重建：

```powershell
python data/patches/stm32f7-otp/regenerate.py --workspace . --output target/f7-otp-replay
```

需要已配置的 Rust 1.98.1、Python 3.11+ 及项目固定版本 PAC 生成器。编译与来源检查不能替代实机 OTP 验证；没有将锁定区访问或电气行为记为通过。
