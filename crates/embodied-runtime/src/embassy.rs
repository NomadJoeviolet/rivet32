//! Optional Embassy time/synchronization adapters; no STM32 dependency.
use crate::clock::{Clock, ConversionError, micros_to_ticks_ceil, ticks_to_instant};
use embodied_core::time::{Instant, Periodic, Tick, TimeError};

pub use embassy_sync;
pub type EmbassyMutex<T> =
    embassy_sync::mutex::Mutex<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, T>;

#[derive(Clone, Copy, Debug, Default)]
pub struct EmbassyClock;
impl Clock for EmbassyClock {
    fn now(&self) -> Instant {
        ticks_to_instant(
            embassy_time::Instant::now().as_ticks(),
            embassy_time::TICK_HZ,
        )
        .expect("Embassy clock exceeded u64 microsecond range")
    }
    /// Panics if the deadline exceeds the configured driver's representable tick
    /// range; call `try_deadline` first when handling untrusted timestamps.
    fn sleep_until(&self, deadline: Instant) -> impl core::future::Future<Output = ()> {
        embassy_time::Timer::at(
            try_deadline(deadline).expect("deadline exceeds Embassy driver range"),
        )
    }
}

pub fn try_deadline(deadline: Instant) -> Result<embassy_time::Instant, ConversionError> {
    micros_to_ticks_ceil(deadline, embassy_time::TICK_HZ).map(embassy_time::Instant::from_ticks)
}

pub async fn next_tick(periodic: &mut Periodic) -> Result<Tick, TimeError> {
    let clock = EmbassyClock;
    loop {
        clock.sleep_until(periodic.deadline()).await;
        if let Some(tick) = periodic.tick(clock.now())? {
            return Ok(tick);
        }
    }
}
