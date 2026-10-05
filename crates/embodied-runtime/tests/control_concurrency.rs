#![cfg(not(target_os = "none"))]
#![forbid(unsafe_code)]

//! Host-thread scheduling checks complement the deterministic worker tests.
//! These exercise the host critical-section implementation, not MCU interrupts.

#[allow(dead_code)]
mod common;

use std::{
    future::Future,
    sync::{Arc, Barrier, mpsc},
    task::{Context, Poll, Wake, Waker},
    thread,
    time::Duration,
};

use common::{WakeCounter, poll_once};
use embodied_runtime::{
    control::{RunControl, RunState},
    wait::MAX_WAITERS,
};

const RACE_ITERATIONS: usize = 128;
const NOTIFICATION_TIMEOUT: Duration = Duration::from_secs(5);

struct WakeNotification(mpsc::Sender<()>);

impl Wake for WakeNotification {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        // A late wake after the receiver is dropped is permitted.
        let _ = self.0.send(());
    }
}

fn request(control: &RunControl, state: RunState) {
    match state {
        RunState::Stopping => control.request_stop(),
        RunState::Aborted => control.request_abort(),
        RunState::Running => unreachable!("Running is not a lifecycle request"),
    }
}

#[test]
fn cross_thread_requests_cannot_be_lost_during_first_poll_registration() {
    // 256 contested schedules: either first poll sees the request, or its
    // registered executor must be notified. No retry polling masks lost wakes.
    for requested in [RunState::Stopping, RunState::Aborted] {
        for iteration in 0..RACE_ITERATIONS {
            let control = RunControl::new();
            let start = Barrier::new(2);
            let (sender, notifications) = mpsc::channel();
            let waker = Waker::from(Arc::new(WakeNotification(sender)));
            let mut changed = Box::pin(control.changed_since(RunState::Running));
            thread::scope(|scope| {
                scope.spawn(|| {
                    start.wait();
                    request(&control, requested);
                });
                start.wait();
                let first = changed.as_mut().poll(&mut Context::from_waker(&waker));
                if first.is_pending() {
                    notifications
                        .recv_timeout(NOTIFICATION_TIMEOUT)
                        .unwrap_or_else(|error| {
                            panic!("missing {requested:?} wake in schedule {iteration}: {error}")
                        });
                    assert_eq!(
                        changed.as_mut().poll(&mut Context::from_waker(&waker)),
                        Poll::Ready(Ok(requested)),
                        "woken waiter must observe the published request"
                    );
                } else {
                    assert_eq!(first, Poll::Ready(Ok(requested)));
                }
            });
        }
    }
}

#[test]
fn concurrent_stop_and_abort_always_publish_abort_and_notify_waiters() {
    for iteration in 0..RACE_ITERATIONS {
        let control = RunControl::new();
        let start = Barrier::new(3);
        let (done, completed) = mpsc::channel();
        let wakes = Arc::new(WakeCounter::default());
        let mut changed = Box::pin(control.changed_since(RunState::Running));
        assert!(poll_once(changed.as_mut(), &wakes).is_pending());
        thread::scope(|scope| {
            for requested in [RunState::Stopping, RunState::Aborted] {
                let done = done.clone();
                let control = &control;
                let start = &start;
                scope.spawn(move || {
                    start.wait();
                    request(control, requested);
                    done.send(()).unwrap();
                });
            }
            start.wait();
            for _ in 0..2 {
                completed
                    .recv_timeout(NOTIFICATION_TIMEOUT)
                    .unwrap_or_else(|error| {
                        panic!("request did not finish in schedule {iteration}: {error}")
                    });
            }
            assert_eq!(control.state(), RunState::Aborted);
            assert!(wakes.count() > 0);
            assert_eq!(
                poll_once(changed.as_mut(), &wakes),
                Poll::Ready(Ok(RunState::Aborted))
            );
            control.request_stop();
            assert_eq!(control.state(), RunState::Aborted);
        });
    }
}

#[test]
fn every_registered_executor_receives_stop_and_then_abort() {
    let control = RunControl::new();
    let wakes: Vec<_> = (0..MAX_WAITERS)
        .map(|_| Arc::new(WakeCounter::default()))
        .collect();
    let mut stopping: Vec<_> = (0..MAX_WAITERS)
        .map(|_| Box::pin(control.changed_since(RunState::Running)))
        .collect();
    for (waiter, wakes) in stopping.iter_mut().zip(&wakes) {
        assert!(poll_once(waiter.as_mut(), wakes).is_pending());
    }
    control.request_stop();
    for (waiter, wakes) in stopping.iter_mut().zip(&wakes) {
        assert_eq!(wakes.count(), 1);
        assert_eq!(
            poll_once(waiter.as_mut(), wakes),
            Poll::Ready(Ok(RunState::Stopping))
        );
    }

    // Completed futures intentionally remain alive: Ready must release all
    // registration capacity, allowing the next lifecycle stage to subscribe.
    let mut aborting: Vec<_> = (0..MAX_WAITERS)
        .map(|_| Box::pin(control.changed_since(RunState::Stopping)))
        .collect();
    for (waiter, wakes) in aborting.iter_mut().zip(&wakes) {
        assert!(poll_once(waiter.as_mut(), wakes).is_pending());
    }
    control.request_stop();
    for wakes in &wakes {
        assert_eq!(wakes.count(), 1, "duplicate stop must not notify again");
    }
    control.request_abort();
    control.request_abort();
    for (waiter, wakes) in aborting.iter_mut().zip(&wakes) {
        assert_eq!(wakes.count(), 2);
        assert_eq!(
            poll_once(waiter.as_mut(), wakes),
            Poll::Ready(Ok(RunState::Aborted))
        );
    }
}

#[test]
fn repolling_routes_notification_only_to_the_latest_executor() {
    let control = RunControl::new();
    let old = Arc::new(WakeCounter::default());
    let current = Arc::new(WakeCounter::default());
    let mut changed = Box::pin(control.changed_since(RunState::Running));
    assert!(poll_once(changed.as_mut(), &old).is_pending());
    for _ in 0..MAX_WAITERS * 2 {
        assert!(poll_once(changed.as_mut(), &current).is_pending());
    }
    control.request_stop();
    assert_eq!(old.count(), 0, "stale executor must not receive the wake");
    assert_eq!(current.count(), 1);
    assert_eq!(
        poll_once(changed.as_mut(), &current),
        Poll::Ready(Ok(RunState::Stopping))
    );
    control.request_stop();
    assert_eq!(current.count(), 1);
}

#[test]
fn dropping_a_waiter_before_slot_reuse_does_not_wake_the_old_executor() {
    let control = RunControl::new();
    let old = Arc::new(WakeCounter::default());
    let current = Arc::new(WakeCounter::default());
    let mut cancelled = Box::pin(control.changed_since(RunState::Running));
    assert!(poll_once(cancelled.as_mut(), &old).is_pending());
    drop(cancelled);
    let mut replacement = Box::pin(control.changed_since(RunState::Running));
    assert!(poll_once(replacement.as_mut(), &current).is_pending());
    control.request_abort();
    assert_eq!(old.count(), 0);
    assert_eq!(current.count(), 1);
    assert_eq!(
        poll_once(replacement.as_mut(), &current),
        Poll::Ready(Ok(RunState::Aborted))
    );
}
