//! Exact H5E4/E5/F4/F5 GPIO EXTI register operations (lines 0..=15 only).
//!
//! Mutating operations are called while holding the HAL critical section.
//! Internal EXTI lines, security attribution, and third-bank events are not
//! configured here. Register types and addresses remain the exact native PAC.

use crate::pac::exti::{Exti, regs};

const GPIO_MASK: u32 = 0xffff;

pub(super) struct GpioRegisters(pub(super) Exti);

fn pin_mask(pin: usize) -> u32 {
    assert!(pin < 16, "GPIO EXTI pin must be in 0..16");
    1 << pin
}

impl GpioRegisters {
    pub(super) fn route(&self, pin: usize, port: usize) {
        let _ = pin_mask(pin);
        assert!(port < 16, "GPIO port encoding must fit four bits");
        let register = self.0.exticr(pin / 4);
        let shift = (pin % 4) * 8;
        // SAFETY: exact CMSIS EXTI_TypeDef exposes volatile EXTICR[4] at
        // 0x60 + 4*n. ST H5 HAL ba20038d, HAL_GPIO_Init, explicitly reads
        // and writes this array with a 0xF mask and 8-bit slot stride.
        // The SVD/CMSIS routing declarations disagree, hence PAC Raw.
        // These accesses use the compiled CMSIS layout and HAL expression.
        // pin/port bounds above and the caller's critical section confine
        // this access to one real GPIO routing field and preserve all others.
        let current = unsafe { register.read_unchecked() };
        let value = (current & !(0xf << shift)) | ((port as u32) << shift);
        // SAFETY: same proven RW register and critical section as the read;
        // only the selected four-bit port field differs from the read value.
        unsafe { register.write_unchecked(value) };
    }

    pub(super) fn edges(&self, pin: usize, rising: bool, falling: bool) {
        let mask = pin_mask(pin);
        self.0
            .rtsr1()
            .modify(|w| w.0 = (w.0 & !mask) | if rising { mask } else { 0 });
        self.0
            .ftsr1()
            .modify(|w| w.0 = (w.0 & !mask) | if falling { mask } else { 0 });
    }

    pub(super) fn set_enabled(&self, pin: usize, enabled: bool) {
        let mask = pin_mask(pin);
        self.0
            .imr1()
            .modify(|w| w.0 = (w.0 & !mask) | if enabled { mask } else { 0 });
    }

    pub(super) fn is_enabled(&self, pin: usize) -> bool {
        self.0.imr1().read().0 & pin_mask(pin) != 0
    }

    pub(super) fn mask_interrupts(&self, mask: u32) {
        self.0.imr1().modify(|w| w.0 &= !(mask & GPIO_MASK));
    }

    pub(super) fn pending(&self) -> u32 {
        (self.0.rpr1().read().0 | self.0.fpr1().read().0) & GPIO_MASK
    }

    pub(super) fn clear_pending(&self, mask: u32) {
        let mask = mask & GPIO_MASK;
        // ST HAL __HAL_GPIO_EXTI_CLEAR_{RISING,FALLING}_IT writes the
        // requested bits directly. Never RMW W1C pending registers: doing
        // so could clear another pin's pending flag read in the same word.
        self.0.rpr1().write_value(regs::ExtiRpr1(mask));
        self.0.fpr1().write_value(regs::ExtiFpr1(mask));
    }
}
