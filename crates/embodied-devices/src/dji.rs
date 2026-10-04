//! DJI C610/C620/GM6020 feedback and simultaneous four-motor command frames.
use crate::{Connection, Error, Freshness, Unwrap, be16, exact, finite};
use embodied_core::can::{CanFrame, Id};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Model {
    M3508,
    M2006,
    Gm6020Voltage,
    Gm6020Current,
}
impl Model {
    fn valid_id(self, id: u16) -> bool {
        match self {
            Self::M3508 | Self::M2006 => (0x201..=0x208).contains(&id),
            _ => (0x205..=0x20b).contains(&id),
        }
    }
    fn current_scale(self) -> f32 {
        match self {
            Self::M3508 => 20.0 / 16384.0,
            Self::M2006 => 10.0 / 10000.0,
            _ => 3.0 / 16384.0,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Feedback {
    pub encoder: u16,
    pub position_radians: f32,
    pub continuous_radians: f32,
    pub shaft_radians: f32,
    pub speed_rpm: i16,
    pub shaft_speed_rpm: f32,
    pub current_amperes: f32,
    pub temperature_c: u8,
}

pub struct Motor {
    model: Model,
    id: u16,
    gear: f32,
    offset: f32,
    continuous_offset: f32,
    unwrap: Unwrap,
    feedback: Option<Feedback>,
    freshness: Freshness,
    pub output: i16,
}
impl Motor {
    pub fn new(model: Model, id: u16, gear: f32, offset: f32) -> Result<Self, Error> {
        finite(&[gear, offset])?;
        if !model.valid_id(id) {
            return Err(Error::Id);
        }
        if gear <= 0.0 {
            return Err(Error::Range);
        }
        Ok(Self {
            model,
            id,
            gear,
            offset,
            continuous_offset: 0.0,
            unwrap: Unwrap::new(),
            feedback: None,
            freshness: Freshness::new(),
            output: 0,
        })
    }
    pub fn receive(&mut self, frame: &CanFrame, now_us: u64) -> Result<(), Error> {
        if frame.id() != Id::Standard(self.id) || frame.is_fd() {
            return Err(Error::Id);
        }
        let data = frame.data();
        exact(data, 8)?;
        self.freshness.validate(now_us)?;
        let encoder = be16(data, 0);
        if encoder >= 8192 {
            return Err(Error::Range);
        }
        let raw = encoder as f32 * core::f32::consts::TAU / 8192.0;
        let mut limited = (raw - self.offset) % core::f32::consts::TAU;
        if limited > core::f32::consts::PI {
            limited -= core::f32::consts::TAU;
        }
        if limited <= -core::f32::consts::PI {
            limited += core::f32::consts::TAU;
        }
        let continuous =
            self.unwrap.update(limited, core::f32::consts::TAU) - self.continuous_offset;
        let speed = be16(data, 2) as i16;
        self.feedback = Some(Feedback {
            encoder,
            position_radians: raw,
            continuous_radians: continuous,
            shaft_radians: continuous / self.gear,
            speed_rpm: speed,
            shaft_speed_rpm: speed as f32 / self.gear,
            current_amperes: be16(data, 4) as i16 as f32 * self.model.current_scale(),
            temperature_c: data[6],
        });
        self.freshness.update(now_us)
    }
    pub fn feedback(&self) -> Option<&Feedback> {
        self.feedback.as_ref()
    }
    pub fn connection(&self, now_us: u64) -> Connection {
        self.freshness.connection(now_us, 200_000)
    }
    pub fn reset(&mut self) {
        self.feedback = None;
        self.output = 0;
        self.unwrap = Unwrap::new();
        self.freshness = Freshness::new();
    }
    pub const fn id(&self) -> u16 {
        self.id
    }
    pub const fn gear_ratio(&self) -> f32 {
        self.gear
    }
    /// Change the encoder angle reference for the next accepted feedback frame.
    /// The last published feedback and turn history remain available. Reset does
    /// not erase this setting. Values are motor-side radians, before gearing.
    pub fn set_angle_offset_radians(&mut self, offset: f32) -> Result<(), Error> {
        finite(&[offset])?;
        self.offset = offset;
        Ok(())
    }
    /// Subtract this motor-side zero point from subsequent continuous and shaft
    /// positions, without changing the internal turn history or last feedback.
    /// Reset preserves the configured zero point.
    pub fn set_continuous_angle_offset_radians(&mut self, offset: f32) -> Result<(), Error> {
        finite(&[offset])?;
        self.continuous_offset = offset;
        Ok(())
    }
}

/// Unspecified slots are zero. Duplicate IDs and IDs outside this transmit group are errors.
pub fn group_frame(tx_id: u16, outputs: &[(u16, i16)]) -> Result<CanFrame, Error> {
    let base = match tx_id {
        0x200 => 0x201,
        0x1ff | 0x1fe => 0x205,
        0x2ff | 0x2fe => 0x209,
        _ => return Err(Error::Id),
    };
    if outputs.is_empty() || outputs.len() > 4 {
        return Err(Error::Length);
    }
    let mut data = [0; 8];
    let mut occupied = 0u8;
    for &(id, output) in outputs {
        if !(base..base + 4).contains(&id) {
            return Err(Error::Id);
        }
        let slot = (id - base) as usize;
        if occupied & (1 << slot) != 0 {
            return Err(Error::Id);
        }
        occupied |= 1 << slot;
        data[slot * 2..slot * 2 + 2].copy_from_slice(&output.to_be_bytes());
    }
    Ok(CanFrame::new(Id::Standard(tx_id), &data)?)
}
