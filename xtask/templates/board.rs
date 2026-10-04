//! Configure clocks and pins from the schematic and device manuals here.
use embodied_framework::stm32::hal;

pub fn init() -> hal::Peripherals {
    // Internal oscillator defaults. Configure your board's clocks and pins explicitly.
    hal::init(Default::default())
}
