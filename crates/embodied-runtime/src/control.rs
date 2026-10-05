//! Cooperative lifecycle requests with fixed-capacity notification storage.
//!
//! The distinction between graceful stop and abort is inspired by Dynamo's
//! [`AsyncEngineContext::stop_generating` and `kill`][source]. This is an
//! independently written `no_std` adaptation: it has no heap allocation, stream
//! graph, child contexts, or executor dependency. Each service defines what a
//! graceful stop finishes and which pending operations an abort may cancel.
//!
//! [source]: https://github.com/ai-dynamo/dynamo/blob/519e735550c1a4aac67c2fd37d4a56ed0a014653/lib/runtime/src/engine.rs#L105-L154

use crate::wait::{WaitError, WaitQueue};
use core::{cell::RefCell, future::poll_fn, task::Poll};
use critical_section::Mutex;

/// A latched request, rather than confirmation that a worker has exited.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunState {
    Running,
    Stopping,
    Aborted,
}

/// Reason a controlled worker finished successfully.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunExit {
    Stopped,
    Aborted,
}

/// One-way lifecycle control: Running -> Stopping -> Aborted, or Running -> Aborted.
/// Requests are idempotent, and abort always dominates stop. Create a new control
/// for a new run; there is deliberately no reset that could erase a pending request.
///
/// Transitions and waiter registration are serialized by critical sections.
/// Notifications run after the state borrow is released. Requests may come from
/// an ISR when the platform critical-section implementation and wakers are ISR-safe.
/// This does not preempt executing Rust code or cancel hardware-owned work.
pub struct RunControl {
    state: Mutex<RefCell<RunState>>,
    changed: WaitQueue,
}

impl Default for RunControl {
    fn default() -> Self {
        Self::new()
    }
}

impl RunControl {
    pub const fn new() -> Self {
        Self {
            state: Mutex::new(RefCell::new(RunState::Running)),
            changed: WaitQueue::new(),
        }
    }

    pub fn state(&self) -> RunState {
        critical_section::with(|cs| *self.state.borrow(cs).borrow())
    }

    /// Request graceful completion according to the consuming service's contract.
    pub fn request_stop(&self) {
        self.request(RunState::Stopping);
    }

    /// Escalate to cancellation of operations the service can safely drop.
    pub fn request_abort(&self) {
        self.request(RunState::Aborted);
    }

    fn request(&self, requested: RunState) {
        let changed = critical_section::with(|cs| {
            let mut state = self.state.borrow(cs).borrow_mut();
            if *state == RunState::Running
                || (*state == RunState::Stopping && requested == RunState::Aborted)
            {
                *state = requested;
                true
            } else {
                false
            }
        });
        if changed {
            self.changed.wake_all();
        }
    }

    /// Wait until the current state differs from `observed`, returning its latest
    /// value. A request made before the first poll is observed immediately; a fast
    /// stop followed by abort may be observed as Aborted without seeing Stopping.
    ///
    /// Checking state and registering the waiter share one critical section, so
    /// a request cannot be lost between those steps. At most
    /// [`MAX_WAITERS`](crate::wait::MAX_WAITERS) operations may wait concurrently;
    /// an additional pending operation returns [`WaitError::TooManyWaiters`].
    /// Dropping the future releases its slot. Waiting on Aborted after Aborted
    /// waits forever because this control cannot be reset.
    pub async fn changed_since(&self, observed: RunState) -> Result<RunState, WaitError> {
        let mut registration = self.changed.registration();
        poll_fn(|cx| {
            critical_section::with(|cs| {
                let current = *self.state.borrow(cs).borrow();
                if current != observed {
                    Poll::Ready(Ok(current))
                } else {
                    registration.pending(cx)
                }
            })
        })
        .await
    }
}
