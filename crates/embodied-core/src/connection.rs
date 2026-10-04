use crate::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionState {
    Online,
    Offline,
}

/// A link is online from its last valid observation up to (excluding) its timeout.
#[derive(Clone, Copy, Debug)]
pub struct ConnectionGuard {
    timeout: Duration,
    last_seen: Option<Instant>,
}

impl ConnectionGuard {
    pub const fn new(timeout: Duration) -> Self {
        Self {
            timeout,
            last_seen: None,
        }
    }
    /// Call only after a packet has passed the device protocol's validation.
    pub fn observe(&mut self, now: Instant) {
        self.last_seen = Some(now);
    }
    pub fn disconnect(&mut self) {
        self.last_seen = None;
    }
    pub const fn last_seen(&self) -> Option<Instant> {
        self.last_seen
    }
    pub fn state(&self, now: Instant) -> ConnectionState {
        match self
            .last_seen
            .and_then(|seen| now.checked_duration_since(seen))
        {
            Some(elapsed) if elapsed < self.timeout => ConnectionState::Online,
            _ => ConnectionState::Offline,
        }
    }
    pub fn is_online(&self, now: Instant) -> bool {
        self.state(now) == ConnectionState::Online
    }
}
