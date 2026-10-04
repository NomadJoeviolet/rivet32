#![no_std]
#![no_main]

mod boards;
mod tasks;

#[path = "../../shared/dual_core.rs"]
mod dual_core;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embodied_framework::core::init::{InitRegistry, InitStage};
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut init = InitRegistry::<(), core::convert::Infallible, 4>::new();
    let ready = init
        .run(&mut (), |stage, _| {
            if stage == InitStage::Env {
                boards::init();
            }
            Ok(())
        })
        .unwrap();
    ready.start(|| spawner.spawn(tasks::heartbeat().unwrap()));
}
