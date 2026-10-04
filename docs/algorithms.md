# 算法模块 / Algorithms

[中文手册](zh-CN/README.md) · [English handbook](en/README.md)

需要加入 PID 控制、滤波、姿态计算或数据校验时，先从下表选择模块。统一入口为 [`embodied_framework::algorithms`](../crates/embodied-algorithms/src/lib.rs)。算法只处理数据，不直接访问 STM32 外设；App 应用代码负责传入采样值、时间步长和参数，并限制输出范围。

Use the table below when adding PID control, filtering, attitude calculations or data checks. Import through `embodied_framework::algorithms`. The algorithms work on data without accessing STM32 peripherals directly. Your App code supplies samples, time steps and parameters, and sets output limits.

| 模块 / Module | 功能 / Function |
|---|---|
| [pid](../crates/embodied-algorithms/src/pid.rs), [filter](../crates/embodied-algorithms/src/filter.rs) | PID（比例、积分、微分）控制和滤波 / PID (proportional, integral, derivative) control and filtering |
| [matrix](../crates/embodied-algorithms/src/matrix.rs), [vector](../crates/embodied-algorithms/src/vector.rs) | 固定大小的矩阵和向量 / Fixed-size matrices and vectors |
| [kalman](../crates/embodied-algorithms/src/kalman.rs), [qekf](../crates/embodied-algorithms/src/qekf.rs), [rls](../crates/embodied-algorithms/src/rls.rs) | 状态估计和递归最小二乘，用测量值估计状态或模型参数 / State estimation and recursive least squares, for estimating states or model parameters from measurements |
| [quaternion](../crates/embodied-algorithms/src/quaternion.rs), [transform](../crates/embodied-algorithms/src/transform.rs) | 姿态计算和坐标变换 / Attitude calculations and coordinate transforms |
| [power](../crates/embodied-algorithms/src/power.rs), [sigmoid](../crates/embodied-algorithms/src/sigmoid.rs) | 功率模型和平滑插值 / Power models and smooth interpolation |
| [crc](../crates/embodied-algorithms/src/crc.rs), [ring](../crates/embodied-algorithms/src/ring.rs), [fsm](../crates/embodied-algorithms/src/fsm.rs) | CRC 数据校验、固定容量缓冲区和有限状态机 / CRC data checks, fixed-capacity buffers and finite state machines |

调用前明确每个参数的单位、采样周期和有效范围。矩阵不可逆、输入含 NaN 或无穷大、缓冲区已满、估计失败时，要处理接口返回的错误，避免把无效结果送到电机。算法内部状态由控制任务负责修改；通信任务只把收到的数据副本传给它。实际使用的参数需要根据采样情况、执行延迟和负载确定。

Specify each parameter's units, sampling period and valid range before calling an algorithm. Handle returned errors for matrices that cannot be inverted, NaN or infinite inputs, full buffers and failed estimates before a result reaches motor output. Let the control task own and update algorithm state; communication tasks should pass it copies of received data. Choose operating parameters using actual sampling, actuation delay and load behavior.
