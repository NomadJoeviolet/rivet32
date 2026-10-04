//! User application package. The firmware entry point is supplied by the selected board integration.
#![no_std]

#[cfg(feature = "reference-boards")]
pub mod boards;
