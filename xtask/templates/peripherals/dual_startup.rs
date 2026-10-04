//! Full system reset, both boot addresses enabled, M7 D-cache disabled.
//! HSEM hardware bit 0 belongs to HAL SharedData publication. Bit 1 publishes
//! secondary HAL completion; bit 2 releases the probe after both HALs finish.
//! Only one core in each image pair owns a target constructor. Its peer retains
//! no driver tokens and only runs its private TIM5 (M7) or TIM2 (M4) timebase.
//! No low-power/STOP resume or later peer driver construction/drop is enabled.
use core::mem::MaybeUninit;
use core::sync::atomic::{compiler_fence, Ordering};
use embodied_stm32::hal;

// Same fixed address, NOLOAD section and HAL ABI as the App dual-core policy.
// Only M7 initializes it. No warm core reset or cacheable shared window is valid.
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".embassy_shared")]
static __embodied_shared_data: MaybeUninit<hal::SharedData> = MaybeUninit::uninit();

fn wait(core_index: usize, bit: usize) {
    // Sticky raw status supports a late receiver. Never clear before observing.
    while !hal::pac::HSEM.isr(core_index).read().isf(bit) {
        core::hint::spin_loop();
    }
    hal::pac::HSEM.icr(core_index).write(|w| w.set_isc(bit, true));
    cortex_m::asm::dmb();
    compiler_fence(Ordering::Acquire);
}

fn notify(bit: usize, bus_master: u8) {
    // Same 1-step acquire/release protocol as HAL blocking_notify; these two
    // hardware bits are exclusively reserved by this paired probe. M7 bus ID=3,
    // M4 bus ID=1 (HAL cpu::CoreId / ST RM0399). HSEM is already enabled by HAL.
    loop {
        let lock = hal::pac::HSEM.rlr(bit).read();
        if lock.lock() && lock.coreid() == bus_master && lock.procid() == 0 {
            break;
        }
    }
    compiler_fence(Ordering::Release);
    cortex_m::asm::dsb();
    hal::pac::HSEM.r(bit).write(|w| {
        w.set_coreid(bus_master);
        w.set_procid(0);
        w.set_lock(false);
    });
}

pub fn init() -> hal::Peripherals {
    #[cfg(embodied_core_cm7)]
    { primary() }
    #[cfg(embodied_core_cm4)]
    { secondary() }
}

#[cfg(embodied_core_cm7)]
fn primary() -> hal::Peripherals {
    assert!(!cortex_m::peripheral::SCB::dcache_enabled());
    let p = hal::init_primary(super::probe_config(), &__embodied_shared_data);
    wait(0, 1);
    notify(2, 3);
    p
}

#[cfg(embodied_core_cm4)]
fn secondary() -> hal::Peripherals {
    let p = hal::init_secondary(&__embodied_shared_data);
    notify(1, 1);
    wait(1, 2);
    p
}
