#![no_std]
#![forbid(unsafe_code)]

//! Fixed-capacity runtime storage and optional Embassy adapters.
//!
//! The default build needs neither an allocator nor an executor. The board must
//! supply a `critical-section` implementation. ISR code uses `try_*` operations;
//! these never wait for a resource, although they briefly enter a critical section.
//! Wakers supplied by an ISR-facing executor must themselves be ISR-safe.
//! Async operations are cooperatively scheduled and cancellation is Future Drop.
//! This does not emulate arbitrary FreeRTOS thread preemption, suspension or kill.

pub mod can;
pub mod clock;
pub mod control;
pub mod hardware_timer;
pub mod mutex;
pub mod pool;
pub mod queue;
pub mod semaphore;
pub mod timer;
pub mod wait;

#[cfg(feature = "embassy")]
pub mod embassy;
