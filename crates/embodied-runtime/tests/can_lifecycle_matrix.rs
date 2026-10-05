#![cfg(not(target_os = "none"))]
#![forbid(unsafe_code)]

//! Explicit executor boundaries keep cancellation and acceptance reproducible.
//! Accepted frames, retained frames, and counters are checked against each
//! event schedule, rather than against private worker implementation details.

#[allow(dead_code)]
mod common;

use std::{
    cell::{Cell, RefCell},
    future::Future,
    pin::Pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll, Waker},
};

use common::{WakeCounter, poll_once};
use embodied_core::{
    can::{CanFrame, Id},
    communication::{AsyncCanTx, CanError, CanTxOutcome},
};
use embodied_runtime::{
    can::{CanTxService, CanTxStats, CanWorkerError},
    control::{RunControl, RunExit, RunState},
    wait::{MAX_WAITERS, WaitError},
};

#[derive(Clone, Copy, Debug)]
enum Request {
    Stop,
    Abort,
}

impl Request {
    fn apply(self, control: &RunControl) {
        match self {
            Self::Stop => control.request_stop(),
            Self::Abort => control.request_abort(),
        }
    }

    fn exit(self) -> RunExit {
        match self {
            Self::Stop => RunExit::Stopped,
            Self::Abort => RunExit::Aborted,
        }
    }
}

#[derive(Default)]
struct DriverState {
    next: Option<Result<CanTxOutcome, CanError>>,
    request_in_poll: Option<Request>,
    waker: Option<Waker>,
    attempted: Vec<CanFrame>,
    accepted: Vec<CanFrame>,
    cancelled: usize,
    polls: usize,
}

#[derive(Clone, Default)]
struct Probe(Rc<RefCell<DriverState>>);

impl Probe {
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

struct Driver<'a> {
    probe: Probe,
    control: &'a RunControl,
}

impl AsyncCanTx for Driver<'_> {
    fn transmit(
        &mut self,
        frame: &CanFrame,
    ) -> impl Future<Output = Result<CanTxOutcome, CanError>> {
        self.probe.0.borrow_mut().attempted.push(*frame);
        Attempt {
            probe: self.probe.clone(),
            control: self.control,
            frame: *frame,
            completed: false,
        }
    }
}

struct Attempt<'a> {
    probe: Probe,
    control: &'a RunControl,
    frame: CanFrame,
    completed: bool,
}

impl Future for Attempt<'_> {
    type Output = Result<CanTxOutcome, CanError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let (request, result) = {
            let mut state = self.probe.0.borrow_mut();
            state.polls += 1;
            let request = state.request_in_poll.take();
            let result = state.next.take();
            if result.is_none() {
                state.waker = Some(cx.waker().clone());
            } else {
                state.waker = None;
                if result.is_some_and(|result| result.is_ok()) {
                    state.accepted.push(self.frame);
                }
            }
            (request, result)
        };
        if let Some(request) = request {
            request.apply(self.control);
        }
        match result {
            Some(result) => {
                self.completed = true;
                Poll::Ready(result)
            }
            None => Poll::Pending,
        }
    }
}

impl Drop for Attempt<'_> {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Event {
    Stop,
    Abort,
    DriverReady,
}

#[test]
fn all_stop_abort_driver_readiness_orders_preserve_acceptance_and_retry_ownership() {
    use Event::{Abort, DriverReady, Stop};
    let orders = [
        [Stop, Abort, DriverReady],
        [Stop, DriverReady, Abort],
        [Abort, Stop, DriverReady],
        [Abort, DriverReady, Stop],
        [DriverReady, Stop, Abort],
        [DriverReady, Abort, Stop],
    ];
    // Six event orders for each driver outcome = 12 deterministic scenarios.
    for outcome in [Ok(CanTxOutcome::default()), Err(CanError::BusOff)] {
        for order in orders {
            let service = CanTxService::<2>::new();
            service.try_enqueue(frame(1)).unwrap();
            service.try_enqueue(frame(2)).unwrap();
            let control = RunControl::new();
            let probe = Probe::default();
            let mut driver = Driver {
                probe: probe.clone(),
                control: &control,
            };
            let wakes = Arc::new(WakeCounter::default());
            let mut worker = Box::pin(service.run_controlled(&mut driver, &control));
            assert!(poll_once(worker.as_mut(), &wakes).is_pending());
            let mut actual_exit = None;
            for event in order {
                match event {
                    Stop => control.request_stop(),
                    Abort => control.request_abort(),
                    DriverReady => probe.resolve(outcome),
                }
                if actual_exit.is_none()
                    && let Poll::Ready(result) = poll_once(worker.as_mut(), &wakes)
                {
                    actual_exit = Some(result);
                }
            }
            let position = |event| {
                order
                    .iter()
                    .position(|candidate| *candidate == event)
                    .unwrap()
            };
            let accepted_or_failed = position(DriverReady) < position(Abort);
            let accepted = accepted_or_failed && outcome.is_ok();
            let failed = accepted_or_failed && outcome.is_err();
            let expected_exit = if failed {
                Err(CanWorkerError::Driver(CanError::BusOff))
            } else if accepted && position(Stop) < position(Abort) {
                Ok(RunExit::Stopped)
            } else {
                Ok(RunExit::Aborted)
            };
            assert_eq!(
                actual_exit,
                Some(expected_exit),
                "order {order:?}, {outcome:?}"
            );
            assert_eq!(service.stats().sent, u64::from(accepted));
            assert_eq!(service.stats().driver_errors, u64::from(failed));
            assert_eq!(service.queued_len(), 1);
            assert_eq!(service.in_flight(), (!accepted).then(|| frame(1)));
            assert_eq!(probe.0.borrow().attempted, [frame(1)]);
            assert_eq!(probe.0.borrow().cancelled, usize::from(!accepted_or_failed));
            drop(worker);

            // A fresh control must retry only an unaccepted frame, then deliver
            // the queued frame. The resulting accepted sequence is always 1,2.
            let restart = RunControl::new();
            let mut resumed = Box::pin(service.run_controlled(&mut driver, &restart));
            for _ in 0..if accepted { 1 } else { 2 } {
                probe.resolve(Ok(CanTxOutcome::default()));
                assert!(poll_once(resumed.as_mut(), &wakes).is_pending());
            }
            restart.request_stop();
            assert_eq!(
                poll_once(resumed.as_mut(), &wakes),
                Poll::Ready(Ok(RunExit::Stopped))
            );
            assert_eq!(probe.0.borrow().accepted, [frame(1), frame(2)]);
            assert_eq!(service.stats().sent, 2);
            assert_eq!(service.stats().driver_errors, u64::from(failed));
            assert_eq!(service.queued_len(), 0);
            assert_eq!(service.in_flight(), None);
        }
    }
}

#[test]
fn control_waiter_exhaustion_after_selection_retains_frame_without_polling_driver() {
    type ControlWait<'a> = Pin<Box<dyn Future<Output = Result<RunState, WaitError>> + 'a>>;

    struct ExhaustingDriver<'control, 'probes> {
        control: &'control RunControl,
        blockers: &'probes RefCell<Vec<ControlWait<'control>>>,
        wakes: Arc<WakeCounter>,
        polls: &'probes Cell<usize>,
        cancelled: &'probes Cell<usize>,
    }

    struct PendingAttempt<'a> {
        polls: &'a Cell<usize>,
        cancelled: &'a Cell<usize>,
    }

    impl Future for PendingAttempt<'_> {
        type Output = Result<CanTxOutcome, CanError>;

        fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
            self.polls.set(self.polls.get() + 1);
            Poll::Pending
        }
    }

    impl Drop for PendingAttempt<'_> {
        fn drop(&mut self) {
            self.cancelled.set(self.cancelled.get() + 1);
        }
    }

    impl AsyncCanTx for ExhaustingDriver<'_, '_> {
        fn transmit(
            &mut self,
            _: &CanFrame,
        ) -> impl Future<Output = Result<CanTxOutcome, CanError>> {
            // Selection has completed, but the driver's future has not yet
            // been polled. Competing services consume control capacity here.
            for _ in 0..MAX_WAITERS {
                let mut changed = Box::pin(self.control.changed_since(RunState::Running));
                assert!(poll_once(changed.as_mut(), &self.wakes).is_pending());
                self.blockers.borrow_mut().push(changed);
            }
            PendingAttempt {
                polls: self.polls,
                cancelled: self.cancelled,
            }
        }
    }

    let service = CanTxService::<2>::new();
    service.try_enqueue(frame(1)).unwrap();
    service.try_enqueue(frame(2)).unwrap();
    let control = RunControl::new();
    let blockers = RefCell::new(Vec::new());
    let wakes = Arc::new(WakeCounter::default());
    let polls = Cell::new(0);
    let cancelled = Cell::new(0);
    let mut driver = ExhaustingDriver {
        control: &control,
        blockers: &blockers,
        wakes: Arc::clone(&wakes),
        polls: &polls,
        cancelled: &cancelled,
    };
    let mut failed = Box::pin(service.run_controlled(&mut driver, &control));
    assert_eq!(
        poll_once(failed.as_mut(), &wakes),
        Poll::Ready(Err(CanWorkerError::Wait(WaitError::TooManyWaiters)))
    );
    assert_eq!(polls.get(), 0);
    assert_eq!(cancelled.get(), 1);
    assert_eq!(service.in_flight(), Some(frame(1)));
    assert_eq!(service.queued_len(), 1);
    assert_eq!(service.stats().sent, 0);
    assert_eq!(service.stats().driver_errors, 0);
    blockers.borrow_mut().clear();

    // The failed future remains alive; Ready must already release ownership.
    let probe = Probe::default();
    probe.resolve(Ok(CanTxOutcome::default()));
    let mut retry_driver = Driver {
        probe: probe.clone(),
        control: &control,
    };
    let mut retry = Box::pin(service.run_controlled(&mut retry_driver, &control));
    assert!(poll_once(retry.as_mut(), &wakes).is_pending());
    control.request_stop();
    assert_eq!(
        poll_once(retry.as_mut(), &wakes),
        Poll::Ready(Ok(RunExit::Stopped))
    );
    assert_eq!(probe.0.borrow().accepted, [frame(1)]);
    assert_eq!(service.stats().sent, 1);
    assert_eq!(service.in_flight(), None);
    assert_eq!(service.queued_len(), 1);
}

#[test]
fn requests_inside_driver_poll_respect_pending_success_and_failure_boundaries() {
    for request in [Request::Stop, Request::Abort] {
        for outcome in [
            None,
            Some(Ok(CanTxOutcome::default())),
            Some(Err(CanError::BusOff)),
        ] {
            let service = CanTxService::<2>::new();
            service.try_enqueue(frame(1)).unwrap();
            service.try_enqueue(frame(2)).unwrap();
            let control = RunControl::new();
            let probe = Probe::default();
            {
                let mut state = probe.0.borrow_mut();
                state.request_in_poll = Some(request);
                state.next = outcome;
            }
            let mut driver = Driver {
                probe: probe.clone(),
                control: &control,
            };
            let wakes = Arc::new(WakeCounter::default());
            let mut worker = Box::pin(service.run_controlled(&mut driver, &control));
            let first = poll_once(worker.as_mut(), &wakes);
            assert!(wakes.count() > 0, "in-poll requests must notify the worker");
            match outcome {
                None => {
                    assert!(first.is_pending());
                    probe.resolve(Ok(CanTxOutcome::default()));
                    assert_eq!(
                        poll_once(worker.as_mut(), &wakes),
                        Poll::Ready(Ok(request.exit()))
                    );
                    let graceful = matches!(request, Request::Stop);
                    assert_eq!(service.stats().sent, u64::from(graceful));
                    assert_eq!(probe.0.borrow().cancelled, usize::from(!graceful));
                    assert_eq!(probe.0.borrow().polls, if graceful { 2 } else { 1 });
                    assert_eq!(service.in_flight(), (!graceful).then(|| frame(1)));
                }
                Some(Ok(_)) => {
                    assert_eq!(first, Poll::Ready(Ok(request.exit())));
                    assert_eq!(service.stats().sent, 1);
                    assert_eq!(service.in_flight(), None);
                    assert_eq!(probe.0.borrow().accepted, [frame(1)]);
                }
                Some(Err(error)) => {
                    assert_eq!(first, Poll::Ready(Err(CanWorkerError::Driver(error))));
                    assert_eq!(service.stats().sent, 0);
                    assert_eq!(service.stats().driver_errors, 1);
                    assert_eq!(service.in_flight(), Some(frame(1)));
                    assert_eq!(probe.0.borrow().cancelled, 0);
                }
            }
            assert_eq!(service.queued_len(), 1);
            assert_eq!(probe.0.borrow().attempted, [frame(1)]);
        }
    }
}

#[test]
fn overflow_while_pending_protects_in_flight_across_abort_and_new_control_restart() {
    let service = CanTxService::<2>::new();
    service.try_enqueue(frame(1)).unwrap();
    let control = RunControl::new();
    let probe = Probe::default();
    let mut driver = Driver {
        probe: probe.clone(),
        control: &control,
    };
    let wakes = Arc::new(WakeCounter::default());
    let mut worker = Box::pin(service.run_controlled(&mut driver, &control));
    assert!(poll_once(worker.as_mut(), &wakes).is_pending());
    for value in 2..=5 {
        let outcome = service.try_enqueue(frame(value)).unwrap();
        assert_eq!(outcome.dropped, (value >= 4).then(|| frame(value - 2)));
        assert_eq!(service.in_flight(), Some(frame(1)));
    }
    control.request_abort();
    assert_eq!(
        poll_once(worker.as_mut(), &wakes),
        Poll::Ready(Ok(RunExit::Aborted))
    );
    drop(worker);
    assert_eq!(service.stats().dropped, 2);
    assert_eq!(service.stats().sent, 0);
    assert_eq!(probe.0.borrow().cancelled, 1);

    // A latched aborted control cannot accidentally consume the retained frame.
    let attempts = probe.0.borrow().attempted.len();
    let mut stale = Box::pin(service.run_controlled(&mut driver, &control));
    assert_eq!(
        poll_once(stale.as_mut(), &wakes),
        Poll::Ready(Ok(RunExit::Aborted))
    );
    assert_eq!(probe.0.borrow().attempted.len(), attempts);
    drop(stale);

    let restart = RunControl::new();
    let mut resumed = Box::pin(service.run_controlled(&mut driver, &restart));
    for expected in [frame(1), frame(4), frame(5)] {
        probe.resolve(Ok(CanTxOutcome::default()));
        assert!(poll_once(resumed.as_mut(), &wakes).is_pending());
        assert_eq!(probe.0.borrow().accepted.last(), Some(&expected));
    }
    restart.request_stop();
    assert_eq!(
        poll_once(resumed.as_mut(), &wakes),
        Poll::Ready(Ok(RunExit::Stopped))
    );
    assert_eq!(probe.0.borrow().accepted, [frame(1), frame(4), frame(5)]);
    assert_eq!(service.queued_len(), 0);
    assert_eq!(service.in_flight(), None);
    let stats = service.stats();
    assert_eq!(stats.enqueued, 5);
    assert_eq!(stats.sent + stats.dropped, stats.enqueued);
    assert_eq!(stats.driver_errors, 0);
    assert_eq!(stats.discarded, 0);
}

#[test]
fn zero_capacity_rejects_owned_frames_and_still_allows_idle_stop_and_abort() {
    for request in [Request::Stop, Request::Abort] {
        let service = CanTxService::<0>::new();
        assert_eq!(service.capacity(), 0);
        assert_eq!(service.try_enqueue(frame(1)), Err(frame(1)));
        let control = RunControl::new();
        let probe = Probe::default();
        let mut driver = Driver {
            probe: probe.clone(),
            control: &control,
        };
        let wakes = Arc::new(WakeCounter::default());
        let mut worker = Box::pin(service.run_controlled(&mut driver, &control));
        assert!(poll_once(worker.as_mut(), &wakes).is_pending());
        request.apply(&control);
        assert_eq!(wakes.count(), 1);
        assert_eq!(
            poll_once(worker.as_mut(), &wakes),
            Poll::Ready(Ok(request.exit()))
        );
        assert!(probe.0.borrow().attempted.is_empty());
        assert_eq!(service.stats(), CanTxStats::default());
        assert_eq!(service.queued_len(), 0);
        assert_eq!(service.in_flight(), None);
        assert_eq!(service.try_discard_in_flight(), Ok(None));
    }
}

#[test]
fn shared_controls_broadcast_to_services_while_independent_controls_remain_isolated() {
    for shared in [false, true] {
        let service_a = CanTxService::<1>::new();
        let service_b = CanTxService::<1>::new();
        service_a.try_enqueue(frame(1)).unwrap();
        service_b.try_enqueue(frame(2)).unwrap();
        let control_a = RunControl::new();
        let independent_b = RunControl::new();
        let control_b = if shared { &control_a } else { &independent_b };
        let probe_a = Probe::default();
        let probe_b = Probe::default();
        let mut driver_a = Driver {
            probe: probe_a.clone(),
            control: &control_a,
        };
        let mut driver_b = Driver {
            probe: probe_b.clone(),
            control: control_b,
        };
        let wakes_a = Arc::new(WakeCounter::default());
        let wakes_b = Arc::new(WakeCounter::default());
        let mut worker_a = Box::pin(service_a.run_controlled(&mut driver_a, &control_a));
        let mut worker_b = Box::pin(service_b.run_controlled(&mut driver_b, control_b));
        assert!(poll_once(worker_a.as_mut(), &wakes_a).is_pending());
        assert!(poll_once(worker_b.as_mut(), &wakes_b).is_pending());
        control_a.request_stop();
        assert_eq!(wakes_a.count(), 1);
        assert_eq!(wakes_b.count(), usize::from(shared));
        assert!(poll_once(worker_a.as_mut(), &wakes_a).is_pending());
        assert!(poll_once(worker_b.as_mut(), &wakes_b).is_pending());

        probe_b.resolve(Ok(CanTxOutcome::default()));
        let result_b = poll_once(worker_b.as_mut(), &wakes_b);
        if shared {
            assert_eq!(result_b, Poll::Ready(Ok(RunExit::Stopped)));
        } else {
            assert!(result_b.is_pending());
            control_b.request_abort();
            assert_eq!(
                poll_once(worker_b.as_mut(), &wakes_b),
                Poll::Ready(Ok(RunExit::Aborted))
            );
        }
        assert_eq!(service_a.in_flight(), Some(frame(1)));
        assert_eq!(probe_a.0.borrow().cancelled, 0);
        probe_a.resolve(Ok(CanTxOutcome::default()));
        assert_eq!(
            poll_once(worker_a.as_mut(), &wakes_a),
            Poll::Ready(Ok(RunExit::Stopped))
        );
        assert_eq!(probe_a.0.borrow().accepted, [frame(1)]);
        assert_eq!(probe_b.0.borrow().accepted, [frame(2)]);
        assert_eq!(service_a.stats().sent, 1);
        assert_eq!(service_b.stats().sent, 1);
    }
}
