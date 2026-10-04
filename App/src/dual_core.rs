//! Minimal dual-core H7 startup. The two binaries own separate Flash/RAM and
//! independent executors. This module intentionally gives application code no
//! duplicate peripheral tokens: future peripherals need an explicit core owner.
use core::mem::MaybeUninit;
use embodied_framework::stm32::hal;

// SAFETY: App/build.rs places this exact symbol at 0x38000000 in both images,
// aligned to a cache line, NOLOAD and outside both cores' private RAM regions.
// Both images must use matching HAL features/revision and start after a full
// system reset. CM7 alone initializes it; CM4 reads after the HSEM publication.
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".embassy_shared")]
static __embodied_shared_data: MaybeUninit<hal::SharedData> = MaybeUninit::uninit();

pub fn init() {
    #[cfg(embodied_core_cm7)]
    {
        // Cortex-M7 reset leaves D-cache off. Turning it on later requires a
        // non-cacheable MPU region for this shared window. Reject an unsupported
        // bootloader cache state instead of publishing data visible only in L1.
        assert!(!cortex_m::peripheral::SCB::dcache_enabled());
        // Defaults use the internal oscillator; board supply requirements must
        // still match the selected HAL power configuration (see dual-core.md).
        let _peripherals = hal::init_primary(Default::default(), &__embodied_shared_data);
        defmt::info!("dual-core CM7 primary initialized; private TIM5 timebase");
    }
    #[cfg(embodied_core_cm4)]
    {
        let _peripherals = hal::init_secondary(&__embodied_shared_data);
        defmt::info!("dual-core CM4 secondary initialized; private TIM2 timebase");
    }
}
