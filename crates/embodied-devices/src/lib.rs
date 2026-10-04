#![no_std]
#![forbid(unsafe_code)]
//! Allocation-free device protocols. Times are monotonic microseconds;
//! physical motor values use radians, radians/second, amperes and newton-metres.
//! Constructors validate configuration; rejected input never refreshes liveness.

pub mod damiao;
pub mod dji;
pub mod imu;
pub mod lk;
pub mod referee;
pub mod robstride;
pub mod wfly;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Length,
    Range,
    NonFinite,
    Id,
    Checksum,
    Header,
    Unsupported,
    Capacity,
    Timestamp,
}

impl From<embodied_core::can::FrameError> for Error {
    fn from(e: embodied_core::can::FrameError) -> Self {
        match e {
            embodied_core::can::FrameError::InvalidId => Self::Id,
            embodied_core::can::FrameError::InvalidLength => Self::Length,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connection {
    NeverReceived,
    Connected,
    Lost,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Freshness {
    last: Option<u64>,
}
impl Freshness {
    pub const fn new() -> Self {
        Self { last: None }
    }
    pub const fn last_update_us(&self) -> Option<u64> {
        self.last
    }
    pub fn validate(&self, now: u64) -> Result<(), Error> {
        if self.last.is_some_and(|last| now < last) {
            Err(Error::Timestamp)
        } else {
            Ok(())
        }
    }
    pub fn update(&mut self, now: u64) -> Result<(), Error> {
        self.validate(now)?;
        self.last = Some(now);
        Ok(())
    }
    pub fn connection(&self, now: u64, timeout: u64) -> Connection {
        match self.last {
            None => Connection::NeverReceived,
            Some(last) if now >= last && now - last <= timeout => Connection::Connected,
            _ => Connection::Lost,
        }
    }
}

pub(crate) fn exact(data: &[u8], len: usize) -> Result<(), Error> {
    if data.len() == len {
        Ok(())
    } else {
        Err(Error::Length)
    }
}
pub(crate) fn finite(values: &[f32]) -> Result<(), Error> {
    if values.iter().all(|v| v.is_finite()) {
        Ok(())
    } else {
        Err(Error::NonFinite)
    }
}
pub(crate) fn quantize(value: f32, min: f32, max: f32, bits: u32) -> Result<u16, Error> {
    finite(&[value, min, max])?;
    if min >= max || !(max - min).is_finite() || bits == 0 || bits > 16 {
        return Err(Error::Range);
    }
    let offset = value.clamp(min, max) - min;
    let scale = ((1u32 << bits) - 1) as f32;
    let numerator = offset * scale;
    // Preserve the original encoders' multiply-then-divide order at quantization
    // boundaries. For finite wide ranges, avoid their overflowing intermediate.
    let encoded = if numerator.is_finite() {
        numerator / (max - min)
    } else {
        offset / (max - min) * scale
    };
    Ok(encoded as u16)
}
pub(crate) fn expand(value: u16, min: f32, max: f32, bits: u32) -> f32 {
    value as f32 / ((1u32 << bits) - 1) as f32 * (max - min) + min
}
pub(crate) fn be16(data: &[u8], at: usize) -> u16 {
    u16::from_be_bytes([data[at], data[at + 1]])
}
pub(crate) fn le16(data: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([data[at], data[at + 1]])
}
pub(crate) fn le32(data: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([data[at], data[at + 1], data[at + 2], data[at + 3]])
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Unwrap {
    previous: Option<f32>,
    continuous: f32,
}
impl Unwrap {
    pub const fn new() -> Self {
        Self {
            previous: None,
            continuous: 0.0,
        }
    }
    pub fn update(&mut self, raw: f32, span: f32) -> f32 {
        if let Some(last) = self.previous {
            let mut delta = raw - last;
            if delta > span * 0.5 {
                delta -= span;
            } else if delta < -span * 0.5 {
                delta += span;
            }
            self.continuous += delta;
        } else {
            self.continuous = raw;
        }
        self.previous = Some(raw);
        self.continuous
    }
}

/// Last two accepted packets. Invalid packets do not overwrite either snapshot.
#[derive(Clone, Debug)]
pub struct Receiver<T> {
    current: Option<T>,
    previous: Option<T>,
    freshness: Freshness,
    timeout_us: u64,
}
impl<T: Copy> Receiver<T> {
    pub const fn new(timeout_us: u64) -> Self {
        Self {
            current: None,
            previous: None,
            freshness: Freshness::new(),
            timeout_us,
        }
    }
    pub fn accept(&mut self, packet: T, now_us: u64) -> Result<(), Error> {
        self.freshness.update(now_us)?;
        self.previous = self.current;
        self.current = Some(packet);
        Ok(())
    }
    pub fn current(&self) -> Option<&T> {
        self.current.as_ref()
    }
    pub fn previous(&self) -> Option<&T> {
        self.previous.as_ref()
    }
    pub fn connection(&self, now_us: u64) -> Connection {
        self.freshness.connection(now_us, self.timeout_us)
    }
    pub fn reset(&mut self) {
        self.current = None;
        self.previous = None;
        self.freshness = Freshness::new();
    }
}
