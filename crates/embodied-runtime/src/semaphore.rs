use crate::wait::{WaitError, WaitQueue};
use core::cell::Cell;
use core::{
    future::{Future, poll_fn},
    task::Poll,
};
use critical_section::Mutex;

/// Counting semaphore with `N` initially available permits, safe across ISR/tasks.
/// Permit Drop returns a count without allowing over-release or double release.
pub struct CountingSemaphore<const N: usize> {
    available: Mutex<Cell<usize>>,
    waiters: WaitQueue,
}

impl<const N: usize> Default for CountingSemaphore<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> CountingSemaphore<N> {
    pub const fn new() -> Self {
        Self {
            available: Mutex::new(Cell::new(N)),
            waiters: WaitQueue::new(),
        }
    }
    pub fn try_acquire(&self) -> Option<Permit<'_, N>> {
        critical_section::with(|cs| {
            let available = self.available.borrow(cs);
            let count = available.get().checked_sub(1)?;
            available.set(count);
            Some(Permit { semaphore: self })
        })
    }
    pub fn available(&self) -> usize {
        critical_section::with(|cs| self.available.borrow(cs).get())
    }
    pub async fn acquire(&self) -> Result<Permit<'_, N>, WaitError> {
        let mut registration = self.waiters.registration();
        poll_fn(|cx| {
            critical_section::with(|_| match self.try_acquire() {
                Some(permit) => Poll::Ready(Ok(permit)),
                None => registration.pending(cx),
            })
        })
        .await
    }
    pub async fn acquire_timeout(
        &self,
        timeout: impl Future<Output = ()>,
    ) -> Result<Permit<'_, N>, WaitError> {
        crate::wait::with_timeout(self.acquire(), timeout).await?
    }
}

#[must_use = "dropping a permit immediately releases it"]
pub struct Permit<'a, const N: usize> {
    semaphore: &'a CountingSemaphore<N>,
}

impl<const N: usize> Drop for Permit<'_, N> {
    fn drop(&mut self) {
        critical_section::with(|cs| {
            let available = self.semaphore.available.borrow(cs);
            available.set(available.get() + 1);
        });
        self.semaphore.waiters.wake_all();
    }
}
