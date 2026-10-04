//! Board setup for @CHIP@. Assign each added peripheral/pin/DMA to one core.
//! Startup intentionally does not return duplicate HAL peripheral tokens.
pub fn init() {
    crate::dual_core::init();
}
