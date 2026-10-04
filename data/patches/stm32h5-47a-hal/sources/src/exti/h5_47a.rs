//! GPIO-only EXTI adaptation for the exact H5E4/E5/F4/F5 DIE47A PAC.
//!
//! GPIO ownership, interrupts, waiting and cancellation use the public HAL
//! driver. This module deliberately does not expose internal EXTI lines.

use super::h5_47a_regs::GpioRegisters;
use crate::gpio::{Input, Pin as GpioPin, PinNumber};
use crate::pac::EXTI;

/// GPIO edge triggering mode.
pub enum TriggerEdge {
    /// Falling edge.
    Falling,
    /// Rising edge.
    Rising,
    /// Both rising and falling edges.
    Any,
}

/// GPIO interrupt mask state.
pub enum InterruptState {
    /// Unmask interrupt requests.
    Enabled,
    /// Mask interrupt requests.
    Disabled,
}

impl From<InterruptState> for bool {
    fn from(state: InterruptState) -> bool {
        matches!(state, InterruptState::Enabled)
    }
}

pub(super) fn configure_exti_pin(pin: PinNumber, port: PinNumber, trigger_edge: TriggerEdge) {
    critical_section::with(|_| {
        let registers = GpioRegisters(EXTI);
        registers.route(pin as usize, port as usize);
        let (rising, falling) = match trigger_edge {
            TriggerEdge::Falling => (false, true),
            TriggerEdge::Rising => (true, false),
            TriggerEdge::Any => (true, true),
        };
        registers.edges(pin as usize, rising, falling);
        registers.clear_pending(1 << pin);
    });
}

pub(super) fn set_exti_interrupt_enabled(pin: PinNumber, state: InterruptState) {
    critical_section::with(|_| GpioRegisters(EXTI).set_enabled(pin as usize, state.into()));
}

pub(super) fn configure_and_enable_exti(pin: &Input, trigger_edge: TriggerEdge) {
    critical_section::with(|_| {
        configure_exti_pin(pin.pin.pin.pin(), pin.pin.pin.port(), trigger_edge);
        set_exti_interrupt_enabled(pin.pin.pin.pin(), InterruptState::Enabled);
    });
}

pub(super) fn mask_interrupts(mask: u32) {
    critical_section::with(|_| GpioRegisters(EXTI).mask_interrupts(mask));
}

pub(super) fn is_interrupt_enabled(pin: PinNumber) -> bool {
    GpioRegisters(EXTI).is_enabled(pin as usize)
}

pub(super) fn read_pending() -> u32 {
    GpioRegisters(EXTI).pending()
}

pub(super) fn clear_exti_pending_mask(mask: u32) {
    GpioRegisters(EXTI).clear_pending(mask);
}

pub(super) fn clear_exti_pending(pin: PinNumber) {
    assert!(pin < 16);
    critical_section::with(|_| clear_exti_pending_mask(1 << pin));
}

pub(super) fn is_exti_pending(pin: PinNumber) -> bool {
    assert!(pin < 16);
    read_pending() & (1 << pin) != 0
}
