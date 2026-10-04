#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embodied_app::boards::selected as board;
use embodied_framework::core::init::{InitRegistry, InitStage};
use embodied_framework::stm32::hal;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut peripherals = None;
    let mut resources = None;
    let mut init = InitRegistry::<(), core::convert::Infallible, 0>::new();
    let ready = init
        .run(&mut (), |stage, _| {
            match stage {
                InitStage::PreCore => {
                    #[cfg(feature = "board-stm32h723vg")]
                    assert!(
                        !cortex_m::peripheral::SCB::dcache_enabled(),
                        "H723 reference profile requires DCache disabled"
                    );
                }
                InitStage::Env => peripherals = Some(hal::init(board::clock_config())),
                InitStage::Device => {
                    resources = Some(board::Resources::new(peripherals.take().unwrap()))
                }
                _ => {}
            }
            Ok(())
        })
        .unwrap();
    ready.start(|| {
        defmt::info!(
            "reference board {}: initialized, actuator outputs disabled",
            board::NAME
        )
    });
    let resources = resources.unwrap();
    loop {
        // Keep ownership and clocks alive. Applications can move individual
        // public fields into tasks after Ready; this firmware transmits no data.
        core::hint::black_box(&resources);
        Timer::after(Duration::from_secs(1)).await;
        defmt::info!("reference board {}: idle", board::NAME);
    }
}
