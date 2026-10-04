use crate::wait::{SendError, WaitError, WaitQueue};
use core::cell::RefCell;
use core::{
    future::{Future, poll_fn},
    task::Poll,
};
use critical_section::Mutex;
use heapless::Deque;

/// Owned fixed-capacity FIFO, with explicit front-priority insertion.
pub struct Queue<T, const N: usize> {
    values: Deque<T, N>,
}

impl<T, const N: usize> Default for Queue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Queue<T, N> {
    pub const fn new() -> Self {
        Self {
            values: Deque::new(),
        }
    }
    pub fn try_send(&mut self, value: T) -> Result<(), T> {
        self.values.push_back(value)
    }
    pub fn try_send_front(&mut self, value: T) -> Result<(), T> {
        self.values.push_front(value)
    }
    pub fn try_receive(&mut self) -> Option<T> {
        self.values.pop_front()
    }
    pub fn peek(&self) -> Option<&T> {
        self.values.front()
    }
    pub fn clear(&mut self) {
        self.values.clear();
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn is_full(&self) -> bool {
        self.values.is_full()
    }
    pub const fn capacity(&self) -> usize {
        N
    }
}

/// Critical-section protected queue, suitable for sharing between ISR and tasks.
/// The platform supplies a `critical-section` implementation; operations never await.
pub struct SharedQueue<T, const N: usize> {
    inner: Mutex<RefCell<Queue<T, N>>>,
    senders: WaitQueue,
    receivers: WaitQueue,
}

impl<T, const N: usize> Default for SharedQueue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> SharedQueue<T, N> {
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(RefCell::new(Queue::new())),
            senders: WaitQueue::new(),
            receivers: WaitQueue::new(),
        }
    }
    pub fn try_send(&self, value: T) -> Result<(), T> {
        let result =
            critical_section::with(|cs| self.inner.borrow(cs).borrow_mut().try_send(value));
        if result.is_ok() {
            self.receivers.wake_all();
        }
        result
    }
    pub fn try_send_front(&self, value: T) -> Result<(), T> {
        let result =
            critical_section::with(|cs| self.inner.borrow(cs).borrow_mut().try_send_front(value));
        if result.is_ok() {
            self.receivers.wake_all();
        }
        result
    }
    pub fn try_receive(&self) -> Option<T> {
        let result = critical_section::with(|cs| self.inner.borrow(cs).borrow_mut().try_receive());
        if result.is_some() {
            self.senders.wake_all();
        }
        result
    }
    /// A snapshot: another consumer may remove the value immediately afterward.
    pub fn peek(&self) -> Option<T>
    where
        T: Clone,
    {
        critical_section::with(|cs| self.inner.borrow(cs).borrow().peek().cloned())
    }
    pub fn clear(&self) {
        let old =
            critical_section::with(|cs| core::mem::take(&mut *self.inner.borrow(cs).borrow_mut()));
        // Drop message leases outside the queue's borrow, permitting reentrant resource returns.
        drop(old);
        self.senders.wake_all();
    }
    pub fn len(&self) -> usize {
        critical_section::with(|cs| self.inner.borrow(cs).borrow().len())
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub const fn capacity(&self) -> usize {
        N
    }
    pub fn is_full(&self) -> bool {
        self.len() == N
    }

    /// Wait for capacity. Cancellation drops the unsent value exactly once.
    pub async fn send(&self, value: T) -> Result<(), SendError<T>> {
        self.send_wait(value, false, core::future::pending()).await
    }
    pub async fn send_front(&self, value: T) -> Result<(), SendError<T>> {
        self.send_wait(value, true, core::future::pending()).await
    }
    /// The supplied future is an absolute deadline sleep or cancellation signal.
    /// It wins simultaneous readiness, and an unsent value is returned in the error.
    pub async fn send_timeout(
        &self,
        value: T,
        timeout: impl Future<Output = ()>,
    ) -> Result<(), SendError<T>> {
        self.send_wait(value, false, timeout).await
    }
    pub async fn send_front_timeout(
        &self,
        value: T,
        timeout: impl Future<Output = ()>,
    ) -> Result<(), SendError<T>> {
        self.send_wait(value, true, timeout).await
    }
    async fn send_wait(
        &self,
        value: T,
        front: bool,
        timeout: impl Future<Output = ()>,
    ) -> Result<(), SendError<T>> {
        let mut value = Some(value);
        let mut timeout = core::pin::pin!(timeout);
        let mut registration = self.senders.registration();
        let result = poll_fn(|cx| {
            if timeout.as_mut().poll(cx).is_ready() {
                return Poll::Ready(Err(WaitError::Timeout));
            }
            let result = critical_section::with(|cs| {
                let item = value.take().expect("pending send owns its value");
                let pushed = {
                    let mut inner = self.inner.borrow(cs).borrow_mut();
                    if front {
                        inner.try_send_front(item)
                    } else {
                        inner.try_send(item)
                    }
                };
                match pushed {
                    Ok(()) => Poll::Ready(Ok(())),
                    Err(item) => {
                        value = Some(item);
                        registration.pending(cx)
                    }
                }
            });
            if matches!(result, Poll::Ready(Ok(()))) {
                self.receivers.wake_all();
            }
            result
        })
        .await;
        result.map_err(|reason| SendError {
            reason,
            value: value.take().expect("failed send owns its value"),
        })
    }

    /// No message is removed until this future returns Ready(Ok(message)).
    pub async fn receive(&self) -> Result<T, WaitError> {
        let mut registration = self.receivers.registration();
        poll_fn(|cx| {
            let result = critical_section::with(|cs| {
                let value = self.inner.borrow(cs).borrow_mut().try_receive();
                match value {
                    Some(value) => Poll::Ready(Ok(value)),
                    None => registration.pending(cx),
                }
            });
            if matches!(result, Poll::Ready(Ok(_))) {
                self.senders.wake_all();
            }
            result
        })
        .await
    }
    pub async fn receive_timeout(&self, timeout: impl Future<Output = ()>) -> Result<T, WaitError> {
        crate::wait::with_timeout(self.receive(), timeout).await?
    }
}
