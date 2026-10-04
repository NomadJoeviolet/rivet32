//! LingKong firmware 2.35 and 2.36. V236 feedback ID is command ID + 0x40.
use crate::{Connection, Error, Freshness, Unwrap, exact, finite, le16};
use embodied_core::can::{CanFrame, Id};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Version {
    V235,
    V236,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Model {
    Ms,
    Mf,
    Mg,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PidLoop {
    Angle = 0x0a,
    Speed = 0x0b,
    Current = 0x0c,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Feedback {
    pub position_degrees: f32,
    pub continuous_degrees: f32,
    pub shaft_degrees: f32,
    pub speed_degrees_per_second: i16,
    pub shaft_speed_degrees_per_second: f32,
    pub current_amperes: f32,
    pub power_watts: f32,
    pub temperature_c: u8,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EncoderFeedback {
    pub position: u16,
    pub original_position: u16,
    pub offset: u16,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorStatus {
    pub temperature_c: u8,
    pub voltage_decivolts: u16,
    pub error_flags: u8,
}
pub struct Motor {
    version: Version,
    model: Model,
    id: u16,
    bits: u8,
    gear: f32,
    offset: f32,
    unwrap: Unwrap,
    freshness: Freshness,
    feedback: Option<Feedback>,
    multi: Option<f32>,
    encoder: Option<EncoderFeedback>,
    error_status: Option<ErrorStatus>,
    pub pid: [[u16; 3]; 3],
    enabled: bool,
}
impl Motor {
    pub fn new(
        version: Version,
        model: Model,
        id: u16,
        bits: u8,
        gear: f32,
        offset: f32,
    ) -> Result<Self, Error> {
        finite(&[gear, offset])?;
        if !(1..=16).contains(&bits) || gear <= 0.0 {
            return Err(Error::Range);
        }
        if id > 0x7bf {
            return Err(Error::Id);
        }
        Ok(Self {
            version,
            model,
            id,
            bits,
            gear,
            offset,
            unwrap: Unwrap::new(),
            freshness: Freshness::new(),
            feedback: None,
            multi: None,
            encoder: None,
            error_status: None,
            pid: [[0; 3]; 3],
            enabled: false,
        })
    }
    fn frame(&self, d: [u8; 8]) -> Result<CanFrame, Error> {
        Ok(CanFrame::new(Id::Standard(self.id), &d)?)
    }
    pub fn request(&self, command: u8) -> Result<CanFrame, Error> {
        if !matches!(command, 0x80 | 0x88 | 0x90 | 0x92 | 0x9b | 0x9c) {
            return Err(Error::Unsupported);
        }
        self.frame([command, 0, 0, 0, 0, 0, 0, 0])
    }
    pub fn enable(&mut self) -> Result<CanFrame, Error> {
        self.enabled = true;
        self.request(0x88)
    }
    pub fn disable(&mut self) -> Result<CanFrame, Error> {
        self.enabled = false;
        self.request(0x80)
    }
    pub fn enabled(&self, now_us: u64) -> bool {
        self.enabled && self.connection(now_us) == Connection::Connected
    }
    pub fn torque(&self, raw: i16) -> Result<CanFrame, Error> {
        let (code, limit) = if self.model == Model::Ms {
            (0xa0, 850)
        } else {
            (0xa1, 2048)
        };
        let v = raw.clamp(-limit, limit).to_le_bytes();
        self.frame([code, 0, 0, 0, v[0], v[1], 0, 0])
    }
    fn scaled(value: f32) -> Result<i32, Error> {
        finite(&[value])?;
        if value < (i32::MIN as f32) || value >= (i32::MAX as f32) {
            return Err(Error::Range);
        }
        Ok(value as i32)
    }
    pub fn position(
        &self,
        shaft_degrees: f32,
        speed_limit: Option<u16>,
    ) -> Result<CanFrame, Error> {
        let v = Self::scaled(shaft_degrees * self.gear * 100.0)?.to_le_bytes();
        let s = speed_limit.unwrap_or(0).to_le_bytes();
        self.frame([
            if speed_limit.is_some() { 0xa4 } else { 0xa3 },
            0,
            s[0],
            s[1],
            v[0],
            v[1],
            v[2],
            v[3],
        ])
    }
    pub fn speed(
        &self,
        shaft_degrees_per_second: f32,
        torque_limit: i16,
    ) -> Result<CanFrame, Error> {
        // Preserve the original 2.35 multiplication order at integer boundaries.
        // The 2.36 API first converts shaft speed to its rotor-speed argument.
        let scaled = match self.version {
            Version::V235 => shaft_degrees_per_second * 100.0 * self.gear,
            Version::V236 => shaft_degrees_per_second * self.gear * 100.0,
        };
        let v = Self::scaled(scaled)?.to_le_bytes();
        let t = torque_limit.clamp(-2048, 2048).to_le_bytes();
        self.frame([0xa2, 0, t[0], t[1], v[0], v[1], v[2], v[3]])
    }
    pub fn read_pid(&self, kind: PidLoop) -> Result<CanFrame, Error> {
        if self.version != Version::V236 {
            return Err(Error::Unsupported);
        }
        self.frame([0xc0, kind as u8, 0, 0, 0, 0, 0, 0])
    }
    pub fn write_pid(&self, kind: PidLoop, gains: [u16; 3]) -> Result<CanFrame, Error> {
        if self.version != Version::V236 {
            return Err(Error::Unsupported);
        }
        let mut d = [0xc1, kind as u8, 0, 0, 0, 0, 0, 0];
        for (i, gain) in gains.iter().enumerate() {
            d[2 + i * 2..4 + i * 2].copy_from_slice(&gain.to_le_bytes());
        }
        self.frame(d)
    }
    pub fn receive(&mut self, frame: &CanFrame, now_us: u64) -> Result<(), Error> {
        let expected = self.id
            + if self.version == Version::V236 {
                0x40
            } else {
                0
            };
        if frame.id() != Id::Standard(expected) || frame.is_fd() {
            return Err(Error::Id);
        }
        let d = frame.data();
        exact(d, 8)?;
        self.freshness.validate(now_us)?;
        match d[0] {
            0x9c | 0xa0..=0xa4 => {
                let raw = le16(d, 6) as u32;
                if raw >= 1u32 << self.bits {
                    return Err(Error::Range);
                }
                let denom = if self.version == Version::V236 {
                    (1u32 << self.bits) - 1
                } else {
                    1u32 << self.bits
                };
                let position = raw as f32 * 360.0 / denom as f32;
                let continuous = self.unwrap.update(position, 360.0) + self.offset;
                let speed = le16(d, 4) as i16;
                let amps = le16(d, 2) as i16 as f32;
                self.feedback = Some(Feedback {
                    position_degrees: position,
                    continuous_degrees: continuous,
                    shaft_degrees: continuous / self.gear,
                    speed_degrees_per_second: speed,
                    shaft_speed_degrees_per_second: speed as f32 / self.gear,
                    current_amperes: match self.model {
                        Model::Mg => amps * 66.0 / 4096.0,
                        Model::Mf => amps * 33.0 / 4096.0,
                        Model::Ms => 0.0,
                    },
                    power_watts: if self.model == Model::Ms { amps } else { 0.0 },
                    temperature_c: d[1],
                });
            }
            0x92 => {
                let mut bytes = [0; 8];
                bytes[..7].copy_from_slice(&d[1..]);
                bytes[7] = if d[7] & 0x80 != 0 { 255 } else { 0 };
                self.multi = Some(i64::from_le_bytes(bytes) as f32 * 0.01);
            }
            0xc0 | 0xc1 if self.version == Version::V236 => {
                if !(0x0a..=0x0c).contains(&d[1]) {
                    return Err(Error::Unsupported);
                }
                self.pid[(d[1] - 0x0a) as usize] = [le16(d, 2), le16(d, 4), le16(d, 6)];
            }
            0x80 => self.enabled = false,
            0x88 => self.enabled = true,
            0x90 => {
                self.encoder = Some(EncoderFeedback {
                    position: le16(d, 2),
                    original_position: le16(d, 4),
                    offset: le16(d, 6),
                })
            }
            0x9b => {
                self.error_status = Some(ErrorStatus {
                    temperature_c: d[1],
                    voltage_decivolts: le16(d, 3),
                    error_flags: d[7],
                })
            }
            _ => return Err(Error::Unsupported),
        }
        self.freshness.update(now_us)
    }
    pub fn feedback(&self) -> Option<&Feedback> {
        self.feedback.as_ref()
    }
    pub const fn multi_angle_degrees(&self) -> Option<f32> {
        self.multi
    }
    pub fn encoder_feedback(&self) -> Option<&EncoderFeedback> {
        self.encoder.as_ref()
    }
    pub fn error_status(&self) -> Option<&ErrorStatus> {
        self.error_status.as_ref()
    }
    pub fn connection(&self, now_us: u64) -> Connection {
        self.freshness.connection(now_us, 1_000_000)
    }
}
