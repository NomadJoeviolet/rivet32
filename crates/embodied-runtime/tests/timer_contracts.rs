#![cfg(not(target_os = "none"))]
#![forbid(unsafe_code)]

//! Scheduler contracts use the monotonic `Clock` interface and explicit polls.

#[allow(dead_code)]
mod common;

use std::{cell::Cell, sync::Arc, task::Poll};

use common::{ManualClock, WakeCounter, poll_once};
use embodied_core::time::{Duration, Instant};
use embodied_runtime::{
    timer::{TimerError, TimerScheduler},
    wait::MAX_WAITERS,
};

#[test]
fn changing_a_timer_rearms_the_pending_sleep_for_earlier_and_later_deadlines() {
    for (old_deadline, new_deadline) in [(100, 20), (20, 100)] {
        let scheduler = TimerScheduler::<1>::new();
        let id = scheduler.create().unwrap();
        scheduler
            .start_once(id, Instant::ZERO, Duration::from_micros(old_deadline))
            .unwrap();
        let clock = ManualClock::default();
        let wakes = Arc::new(WakeCounter::default());
        let mut next = Box::pin(scheduler.next_event(&clock));
        assert!(poll_once(next.as_mut(), &wakes).is_pending());
        assert_eq!(clock.pending_sleeps(), 1);

        scheduler
            .change_period(id, Instant::ZERO, Duration::from_micros(new_deadline))
            .unwrap();
        assert_eq!(
            wakes.count(),
            1,
            "a changed deadline must wake the dispatcher"
        );
        assert!(poll_once(next.as_mut(), &wakes).is_pending());
        assert_eq!(clock.pending_sleeps(), 1, "the old sleep must be cancelled");
        if old_deadline < new_deadline {
            clock.advance_to(old_deadline);
            assert_eq!(wakes.count(), 1, "the cancelled sleep must not wake again");
            assert!(poll_once(next.as_mut(), &wakes).is_pending());
        }
        clock.advance_to(new_deadline);
        let Poll::Ready(Ok(event)) = poll_once(next.as_mut(), &wakes) else {
            panic!("the rearmed timer must fire at its new deadline");
        };
        assert_eq!(event.id(), id);
        assert_eq!(event.scheduled(), Instant::from_micros(new_deadline));
        assert_eq!(clock.pending_sleeps(), 0);
        assert_eq!(scheduler.next_deadline(), None);
    }
}

#[test]
fn stop_cancels_pending_sleep_and_suppresses_an_uncommitted_due_event() {
    let scheduler = TimerScheduler::<1>::new();
    let id = scheduler.create().unwrap();
    scheduler
        .start_once(id, Instant::ZERO, Duration::from_micros(10))
        .unwrap();
    let clock = ManualClock::default();
    let wakes = Arc::new(WakeCounter::default());
    let mut next = Box::pin(scheduler.next_event(&clock));
    assert!(poll_once(next.as_mut(), &wakes).is_pending());
    scheduler.stop(id).unwrap();
    assert_eq!(wakes.count(), 1);
    assert!(poll_once(next.as_mut(), &wakes).is_pending());
    assert_eq!(clock.pending_sleeps(), 0);
    clock.advance_to(10);
    assert_eq!(wakes.count(), 1);
    assert!(poll_once(next.as_mut(), &wakes).is_pending());

    scheduler
        .start_once(id, Instant::from_micros(10), Duration::ZERO)
        .unwrap();
    let Poll::Ready(Ok(event)) = poll_once(next.as_mut(), &wakes) else {
        panic!("a new immediate timer must wake an idle dispatcher");
    };
    scheduler.stop(id).unwrap();
    let called = Cell::new(false);
    assert!(!scheduler.dispatch(event, |_| called.set(true)));
    assert!(!called.get());
    assert_eq!(scheduler.next_deadline(), None);
}

#[test]
fn cancelling_a_woken_event_wait_neither_consumes_the_event_nor_leaks_waiter_slots() {
    let scheduler = TimerScheduler::<1>::new();
    let id = scheduler.create().unwrap();
    let clock = ManualClock::default();
    let wakes = Arc::new(WakeCounter::default());
    for deadline in 1..=(MAX_WAITERS as u64 * 2) {
        scheduler
            .start_once(
                id,
                Instant::from_micros(deadline - 1),
                Duration::from_micros(1),
            )
            .unwrap();
        let mut cancelled = Box::pin(scheduler.next_event(&clock));
        assert!(poll_once(cancelled.as_mut(), &wakes).is_pending());
        assert_eq!(clock.pending_sleeps(), 1);
        clock.advance_to(deadline);
        drop(cancelled);
        assert_eq!(clock.pending_sleeps(), 0);

        let mut fresh = Box::pin(scheduler.next_event(&clock));
        let Poll::Ready(Ok(event)) = poll_once(fresh.as_mut(), &wakes) else {
            panic!("cancelling before Ready must leave the due event available");
        };
        assert_eq!(event.id(), id);
        assert_eq!(event.scheduled(), Instant::from_micros(deadline));
        assert!(
            scheduler
                .take_due(Instant::from_micros(deadline))
                .unwrap()
                .is_none()
        );
    }
}

#[test]
fn timer_ids_reject_foreign_owners_and_survive_moving_their_scheduler() {
    let first = TimerScheduler::<1>::new();
    let second = TimerScheduler::<1>::new();
    let first_id = first.create().unwrap();
    let second_id = second.create().unwrap();
    assert_ne!(first_id, second_id);
    assert_eq!(first.create(), Err(TimerError::Full));
    assert_eq!(TimerScheduler::<0>::new().create(), Err(TimerError::Full));
    let moved = first;
    moved
        .start_once(first_id, Instant::ZERO, Duration::ZERO)
        .unwrap();
    assert_eq!(
        second.start_once(first_id, Instant::ZERO, Duration::ZERO),
        Err(TimerError::InvalidId)
    );
    assert_eq!(
        second.change_period(first_id, Instant::ZERO, Duration::ZERO),
        Err(TimerError::InvalidId)
    );
    assert_eq!(second.stop(first_id), Err(TimerError::InvalidId));
    let foreign_event = moved.take_due(Instant::ZERO).unwrap().unwrap();
    assert!(!second.dispatch(foreign_event, |_| panic!("foreign callback must not run")));
    assert_eq!(second.next_deadline(), None);
    moved
        .start_once(first_id, Instant::ZERO, Duration::ZERO)
        .unwrap();
    let own_event = moved.take_due(Instant::ZERO).unwrap().unwrap();
    assert!(moved.dispatch(own_event, |event| assert_eq!(event.id(), first_id)));
}

#[test]
fn late_periodic_events_skip_missed_periods_keep_phase_and_preserve_deadline_order() {
    let scheduler = TimerScheduler::<3>::new();
    let periodic = scheduler.create().unwrap();
    let tied = scheduler.create().unwrap();
    let earlier = scheduler.create().unwrap();
    scheduler
        .start_periodic(periodic, Instant::ZERO, Duration::from_micros(10))
        .unwrap();
    scheduler
        .start_once(tied, Instant::ZERO, Duration::from_micros(10))
        .unwrap();
    scheduler
        .start_once(earlier, Instant::ZERO, Duration::from_micros(5))
        .unwrap();
    let now = Instant::from_micros(35);
    assert_eq!(scheduler.take_due(now).unwrap().unwrap().id(), earlier);
    let event = scheduler.take_due(now).unwrap().unwrap();
    assert_eq!(
        event.id(),
        periodic,
        "equal deadlines use registration order"
    );
    assert_eq!(event.scheduled(), Instant::from_micros(10));
    assert_eq!(event.observed(), now);
    assert_eq!(event.missed(), 2);
    assert_eq!(scheduler.take_due(now).unwrap().unwrap().id(), tied);
    assert!(scheduler.take_due(now).unwrap().is_none());
    assert_eq!(scheduler.next_deadline(), Some(Instant::from_micros(40)));
    let event = scheduler
        .take_due(Instant::from_micros(40))
        .unwrap()
        .unwrap();
    assert_eq!(event.scheduled(), Instant::from_micros(40));
    assert_eq!(event.missed(), 0);
    assert_eq!(scheduler.next_deadline(), Some(Instant::from_micros(50)));
}

#[test]
fn overflow_errors_preserve_the_schedule_and_an_already_issued_event() {
    let scheduler = TimerScheduler::<1>::new();
    let id = scheduler.create().unwrap();
    scheduler
        .start_periodic(id, Instant::ZERO, Duration::from_micros(10))
        .unwrap();
    let issued = scheduler
        .take_due(Instant::from_micros(10))
        .unwrap()
        .unwrap();
    assert_eq!(
        scheduler.start_once(id, Instant::from_micros(u64::MAX), Duration::from_micros(1)),
        Err(TimerError::Overflow)
    );
    assert_eq!(scheduler.next_deadline(), Some(Instant::from_micros(20)));
    assert_eq!(
        scheduler.change_period(id, Instant::from_micros(u64::MAX), Duration::from_micros(1)),
        Err(TimerError::Overflow)
    );
    assert_eq!(scheduler.next_deadline(), Some(Instant::from_micros(20)));
    assert!(
        scheduler.dispatch(issued, |_| {}),
        "failed changes must not invalidate an event"
    );
    assert!(matches!(
        scheduler.take_due(Instant::from_micros(u64::MAX)),
        Err(TimerError::Overflow)
    ));
    assert_eq!(scheduler.next_deadline(), Some(Instant::from_micros(20)));
    let retry = scheduler
        .take_due(Instant::from_micros(20))
        .unwrap()
        .unwrap();
    assert_eq!(retry.scheduled(), Instant::from_micros(20));
    assert_eq!(retry.missed(), 0);
}

#[test]
fn zero_period_is_rejected_without_mutation_and_stopped_timers_keep_their_mode() {
    let scheduler = TimerScheduler::<1>::new();
    let id = scheduler.create().unwrap();
    scheduler
        .start_periodic(id, Instant::ZERO, Duration::from_micros(10))
        .unwrap();
    assert_eq!(
        scheduler.start_periodic(id, Instant::ZERO, Duration::ZERO),
        Err(TimerError::ZeroPeriod)
    );
    assert_eq!(
        scheduler.change_period(id, Instant::ZERO, Duration::ZERO),
        Err(TimerError::ZeroPeriod)
    );
    assert_eq!(scheduler.next_deadline(), Some(Instant::from_micros(10)));
    scheduler.stop(id).unwrap();
    assert_eq!(
        scheduler.change_period(id, Instant::ZERO, Duration::ZERO),
        Err(TimerError::ZeroPeriod)
    );
    assert_eq!(scheduler.next_deadline(), None);
    scheduler
        .change_period(id, Instant::from_micros(10), Duration::from_micros(5))
        .unwrap();
    scheduler
        .take_due(Instant::from_micros(15))
        .unwrap()
        .unwrap();
    assert_eq!(scheduler.next_deadline(), Some(Instant::from_micros(20)));

    scheduler
        .start_once(id, Instant::from_micros(15), Duration::ZERO)
        .unwrap();
    let immediate = scheduler
        .take_due(Instant::from_micros(15))
        .unwrap()
        .unwrap();
    assert_eq!(immediate.missed(), 0);
    assert_eq!(scheduler.next_deadline(), None);
    scheduler
        .change_period(id, Instant::from_micros(15), Duration::ZERO)
        .unwrap();
    assert!(
        scheduler
            .take_due(Instant::from_micros(15))
            .unwrap()
            .is_some()
    );
    assert_eq!(scheduler.next_deadline(), None);
}

#[test]
fn changed_timers_reject_stale_events_and_dispatch_callbacks_can_rearm() {
    let scheduler = TimerScheduler::<1>::new();
    let id = scheduler.create().unwrap();
    scheduler
        .start_once(id, Instant::ZERO, Duration::ZERO)
        .unwrap();
    let stale = scheduler.take_due(Instant::ZERO).unwrap().unwrap();
    scheduler
        .change_period(id, Instant::ZERO, Duration::from_micros(5))
        .unwrap();
    assert!(!scheduler.dispatch(stale, |_| panic!("stale callback must not run")));
    let current = scheduler
        .take_due(Instant::from_micros(5))
        .unwrap()
        .unwrap();
    assert!(scheduler.dispatch(current, |event| {
        scheduler
            .start_once(event.id(), event.observed(), Duration::from_micros(7))
            .unwrap();
    }));
    assert_eq!(scheduler.next_deadline(), Some(Instant::from_micros(12)));
    let next = scheduler
        .take_due(Instant::from_micros(12))
        .unwrap()
        .unwrap();
    assert_eq!(next.id(), id);
    assert!(scheduler.dispatch(next, |event| scheduler.stop(event.id()).unwrap()));
    assert_eq!(scheduler.next_deadline(), None);
}
