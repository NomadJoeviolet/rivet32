//! DaMiao MIT, position/velocity and velocity modes, register access and feedback.
use crate::{Connection, Error, Freshness, Unwrap, be16, exact, expand, finite, quantize};
use embodied_core::can::{CanFrame, Id};

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub position: f32,
    pub velocity: f32,
    pub torque: f32,
}
impl Limits {
    fn validate(self) -> Result<(), Error> {
        finite(&[self.position, self.velocity, self.torque])?;
        if self.position <= 0.0
            || self.velocity <= 0.0
            || self.torque <= 0.0
            || self.position > f32::MAX / 2.0
            || self.velocity > f32::MAX / 2.0
            || self.torque > f32::MAX / 2.0
        {
            Err(Error::Range)
        } else {
            Ok(())
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct MitCommand {
    pub position: f32,
    pub velocity: f32,
    pub kp: f32,
    pub kd: f32,
    pub torque: f32,
}
impl MitCommand {
    /// Encode a raw protocol setpoint with saturation, without a motor zero offset.
    pub fn encode(self, l: Limits) -> Result<[u8; 8], Error> {
        l.validate()?;
        let p = quantize(self.position, -l.position, l.position, 16)?;
        let v = quantize(self.velocity, -l.velocity, l.velocity, 12)?;
        let kp = quantize(self.kp, 0.0, 500.0, 12)?;
        let kd = quantize(self.kd, 0.0, 5.0, 12)?;
        let t = quantize(self.torque, -l.torque, l.torque, 12)?;
        Ok([
            (p >> 8) as u8,
            p as u8,
            (v >> 4) as u8,
            ((v & 15) << 4 | kp >> 8) as u8,
            kp as u8,
            (kd >> 4) as u8,
            ((kd & 15) << 4 | t >> 8) as u8,
            t as u8,
        ])
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Feedback {
    pub motor_id: u8,
    pub status: u8,
    pub position: f32,
    pub velocity: f32,
    pub torque: f32,
    pub mos_temperature: u8,
    pub rotor_temperature: u8,
    pub continuous_position: f32,
    pub shaft_position: f32,
    pub shaft_velocity: f32,
}
impl Feedback {
    pub const fn enabled(self) -> bool {
        self.status == 1
    }
    pub fn decode(data: &[u8], l: Limits) -> Result<Self, Error> {
        exact(data, 8)?;
        l.validate()?;
        let position = expand(be16(data, 1), -l.position, l.position, 16);
        let velocity = expand(
            (data[3] as u16) << 4 | (data[4] >> 4) as u16,
            -l.velocity,
            l.velocity,
            12,
        );
        Ok(Self {
            motor_id: data[0] & 15,
            status: data[0] >> 4,
            position,
            velocity,
            torque: expand(
                ((data[4] & 15) as u16) << 8 | data[5] as u16,
                -l.torque,
                l.torque,
                12,
            ),
            mos_temperature: data[6],
            rotor_temperature: data[7],
            continuous_position: position,
            shaft_position: position,
            shaft_velocity: velocity,
        })
    }
}
pub struct Motor {
    id: u16,
    master_id: u16,
    limits: Limits,
    gear: f32,
    offset: f32,
    can_fd: bool,
    unwrap: Unwrap,
    freshness: Freshness,
    feedback: Option<Feedback>,
}
impl Motor {
    pub fn new(
        id: u16,
        master_id: u16,
        limits: Limits,
        gear: f32,
        offset: f32,
    ) -> Result<Self, Error> {
        limits.validate()?;
        finite(&[gear, offset])?;
        if gear <= 0.0 {
            return Err(Error::Range);
        }
        if id == 0 || id > 0x5ff || master_id > 0x7ff {
            return Err(Error::Id);
        }
        Ok(Self {
            id,
            master_id,
            limits,
            gear,
            offset,
            can_fd: false,
            unwrap: Unwrap::new(),
            freshness: Freshness::new(),
            feedback: None,
        })
    }
    fn frame(&self, id: u16, data: &[u8]) -> Result<CanFrame, Error> {
        Ok(if self.can_fd {
            CanFrame::new_fd(Id::Standard(id), data)?
        } else {
            CanFrame::new(Id::Standard(id), data)?
        })
    }
    /// Opt in to FD transmit and receive. Classic feedback remains accepted.
    /// The STM32 adapter must separately enable FD and configure its BRS policy.
    pub fn with_can_fd(mut self, enabled: bool) -> Self {
        self.can_fd = enabled;
        self
    }
    pub const fn can_fd(&self) -> bool {
        self.can_fd
    }
    pub const fn id(&self) -> u16 {
        self.id
    }
    pub const fn master_id(&self) -> u16 {
        self.master_id
    }
    pub const fn gear_ratio(&self) -> f32 {
        self.gear
    }
    pub const fn limits(&self) -> Limits {
        self.limits
    }
    pub const fn angle_offset_radians(&self) -> f32 {
        self.offset
    }
    pub const fn last_update_us(&self) -> Option<u64> {
        self.freshness.last_update_us()
    }
    /// Set the motor-side zero used by the next feedback and position command.
    /// Already published feedback and internal turn history remain unchanged.
    pub fn set_continuous_angle_offset_radians(&mut self, offset: f32) -> Result<(), Error> {
        finite(&[offset])?;
        self.offset = offset;
        Ok(())
    }
    fn special(&self, code: u8) -> Result<CanFrame, Error> {
        let mut data = [255; 8];
        data[7] = code;
        self.frame(self.id, &data)
    }
    pub fn enable(&self) -> Result<CanFrame, Error> {
        self.special(0xfc)
    }
    /// Explicit poll of the original optional enable keepalive policy.
    /// Only an enabled feedback state produces a frame. App chooses polling and
    /// actual transmission; constructing or querying a motor performs no I/O.
    pub fn enable_guard_frame(&self) -> Result<Option<CanFrame>, Error> {
        if self.feedback.is_some_and(Feedback::enabled) {
            Ok(Some(self.enable()?))
        } else {
            Ok(None)
        }
    }
    pub fn disable(&self) -> Result<CanFrame, Error> {
        self.special(0xfd)
    }
    pub fn clear_error(&self) -> Result<CanFrame, Error> {
        self.special(0xfb)
    }
    pub fn mechanical_zero(&self) -> Result<CanFrame, Error> {
        self.special(0xfe)
    }
    fn command_position(&self, position: f32) -> Result<f32, Error> {
        finite(&[position])?;
        // The original public API normalizes in degrees, then writes radians.
        // Preserve its operation order: changing to direct radian arithmetic
        // changes valid wire f32 values at rounding midpoints.
        let pi = core::f64::consts::PI;
        let angle = position as f64 * 180.0 / pi;
        let offset = self.offset as f64 * 180.0 / pi;
        let maximum = self.limits.position as f64 * 180.0 / pi;
        let span = maximum * 2.0;
        let shifted = angle + offset + maximum;
        let quotient = shifted / span;
        let mut wrapped = if quotient >= i32::MIN as f64 && quotient <= i32::MAX as f64 {
            shifted - (quotient as i32) as f64 * span
        } else {
            // The original float-to-int conversion is undefined outside this
            // range. Keep finite extreme inputs defined without that conversion.
            shifted % span
        };
        if wrapped < 0.0 {
            wrapped += span;
        }
        Ok(((wrapped - maximum) / 180.0 * pi) as f32)
    }
    /// Add the configured motor zero offset and wrap position into [-max, max).
    /// Velocity, gains and torque retain the raw codec's saturation behavior.
    pub fn mit(&self, mut command: MitCommand) -> Result<CanFrame, Error> {
        command.position = self.command_position(command.position)?;
        self.frame(self.id, &command.encode(self.limits)?)
    }
    /// Position uses the configured zero offset and signed protocol wrap range.
    pub fn position_velocity(&self, position: f32, velocity: f32) -> Result<CanFrame, Error> {
        finite(&[position, velocity])?;
        let position = self.command_position(position)?;
        let mut d = [0; 8];
        d[..4].copy_from_slice(&position.to_le_bytes());
        d[4..].copy_from_slice(&velocity.to_le_bytes());
        self.frame(self.id + 0x100, &d)
    }
    pub fn velocity(&self, velocity: f32) -> Result<CanFrame, Error> {
        finite(&[velocity])?;
        self.frame(self.id + 0x200, &velocity.to_le_bytes())
    }
    pub fn read_register(&self, address: u8) -> Result<CanFrame, Error> {
        self.frame(
            0x7ff,
            &[
                self.id as u8,
                (self.id >> 8) as u8,
                0x33,
                address,
                0,
                0,
                0,
                0,
            ],
        )
    }
    pub fn write_register(&self, address: u8, value: [u8; 4]) -> Result<CanFrame, Error> {
        self.frame(
            0x7ff,
            &[
                self.id as u8,
                (self.id >> 8) as u8,
                0x55,
                address,
                value[0],
                value[1],
                value[2],
                value[3],
            ],
        )
    }
    pub fn save_registers(&self) -> Result<CanFrame, Error> {
        self.frame(0x7ff, &[self.id as u8, (self.id >> 8) as u8, 0xaa, 1])
    }
    pub fn receive(&mut self, frame: &CanFrame, now_us: u64) -> Result<(), Error> {
        if frame.id() != Id::Standard(self.master_id) || (frame.is_fd() && !self.can_fd) {
            return Err(Error::Id);
        }
        let mut f = Feedback::decode(frame.data(), self.limits)?;
        if f.motor_id != (self.id & 15) as u8 {
            return Err(Error::Id);
        }
        self.freshness.validate(now_us)?;
        f.continuous_position =
            self.unwrap.update(f.position, self.limits.position * 2.0) - self.offset;
        f.shaft_position = f.continuous_position / self.gear;
        f.shaft_velocity = f.velocity / self.gear;
        self.feedback = Some(f);
        self.freshness.update(now_us)
    }
    pub fn feedback(&self) -> Option<&Feedback> {
        self.feedback.as_ref()
    }
    pub fn connection(&self, now_us: u64) -> Connection {
        self.freshness.connection(now_us, 200_000)
    }
}
