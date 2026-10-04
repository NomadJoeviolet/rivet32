#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embodied_framework::core::init::{InitRegistry, InitStage};
use embodied_framework::core::time::{Duration, Periodic};
use embodied_framework::runtime::{
    clock::Clock,
    embassy::{EmbassyClock, next_tick},
};
#[cfg(not(embodied_dual_core))]
use embodied_framework::stm32::hal;
use panic_probe as _;

#[cfg(embodied_dual_core)]
#[path = "../dual_core.rs"]
mod dual_core;

#[derive(Default)]
struct Context {
    #[cfg(not(embodied_dual_core))]
    peripherals: Option<hal::Peripherals>,
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut context = Context::default();
    let mut init = InitRegistry::<Context, core::convert::Infallible, 4>::new();
    let ready = init
        .run(&mut context, |stage, _ctx| {
            if stage == InitStage::Env {
                #[cfg(embodied_dual_core)]
                dual_core::init();
                #[cfg(not(embodied_dual_core))]
                // Internal oscillator defaults: no external crystal/board assumption.
                {
                    _ctx.peripherals = Some(hal::init(Default::default()));
                }
            }
            Ok(())
        })
        .unwrap();
    ready.start(|| spawner.spawn(heartbeat().unwrap()));
}

#[embassy_executor::task]
async fn heartbeat() {
    let mut schedule = Periodic::new(EmbassyClock.now(), Duration::from_secs(1)).unwrap();
    let mut count = 0_u64;
    loop {
        let tick = next_tick(&mut schedule).await.unwrap();
        count = count.wrapping_add(1);
        defmt::info!("App heartbeat={} missed={}", count, tick.missed);
    }
}
