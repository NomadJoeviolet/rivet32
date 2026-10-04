# 设备模块 / Device modules

[中文手册](zh-CN/README.md) · [English handbook](en/README.md)

接入电机、遥控器、裁判系统或 IMU（惯性测量单元）时，从下表找到对应模块。统一入口是 [`embodied_framework::devices`](../crates/embodied-devices/src/lib.rs)。模块负责报文编解码和数值处理；App 应用代码负责配置总线、设备地址和单位，并决定断线后如何处理。

Use the table below when connecting a motor, remote control, referee system or IMU (inertial measurement unit). Import through `embodied_framework::devices`. The modules encode and decode packets and process values. Your App code configures the bus, device addresses and units, and decides what to do after a disconnection.

| 模块 / Module | 用途 / Purpose |
|---|---|
| [dji](../crates/embodied-devices/src/dji.rs) | DJI 电机反馈和分组 CAN 指令 / DJI feedback and grouped CAN commands |
| [damiao](../crates/embodied-devices/src/damiao.rs) | 达妙电机指令和反馈 / Damiao motor commands and feedback |
| [robstride](../crates/embodied-devices/src/robstride.rs) | RobStride CAN 协议 / RobStride CAN protocol |
| [lk](../crates/embodied-devices/src/lk.rs) | LK 235/236 指令和反馈 / LK 235/236 commands and feedback |
| [wfly](../crates/embodied-devices/src/wfly.rs) | WFLY 遥控器 SBUS 报文解码 / WFLY remote-control SBUS decoding |
| [referee](../crates/embodied-devices/src/referee.rs) | 裁判系统报文、CRC 校验和已支持的数据格式 / Referee frames, CRC checksums and supported payload formats |
| [imu](../crates/embodied-devices/src/imu.rs) | BMI088 / ICM42688 接口和标定 / BMI088 / ICM42688 interfaces and calibration |

收到数据后，先校验报文，再更新保存接收数据的 `Receiver` 或记录最近有效数据时间的 `Freshness`。无效报文不能延长设备的在线时间。时间以微秒计，时钟不能因校时等操作而倒退。接入设备前，核对模块支持的协议版本和报文长度；设备固件版本变化后也要重新核对。WFLY 的三档开关类型为 `wfly::Switch`。

Validate incoming packets before updating `Receiver`, which stores received data, or `Freshness`, which records when valid data last arrived. Invalid packets must not extend the device's online time. Use time in microseconds from a clock that never moves backwards. Check the module's supported protocol versions and packet lengths before connecting a device, and check again when its firmware version changes. The WFLY three-position switch type is `wfly::Switch`.

由 App 明确调用电机输出。每条 IMU 总线及其片选引脚都应由一处代码负责；将数据交给算法前，先确认量程、轴向、单位和标定。在线状态只说明近期收到了有效数据；确认设备执行命令或完成机械动作，还需要相应反馈，不能只看发送是否成功。

Issue motor output explicitly from App code. Give each IMU bus and chip-select path one owner in the code. Confirm ranges, axes, units and calibration before passing measurements to algorithms. Online status only indicates that valid data arrived recently. Use appropriate feedback to confirm command execution or completed motion; successful transmission alone cannot confirm either.
