#![no_std]
#![forbid(unsafe_code)]
//! Allocation-free algorithms. Time parameters are explicit seconds, angles radians.

pub mod crc;
pub mod filter;
pub mod fsm;
pub mod kalman;
pub mod math;
pub mod matrix;
pub mod pid;
pub mod power;
pub mod qekf;
pub mod quaternion;
pub mod ring;
pub mod rls;
pub mod sigmoid;
pub mod transform;
pub mod vector;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidParameter,
    NonFinite,
    Singular,
    ZeroNorm,
    Full,
    NotFound,
}
