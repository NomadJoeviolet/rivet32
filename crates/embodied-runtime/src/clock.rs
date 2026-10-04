use core::future::Future;
use embodied_core::time::Instant;

/// A monotonic microsecond clock. Dropping a sleep cancels the logical wait.
/// Drivers may retain a harmless stale task wake (as Embassy does), but may not
/// complete a new sleep because an old deadline fired. Round deadlines upward.
pub trait Clock {
    fn now(&self) -> Instant;
    fn sleep_until(&self, deadline: Instant) -> impl Future<Output = ()>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConversionError {
    ZeroFrequency,
    Overflow,
}

/// Convert microseconds to hardware ticks without an early deadline or intermediate overflow.
pub fn micros_to_ticks_ceil(deadline: Instant, tick_hz: u64) -> Result<u64, ConversionError> {
    if tick_hz == 0 {
        return Err(ConversionError::ZeroFrequency);
    }
    let ticks = (u128::from(deadline.as_micros()) * u128::from(tick_hz)).div_ceil(1_000_000);
    u64::try_from(ticks).map_err(|_| ConversionError::Overflow)
}

/// Round observations downward; a tick that has not happened is never reported.
pub fn ticks_to_instant(ticks: u64, tick_hz: u64) -> Result<Instant, ConversionError> {
    if tick_hz == 0 {
        return Err(ConversionError::ZeroFrequency);
    }
    let micros = u128::from(ticks) * 1_000_000 / u128::from(tick_hz);
    u64::try_from(micros)
        .map(Instant::from_micros)
        .map_err(|_| ConversionError::Overflow)
}
