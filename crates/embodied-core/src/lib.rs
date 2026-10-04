#![no_std]
#![forbid(unsafe_code)]

//! Hardware-independent, allocation-free framework contracts.

pub mod can;
pub mod communication;
pub mod connection;
pub mod init;
pub mod signal;
pub mod time;
