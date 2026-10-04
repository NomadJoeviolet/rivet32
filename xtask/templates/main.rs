#![no_std]
#![no_main]

mod boards;
mod tasks;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embodied_framework::core::init::{InitRegistry, InitStage};
use embodied_framework::stm32::hal;
use panic_probe as _;

#[derive(Default)]
struct Context {
    peripherals: Option<hal::Peripherals>,
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut context = Context::default();
    let mut init = InitRegistry::<Context, core::convert::Infallible, 4>::new();
    let ready = init
        .run(&mut context, |stage, ctx| {
            if stage == InitStage::Env {
                ctx.peripherals = Some(boards::init());
            }
            Ok(())
        })
        .unwrap();
    ready.start(|| spawner.spawn(tasks::heartbeat().unwrap()));
}
