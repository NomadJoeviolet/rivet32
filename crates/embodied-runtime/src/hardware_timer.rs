//! Portable exclusive hardware compare-channel cancellation contract.
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use embodied_core::time::Instant;

/// An exclusively owned hardware compare channel, never shared with an executor clock.
///
/// Implementations must clear old compare flags and registrations before arming,
/// reject stale IRQ generations, and make `cancel` synchronously disarm the channel,
/// acknowledge pending interrupts and remove its waker. A late IRQ after cancellation
/// must not complete a later alarm. `cancel` must be safe after a failed `arm`.
/// These are driver obligations; this trait does not claim target-specific validation.
pub trait HardwareTimerChannel {
    type Error;
    fn arm(&mut self, deadline: Instant) -> Result<(), Self::Error>;
    fn poll_expired(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
    fn cancel(&mut self);
}

/// A future exclusively borrowing one compare channel. Drop always cancels an
/// attempted arm, including driver errors and cancellation while pending.
pub struct HardwareAlarm<'a, C: HardwareTimerChannel> {
    channel: &'a mut C,
    deadline: Instant,
    armed: bool,
    complete: bool,
}
impl<'a, C: HardwareTimerChannel> HardwareAlarm<'a, C> {
    pub fn new(channel: &'a mut C, deadline: Instant) -> Self {
        Self {
            channel,
            deadline,
            armed: false,
            complete: false,
        }
    }
}
impl<C: HardwareTimerChannel> Future for HardwareAlarm<'_, C> {
    type Output = Result<(), C::Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        assert!(!this.complete, "completed hardware alarm polled again");
        if !this.armed {
            this.armed = true;
            if let Err(error) = this.channel.arm(this.deadline) {
                this.channel.cancel();
                this.armed = false;
                this.complete = true;
                return Poll::Ready(Err(error));
            }
        }
        let result = this.channel.poll_expired(cx);
        if result.is_ready() {
            this.channel.cancel();
            this.armed = false;
            this.complete = true;
        }
        result
    }
}
impl<C: HardwareTimerChannel> Drop for HardwareAlarm<'_, C> {
    fn drop(&mut self) {
        if self.armed {
            self.channel.cancel();
        }
    }
}
