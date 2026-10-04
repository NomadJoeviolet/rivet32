use crate::wait::{WaitError, WaitQueue};
use core::{
    cell::RefCell,
    future::{Future, poll_fn},
    ops::{Deref, DerefMut},
    task::Poll,
};
use critical_section::Mutex;

/// Allocation-free async mutex. The guard owns the value, so no unsafe interior
/// references or critical sections held across await are necessary. Not FIFO.
pub struct AsyncMutex<T> {
    value: Mutex<RefCell<Option<T>>>,
    waiters: WaitQueue,
}

impl<T> AsyncMutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value: Mutex::new(RefCell::new(Some(value))),
            waiters: WaitQueue::new(),
        }
    }
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        critical_section::with(|cs| self.value.borrow(cs).borrow_mut().take()).map(|value| {
            MutexGuard {
                mutex: self,
                value: Some(value),
            }
        })
    }
    pub async fn lock(&self) -> Result<MutexGuard<'_, T>, WaitError> {
        let mut registration = self.waiters.registration();
        poll_fn(|cx| {
            critical_section::with(|_| match self.try_lock() {
                Some(guard) => Poll::Ready(Ok(guard)),
                None => registration.pending(cx),
            })
        })
        .await
    }
    pub async fn lock_timeout(
        &self,
        timeout: impl Future<Output = ()>,
    ) -> Result<MutexGuard<'_, T>, WaitError> {
        crate::wait::with_timeout(self.lock(), timeout).await?
    }
    pub fn into_inner(self) -> T {
        self.value
            .into_inner()
            .into_inner()
            .expect("mutex guard was forgotten")
    }
}

#[must_use = "dropping the guard unlocks the mutex"]
pub struct MutexGuard<'a, T> {
    mutex: &'a AsyncMutex<T>,
    value: Option<T>,
}
impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.value.as_ref().expect("live mutex guard")
    }
}
impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value.as_mut().expect("live mutex guard")
    }
}
impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        critical_section::with(|cs| {
            *self.mutex.value.borrow(cs).borrow_mut() = self.value.take();
        });
        self.mutex.waiters.wake_all();
    }
}
