//! Central, fixed-capacity software timer scheduler. All deadlines are absolute.
//! One dispatcher should await `next_event` and call `dispatch` synchronously.
use crate::{
    clock::Clock,
    wait::{WaitError, WaitQueue},
};
use core::cell::{Cell, RefCell};
use critical_section::Mutex;
use embodied_core::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimerId {
    owner: u64,
    index: usize,
}
impl TimerId {
    pub const fn index(self) -> usize {
        self.index
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerError {
    Full,
    InvalidId,
    ZeroPeriod,
    Overflow,
    GenerationExhausted,
    OwnerIdExhausted,
    TooManyWaiters,
}

/// A single, non-cloneable due event. Start, stop and change invalidate older events.
#[derive(Debug)]
pub struct TimerEvent {
    id: TimerId,
    generation: u64,
    scheduled: Instant,
    observed: Instant,
    missed: u64,
}
impl TimerEvent {
    pub const fn id(&self) -> TimerId {
        self.id
    }
    pub const fn scheduled(&self) -> Instant {
        self.scheduled
    }
    pub const fn observed(&self) -> Instant {
        self.observed
    }
    pub const fn missed(&self) -> u64 {
        self.missed
    }
}

#[derive(Clone, Copy)]
struct Entry {
    used: bool,
    generation: u64,
    deadline: Option<Instant>,
    periodic: bool,
    period: Duration,
}
impl Entry {
    const EMPTY: Self = Self {
        used: false,
        generation: 0,
        deadline: None,
        periodic: false,
        period: Duration::ZERO,
    };
}

/// Timer IDs are local to this scheduler and remain allocated for its lifetime.
/// No allocation, executor dependency, busy waiting, or per-timer task is used.
pub struct TimerScheduler<const N: usize> {
    owner: Mutex<Cell<Option<u64>>>,
    entries: Mutex<RefCell<[Entry; N]>>,
    changed: WaitQueue,
}
// Independent of object address, so moving a scheduler preserves handle identity.
// Critical-section protection works on targets without native 64-bit atomics.
static NEXT_OWNER: Mutex<Cell<u64>> = Mutex::new(Cell::new(1));
impl<const N: usize> Default for TimerScheduler<N> {
    fn default() -> Self {
        Self::new()
    }
}
impl<const N: usize> TimerScheduler<N> {
    pub const fn new() -> Self {
        Self {
            owner: Mutex::new(Cell::new(None)),
            entries: Mutex::new(RefCell::new([Entry::EMPTY; N])),
            changed: WaitQueue::new(),
        }
    }
    pub fn create(&self) -> Result<TimerId, TimerError> {
        critical_section::with(|cs| {
            let mut entries = self.entries.borrow(cs).borrow_mut();
            let index = entries
                .iter()
                .position(|e| !e.used)
                .ok_or(TimerError::Full)?;
            let owner = match self.owner.borrow(cs).get() {
                Some(owner) => owner,
                None => {
                    let owner = NEXT_OWNER.borrow(cs).get();
                    let next = owner.checked_add(1).ok_or(TimerError::OwnerIdExhausted)?;
                    NEXT_OWNER.borrow(cs).set(next);
                    self.owner.borrow(cs).set(Some(owner));
                    owner
                }
            };
            entries[index].used = true;
            Ok(TimerId { owner, index })
        })
    }
    pub fn start_once(&self, id: TimerId, now: Instant, delay: Duration) -> Result<(), TimerError> {
        self.start(id, now, delay, false)
    }
    pub fn start_periodic(
        &self,
        id: TimerId,
        now: Instant,
        period: Duration,
    ) -> Result<(), TimerError> {
        self.start(id, now, period, true)
    }
    fn start(
        &self,
        id: TimerId,
        now: Instant,
        period: Duration,
        periodic: bool,
    ) -> Result<(), TimerError> {
        if periodic && period == Duration::ZERO {
            return Err(TimerError::ZeroPeriod);
        }
        let deadline = now.checked_add(period).ok_or(TimerError::Overflow)?;
        critical_section::with(|cs| {
            if self.owner.borrow(cs).get() != Some(id.owner) {
                return Err(TimerError::InvalidId);
            }
            let mut entries = self.entries.borrow(cs).borrow_mut();
            let entry = entries
                .get_mut(id.index)
                .filter(|e| e.used)
                .ok_or(TimerError::InvalidId)?;
            let generation = entry
                .generation
                .checked_add(1)
                .ok_or(TimerError::GenerationExhausted)?;
            *entry = Entry {
                used: true,
                generation,
                deadline: Some(deadline),
                periodic,
                period,
            };
            Ok(())
        })?;
        self.changed.wake_all();
        Ok(())
    }
    /// Rearm from `now`, preserving one-shot versus periodic mode, even if stopped.
    pub fn change_period(
        &self,
        id: TimerId,
        now: Instant,
        period: Duration,
    ) -> Result<(), TimerError> {
        // Read and update in one critical section so a concurrent start cannot change mode.
        let result = critical_section::with(|cs| {
            if self.owner.borrow(cs).get() != Some(id.owner) {
                return Err(TimerError::InvalidId);
            }
            let mut entries = self.entries.borrow(cs).borrow_mut();
            let entry = entries
                .get_mut(id.index)
                .filter(|e| e.used)
                .ok_or(TimerError::InvalidId)?;
            if entry.periodic && period == Duration::ZERO {
                return Err(TimerError::ZeroPeriod);
            }
            let deadline = now.checked_add(period).ok_or(TimerError::Overflow)?;
            let generation = entry
                .generation
                .checked_add(1)
                .ok_or(TimerError::GenerationExhausted)?;
            entry.period = period;
            entry.deadline = Some(deadline);
            entry.generation = generation;
            Ok(())
        });
        if result.is_ok() {
            self.changed.wake_all();
        }
        result
    }
    pub fn stop(&self, id: TimerId) -> Result<(), TimerError> {
        critical_section::with(|cs| {
            if self.owner.borrow(cs).get() != Some(id.owner) {
                return Err(TimerError::InvalidId);
            }
            let mut entries = self.entries.borrow(cs).borrow_mut();
            let entry = entries
                .get_mut(id.index)
                .filter(|e| e.used)
                .ok_or(TimerError::InvalidId)?;
            entry.generation = entry
                .generation
                .checked_add(1)
                .ok_or(TimerError::GenerationExhausted)?;
            entry.deadline = None;
            Ok(())
        })?;
        self.changed.wake_all();
        Ok(())
    }
    pub fn next_deadline(&self) -> Option<Instant> {
        critical_section::with(|cs| {
            self.entries
                .borrow(cs)
                .borrow()
                .iter()
                .filter_map(|e| e.deadline)
                .min()
        })
    }
    /// Pop the earliest due timer; equal deadlines use registration order.
    /// Periodic timers skip missed periods and preserve phase. Overflow leaves state unchanged.
    pub fn take_due(&self, now: Instant) -> Result<Option<TimerEvent>, TimerError> {
        critical_section::with(|cs| {
            let mut entries = self.entries.borrow(cs).borrow_mut();
            let Some((index, scheduled)) = entries
                .iter()
                .enumerate()
                .filter_map(|(i, e)| e.deadline.filter(|d| *d <= now).map(|d| (i, d)))
                .min_by_key(|(i, d)| (*d, *i))
            else {
                return Ok(None);
            };
            let entry = &mut entries[index];
            let missed = if entry.periodic {
                let late = now
                    .checked_duration_since(scheduled)
                    .expect("due timer")
                    .as_micros();
                let missed = late / entry.period.as_micros();
                let advance = missed
                    .checked_add(1)
                    .and_then(|n| n.checked_mul(entry.period.as_micros()))
                    .ok_or(TimerError::Overflow)?;
                let next = scheduled
                    .checked_add(Duration::from_micros(advance))
                    .ok_or(TimerError::Overflow)?;
                entry.deadline = Some(next);
                missed
            } else {
                entry.deadline = None;
                0
            };
            Ok(Some(TimerEvent {
                id: TimerId {
                    owner: self
                        .owner
                        .borrow(cs)
                        .get()
                        .expect("created timer has an owner"),
                    index,
                },
                generation: entry.generation,
                scheduled,
                observed: now,
                missed,
            }))
        })
    }
    /// Commit a current event for execution. Stop/change suppress an uncommitted
    /// event, but cannot interrupt a callback already committed or executing.
    /// The callback runs outside critical sections and may control the scheduler.
    pub fn dispatch(&self, event: TimerEvent, callback: impl FnOnce(TimerEvent)) -> bool {
        let current = critical_section::with(|cs| {
            if self.owner.borrow(cs).get() != Some(event.id.owner) {
                return false;
            }
            self.entries
                .borrow(cs)
                .borrow()
                .get(event.id.index)
                .is_some_and(|entry| entry.used && entry.generation == event.generation)
        });
        if current {
            callback(event);
        }
        current
    }
    /// Wait for the next due event, rearming the single clock sleep whenever a
    /// producer starts, stops or changes a timer. Drop cancels the logical sleep;
    /// a driver's stale task wake is harmless because due events are rechecked.
    pub async fn next_event(&self, clock: &impl Clock) -> Result<TimerEvent, TimerError> {
        loop {
            // Listen before reading deadlines, so a concurrent update is never lost.
            let changed = self.changed.listen();
            if let Some(event) = self.take_due(clock.now())? {
                return Ok(event);
            }
            match self.next_deadline() {
                Some(deadline) => {
                    match crate::wait::with_timeout(changed, clock.sleep_until(deadline)).await {
                        Ok(Ok(())) | Err(WaitError::Timeout) => {}
                        Ok(Err(_)) | Err(WaitError::TooManyWaiters) => {
                            return Err(TimerError::TooManyWaiters);
                        }
                    }
                }
                None => changed.await.map_err(|_| TimerError::TooManyWaiters)?,
            }
        }
    }
}
