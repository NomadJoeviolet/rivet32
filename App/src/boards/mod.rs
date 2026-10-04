//! Board resources and initialization. See docs/reference-boards.md for wiring.
//! Constructors keep all motor PWM outputs disabled and CAN in internal loopback.
//! No UART, SPI, CAN or USB data is transmitted by the reference entry point.

pub mod common;

#[cfg(feature = "board-stm32h723vg")]
pub mod stm32h723vg;
#[cfg(feature = "board-stm32h723vg")]
pub use stm32h723vg as selected;
#[cfg(feature = "board-stm32g431kb")]
pub mod stm32g431kb;
#[cfg(feature = "board-stm32g431kb")]
pub use stm32g431kb as selected;
#[cfg(feature = "board-stm32g474ce")]
pub mod stm32g474ce;
#[cfg(feature = "board-stm32g474ce")]
pub use stm32g474ce as selected;
#[cfg(feature = "board-stm32f407ig")]
pub mod stm32f407ig;
#[cfg(feature = "board-stm32f407ig")]
pub use stm32f407ig as selected;
#[cfg(feature = "board-stm32f427ii")]
pub mod stm32f427ii;
#[cfg(feature = "board-stm32f427ii")]
pub use stm32f427ii as selected;

#[cfg(not(any(
    feature = "board-stm32h723vg",
    feature = "board-stm32g431kb",
    feature = "board-stm32g474ce",
    feature = "board-stm32f407ig",
    feature = "board-stm32f427ii"
)))]
compile_error!("select one board-* reference profile, not reference-boards alone");
