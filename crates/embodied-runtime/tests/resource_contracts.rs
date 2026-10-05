#![cfg(not(target_os = "none"))]
#![forbid(unsafe_code)]

//! Resource ownership at real cancellation, timeout, and capacity boundaries.
//! All time is supplied by the host-only manual clock; no executor or sleep is used.

#[allow(dead_code)]
mod common;

use std::{cell::Cell, future::pending, rc::Rc, sync::Arc, task::Poll};

use common::{ManualClock, WakeCounter, poll_once};
use embodied_core::time::Instant;
use embodied_runtime::{
    clock::Clock,
    mutex::AsyncMutex,
    pool::MessagePool,
    queue::SharedQueue,
    semaphore::CountingSemaphore,
    wait::{MAX_WAITERS, WaitError},
};

#[test]
fn cancelling_a_guard_holder_restores_mutated_value_and_wakes_the_next_lock() {
    let mutex = AsyncMutex::new(7);
    let wakes = Arc::new(WakeCounter::default());
    let mut holder = Box::pin(async {
        let mut guard = mutex.lock().await.unwrap();
        *guard = 9;
        pending::<()>().await;
        drop(guard);
    });
    assert!(poll_once(holder.as_mut(), &wakes).is_pending());
    assert!(mutex.try_lock().is_none());
    let mut waiting = Box::pin(mutex.lock());
    assert!(poll_once(waiting.as_mut(), &wakes).is_pending());

    drop(holder);
    assert_eq!(wakes.count(), 1);
    let Poll::Ready(Ok(mut guard)) = poll_once(waiting.as_mut(), &wakes) else {
        panic!("cancelling the holder must unlock the mutex");
    };
    assert_eq!(*guard, 9);
    *guard = 11;
    drop(guard);
    drop(waiting);
    assert_eq!(mutex.into_inner(), 11);
}

#[test]
fn mutex_timeout_wins_a_simultaneous_unlock_without_taking_the_value() {
    let mutex = AsyncMutex::new(7);
    let guard = mutex.try_lock().unwrap();
    let clock = ManualClock::default();
    let wakes = Arc::new(WakeCounter::default());
    let mut waiting = Box::pin(mutex.lock_timeout(clock.sleep_until(Instant::from_micros(10))));
    assert!(poll_once(waiting.as_mut(), &wakes).is_pending());
    clock.advance_to(10);
    drop(guard);
    assert!(matches!(
        poll_once(waiting.as_mut(), &wakes),
        Poll::Ready(Err(WaitError::Timeout))
    ));
    assert_eq!(clock.pending_sleeps(), 0);
    assert_eq!(*mutex.try_lock().unwrap(), 7);
}

#[test]
fn mutex_waiter_exhaustion_is_recoverable_after_cancellation() {
    let mutex = AsyncMutex::new(7);
    let held = mutex.try_lock().unwrap();
    let wakes = Arc::new(WakeCounter::default());
    for _ in 0..2 {
        let mut waiters: Vec<_> = (0..MAX_WAITERS).map(|_| Box::pin(mutex.lock())).collect();
        for waiter in &mut waiters {
            assert!(poll_once(waiter.as_mut(), &wakes).is_pending());
        }
        let mut excess = Box::pin(mutex.lock());
        assert!(matches!(
            poll_once(excess.as_mut(), &wakes),
            Poll::Ready(Err(WaitError::TooManyWaiters))
        ));
        drop(waiters);
    }
    drop(held);
    assert_eq!(wakes.count(), 0, "cancelled lockers must unregister");
    let mut lock = Box::pin(mutex.lock());
    let Poll::Ready(Ok(guard)) = poll_once(lock.as_mut(), &wakes) else {
        panic!("a free mutex must still be acquirable after waiter exhaustion");
    };
    assert_eq!(*guard, 7);
}

#[test]
fn semaphore_cancellation_returns_only_owned_permits_once() {
    let semaphore = CountingSemaphore::<2>::new();
    let wakes = Arc::new(WakeCounter::default());
    let first = semaphore.try_acquire().unwrap();
    let mut holder = Box::pin(async {
        let permit = semaphore.acquire().await.unwrap();
        pending::<()>().await;
        drop(permit);
    });
    assert!(poll_once(holder.as_mut(), &wakes).is_pending());
    assert_eq!(semaphore.available(), 0);

    let mut waiter = Box::pin(semaphore.acquire());
    assert!(poll_once(waiter.as_mut(), &wakes).is_pending());
    drop(waiter);
    assert_eq!(semaphore.available(), 0, "waiting is not ownership");
    drop(holder);
    assert_eq!(semaphore.available(), 1);
    assert_eq!(wakes.count(), 0, "cancelled waiter must not be woken");
    drop(first);
    assert_eq!(semaphore.available(), 2);
    let permits: Vec<_> = (0..2).map(|_| semaphore.try_acquire().unwrap()).collect();
    assert!(semaphore.try_acquire().is_none());
    drop(permits);
    assert_eq!(semaphore.available(), 2);
}

#[test]
fn zero_capacity_semaphore_exhaustion_and_timeouts_never_create_permits() {
    let semaphore = CountingSemaphore::<0>::new();
    let clock = ManualClock::default();
    let wakes = Arc::new(WakeCounter::default());
    for deadline in [10, 20] {
        let mut waiters: Vec<_> = (0..MAX_WAITERS)
            .map(|_| {
                Box::pin(
                    semaphore.acquire_timeout(clock.sleep_until(Instant::from_micros(deadline))),
                )
            })
            .collect();
        for waiter in &mut waiters {
            assert!(poll_once(waiter.as_mut(), &wakes).is_pending());
        }
        let mut excess = Box::pin(semaphore.acquire());
        assert!(matches!(
            poll_once(excess.as_mut(), &wakes),
            Poll::Ready(Err(WaitError::TooManyWaiters))
        ));
        clock.advance_to(deadline);
        for waiter in &mut waiters {
            assert!(matches!(
                poll_once(waiter.as_mut(), &wakes),
                Poll::Ready(Err(WaitError::Timeout))
            ));
        }
        assert_eq!(clock.pending_sleeps(), 0);
        assert_eq!(semaphore.available(), 0);
        assert!(semaphore.try_acquire().is_none());
    }
}

#[test]
fn pool_spare_slots_do_not_create_messages_and_returned_mutations_survive() {
    let empty = MessagePool::<u8, 0>::new([]);
    assert_eq!(empty.available(), 0);
    assert!(empty.try_acquire().is_none());
    let spare = MessagePool::<u8, 2>::from_slots([None, None]);
    assert_eq!(spare.available(), 0);
    assert!(spare.try_acquire().is_none());

    let pool = MessagePool::from_slots([None, Some(7), None, Some(9)]);
    for _ in 0..3 {
        assert_eq!(pool.available(), 2);
        let mut first = pool.try_acquire().unwrap();
        let second = pool.try_acquire().unwrap();
        let mut values = [*first, *second];
        values.sort_unstable();
        assert_eq!(values, [7, 9]);
        assert!(pool.try_acquire().is_none());
        let original = *first;
        *first = 8;
        drop(first);
        let mut returned = pool.try_acquire().unwrap();
        assert_eq!(*returned, 8);
        *returned = original;
        drop(returned);
        drop(second);
    }
    assert_eq!(pool.available(), 2);
}

#[test]
fn pool_cancelled_waits_and_timeouts_do_not_steal_a_returned_message() {
    let pool = MessagePool::new([7]);
    let lease = pool.try_acquire().unwrap();
    let clock = ManualClock::default();
    let wakes = Arc::new(WakeCounter::default());
    for _ in 0..MAX_WAITERS * 2 {
        let mut cancelled = Box::pin(pool.acquire());
        assert!(poll_once(cancelled.as_mut(), &wakes).is_pending());
        drop(cancelled);
    }
    let mut timed = Box::pin(pool.acquire_timeout(clock.sleep_until(Instant::from_micros(10))));
    assert!(poll_once(timed.as_mut(), &wakes).is_pending());
    clock.advance_to(10);
    drop(lease);
    assert!(matches!(
        poll_once(timed.as_mut(), &wakes),
        Poll::Ready(Err(WaitError::Timeout))
    ));
    assert_eq!(pool.available(), 1);
    assert_eq!(clock.pending_sleeps(), 0);
    assert_eq!(*pool.try_acquire().unwrap(), 7);
}

#[test]
fn front_sends_preserve_priority_and_timeout_returns_the_unsent_value() {
    let queue = SharedQueue::<u8, 2>::new();
    queue.try_send(1).unwrap();
    queue.try_send(2).unwrap();
    let clock = ManualClock::default();
    let wakes = Arc::new(WakeCounter::default());
    let mut priority = Box::pin(queue.send_front(3));
    assert!(poll_once(priority.as_mut(), &wakes).is_pending());
    assert_eq!(queue.try_receive(), Some(1));
    assert_eq!(poll_once(priority.as_mut(), &wakes), Poll::Ready(Ok(())));
    assert_eq!(queue.peek(), Some(3));

    let mut timed =
        Box::pin(queue.send_front_timeout(4, clock.sleep_until(Instant::from_micros(10))));
    assert!(poll_once(timed.as_mut(), &wakes).is_pending());
    clock.advance_to(10);
    assert_eq!(queue.try_receive(), Some(3));
    let Poll::Ready(Err(error)) = poll_once(timed.as_mut(), &wakes) else {
        panic!("an expired front send must return its value");
    };
    assert_eq!((error.reason, error.value), (WaitError::Timeout, 4));
    assert_eq!(queue.try_receive(), Some(2));
    assert!(queue.is_empty());
}

#[test]
fn receive_timeout_leaves_a_simultaneously_arriving_message_for_the_next_consumer() {
    let queue = SharedQueue::<u8, 1>::new();
    let clock = ManualClock::default();
    let wakes = Arc::new(WakeCounter::default());
    let mut receive = Box::pin(queue.receive_timeout(clock.sleep_until(Instant::from_micros(10))));
    assert!(poll_once(receive.as_mut(), &wakes).is_pending());
    clock.advance_to(10);
    queue.try_send(7).unwrap();
    assert_eq!(
        poll_once(receive.as_mut(), &wakes),
        Poll::Ready(Err(WaitError::Timeout))
    );
    assert_eq!(clock.pending_sleeps(), 0);
    assert_eq!(queue.try_receive(), Some(7));
}

#[test]
fn clearing_a_queue_allows_message_destructors_to_reenter_that_queue() {
    struct OnDrop(Option<Box<dyn FnOnce()>>);
    impl Drop for OnDrop {
        fn drop(&mut self) {
            if let Some(action) = self.0.take() {
                action();
            }
        }
    }

    let queue = Rc::new(SharedQueue::<OnDrop, 2>::new());
    let drops = Rc::new(Cell::new(0));
    for _ in 0..2 {
        let queue_ref = Rc::downgrade(&queue);
        let drops = Rc::clone(&drops);
        let value = OnDrop(Some(Box::new(move || {
            let queue = queue_ref.upgrade().unwrap();
            drops.set(drops.get() + 1);
            assert!(queue.try_send(OnDrop(None)).is_ok());
        })));
        assert!(queue.try_send(value).is_ok());
    }
    queue.clear();
    assert_eq!(drops.get(), 2);
    assert_eq!(
        queue.len(),
        2,
        "messages inserted during Drop belong to the new queue"
    );
    queue.clear();
    assert!(queue.is_empty());
    assert_eq!(
        drops.get(),
        2,
        "each removed message is dropped exactly once"
    );
}
