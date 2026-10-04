use heapless::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubscriptionId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignalError {
    Full,
    ZeroCount,
    IdExhausted,
}

struct Subscription<'a, T> {
    id: SubscriptionId,
    callback: &'a mut dyn FnMut(&T),
    remaining: Option<u32>,
}

/// Synchronous callbacks run in registration order. Borrows prevent mutation or
/// reentrant emission of the same signal while callbacks are running.
/// Callbacks remain borrowed for the signal's lifetime, even after unsubscribe.
pub struct Signal<'a, T, const N: usize> {
    subscribers: Vec<Subscription<'a, T>, N>,
    next_id: u64,
}

impl<T, const N: usize> Default for Signal<'_, T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, const N: usize> Signal<'a, T, N> {
    pub const fn new() -> Self {
        Self {
            subscribers: Vec::new(),
            next_id: 0,
        }
    }
    pub fn subscribe(
        &mut self,
        callback: &'a mut dyn FnMut(&T),
    ) -> Result<SubscriptionId, SignalError> {
        self.insert(callback, None)
    }
    pub fn subscribe_once(
        &mut self,
        callback: &'a mut dyn FnMut(&T),
    ) -> Result<SubscriptionId, SignalError> {
        self.insert(callback, Some(1))
    }
    pub fn subscribe_n(
        &mut self,
        callback: &'a mut dyn FnMut(&T),
        count: u32,
    ) -> Result<SubscriptionId, SignalError> {
        if count == 0 {
            return Err(SignalError::ZeroCount);
        }
        self.insert(callback, Some(count))
    }
    fn insert(
        &mut self,
        callback: &'a mut dyn FnMut(&T),
        remaining: Option<u32>,
    ) -> Result<SubscriptionId, SignalError> {
        if self.subscribers.is_full() {
            return Err(SignalError::Full);
        }
        let next = self
            .next_id
            .checked_add(1)
            .ok_or(SignalError::IdExhausted)?;
        let id = SubscriptionId(self.next_id);
        self.subscribers
            .push(Subscription {
                id,
                callback,
                remaining,
            })
            .map_err(|_| SignalError::Full)?;
        self.next_id = next;
        Ok(id)
    }
    pub fn unsubscribe(&mut self, id: SubscriptionId) -> bool {
        if let Some(index) = self.subscribers.iter().position(|s| s.id == id) {
            self.subscribers.remove(index);
            true
        } else {
            false
        }
    }
    pub fn emit(&mut self, value: &T) {
        let mut index = 0;
        while index < self.subscribers.len() {
            // Remove exhausted callbacks before invocation so unwinding cannot repeat one-shot delivery.
            let expires = self.subscribers[index].remaining == Some(1);
            if expires {
                let subscription = self.subscribers.remove(index);
                (subscription.callback)(value);
            } else {
                let subscription = &mut self.subscribers[index];
                if let Some(count) = &mut subscription.remaining {
                    *count -= 1;
                }
                (subscription.callback)(value);
                index += 1;
            }
        }
    }
    pub fn len(&self) -> usize {
        self.subscribers.len()
    }
    pub fn is_empty(&self) -> bool {
        self.subscribers.is_empty()
    }
    pub fn clear(&mut self) {
        self.subscribers.clear();
    }
}
