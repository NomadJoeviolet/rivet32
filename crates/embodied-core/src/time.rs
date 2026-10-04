//! Monotonic time in microseconds. Arithmetic reports overflow rather than wrapping.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration(u64);

impl Duration {
    pub const ZERO: Self = Self(0);
    pub const fn from_micros(micros: u64) -> Self {
        Self(micros)
    }
    pub const fn from_millis(millis: u64) -> Self {
        Self(millis.checked_mul(1_000).expect("duration overflow"))
    }
    pub const fn from_secs(seconds: u64) -> Self {
        Self(seconds.checked_mul(1_000_000).expect("duration overflow"))
    }
    pub const fn as_micros(self) -> u64 {
        self.0
    }
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.0.checked_add(other.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(u64);

impl Instant {
    pub const ZERO: Self = Self(0);
    pub const fn from_micros(micros: u64) -> Self {
        Self(micros)
    }
    pub const fn as_micros(self) -> u64 {
        self.0
    }
    pub const fn checked_add(self, duration: Duration) -> Option<Self> {
        match self.0.checked_add(duration.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
    pub const fn checked_duration_since(self, earlier: Self) -> Option<Duration> {
        match self.0.checked_sub(earlier.0) {
            Some(value) => Some(Duration(value)),
            None => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeError {
    ZeroPeriod,
    Overflow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tick {
    /// The original deadline that became due, before skipping missed periods.
    pub scheduled: Instant,
    pub observed: Instant,
    /// Additional deadlines skipped after `scheduled` and at or before `observed`.
    pub missed: u64,
}

/// SkipMissed scheduling: a late observation emits once and preserves the original phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Periodic {
    deadline: Instant,
    period: Duration,
}

impl Periodic {
    /// The first deadline is `start + period`.
    pub fn new(start: Instant, period: Duration) -> Result<Self, TimeError> {
        if period == Duration::ZERO {
            return Err(TimeError::ZeroPeriod);
        }
        let deadline = start.checked_add(period).ok_or(TimeError::Overflow)?;
        Ok(Self { deadline, period })
    }

    pub const fn deadline(&self) -> Instant {
        self.deadline
    }
    pub const fn period(&self) -> Duration {
        self.period
    }

    /// An early observation or an overflow leaves the schedule unchanged.
    pub fn tick(&mut self, now: Instant) -> Result<Option<Tick>, TimeError> {
        let Some(late) = now.checked_duration_since(self.deadline) else {
            return Ok(None);
        };
        let missed = late.0 / self.period.0;
        let advance = missed
            .checked_add(1)
            .and_then(|n| n.checked_mul(self.period.0))
            .ok_or(TimeError::Overflow)?;
        let next = self
            .deadline
            .checked_add(Duration(advance))
            .ok_or(TimeError::Overflow)?;
        let tick = Tick {
            scheduled: self.deadline,
            observed: now,
            missed,
        };
        self.deadline = next;
        Ok(Some(tick))
    }
}
