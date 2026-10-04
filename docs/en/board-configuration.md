# 3. Rust board configuration

<!-- handbook-nav -->
[Handbook index](README.md) · [简体中文](../zh-CN/board-configuration.md) · [Project README](../../README.en.md)

Board configuration describes the chip, clocks, pins and peripherals on a particular board. This project keeps it in App's Rust code, with `embassy-stm32` drivers initializing the MCU. Before editing it, obtain the schematic for your board revision, the chip datasheet and its reference manual.

## From planning to registers

```mermaid
flowchart LR
    S[Schematic and device manuals] --> R[Rust board configuration]
    R --> H[embassy-stm32 HAL]
    H --> P[stm32-metapac]
    P --> M[MCU registers]
```

| Layer | Responsibility | Use in this project |
|---|---|---|
| STM32 C HAL | ST's C driver library | Not linked into standard Rust firmware |
| `embedded-hal` | Common device and bus interfaces, expressed as Rust traits | Lets device drivers work with different MCUs; it is not part of Embassy and does not initialize a chip |
| `embassy-stm32` | Concrete STM32 Rust HAL | Controls clocks, GPIO, DMA, peripherals and interrupt integration |
| `stm32-metapac` | Typed register access and chip metadata | Supplies exact-device definitions to the HAL |
| `embassy-executor` | Schedules async tasks | Checks whether a task can continue after an interrupt or timer wakes it |

The HAL is the hardware driver layer. Applications create peripherals through constructors, then call methods to read and write. A PAC provides register access; using it directly means managing register state, exclusive peripheral use and interrupt synchronization yourself. A trait is Rust's way to specify which operations a type must provide.

Use `embodied_framework::stm32::hal` to access the drivers. It exposes the version of the Embassy HAL fixed by this repository.

## A board configuration checklist

Record the exact part, package and board revision, together with the supply arrangement, HSE/LSE type and frequency, Flash bank/TrustZone settings, debug interface and peripheral wiring. Different revisions of a board may need different configurations.

Configure the Rust board directly in this order:

1. Identify the exact chip, package and pins from the schematic and datasheet; select the matching App chip feature.
2. Record each peripheral instance, TX/RX or SCK/MOSI/MISO, CS/enable pins, initial GPIO levels and pulls.
3. Check the clock tree: system clock, bus dividers, the clocks driving each peripheral (kernel clocks), USB's 48 MHz source and supply/voltage settings.
4. Check interrupts (IRQs) and DMA requests/channels for resources also used by the timebase, PWM or hardware compare events.
5. Fill in `hal::Config` in Rust `clock_config()`. Call `hal::init` once during Env, the hardware-environment stage, and construct peripherals during Device, the device-initialization stage.
6. Assign ownership to application tasks, then verify the build, wiring and actual transfers in that order.

A value such as `p.PA2` is called a token and represents the right to use PA2. Passing it to a peripheral constructor gives that peripheral ownership of the pin. The compiler uses the constructor's trait constraints to check that the alternate-function mapping is supported. Check assembly, external circuitry, voltage and board revision against the actual hardware.

## Concrete G474 reference-board mapping

The [STM32G474CE configuration](../../App/src/boards/stm32g474ce.rs) is for `STM32G474CET6`; its clocks are in [common.rs](../../App/src/boards/common.rs). A 12 MHz HSE feeds PLL `/3 ×85 /2` for 170 MHz SYSCLK. USB separately selects HSI48. These are this board's choices, not defaults for every G4.

| Function | Resources selected by this profile |
|---|---|
| FDCAN1 / 2 / 3 | PB8/PB9, PB5/PB6, PA8/PB4; internal loopback by default |
| USART1 RX | PA10, DMA1_CH2, 2,000,000 baud |
| USART2 RX | PA3, DMA1_CH3, 921,600 baud |
| IMU SPI2 | PB13/PB15/PB14; PB12/PB11 CS; 5,312,500 Hz |
| Encoder SPI1 | PA5/PA7/PA6; PB1/PB2 CS; 10,625,000 Hz |
| Servo PWM | TIM3 CH2 / PA4, 50 Hz; output disabled by default |
| USB | PA12/PA11, USB_LP IRQ |

Before reusing these resources, check the wiring, power and oscillators on your PCB. The reference firmware initializes resources; add sensor calibration and control tasks in your application.

## Reading GPIO, IRQ and DMA configuration

This illustrative GPIO constructor is not a standalone program. Import the HAL's `gpio::{Output, Level, Speed}`, obtain `p` from one HAL initialization and confirm PB0 is available on the board:

```rust
Output::new(p.PB0, Level::Low, Speed::Low)
```

PB0 starts low. UART/SPI/CAN construction also requires the peripheral instance, pins, DMA and the interrupt-handler bindings specified through `bind_interrupts!`. When reusing a UART constructor, check its TX/RX pins, clocks, DMA and interrupt bindings together.

A DMA buffer must remain valid throughout the transfer and sit in memory the DMA controller can access. `'static` describes how long it exists; it does not prove the address is accessible to DMA. H723 DMA1/2 needs reachable SRAM. If DCache, the data cache, is enabled, use cache maintenance or memory protection unit (MPU) configuration to keep CPU and DMA views of memory consistent. Check the memory layout for each H7 model.

## Time resources and special device configurations

- Embassy's time driver reserves a timer, which is then unavailable for PWM or hardware compare events. The F427 reference board selects TIM2. Check the configuration for other families.
- Assign peripherals, IRQs, Flash/RAM and time resources separately for each H7 core. The current configuration uses TIM5 for CM7 and TIM2 for CM4. See the [dual-core guide](../dual-core.md).
- H5 defaults assume TrustZone disabled. A complete product template for arbitrary secure/non-secure partitioning is not provided.
- Enable only required functions on small devices. Supporting an MCU does not imply CAN/USB hardware exists or that all modules fit simultaneously.

## Minimum work for a new board

Record the board revision and pin assignments in App, then follow an existing board module to write `clock_config()`. Store the constructed peripherals in a `Resources` structure whose fields use the corresponding HAL types.

Add a board feature in the application's Cargo configuration to select the exact chip, bank layout and time driver, then add the board to the selection module. Build the reference entry point or your own entry point with that feature, selecting one board at a time.

If the framework already supports the exact chip, adding a board usually only requires App changes. Adding an unsupported chip requires checking its PAC, memory, interrupt, clock and package data. An alias for a similar chip cannot establish that these settings are correct.

Firmware starts through `cortex-m-rt`; the Rust HAL initializes clocks, DMA and IRQs. Give each hardware resource a single owner and avoid duplicate initialization.

---

[Previous](application-guide.md) · [Index](README.md) · [Next](architecture.md)
