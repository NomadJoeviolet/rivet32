#![cfg(not(target_os = "none"))]
#![forbid(unsafe_code)]

//! Cancellation is exercised at actual `Pending`/`Ready` boundaries. The
//! Drop-before-completion distinction is informed by Warp's `OnCancelFuture`:
//! https://github.com/warpdotdev/warp/blob/b865631c9a0e46b548c7ec7dc32e228a148171d1/crates/warp_util/src/on_cancel.rs
//! These are independent tests of rivet32's owned leases and worker contract;
//! they do not import Warp, copy its wrapper, or rely on real elapsed time.

mod common;

use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll, Waker},
};

use common::{ManualClock, WakeCounter, poll_once};
use embodied_core::{
    can::{CanFrame, Id},
    communication::{AsyncCanTx, CanError, CanTxOutcome},
    time::Instant,
};
use embodied_runtime::{
    can::{CanTxService, CanWorkerError},
    clock::Clock,
    control::{RunControl, RunExit, RunState},
    pool::MessagePool,
    queue::SharedQueue,
    wait::{MAX_WAITERS, WaitError},
};

#[test]
fn exact_deadline_wins_over_newly_available_queue_capacity() {
    let queue = SharedQueue::<u8, 1>::new();
    queue.try_send(1).unwrap();
    let clock = ManualClock::default();
    let wakes = Arc::new(WakeCounter::default());
    let mut send = Box::pin(queue.send_timeout(2, clock.sleep_until(Instant::from_micros(10))));
    assert!(poll_once(send.as_mut(), &wakes).is_pending());
    clock.advance_to(9);
    assert_eq!(wakes.count(), 0);
    assert!(poll_once(send.as_mut(), &wakes).is_pending());

    clock.advance_to(10);
    assert_eq!(queue.try_receive(), Some(1));
    let Poll::Ready(Err(error)) = poll_once(send.as_mut(), &wakes) else {
        panic!("timeout must win simultaneous capacity and deadline readiness");
    };
    assert_eq!(error.reason, WaitError::Timeout);
    assert_eq!(error.value, 2);
    assert!(queue.is_empty());
    drop(send);
    assert_eq!(clock.pending_sleeps(), 0);
}

#[test]
fn cancelled_sleep_cannot_complete_a_new_sleep_at_its_old_deadline() {
    let clock = ManualClock::default();
    let wakes = Arc::new(WakeCounter::default());
    let mut old = Box::pin(clock.sleep_until(Instant::from_micros(10)));
    assert!(poll_once(old.as_mut(), &wakes).is_pending());
    assert_eq!(clock.pending_sleeps(), 1);
    drop(old);
    assert_eq!(clock.pending_sleeps(), 0);

    let mut fresh = Box::pin(clock.sleep_until(Instant::from_micros(20)));
    assert!(poll_once(fresh.as_mut(), &wakes).is_pending());
    clock.advance_to(10);
    assert_eq!(wakes.count(), 0);
    assert!(poll_once(fresh.as_mut(), &wakes).is_pending());
    clock.advance_to(20);
    assert_eq!(wakes.count(), 1);
    assert_eq!(poll_once(fresh.as_mut(), &wakes), Poll::Ready(()));
    assert_eq!(clock.pending_sleeps(), 0);
}

#[test]
#[should_panic(expected = "monotonic clock cannot move backwards")]
fn manual_clock_rejects_backwards_time() {
    let clock = ManualClock::default();
    clock.advance_to(10);
    clock.advance_to(9);
}

#[test]
fn cancelled_send_returns_its_lease_once_and_reuses_sender_waiters() {
    let pool = MessagePool::new([1_u8, 2]);
    let queue = SharedQueue::<_, 1>::new();
    let wakes = Arc::new(WakeCounter::default());

    let unpolled = queue.send(pool.try_acquire().unwrap());
    assert_eq!(pool.available(), 1);
    drop(unpolled);
    assert_eq!(pool.available(), 2);
    assert!(queue.is_empty());

    let mut completed = Box::pin(queue.send(pool.try_acquire().unwrap()));
    assert!(matches!(
        poll_once(completed.as_mut(), &wakes),
        Poll::Ready(Ok(()))
    ));
    drop(completed);
    assert_eq!(
        pool.available(),
        1,
        "completed send transferred the lease to the queue"
    );
    assert_eq!(queue.len(), 1);

    for _ in 0..MAX_WAITERS * 2 {
        let lease = pool.try_acquire().unwrap();
        let mut send = Box::pin(queue.send(lease));
        assert!(poll_once(send.as_mut(), &wakes).is_pending());
        assert_eq!(pool.available(), 0);
        drop(send);
        assert_eq!(pool.available(), 1);
        let returned = pool.try_acquire().unwrap();
        assert_eq!(*returned, 2);
        assert!(pool.try_acquire().is_none());
        drop(returned);
    }
    assert_eq!(queue.len(), 1);
    drop(queue.try_receive());
    assert_eq!(pool.available(), 2);
}

#[test]
fn cancelled_receives_release_all_bounded_waiter_slots() {
    let queue = SharedQueue::<u8, 1>::new();
    let wakes = Arc::new(WakeCounter::default());
    for _ in 0..2 {
        let mut waiters: Vec<_> = (0..MAX_WAITERS)
            .map(|_| Box::pin(queue.receive()))
            .collect();
        for waiter in &mut waiters {
            assert!(poll_once(waiter.as_mut(), &wakes).is_pending());
        }
        let mut excess = Box::pin(queue.receive());
        assert_eq!(
            poll_once(excess.as_mut(), &wakes),
            Poll::Ready(Err(WaitError::TooManyWaiters))
        );
        drop(waiters);
    }
    let mut receiver = Box::pin(queue.receive());
    assert!(poll_once(receiver.as_mut(), &wakes).is_pending());
    queue.try_send(7).unwrap();
    assert_eq!(poll_once(receiver.as_mut(), &wakes), Poll::Ready(Ok(7)));
}

#[derive(Default)]
struct DriverState {
    created: usize,
    polls: usize,
    cancelled: usize,
    accepted: Vec<CanFrame>,
    next: Option<Result<CanTxOutcome, CanError>>,
    waker: Option<Waker>,
}

#[derive(Clone, Default)]
struct DriverProbe(Rc<RefCell<DriverState>>);

impl DriverProbe {
    fn resolve(&self, result: Result<CanTxOutcome, CanError>) {
        let waker = {
            let mut state = self.0.borrow_mut();
            state.next = Some(result);
            state.waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

struct MockDriver(DriverProbe);

impl AsyncCanTx for MockDriver {
    fn transmit(
        &mut self,
        frame: &CanFrame,
    ) -> impl Future<Output = Result<CanTxOutcome, CanError>> {
        self.0.0.borrow_mut().created += 1;
        Transmission {
            probe: self.0.clone(),
            frame: *frame,
            completed: false,
        }
    }
}

struct Transmission {
    probe: DriverProbe,
    frame: CanFrame,
    completed: bool,
}

impl Future for Transmission {
    type Output = Result<CanTxOutcome, CanError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let result = {
            let mut state = self.probe.0.borrow_mut();
            state.polls += 1;
            match state.next.take() {
                Some(result) => {
                    state.waker = None;
                    if result.is_ok() {
                        state.accepted.push(self.frame);
                    }
                    Some(result)
                }
                None => {
                    state.waker = Some(cx.waker().clone());
                    None
                }
            }
        };
        match result {
            Some(result) => {
                self.completed = true;
                Poll::Ready(result)
            }
            None => Poll::Pending,
        }
    }
}

impl Drop for Transmission {
    fn drop(&mut self) {
        if !self.completed {
            let mut state = self.probe.0.borrow_mut();
            state.cancelled += 1;
            state.waker = None;
        }
    }
}

fn frame(value: u8) -> CanFrame {
    CanFrame::new(Id::Standard(0x200), &[value]).unwrap()
}

#[test]
fn stop_before_first_poll_leaves_waiting_frames_unselected() {
    let service = CanTxService::<2>::new();
    service.try_enqueue(frame(1)).unwrap();
    let control = RunControl::new();
    let probe = DriverProbe::default();
    let mut driver = MockDriver(probe.clone());
    let wakes = Arc::new(WakeCounter::default());
    let mut worker = Box::pin(service.run_controlled(&mut driver, &control));
    control.request_stop();
    assert_eq!(
        poll_once(worker.as_mut(), &wakes),
        Poll::Ready(Ok(RunExit::Stopped))
    );
    assert_eq!(service.queued_len(), 1);
    assert_eq!(service.in_flight(), None);
    assert_eq!(probe.0.borrow().created, 0);
}

#[test]
fn stop_wakes_an_idle_worker() {
    let service = CanTxService::<2>::new();
    let control = RunControl::new();
    let probe = DriverProbe::default();
    let mut driver = MockDriver(probe.clone());
    let wakes = Arc::new(WakeCounter::default());
    let mut worker = Box::pin(service.run_controlled(&mut driver, &control));
    assert!(poll_once(worker.as_mut(), &wakes).is_pending());
    control.request_stop();
    assert!(wakes.count() > 0);
    assert_eq!(
        poll_once(worker.as_mut(), &wakes),
        Poll::Ready(Ok(RunExit::Stopped))
    );
    assert_eq!(probe.0.borrow().created, 0);
}

#[test]
fn stop_waits_for_current_acceptance_but_leaves_the_next_frame_queued() {
    let service = CanTxService::<2>::new();
    service.try_enqueue(frame(1)).unwrap();
    service.try_enqueue(frame(2)).unwrap();
    let control = RunControl::new();
    let probe = DriverProbe::default();
    let mut driver = MockDriver(probe.clone());
    let wakes = Arc::new(WakeCounter::default());
    let mut worker = Box::pin(service.run_controlled(&mut driver, &control));
    assert!(poll_once(worker.as_mut(), &wakes).is_pending());
    control.request_stop();
    assert!(poll_once(worker.as_mut(), &wakes).is_pending());
    assert_eq!(probe.0.borrow().cancelled, 0);
    assert_eq!(service.in_flight(), Some(frame(1)));
    probe.resolve(Ok(CanTxOutcome::default()));
    assert_eq!(
        poll_once(worker.as_mut(), &wakes),
        Poll::Ready(Ok(RunExit::Stopped))
    );
    assert_eq!(probe.0.borrow().accepted, [frame(1)]);
    assert_eq!(service.in_flight(), None);
    assert_eq!(service.queued_len(), 1);
    assert_eq!(service.stats().sent, 1);
}

#[test]
fn stop_after_acceptance_during_yield_does_not_select_the_next_frame() {
    let service = CanTxService::<2>::new();
    service.try_enqueue(frame(1)).unwrap();
    service.try_enqueue(frame(2)).unwrap();
    let control = RunControl::new();
    let probe = DriverProbe::default();
    probe.resolve(Ok(CanTxOutcome::default()));
    let mut driver = MockDriver(probe.clone());
    let wakes = Arc::new(WakeCounter::default());
    let mut worker = Box::pin(service.run_controlled(&mut driver, &control));
    assert!(poll_once(worker.as_mut(), &wakes).is_pending());
    assert_eq!(service.stats().sent, 1);
    control.request_stop();
    assert_eq!(
        poll_once(worker.as_mut(), &wakes),
        Poll::Ready(Ok(RunExit::Stopped))
    );
    assert_eq!(service.queued_len(), 1);
    assert_eq!(service.in_flight(), None);
    assert_eq!(probe.0.borrow().created, 1);
}

#[test]
fn abort_before_first_poll_leaves_waiting_frames_unselected() {
    let service = CanTxService::<1>::new();
    service.try_enqueue(frame(1)).unwrap();
    let control = RunControl::new();
    control.request_abort();
    let probe = DriverProbe::default();
    let mut driver = MockDriver(probe.clone());
    let wakes = Arc::new(WakeCounter::default());
    let mut worker = Box::pin(service.run_controlled(&mut driver, &control));
    assert_eq!(
        poll_once(worker.as_mut(), &wakes),
        Poll::Ready(Ok(RunExit::Aborted))
    );
    assert_eq!(service.queued_len(), 1);
    assert_eq!(service.in_flight(), None);
    assert_eq!(probe.0.borrow().created, 0);
}

#[test]
fn stop_escalates_to_abort_before_ready_driver_poll_and_restart_retries_retained_frame() {
    let service = CanTxService::<2>::new();
    service.try_enqueue(frame(1)).unwrap();
    service.try_enqueue(frame(2)).unwrap();
    let control = RunControl::new();
    let probe = DriverProbe::default();
    let mut driver = MockDriver(probe.clone());
    let wakes = Arc::new(WakeCounter::default());
    let mut worker = Box::pin(service.run_controlled(&mut driver, &control));
    assert!(poll_once(worker.as_mut(), &wakes).is_pending());
    control.request_stop();
    assert!(poll_once(worker.as_mut(), &wakes).is_pending());
    assert_eq!(probe.0.borrow().cancelled, 0);
    let before_abort = wakes.count();
    control.request_abort();
    assert!(
        wakes.count() > before_abort,
        "stop must rearm the abort notification"
    );
    probe.resolve(Ok(CanTxOutcome::default()));
    assert_eq!(
        poll_once(worker.as_mut(), &wakes),
        Poll::Ready(Ok(RunExit::Aborted))
    );
    drop(worker);
    assert_eq!(
        probe.0.borrow().polls,
        2,
        "abort must preempt the next driver poll"
    );
    assert_eq!(probe.0.borrow().cancelled, 1);
    assert!(probe.0.borrow().accepted.is_empty());
    assert_eq!(service.in_flight(), Some(frame(1)));
    assert_eq!(service.stats().sent, 0);
    assert_eq!(service.stats().driver_errors, 0);

    let restart = RunControl::new();
    let mut worker = Box::pin(service.run_controlled(&mut driver, &restart));
    assert!(poll_once(worker.as_mut(), &wakes).is_pending());
    assert_eq!(probe.0.borrow().accepted, [frame(1)]);
    assert_eq!(service.stats().sent, 1);
    assert_eq!(service.queued_len(), 1);
    restart.request_stop();
    assert_eq!(
        poll_once(worker.as_mut(), &wakes),
        Poll::Ready(Ok(RunExit::Stopped))
    );
}

#[test]
fn abort_requested_inside_accepting_driver_poll_commits_acceptance_before_exit() {
    struct AbortOnAccept<'a> {
        control: &'a RunControl,
        accepted: &'a RefCell<Vec<CanFrame>>,
    }

    impl AsyncCanTx for AbortOnAccept<'_> {
        async fn transmit(&mut self, frame: &CanFrame) -> Result<CanTxOutcome, CanError> {
            self.accepted.borrow_mut().push(*frame);
            self.control.request_abort();
            Ok(CanTxOutcome::default())
        }
    }

    let service = CanTxService::<2>::new();
    service.try_enqueue(frame(1)).unwrap();
    service.try_enqueue(frame(2)).unwrap();
    let control = RunControl::new();
    let accepted = RefCell::new(Vec::new());
    let mut driver = AbortOnAccept {
        control: &control,
        accepted: &accepted,
    };
    let wakes = Arc::new(WakeCounter::default());
    let mut worker = Box::pin(service.run_controlled(&mut driver, &control));
    assert_eq!(
        poll_once(worker.as_mut(), &wakes),
        Poll::Ready(Ok(RunExit::Aborted))
    );
    drop(worker);
    assert_eq!(*accepted.borrow(), [frame(1)]);
    assert_eq!(service.stats().sent, 1);
    assert_eq!(service.stats().driver_errors, 0);
    assert_eq!(
        service.in_flight(),
        None,
        "an accepted frame must never be retried"
    );
    assert_eq!(service.queued_len(), 1);

    let probe = DriverProbe::default();
    probe.resolve(Ok(CanTxOutcome::default()));
    let mut driver = MockDriver(probe.clone());
    let mut next = Box::pin(service.send_next(&mut driver));
    assert_eq!(
        poll_once(next.as_mut(), &wakes),
        Poll::Ready(Ok(CanTxOutcome::default()))
    );
    assert_eq!(probe.0.borrow().accepted, [frame(2)]);
}

#[test]
fn driver_error_retains_frame_releases_worker_and_counts_only_the_failed_attempt() {
    let service = CanTxService::<1>::new();
    service.try_enqueue(frame(1)).unwrap();
    let control = RunControl::new();
    let probe = DriverProbe::default();
    probe.resolve(Err(CanError::BusOff));
    let mut driver = MockDriver(probe.clone());
    let wakes = Arc::new(WakeCounter::default());
    let mut worker = Box::pin(service.run_controlled(&mut driver, &control));
    assert_eq!(
        poll_once(worker.as_mut(), &wakes),
        Poll::Ready(Err(CanWorkerError::Driver(CanError::BusOff)))
    );
    drop(worker);
    assert_eq!(service.in_flight(), Some(frame(1)));
    assert_eq!(service.stats().driver_errors, 1);
    assert_eq!(service.stats().sent, 0);
    probe.resolve(Ok(CanTxOutcome::default()));
    let mut retry = Box::pin(service.send_next(&mut driver));
    assert_eq!(
        poll_once(retry.as_mut(), &wakes),
        Poll::Ready(Ok(CanTxOutcome::default()))
    );
    assert_eq!(probe.0.borrow().accepted, [frame(1)]);
    assert_eq!(service.in_flight(), None);
    assert_eq!(service.stats().driver_errors, 1);
    assert_eq!(service.stats().sent, 1);
}

#[test]
fn dropping_legacy_or_controlled_worker_releases_driver_claim_and_control_waiters() {
    for controlled in [false, true] {
        let service = CanTxService::<1>::new();
        service.try_enqueue(frame(1)).unwrap();
        let control = RunControl::new();
        let probe = DriverProbe::default();
        let mut driver = MockDriver(probe.clone());
        let wakes = Arc::new(WakeCounter::default());
        if controlled {
            let mut worker = Box::pin(service.run_controlled(&mut driver, &control));
            assert!(poll_once(worker.as_mut(), &wakes).is_pending());
            drop(worker);
        } else {
            let mut worker = Box::pin(service.run(&mut driver));
            assert!(poll_once(worker.as_mut(), &wakes).is_pending());
            drop(worker);
        }
        assert_eq!(probe.0.borrow().cancelled, 1);
        assert_eq!(service.in_flight(), Some(frame(1)));
        assert_eq!(service.stats().driver_errors, 0);

        let mut waiters: Vec<_> = (0..MAX_WAITERS)
            .map(|_| Box::pin(control.changed_since(RunState::Running)))
            .collect();
        for waiter in &mut waiters {
            assert!(poll_once(waiter.as_mut(), &wakes).is_pending());
        }
        drop(waiters);

        probe.resolve(Ok(CanTxOutcome::default()));
        let mut retry = Box::pin(service.run_controlled(&mut driver, &control));
        assert!(poll_once(retry.as_mut(), &wakes).is_pending());
        assert_eq!(probe.0.borrow().accepted, [frame(1)]);
        assert_eq!(service.in_flight(), None);
        control.request_stop();
        assert_eq!(
            poll_once(retry.as_mut(), &wakes),
            Poll::Ready(Ok(RunExit::Stopped))
        );
    }
}

#[test]
fn control_waiter_exhaustion_leaves_frame_unselected_and_releases_worker_claim() {
    let service = CanTxService::<1>::new();
    service.try_enqueue(frame(1)).unwrap();
    let control = RunControl::new();
    let wakes = Arc::new(WakeCounter::default());
    let mut waiters: Vec<_> = (0..MAX_WAITERS)
        .map(|_| Box::pin(control.changed_since(RunState::Running)))
        .collect();
    for waiter in &mut waiters {
        assert!(poll_once(waiter.as_mut(), &wakes).is_pending());
    }

    let failed_probe = DriverProbe::default();
    let mut failed_driver = MockDriver(failed_probe.clone());
    let mut failed = Box::pin(service.run_controlled(&mut failed_driver, &control));
    assert_eq!(
        poll_once(failed.as_mut(), &wakes),
        Poll::Ready(Err(CanWorkerError::Wait(WaitError::TooManyWaiters)))
    );
    assert_eq!(service.queued_len(), 1);
    assert_eq!(service.in_flight(), None);
    assert_eq!(service.stats().driver_errors, 0);
    assert_eq!(failed_probe.0.borrow().created, 0);
    drop(waiters);

    // The completed error future still exists: returning Ready must release the
    // worker claim without requiring its caller to immediately drop the future.
    let retry_probe = DriverProbe::default();
    retry_probe.resolve(Ok(CanTxOutcome::default()));
    let mut retry_driver = MockDriver(retry_probe.clone());
    let mut retry = Box::pin(service.run_controlled(&mut retry_driver, &control));
    assert!(poll_once(retry.as_mut(), &wakes).is_pending());
    assert_eq!(retry_probe.0.borrow().accepted, [frame(1)]);
    assert_eq!(service.stats().sent, 1);
    control.request_stop();
    assert_eq!(
        poll_once(retry.as_mut(), &wakes),
        Poll::Ready(Ok(RunExit::Stopped))
    );
}

#[test]
fn active_worker_excludes_other_workers_send_next_and_discard() {
    let service = CanTxService::<1>::new();
    let control = RunControl::new();
    let mut driver = MockDriver(DriverProbe::default());
    let mut other_driver = MockDriver(DriverProbe::default());
    let wakes = Arc::new(WakeCounter::default());
    let mut worker = Box::pin(service.run_controlled(&mut driver, &control));
    assert!(poll_once(worker.as_mut(), &wakes).is_pending());
    let mut other = Box::pin(service.run_controlled(&mut other_driver, &control));
    assert_eq!(
        poll_once(other.as_mut(), &wakes),
        Poll::Ready(Err(CanWorkerError::AlreadyRunning))
    );
    drop(other);
    let mut send = Box::pin(service.send_next(&mut other_driver));
    assert_eq!(
        poll_once(send.as_mut(), &wakes),
        Poll::Ready(Err(CanWorkerError::AlreadyRunning))
    );
    assert_eq!(
        service.try_discard_in_flight(),
        Err(CanWorkerError::AlreadyRunning)
    );
    drop(worker);
    assert_eq!(service.try_discard_in_flight(), Ok(None));
}

#[test]
fn acceptance_counts_hardware_replacements_without_retrying_accepted_frames() {
    let service = CanTxService::<2>::new();
    service.try_enqueue(frame(1)).unwrap();
    service.try_enqueue(frame(2)).unwrap();
    let probe = DriverProbe::default();
    let mut driver = MockDriver(probe.clone());
    let wakes = Arc::new(WakeCounter::default());
    let replaced = CanTxOutcome {
        replaced: Some(frame(9)),
        replaced_error: None,
    };
    probe.resolve(Ok(replaced));
    let mut send = Box::pin(service.send_next(&mut driver));
    assert_eq!(poll_once(send.as_mut(), &wakes), Poll::Ready(Ok(replaced)));
    drop(send);

    let control = RunControl::new();
    probe.resolve(Ok(CanTxOutcome {
        replaced: None,
        replaced_error: Some(CanError::RemoteFrameUnsupported),
    }));
    let mut worker = Box::pin(service.run_controlled(&mut driver, &control));
    assert!(poll_once(worker.as_mut(), &wakes).is_pending());
    control.request_stop();
    assert_eq!(
        poll_once(worker.as_mut(), &wakes),
        Poll::Ready(Ok(RunExit::Stopped))
    );
    let stats = service.stats();
    assert_eq!(stats.enqueued, 2);
    assert_eq!(stats.sent, 2);
    assert_eq!(stats.replaced, 2);
    assert_eq!(stats.replacement_errors, 1);
    assert_eq!(stats.driver_errors, 0);
    assert_eq!(probe.0.borrow().accepted, [frame(1), frame(2)]);
    assert_eq!(service.in_flight(), None);
}

#[test]
fn control_transitions_are_monotonic_and_changes_before_first_poll_are_visible() {
    let control = RunControl::new();
    let wakes = Arc::new(WakeCounter::default());
    assert_eq!(control.state(), RunState::Running);
    let mut changed = Box::pin(control.changed_since(RunState::Running));
    control.request_stop();
    assert_eq!(
        poll_once(changed.as_mut(), &wakes),
        Poll::Ready(Ok(RunState::Stopping))
    );
    let mut aborted = Box::pin(control.changed_since(RunState::Stopping));
    assert!(poll_once(aborted.as_mut(), &wakes).is_pending());
    control.request_stop();
    assert_eq!(wakes.count(), 0);
    control.request_abort();
    assert!(wakes.count() > 0);
    assert_eq!(
        poll_once(aborted.as_mut(), &wakes),
        Poll::Ready(Ok(RunState::Aborted))
    );
    control.request_stop();
    assert_eq!(control.state(), RunState::Aborted);
}

#[test]
fn cancelled_control_waits_release_all_bounded_waiter_slots() {
    let control = RunControl::new();
    let wakes = Arc::new(WakeCounter::default());
    for _ in 0..2 {
        let mut waiters: Vec<_> = (0..MAX_WAITERS)
            .map(|_| Box::pin(control.changed_since(RunState::Running)))
            .collect();
        for waiter in &mut waiters {
            assert!(poll_once(waiter.as_mut(), &wakes).is_pending());
        }
        let mut excess = Box::pin(control.changed_since(RunState::Running));
        assert_eq!(
            poll_once(excess.as_mut(), &wakes),
            Poll::Ready(Err(WaitError::TooManyWaiters))
        );
        drop(waiters);
    }
    let mut changed = Box::pin(control.changed_since(RunState::Running));
    assert!(poll_once(changed.as_mut(), &wakes).is_pending());
    control.request_abort();
    assert_eq!(
        poll_once(changed.as_mut(), &wakes),
        Poll::Ready(Ok(RunState::Aborted))
    );
}
