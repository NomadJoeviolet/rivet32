//! Guarded, table-driven state machine with FIFO transitions from callbacks.
use crate::{Error, ring::RingBuffer};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Enter,
    Execute,
    Exit,
}
enum Pending<S, E> {
    Event(E),
    State(S),
}
pub struct EventQueue<S, E, const Q: usize> {
    pending: RingBuffer<Pending<S, E>, Q>,
    overflow: bool,
}
impl<S, E, const Q: usize> EventQueue<S, E, Q> {
    pub fn event(&mut self, event: E) -> Result<(), Error> {
        self.push(Pending::Event(event))
    }
    pub fn checkout(&mut self, state: S) -> Result<(), Error> {
        self.push(Pending::State(state))
    }
    fn push(&mut self, item: Pending<S, E>) -> Result<(), Error> {
        if self.pending.push(item).is_err() {
            self.overflow = true;
            Err(Error::Full)
        } else {
            Ok(())
        }
    }
}
pub struct Transition<C, S, E> {
    pub from: S,
    pub event: E,
    pub to: S,
    pub guard: fn(&C) -> bool,
}
impl<C, S, E> Transition<C, S, E> {
    pub const fn new(from: S, event: E, to: S, guard: fn(&C) -> bool) -> Self {
        Self {
            from,
            event,
            to,
            guard,
        }
    }
}
type Handler<C, S, E, const Q: usize> = fn(S, Phase, &mut C, &mut EventQueue<S, E, Q>);
pub struct StateMachine<C, S, E, const T: usize, const Q: usize> {
    state: S,
    transitions: [Transition<C, S, E>; T],
    handler: Handler<C, S, E, Q>,
    context: C,
    queue: EventQueue<S, E, Q>,
}
impl<C, S: Copy + Eq, E: Copy + Eq, const T: usize, const Q: usize> StateMachine<C, S, E, T, Q> {
    pub fn new(
        state: S,
        transitions: [Transition<C, S, E>; T],
        handler: Handler<C, S, E, Q>,
        context: C,
    ) -> Result<Self, Error> {
        if Q == 0 {
            return Err(Error::InvalidParameter);
        }
        let mut f = Self {
            state,
            transitions,
            handler,
            context,
            queue: EventQueue {
                pending: RingBuffer::new(),
                overflow: false,
            },
        };
        f.call(Phase::Enter);
        f.drain()?;
        Ok(f)
    }
    pub fn current(&self) -> S {
        self.state
    }
    pub fn context(&self) -> &C {
        &self.context
    }
    pub fn context_mut(&mut self) -> &mut C {
        &mut self.context
    }
    pub fn react(&mut self, event: E) -> Result<(), Error> {
        self.queue.event(event)?;
        self.drain()
    }
    pub fn checkout(&mut self, state: S) -> Result<(), Error> {
        self.queue.checkout(state)?;
        self.drain()
    }
    pub fn execute(&mut self) -> Result<(), Error> {
        self.call(Phase::Execute);
        self.drain()
    }
    fn call(&mut self, phase: Phase) {
        (self.handler)(self.state, phase, &mut self.context, &mut self.queue);
    }
    fn transition(&mut self, next: S) {
        self.call(Phase::Exit);
        self.state = next;
        self.call(Phase::Enter);
    }
    fn drain(&mut self) -> Result<(), Error> {
        // A faulty callback cycle cannot monopolize an executor forever.
        let mut budget = Q.saturating_mul(T.saturating_add(1)).max(16);
        while let Some(p) = self.queue.pending.pop() {
            if budget == 0 {
                self.queue.pending.clear();
                return Err(Error::InvalidParameter);
            }
            budget -= 1;
            match p {
                Pending::State(s) => {
                    if s != self.state {
                        self.transition(s);
                    }
                }
                Pending::Event(e) => {
                    if let Some(t) = self
                        .transitions
                        .iter()
                        .find(|t| t.from == self.state && t.event == e && (t.guard)(&self.context))
                    {
                        let next = t.to;
                        self.transition(next);
                    }
                }
            }
        }
        if core::mem::replace(&mut self.queue.overflow, false) {
            Err(Error::Full)
        } else {
            Ok(())
        }
    }
}
