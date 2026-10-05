//! Host-only deterministic clock and executor probes, implemented for this crate.
//!
//! The idea of controlling observed time in tests is informed by Warp's test
//! clock, not copied from it. Unlike Warp's wall-clock offset, this helper drives
//! the existing monotonic `Clock` trait and unregisters each cancelled sleep:
//! https://github.com/warpdotdev/warp/blob/b865631c9a0e46b548c7ec7dc32e228a148171d1/crates/warpui_core/src/time.rs

use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    future::Future,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
};

use embodied_core::time::Instant;
use embodied_runtime::clock::Clock;

#[derive(Default)]
pub struct WakeCounter(AtomicUsize);

impl WakeCounter {
    pub fn count(&self) -> usize {
        self.0.load(Ordering::Relaxed)
    }
}

impl Wake for WakeCounter {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

pub fn poll_once<F: Future>(future: Pin<&mut F>, wakes: &Arc<WakeCounter>) -> Poll<F::Output> {
    let waker = Waker::from(Arc::clone(wakes));
    future.poll(&mut Context::from_waker(&waker))
}

struct Waiter {
    deadline: Instant,
    waker: Option<Waker>,
}

#[derive(Default)]
pub struct ManualClock {
    now: Cell<Instant>,
    next_id: Cell<u64>,
    waiters: RefCell<BTreeMap<u64, Waiter>>,
}

impl ManualClock {
    pub fn advance_to(&self, micros: u64) {
        let now = Instant::from_micros(micros);
        assert!(
            now >= self.now.get(),
            "monotonic clock cannot move backwards"
        );
        self.now.set(now);
        let wakers: Vec<_> = self
            .waiters
            .borrow_mut()
            .values_mut()
            .filter(|waiter| waiter.deadline <= now)
            .filter_map(|waiter| waiter.waker.take())
            .collect();
        for waker in wakers {
            waker.wake();
        }
    }

    pub fn pending_sleeps(&self) -> usize {
        self.waiters.borrow().len()
    }
}

impl Clock for ManualClock {
    fn now(&self) -> Instant {
        self.now.get()
    }

    fn sleep_until(&self, deadline: Instant) -> impl Future<Output = ()> {
        ManualSleep {
            clock: self,
            deadline,
            id: None,
        }
    }
}

struct ManualSleep<'a> {
    clock: &'a ManualClock,
    deadline: Instant,
    id: Option<u64>,
}

impl Future for ManualSleep<'_> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.clock.now() >= self.deadline {
            if let Some(id) = self.id.take() {
                self.clock.waiters.borrow_mut().remove(&id);
            }
            return Poll::Ready(());
        }
        let id = match self.id {
            Some(id) => id,
            None => {
                let id = self.clock.next_id.get();
                self.clock
                    .next_id
                    .set(id.checked_add(1).expect("test waiter id overflow"));
                self.id = Some(id);
                id
            }
        };
        self.clock.waiters.borrow_mut().insert(
            id,
            Waiter {
                deadline: self.deadline,
                waker: Some(cx.waker().clone()),
            },
        );
        Poll::Pending
    }
}

impl Drop for ManualSleep<'_> {
    fn drop(&mut self) {
        if let Some(id) = self.id.take() {
            self.clock.waiters.borrow_mut().remove(&id);
        }
    }
}
