//! Portable scalar math and explicit per-call slope limits.
use crate::Error;
pub use core::f32::consts::PI;
pub use libm::{
    acosf as acos, asinf as asin, atan2f as atan2, atanf as atan, cosf as cos, sinf as sin,
    sqrtf as sqrt, tanf as tan,
};
pub fn sin_cos(angle: f32) -> (f32, f32) {
    (sin(angle), cos(angle))
}
pub fn inv_sqrt(x: f32) -> f32 {
    1.0 / sqrt(x)
}
pub fn sign(x: f32) -> i8 {
    if x > 0.0 {
        1
    } else if x < 0.0 {
        -1
    } else {
        0
    }
}
pub fn normalize_angle(angle: f32, half_range: f32) -> Result<f32, Error> {
    if !angle.is_finite() || !half_range.is_finite() {
        return Err(Error::NonFinite);
    }
    if half_range <= 0.0 || !(2.0 * half_range).is_finite() {
        return Err(Error::InvalidParameter);
    }
    let mut r = libm::fmodf(angle, 2.0 * half_range);
    if r >= half_range {
        r -= 2.0 * half_range;
    } else if r < -half_range {
        r += 2.0 * half_range;
    }
    Ok(r)
}
pub fn fal(e: f32, alpha: f32, zeta: f32) -> Result<f32, Error> {
    finite(&[e, alpha, zeta])?;
    if zeta <= 0.0 {
        return Err(Error::InvalidParameter);
    }
    Ok(if e.abs() < zeta {
        e / libm::powf(zeta, 1.0 - alpha)
    } else {
        libm::powf(e.abs(), alpha) * sign(e) as f32
    })
}
pub fn fsg(x: f32, d: f32) -> i8 {
    (sign(x + d) - sign(x - d)) / 2
}
pub fn differential(values: &[f32], order: u8) -> Result<f32, Error> {
    match (order, values) {
        (1, [a, b, ..]) => Ok(a - b),
        (2, [a, b, c, ..]) => Ok(a - 2.0 * b + c),
        (0, [a, ..]) => Ok(*a),
        _ => Err(Error::InvalidParameter),
    }
}
pub(crate) fn finite(v: &[f32]) -> Result<(), Error> {
    if v.iter().all(|x| x.is_finite()) {
        Ok(())
    } else {
        Err(Error::NonFinite)
    }
}
pub(crate) fn positive_dt(dt: f32) -> Result<(), Error> {
    finite(&[dt])?;
    if dt <= 0.0 {
        Err(Error::InvalidParameter)
    } else {
        Ok(())
    }
}
pub fn abs_into(src: &[f32], dst: &mut [f32]) -> Result<(), Error> {
    if src.len() != dst.len() {
        return Err(Error::InvalidParameter);
    }
    for (s, d) in src.iter().zip(dst) {
        *d = s.abs();
    }
    Ok(())
}
pub fn add_into(a: &[f32], b: &[f32], dst: &mut [f32]) -> Result<(), Error> {
    if a.len() != b.len() || a.len() != dst.len() {
        return Err(Error::InvalidParameter);
    }
    for ((a, b), d) in a.iter().zip(b).zip(dst) {
        *d = a + b;
    }
    Ok(())
}
pub fn sub_into(a: &[f32], b: &[f32], dst: &mut [f32]) -> Result<(), Error> {
    if a.len() != b.len() || a.len() != dst.len() {
        return Err(Error::InvalidParameter);
    }
    for ((a, b), d) in a.iter().zip(b).zip(dst) {
        *d = a - b;
    }
    Ok(())
}
pub fn read_i16_le(bytes: &[u8]) -> Result<i16, Error> {
    Ok(i16::from_le_bytes(
        bytes
            .get(..2)
            .ok_or(Error::InvalidParameter)?
            .try_into()
            .unwrap(),
    ))
}
pub fn read_f32_le(bytes: &[u8]) -> Result<f32, Error> {
    Ok(f32::from_le_bytes(
        bytes
            .get(..4)
            .ok_or(Error::InvalidParameter)?
            .try_into()
            .unwrap(),
    ))
}
pub fn write_u32_le(value: u32, bytes: &mut [u8]) -> Result<(), Error> {
    bytes
        .get_mut(..4)
        .ok_or(Error::InvalidParameter)?
        .copy_from_slice(&value.to_le_bytes());
    Ok(())
}

#[derive(Clone, Copy, Debug)]
pub struct Slope {
    pub acceleration: f32,
    pub deceleration: f32,
}
impl Slope {
    pub fn new(acceleration: f32, deceleration: f32) -> Result<Self, Error> {
        finite(&[acceleration, deceleration])?;
        if acceleration < 0.0 || deceleration < 0.0 {
            return Err(Error::InvalidParameter);
        }
        Ok(Self {
            acceleration,
            deceleration,
        })
    }
    pub fn step(&self, current: f32, target: f32) -> f32 {
        if self.acceleration == 0.0 || self.deceleration == 0.0 {
            return target;
        }
        if current < target - self.acceleration {
            current + self.acceleration
        } else if current > target + self.deceleration {
            current - self.deceleration
        } else {
            target
        }
    }
    pub fn step_magnitude(&self, current: f32, target: f32) -> f32 {
        if current > 0.0 {
            self.step(current, target)
        } else {
            -self.step(-current, -target)
        }
    }
}
