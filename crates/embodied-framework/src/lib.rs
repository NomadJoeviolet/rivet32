#![no_std]

//! Public application-facing entry point. Framework crates never depend on App.

#[cfg(feature = "stm32")]
pub use embodied_stm32 as stm32;

pub use embodied_algorithms as algorithms;
pub use embodied_core as core;
pub use embodied_devices as devices;
pub use embodied_runtime as runtime;
