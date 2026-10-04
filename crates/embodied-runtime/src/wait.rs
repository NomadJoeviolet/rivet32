//! Fixed storage for async waiters. Waiting operations have no fairness guarantee.
//! Dropping an operation unregisters its waker; ISR callers use only `try_*`.
use core::{
    cell::RefCell,
    future::{Future, poll_fn},
    task::{Context, Poll, Waker},
};
use critical_section::Mutex;

/// Maximum simultaneous pending operations per resource (per direction for queues).
pub const MAX_WAITERS: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WaitError {
    TooManyWaiters,
    Timeout,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SendError<T> {
    pub reason: WaitError,
    pub value: T,
}

struct Slot {
    used: bool,
    waker: Option<Waker>,
}
struct State {
    slots: [Slot; MAX_WAITERS],
    epoch: u64,
}

pub(crate) struct WaitQueue {
    state: Mutex<RefCell<State>>,
}

impl WaitQueue {
    pub const fn new() -> Self {
        Self {
            state: Mutex::new(RefCell::new(State {
                slots: [const {
                    Slot {
                        used: false,
                        waker: None,
                    }
                }; MAX_WAITERS],
                epoch: 0,
            })),
        }
    }

    pub fn registration(&self) -> Registration<'_> {
        Registration {
            queue: self,
            slot: None,
        }
    }

    pub fn wake_all(&self) {
        let wakers = critical_section::with(|cs| {
            let mut state = self.state.borrow(cs).borrow_mut();
            state.epoch = state.epoch.wrapping_add(1);
            core::array::from_fn::<_, MAX_WAITERS, _>(|i| state.slots[i].waker.take())
        });
        // Never call executor wake implementations while a resource is borrowed.
        for waker in wakers.into_iter().flatten() {
            waker.wake();
        }
    }

    /// Capture the change epoch now, including changes before the first poll.
    pub fn listen(&self) -> impl Future<Output = Result<(), WaitError>> + '_ {
        let epoch = critical_section::with(|cs| self.state.borrow(cs).borrow().epoch);
        let mut registration = self.registration();
        poll_fn(move |cx| {
            critical_section::with(|cs| {
                if self.state.borrow(cs).borrow().epoch != epoch {
                    Poll::Ready(Ok(()))
                } else {
                    registration.pending(cx)
                }
            })
        })
    }
}

pub(crate) struct Registration<'a> {
    queue: &'a WaitQueue,
    slot: Option<usize>,
}

impl Registration<'_> {
    pub fn pending<T>(&mut self, cx: &Context<'_>) -> Poll<Result<T, WaitError>> {
        let result = critical_section::with(|cs| {
            let mut state = self.queue.state.borrow(cs).borrow_mut();
            let index = match self.slot {
                Some(index) => index,
                None => {
                    let index = state
                        .slots
                        .iter()
                        .position(|s| !s.used)
                        .ok_or(WaitError::TooManyWaiters)?;
                    state.slots[index].used = true;
                    self.slot = Some(index);
                    index
                }
            };
            let slot = &mut state.slots[index];
            if slot
                .waker
                .as_ref()
                .is_some_and(|old| old.will_wake(cx.waker()))
            {
                Ok(None)
            } else {
                Ok(slot.waker.replace(cx.waker().clone()))
            }
        });
        match result {
            Ok(old) => {
                drop(old);
                Poll::Pending
            }
            Err(error) => Poll::Ready(Err(error)),
        }
    }
}

impl Drop for Registration<'_> {
    fn drop(&mut self) {
        if let Some(index) = self.slot.take() {
            let old = critical_section::with(|cs| {
                let mut state = self.queue.state.borrow(cs).borrow_mut();
                state.slots[index].used = false;
                state.slots[index].waker.take()
            });
            drop(old);
        }
    }
}

/// Cancel an operation when `timeout` resolves. Timeout wins simultaneous readiness.
/// A platform timer, a cancellation future, or any other `Future<Output = ()>` is usable.
/// Dropping this wrapper drops both futures, returning any guards they own.
pub async fn with_timeout<F: Future>(
    operation: F,
    timeout: impl Future<Output = ()>,
) -> Result<F::Output, WaitError> {
    let mut operation = core::pin::pin!(operation);
    let mut timeout = core::pin::pin!(timeout);
    poll_fn(|cx| {
        if timeout.as_mut().poll(cx).is_ready() {
            Poll::Ready(Err(WaitError::Timeout))
        } else {
            operation.as_mut().poll(cx).map(Ok)
        }
    })
    .await
}
