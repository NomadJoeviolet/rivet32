#![no_std]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! STM32 adapters with peripheral ownership retained in Embassy HAL types.
//!
//! * [`bus`] adapts embedded-hal SPI/CS/delay for IMUs and provides generic PWM control.
//! * [`can`] preserves classic/FD frame formats and reports mailbox replacements.
//! * `uart` (when the chip has USART) provides DMA byte-stream adapters.
//! * `usb` (opt-in feature) provides CDC byte streams, buffering and ZLP flushing.
//!
//! Constructors accept already configured HAL instances: the application owns
//! clock, pin, IRQ, DMA memory/cache and board supply choices. The build script
//! reads the selected chip's exact metapac metadata to expose controller-specific
//! APIs. A chip without USB cannot enable the `usb` feature; a chip without CAN
//! exposes conversion helpers but no hardware `CanAdapter`.
//!
//! CAN Pending cancellation is non-consuming; UART and USB byte streams may
//! have transferred a prefix when cancelled. See each adapter's contract.
//! Compiling a peripheral adapter is separate from board/HIL verification.

pub use embedded_hal;
pub use embedded_hal_async;
pub use embedded_io_async;

pub mod bus;
pub mod can;
pub mod hardware_timer;

#[cfg(all(feature = "hal", stm32_has_usart))]
pub mod uart;

#[cfg(all(feature = "usb", any(not(feature = "hal"), stm32_has_usb)))]
pub mod usb;

#[cfg(all(feature = "usb", feature = "hal", not(stm32_has_usb)))]
compile_error!("the selected STM32 chip has no USB peripheral");

/// Type-safe peripheral ownership and pin/alternate-function checking.
/// Available only when exactly one chip/core feature is selected.
#[cfg(feature = "hal")]
pub use embassy_stm32 as hal;
