use crate::wait::{WaitError, WaitQueue};
use core::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};
use core::{
    future::{Future, poll_fn},
    task::Poll,
};
use critical_section::Mutex;

/// Reusable owned messages. Slots and leases allocate no memory.
/// Use `from_slots` for const statics or `new` during startup before sharing.
pub struct MessagePool<T, const N: usize> {
    slots: Mutex<RefCell<[Option<T>; N]>>,
    waiters: WaitQueue,
}

impl<T, const N: usize> MessagePool<T, N> {
    pub fn new(values: [T; N]) -> Self {
        Self::from_slots(values.map(Some))
    }
    /// `None` entries are spare slots, not messages; they do not add available capacity.
    pub const fn from_slots(values: [Option<T>; N]) -> Self {
        Self {
            slots: Mutex::new(RefCell::new(values)),
            waiters: WaitQueue::new(),
        }
    }

    pub fn try_acquire(&self) -> Option<MessageLease<'_, T, N>> {
        critical_section::with(|cs| {
            let mut slots = self.slots.borrow(cs).borrow_mut();
            let value = slots.iter_mut().find_map(Option::take)?;
            Some(MessageLease {
                pool: self,
                value: Some(value),
            })
        })
    }

    pub fn available(&self) -> usize {
        critical_section::with(|cs| {
            self.slots
                .borrow(cs)
                .borrow()
                .iter()
                .filter(|s| s.is_some())
                .count()
        })
    }
    pub async fn acquire(&self) -> Result<MessageLease<'_, T, N>, WaitError> {
        let mut registration = self.waiters.registration();
        poll_fn(|cx| {
            critical_section::with(|_| match self.try_acquire() {
                Some(lease) => Poll::Ready(Ok(lease)),
                None => registration.pending(cx),
            })
        })
        .await
    }
    pub async fn acquire_timeout(
        &self,
        timeout: impl Future<Output = ()>,
    ) -> Result<MessageLease<'_, T, N>, WaitError> {
        crate::wait::with_timeout(self.acquire(), timeout).await?
    }
}

/// Unique ownership of a message until Drop, including cancellation of a holding future.
/// Deliberately neither Clone nor manually releasable: the message returns exactly once.
pub struct MessageLease<'a, T, const N: usize> {
    pool: &'a MessagePool<T, N>,
    value: Option<T>,
}

impl<T, const N: usize> Deref for MessageLease<'_, T, N> {
    type Target = T;
    fn deref(&self) -> &T {
        self.value
            .as_ref()
            .expect("live lease always contains a message")
    }
}

impl<T, const N: usize> DerefMut for MessageLease<'_, T, N> {
    fn deref_mut(&mut self) -> &mut T {
        self.value
            .as_mut()
            .expect("live lease always contains a message")
    }
}

impl<T, const N: usize> Drop for MessageLease<'_, T, N> {
    fn drop(&mut self) {
        let value = self.value.take();
        critical_section::with(|cs| {
            let mut slots = self.pool.slots.borrow(cs).borrow_mut();
            let slot = slots
                .iter_mut()
                .find(|s| s.is_none())
                .expect("acquired message reserves a free return slot");
            *slot = value;
        });
        self.pool.waiters.wake_all();
    }
}
