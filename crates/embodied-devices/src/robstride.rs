//! RobStride extended CAN protocol. Profile ranges must match the physical motor.
use crate::{Connection, Error, Freshness, Unwrap, be16, exact, expand, finite, le16, quantize};
use embodied_core::can::{CanFrame, Id};
#[derive(Clone, Copy, Debug)]
pub struct Profile {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub torque: [f32; 2],
    pub kp: [f32; 2],
    pub kd: [f32; 2],
}
impl Default for Profile {
    fn default() -> Self {
        Self {
            position: [-12.57, 12.57],
            velocity: [-15.0, 15.0],
            torque: [-120.0, 120.0],
            kp: [0.0, 5000.0],
            kd: [0.0, 100.0],
        }
    }
}
impl Profile {
    fn validate(self) -> Result<(), Error> {
        for r in [self.position, self.velocity, self.torque, self.kp, self.kd] {
            quantize(r[0], r[0], r[1], 16)?;
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Parameter {
    RunMode = 0x7005,
    CurrentReference = 0x7006,
    SpeedReference = 0x700a,
    PositionReference = 0x7016,
    SpeedLimit = 0x7017,
    CurrentLimit = 0x7018,
    Acceleration = 0x7022,
    MaxVelocity = 0x7024,
    PointAcceleration = 0x7025,
    ReportTime = 0x7026,
    CanTimeout = 0x7028,
    Deceleration = 0x702e,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RunMode {
    Motion = 0,
    PositionPp = 1,
    Speed = 2,
    Current = 3,
    PositionCsp = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Baudrate {
    Mbit1 = 1,
    Kbit500 = 2,
    Kbit250 = 3,
    Kbit125 = 4,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Feedback {
    pub position: f32,
    pub continuous_position: f32,
    pub shaft_position: f32,
    pub velocity: f32,
    pub shaft_velocity: f32,
    pub torque: f32,
    pub temperature_c: f32,
    pub faults: u8,
    pub mode: u8,
}
impl Feedback {
    pub const fn enabled(self) -> bool {
        self.mode == 2 && self.faults == 0
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParameterReply {
    pub index: u16,
    pub bytes: [u8; 4],
}
impl ParameterReply {
    pub fn as_f32(self) -> f32 {
        f32::from_le_bytes(self.bytes)
    }
    pub fn as_u32(self) -> u32 {
        u32::from_le_bytes(self.bytes)
    }
}
pub struct Motor {
    id: u8,
    master: u8,
    profile: Profile,
    gear: f32,
    offset: f32,
    unwrap: Unwrap,
    freshness: Freshness,
    feedback: Option<Feedback>,
}
impl Motor {
    pub fn new(
        id: u8,
        master: u8,
        profile: Profile,
        gear: f32,
        offset: f32,
    ) -> Result<Self, Error> {
        profile.validate()?;
        finite(&[gear, offset])?;
        if gear <= 0.0 {
            return Err(Error::Range);
        }
        Ok(Self {
            id,
            master,
            profile,
            gear,
            offset,
            unwrap: Unwrap::new(),
            freshness: Freshness::new(),
            feedback: None,
        })
    }
    fn command(&self, kind: u8, extra: u16, data: [u8; 8]) -> Result<CanFrame, Error> {
        Ok(CanFrame::new(
            Id::Extended((kind as u32) << 24 | (extra as u32) << 8 | self.id as u32),
            &data,
        )?)
    }
    pub const fn id(&self) -> u8 {
        self.id
    }
    pub const fn master_id(&self) -> u8 {
        self.master
    }
    pub const fn gear_ratio(&self) -> f32 {
        self.gear
    }
    pub const fn profile(&self) -> Profile {
        self.profile
    }
    pub const fn angle_offset_radians(&self) -> f32 {
        self.offset
    }
    pub const fn last_update_us(&self) -> Option<u64> {
        self.freshness.last_update_us()
    }
    /// Change the motor-side feedback zero on the next accepted frame.
    /// Motion setpoints remain absolute, as in the original RobStride driver.
    pub fn set_continuous_angle_offset_radians(&mut self, offset: f32) -> Result<(), Error> {
        finite(&[offset])?;
        self.offset = offset;
        Ok(())
    }
    pub fn enable(&self) -> Result<CanFrame, Error> {
        self.command(3, self.master as u16, [0; 8])
    }
    /// Explicit poll of the original optional enable retry policy.
    /// App supplies its requested enable state and target mode, then sends the
    /// returned run-mode and enable frames in order. No background I/O is started.
    pub fn enable_guard_frames(
        &self,
        enable_requested: bool,
        target_mode: RunMode,
    ) -> Result<Option<[CanFrame; 2]>, Error> {
        if enable_requested && !self.feedback.is_some_and(Feedback::enabled) {
            Ok(Some([self.run_mode(target_mode)?, self.enable()?]))
        } else {
            Ok(None)
        }
    }
    pub fn stop(&self, clear_fault: bool) -> Result<CanFrame, Error> {
        let mut d = [0; 8];
        d[0] = u8::from(clear_fault);
        self.command(4, self.master as u16, d)
    }
    pub fn mechanical_zero(&self) -> Result<CanFrame, Error> {
        self.command(6, self.master as u16, [1, 0, 0, 0, 0, 0, 0, 0])
    }
    pub fn set_motor_id(&self, new_id: u8) -> Result<CanFrame, Error> {
        self.command(7, (new_id as u16) << 8 | self.master as u16, [0; 8])
    }
    pub fn save_parameters(&self) -> Result<CanFrame, Error> {
        self.command(22, self.master as u16, [1, 2, 3, 4, 5, 6, 7, 8])
    }
    pub fn active_report(&self, enabled: bool) -> Result<CanFrame, Error> {
        self.command(
            24,
            self.master as u16,
            [1, 2, 3, 4, 5, 6, u8::from(enabled), 0],
        )
    }
    pub fn baudrate(&self, baud: Baudrate) -> Result<CanFrame, Error> {
        self.command(23, self.master as u16, [1, 2, 3, 4, 5, 6, baud as u8, 0])
    }
    pub fn read_parameter(&self, index: u16) -> Result<CanFrame, Error> {
        let mut d = [0; 8];
        d[..2].copy_from_slice(&index.to_le_bytes());
        self.command(17, self.master as u16, d)
    }
    pub fn write_bytes(&self, index: u16, value: [u8; 4]) -> Result<CanFrame, Error> {
        let mut d = [0; 8];
        d[..2].copy_from_slice(&index.to_le_bytes());
        d[4..].copy_from_slice(&value);
        self.command(18, self.master as u16, d)
    }
    pub fn write_f32(&self, index: u16, value: f32) -> Result<CanFrame, Error> {
        finite(&[value])?;
        self.write_bytes(index, value.to_le_bytes())
    }
    pub fn write_u32(&self, index: u16, value: u32) -> Result<CanFrame, Error> {
        self.write_bytes(index, value.to_le_bytes())
    }
    pub fn run_mode(&self, mode: RunMode) -> Result<CanFrame, Error> {
        self.write_bytes(Parameter::RunMode as u16, [mode as u8, 0, 0, 0])
    }
    /// Firmware timeout units are 50 microseconds. Round upward so nonzero never disables it.
    pub fn can_timeout(&self, timeout_us: u64) -> Result<CanFrame, Error> {
        let ticks = timeout_us.checked_add(49).ok_or(Error::Range)? / 50;
        if ticks > u32::MAX as u64 {
            return Err(Error::Range);
        }
        self.write_u32(Parameter::CanTimeout as u16, ticks as u32)
    }
    pub fn motion(
        &self,
        position: f32,
        velocity: f32,
        kp: f32,
        kd: f32,
        torque: f32,
    ) -> Result<CanFrame, Error> {
        let mut d = [0; 8];
        for (i, (v, r)) in [
            (position, self.profile.position),
            (velocity, self.profile.velocity),
            (kp, self.profile.kp),
            (kd, self.profile.kd),
        ]
        .iter()
        .enumerate()
        {
            d[i * 2..i * 2 + 2].copy_from_slice(&quantize(*v, r[0], r[1], 16)?.to_be_bytes());
        }
        let t = quantize(torque, self.profile.torque[0], self.profile.torque[1], 16)?;
        self.command(1, t, d)
    }
    fn validate_reply(&self, frame: &CanFrame, kind: u8) -> Result<u32, Error> {
        let Id::Extended(id) = frame.id() else {
            return Err(Error::Id);
        };
        if frame.is_fd()
            || id >> 24 != kind as u32
            || id as u8 != self.master
            || (id >> 8) as u8 != self.id
        {
            return Err(Error::Id);
        }
        exact(frame.data(), 8)?;
        Ok(id)
    }
    pub fn parameter_reply(&self, frame: &CanFrame) -> Result<ParameterReply, Error> {
        self.validate_reply(frame, 17)?;
        let d = frame.data();
        Ok(ParameterReply {
            index: le16(d, 0),
            bytes: [d[4], d[5], d[6], d[7]],
        })
    }
    pub fn receive(&mut self, frame: &CanFrame, now_us: u64) -> Result<(), Error> {
        let id = self.validate_reply(frame, 2)?;
        self.freshness.validate(now_us)?;
        let d = frame.data();
        let p = self.profile;
        let position = expand(be16(d, 0), p.position[0], p.position[1], 16);
        let velocity = expand(be16(d, 2), p.velocity[0], p.velocity[1], 16);
        let continuous_position =
            self.unwrap.update(position, p.position[1] - p.position[0]) - self.offset;
        self.feedback = Some(Feedback {
            position,
            continuous_position,
            shaft_position: continuous_position / self.gear,
            velocity,
            shaft_velocity: velocity / self.gear,
            torque: expand(be16(d, 4), p.torque[0], p.torque[1], 16),
            temperature_c: be16(d, 6) as f32 * 0.1,
            faults: ((id >> 16) & 63) as u8,
            mode: ((id >> 22) & 3) as u8,
        });
        self.freshness.update(now_us)
    }
    pub fn feedback(&self) -> Option<&Feedback> {
        self.feedback.as_ref()
    }
    pub fn connection(&self, now_us: u64) -> Connection {
        self.freshness.connection(now_us, 200_000)
    }
}
