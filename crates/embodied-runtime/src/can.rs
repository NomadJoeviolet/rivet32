//! Fixed-storage CAN transmission and synchronous receive dispatch.
//!
//! `try_enqueue` is the ISR-facing path. It never waits for a resource and never
//! invokes a driver; it briefly enters a platform critical section and wakes the
//! worker. That platform implementation and the executor's waker must be ISR-safe.
use crate::{
    control::{RunControl, RunExit, RunState},
    wait::{WaitError, WaitQueue},
};
use core::{
    cell::RefCell,
    future::{Future, poll_fn},
    task::Poll,
};
use critical_section::Mutex;
use embodied_core::{
    can::CanFrame,
    communication::{AsyncCanRx, AsyncCanTx, CanError, CanTxOutcome},
    signal::Signal,
};

pub const DEFAULT_TX_CAPACITY: usize = 12;

/// All counters saturate at u64::MAX. `sent` means driver acceptance, not bus ACK.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CanTxStats {
    pub enqueued: u64,
    pub sent: u64,
    /// Waiting frames discarded by software queue overflow.
    pub dropped: u64,
    /// Completed transmit calls returning Err; cancellation is not an error.
    pub driver_errors: u64,
    /// Older hardware-owned frames reported as replaced by successful calls.
    pub replaced: u64,
    /// Replaced hardware frames not representable as CanFrame; subset of replaced.
    pub replacement_errors: u64,
    /// Frames explicitly removed by try_discard_in_flight while stopped.
    pub discarded: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnqueueOutcome {
    pub dropped: Option<CanFrame>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanWorkerError {
    AlreadyRunning,
    Wait(WaitError),
    Driver(CanError),
}

struct TxState<const N: usize> {
    // The ring, overflow decision and counters share one critical-section borrow.
    queue: [Option<CanFrame>; N],
    front: usize,
    len: usize,
    in_flight: Option<CanFrame>,
    running: bool,
    stats: CanTxStats,
}

impl<const N: usize> TxState<N> {
    fn pop_front(&mut self) -> Option<CanFrame> {
        if self.len == 0 {
            return None;
        }
        let frame = self.queue[self.front].take();
        self.front = (self.front + 1) % N;
        self.len -= 1;
        frame
    }
}

/// N waiting frames plus one protected in-flight slot, entirely owned by the
/// service. Exactly one worker can run at a time, even with different drivers.
///
/// Dropping a worker preserves all queued/in-flight frames. A later worker retries
/// the in-flight frame first. On driver error it is likewise retained, and the
/// worker returns so the caller can recover/back off explicitly. Repeated retries
/// are an application decision. Dropping the service itself drops its frames.
pub struct CanTxService<const N: usize = DEFAULT_TX_CAPACITY> {
    inner: Mutex<RefCell<TxState<N>>>,
    available: WaitQueue,
}

impl<const N: usize> Default for CanTxService<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> CanTxService<N> {
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(RefCell::new(TxState {
                queue: [None; N],
                front: 0,
                len: 0,
                in_flight: None,
                running: false,
                stats: CanTxStats {
                    enqueued: 0,
                    sent: 0,
                    dropped: 0,
                    driver_errors: 0,
                    replaced: 0,
                    replacement_errors: 0,
                    discarded: 0,
                },
            })),
            available: WaitQueue::new(),
        }
    }

    /// Append a frame, discarding the oldest waiting frame if full. Never evicts
    /// the in-flight frame. Capacity zero returns ownership unchanged in Err.
    pub fn try_enqueue(&self, frame: CanFrame) -> Result<EnqueueOutcome, CanFrame> {
        let result = critical_section::with(|cs| {
            let mut state = self.inner.borrow(cs).borrow_mut();
            if N == 0 {
                return Err(frame);
            }
            let dropped = if state.len == N {
                state.pop_front()
            } else {
                None
            };
            let index = (state.front + state.len) % N;
            state.queue[index] = Some(frame);
            state.len += 1;
            state.stats.enqueued = state.stats.enqueued.saturating_add(1);
            if dropped.is_some() {
                state.stats.dropped = state.stats.dropped.saturating_add(1);
            }
            Ok(EnqueueOutcome { dropped })
        });
        if result.is_ok() {
            self.available.wake_all();
        }
        result
    }

    pub fn stats(&self) -> CanTxStats {
        critical_section::with(|cs| self.inner.borrow(cs).borrow().stats)
    }

    /// Number of waiting frames; excludes the protected in-flight slot.
    pub fn queued_len(&self) -> usize {
        critical_section::with(|cs| self.inner.borrow(cs).borrow().len)
    }

    pub const fn capacity(&self) -> usize {
        N
    }

    /// Snapshot of the pending/retry frame, not a handle to driver storage.
    pub fn in_flight(&self) -> Option<CanFrame> {
        critical_section::with(|cs| self.inner.borrow(cs).borrow().in_flight)
    }

    /// Explicitly abandon a pending/retry frame after stopping the worker. Useful
    /// when a format is permanently unsupported. Automatic cancellation never
    /// performs this operation. Returns the owned frame for logging/re-routing.
    pub fn try_discard_in_flight(&self) -> Result<Option<CanFrame>, CanWorkerError> {
        critical_section::with(|cs| {
            let mut state = self.inner.borrow(cs).borrow_mut();
            if state.running {
                return Err(CanWorkerError::AlreadyRunning);
            }
            let frame = state.in_flight.take();
            if frame.is_some() {
                state.stats.discarded = state.stats.discarded.saturating_add(1);
            }
            Ok(frame)
        })
    }

    fn claim(&self) -> Result<Worker<'_, N>, CanWorkerError> {
        critical_section::with(|cs| {
            let mut state = self.inner.borrow(cs).borrow_mut();
            if state.running {
                return Err(CanWorkerError::AlreadyRunning);
            }
            state.running = true;
            Ok(Worker(self))
        })
    }

    /// Wait for and attempt one frame; return replacement information to callers
    /// that need to inspect the evicted hardware frame.
    pub async fn send_next(
        &self,
        driver: &mut impl AsyncCanTx,
    ) -> Result<CanTxOutcome, CanWorkerError> {
        let worker = self.claim()?;
        worker.send(driver).await
    }

    /// Run until a driver/wait error or Future Drop. Yield after every accepted
    /// frame, including when the driver and queue are continuously ready.
    /// Hardware replacements are counted; use send_next to inspect their values.
    pub async fn run(&self, driver: &mut impl AsyncCanTx) -> Result<(), CanWorkerError> {
        let worker = self.claim()?;
        loop {
            worker.send(driver).await?;
            yield_once().await;
        }
    }

    /// Run with separate graceful-stop and abort requests.
    ///
    /// A stop observed before selecting a frame exits immediately, retaining all
    /// queued/retry frames. Otherwise, finish the selected frame's driver acceptance
    /// and exit before selecting another. This does not drain the queue or wait for
    /// physical transmission. A driver that remains Pending can delay graceful
    /// stop indefinitely; request abort to cancel that attempt instead.
    ///
    /// Abort is checked before each driver poll. If observed, drop the transmit
    /// future and retain the in-flight frame for a later worker, relying on
    /// [`AsyncCanTx`]'s cancellation contract. An abort concurrent with an executing
    /// poll cannot preempt it: Ready(Ok) still commits acceptance before the next
    /// control check, and Ready(Err) returns a driver error. Accepted hardware work
    /// cannot be recalled. No critical section is held while polling driver code.
    ///
    /// Control checks and frame selection share a critical section; this is the
    /// boundary deciding which frame a graceful stop permits to finish. Idle
    /// control changes wake the worker without requiring an enqueue. Errors and
    /// Future Drop release worker ownership and retain unaccepted frames just as
    /// [`Self::run`] does. A restart after stop/abort needs a new [`RunControl`].
    pub async fn run_controlled(
        &self,
        driver: &mut impl AsyncCanTx,
        control: &RunControl,
    ) -> Result<RunExit, CanWorkerError> {
        let worker = self.claim()?;
        loop {
            if let Some(exit) = worker.send_controlled(driver, control).await? {
                return Ok(exit);
            }
            match control.state() {
                RunState::Stopping => return Ok(RunExit::Stopped),
                RunState::Aborted => return Ok(RunExit::Aborted),
                RunState::Running => {}
            }
            yield_once().await;
        }
    }
}

struct Worker<'a, const N: usize>(&'a CanTxService<N>);

enum ControlledFrame {
    Frame(CanFrame),
    Exit(RunExit),
}

impl<const N: usize> Worker<'_, N> {
    async fn send(&self, driver: &mut impl AsyncCanTx) -> Result<CanTxOutcome, CanWorkerError> {
        let mut registration = self.0.available.registration();
        let frame = poll_fn(|cx| {
            critical_section::with(|cs| {
                let frame = {
                    let mut state = self.0.inner.borrow(cs).borrow_mut();
                    if state.in_flight.is_none() {
                        state.in_flight = state.pop_front();
                    }
                    state.in_flight
                };
                match frame {
                    Some(frame) => Poll::Ready(Ok(frame)),
                    None => registration.pending(cx),
                }
            })
        })
        .await
        .map_err(CanWorkerError::Wait)?;
        // Release the waiter before calling user driver code, outside any CS.
        drop(registration);
        let result = driver.transmit(&frame).await;
        self.commit(result)
    }

    async fn send_controlled(
        &self,
        driver: &mut impl AsyncCanTx,
        control: &RunControl,
    ) -> Result<Option<RunExit>, CanWorkerError> {
        let frame = {
            let mut registration = self.0.available.registration();
            let mut changed = core::pin::pin!(control.changed_since(RunState::Running));
            poll_fn(|cx| {
                critical_section::with(|cs| {
                    match changed.as_mut().poll(cx) {
                        Poll::Ready(Ok(RunState::Stopping)) => {
                            return Poll::Ready(Ok(ControlledFrame::Exit(RunExit::Stopped)));
                        }
                        Poll::Ready(Ok(RunState::Aborted)) => {
                            return Poll::Ready(Ok(ControlledFrame::Exit(RunExit::Aborted)));
                        }
                        Poll::Ready(Err(error)) => {
                            return Poll::Ready(Err(CanWorkerError::Wait(error)));
                        }
                        _ => {}
                    }
                    let frame = {
                        let mut state = self.0.inner.borrow(cs).borrow_mut();
                        if state.in_flight.is_none() {
                            state.in_flight = state.pop_front();
                        }
                        state.in_flight
                    };
                    match frame {
                        Some(frame) => Poll::Ready(Ok(ControlledFrame::Frame(frame))),
                        None => registration.pending(cx).map_err(CanWorkerError::Wait),
                    }
                })
            })
            .await?
        };
        // Both selection waiters are dropped before creating a driver future.
        let frame = match frame {
            ControlledFrame::Frame(frame) => frame,
            ControlledFrame::Exit(exit) => return Ok(Some(exit)),
        };
        let mut changed = core::pin::pin!(control.changed_since(RunState::Running));
        let mut transmit = core::pin::pin!(driver.transmit(&frame));
        poll_fn(|cx| {
            loop {
                match changed.as_mut().poll(cx) {
                    Poll::Ready(Ok(RunState::Aborted)) => {
                        return Poll::Ready(Ok(Some(RunExit::Aborted)));
                    }
                    Poll::Ready(Ok(state)) => {
                        // A graceful stop leaves this attempt alive, but re-arm
                        // notification so a later abort can wake a stuck driver.
                        changed.set(control.changed_since(state));
                    }
                    Poll::Ready(Err(error)) => {
                        return Poll::Ready(Err(CanWorkerError::Wait(error)));
                    }
                    Poll::Pending => break,
                }
            }
            transmit
                .as_mut()
                .poll(cx)
                .map(|result| self.commit(result).map(|_| None))
        })
        .await
    }

    fn commit(
        &self,
        result: Result<CanTxOutcome, CanError>,
    ) -> Result<CanTxOutcome, CanWorkerError> {
        // No await between driver acceptance and committing the service state.
        critical_section::with(|cs| {
            let mut state = self.0.inner.borrow(cs).borrow_mut();
            match result {
                Ok(outcome) => {
                    state.in_flight = None;
                    state.stats.sent = state.stats.sent.saturating_add(1);
                    if outcome.replaced.is_some() || outcome.replaced_error.is_some() {
                        state.stats.replaced = state.stats.replaced.saturating_add(1);
                    }
                    if outcome.replaced_error.is_some() {
                        state.stats.replacement_errors =
                            state.stats.replacement_errors.saturating_add(1);
                    }
                    Ok(outcome)
                }
                Err(error) => {
                    state.stats.driver_errors = state.stats.driver_errors.saturating_add(1);
                    Err(CanWorkerError::Driver(error))
                }
            }
        })
    }
}

impl<const N: usize> Drop for Worker<'_, N> {
    fn drop(&mut self) {
        critical_section::with(|cs| self.0.inner.borrow(cs).borrow_mut().running = false);
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CanRxStats {
    /// Valid frames received, even if there are no subscribers.
    pub received: u64,
    pub driver_errors: u64,
}

/// One receive worker, enforced by mutable borrowing. Callbacks run synchronously
/// in task context and in Signal order, outside any runtime critical section.
#[derive(Default)]
pub struct CanRxService {
    stats: CanRxStats,
}

impl CanRxService {
    pub const fn new() -> Self {
        Self {
            stats: CanRxStats {
                received: 0,
                driver_errors: 0,
            },
        }
    }

    pub const fn stats(&self) -> CanRxStats {
        self.stats
    }

    pub async fn dispatch_next<const S: usize>(
        &mut self,
        driver: &mut impl AsyncCanRx,
        signal: &mut Signal<'_, CanFrame, S>,
    ) -> Result<(), CanError> {
        match driver.receive().await {
            Ok(frame) => {
                self.stats.received = self.stats.received.saturating_add(1);
                // No await after consuming the frame: cancellation cannot skip
                // its dispatch. A panicking callback can still interrupt Signal.
                signal.emit(&frame);
                Ok(())
            }
            Err(error) => {
                self.stats.driver_errors = self.stats.driver_errors.saturating_add(1);
                Err(error)
            }
        }
    }

    /// Exit on the first driver error so a fault cannot cause a tight retry loop.
    pub async fn run<const S: usize>(
        &mut self,
        driver: &mut impl AsyncCanRx,
        signal: &mut Signal<'_, CanFrame, S>,
    ) -> Result<(), CanError> {
        loop {
            self.dispatch_next(driver, signal).await?;
            yield_once().await;
        }
    }
}

async fn yield_once() {
    let mut yielded = false;
    poll_fn(|cx| {
        if yielded {
            Poll::Ready(())
        } else {
            yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    })
    .await
}
