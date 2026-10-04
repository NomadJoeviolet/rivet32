//! WFLY SBUS receiver profile (2025-11-16).
use crate::{Error, exact};

/// A three-position switch decoded from the SBUS channels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Switch {
    Up = 1,
    Down = 2,
    Middle = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Packet {
    pub channels: [i16; 16],
    pub switches: [Switch; 8],
    pub digital: [bool; 2],
}
impl Packet {
    pub fn decode(d: &[u8]) -> Result<Self, Error> {
        exact(d, 25)?;
        if d[0] != 0x0f || d[24] != 0 {
            return Err(Error::Header);
        }
        if d[23] & 0xfc != 0 {
            return Err(Error::Range);
        }
        let mut channels = [0; 16];
        for (i, ch) in channels.iter_mut().enumerate() {
            *ch = bits(&d[1..23], i * 11, 11) as i16 - 1024;
        }
        channels[1] = -channels[1];
        channels[2] = -channels[2];
        if channels.iter().any(|ch| ch.abs() > 900) {
            return Err(Error::Range);
        }
        let mut switches = [Switch::Middle; 8];
        for (i, value) in [-channels[6], -channels[7], channels[8], channels[9]]
            .into_iter()
            .enumerate()
        {
            let pair = match value / 100 {
                -8 => [Switch::Down, Switch::Down],
                -6 => [Switch::Down, Switch::Middle],
                -4 => [Switch::Down, Switch::Up],
                -2 => [Switch::Middle, Switch::Down],
                0 => [Switch::Middle, Switch::Middle],
                2 => [Switch::Middle, Switch::Up],
                4 => [Switch::Up, Switch::Down],
                6 => [Switch::Up, Switch::Middle],
                8 => [Switch::Up, Switch::Up],
                _ => return Err(Error::Range),
            };
            switches[i * 2..i * 2 + 2].copy_from_slice(&pair);
        }
        Ok(Self {
            channels,
            switches,
            digital: [d[23] & 1 != 0, d[23] & 2 != 0],
        })
    }
}
pub type Receiver = crate::Receiver<Packet>;

fn bits(d: &[u8], start: usize, width: usize) -> u64 {
    let mut value = 0;
    for bit in 0..width {
        value |= (((d[(start + bit) / 8] >> ((start + bit) % 8)) & 1) as u64) << bit;
    }
    value
}

/// A rejected SBUS candidate slides one byte, allowing recovery after dropped bytes.
pub struct Stream {
    bytes: [u8; 25],
    len: usize,
}
impl Default for Stream {
    fn default() -> Self {
        Self::new()
    }
}
impl Stream {
    pub const fn new() -> Self {
        Self {
            bytes: [0; 25],
            len: 0,
        }
    }
    pub fn reset(&mut self) {
        self.len = 0;
    }
    pub fn feed(&mut self, input: &[u8], mut emit: impl FnMut(Packet)) {
        for &b in input {
            if self.len == 0 && b != 0x0f {
                continue;
            }
            self.bytes[self.len] = b;
            self.len += 1;
            if self.len == 25 {
                if let Ok(p) = Packet::decode(&self.bytes) {
                    emit(p);
                    self.len = 0;
                } else {
                    self.bytes.copy_within(1.., 0);
                    self.len = 24;
                    while self.len > 0 && self.bytes[0] != 0x0f {
                        self.bytes.copy_within(1..self.len, 0);
                        self.len -= 1;
                    }
                }
            }
        }
    }
}
