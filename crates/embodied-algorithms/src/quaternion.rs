//! Scalar-first Hamilton quaternions; active body-to-world rotation, ZYX Euler angles.
use crate::{
    Error,
    math::{finite, sin_cos},
    matrix::Matrix,
};
use core::ops::Mul;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Default for Quaternion {
    fn default() -> Self {
        Self::identity()
    }
}
impl Quaternion {
    pub const fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self { w, x, y, z }
    }
    pub const fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0)
    }
    pub fn as_array(self) -> [f32; 4] {
        [self.w, self.x, self.y, self.z]
    }
    pub fn as_matrix(self) -> Matrix<4, 1> {
        Matrix::from(self.as_array().map(|x| [x]))
    }
    pub fn from_matrix(q: Matrix<4, 1>) -> Self {
        Self::new(q[0][0], q[1][0], q[2][0], q[3][0])
    }
    pub fn normalized(self) -> Result<Self, Error> {
        let a = self.as_array();
        finite(&a)?;
        // Scaling avoids overflow for finite, large quaternions.
        let scale = a.iter().fold(0.0_f32, |m, x| m.max(x.abs()));
        if scale == 0.0 {
            return Err(Error::ZeroNorm);
        }
        let a = a.map(|x| x / scale);
        let norm = libm::sqrtf(a.iter().map(|x| x * x).sum());
        Ok(Self::new(
            a[0] / norm,
            a[1] / norm,
            a[2] / norm,
            a[3] / norm,
        ))
    }
    pub const fn conjugate(self) -> Self {
        Self::new(self.w, -self.x, -self.y, -self.z)
    }
    pub fn from_euler([roll, pitch, yaw]: [f32; 3]) -> Self {
        let (sr, cr) = sin_cos(roll * 0.5);
        let (sp, cp) = sin_cos(pitch * 0.5);
        let (sy, cy) = sin_cos(yaw * 0.5);
        Self::new(
            cr * cp * cy + sr * sp * sy,
            sr * cp * cy - cr * sp * sy,
            cr * sp * cy + sr * cp * sy,
            cr * cp * sy - sr * sp * cy,
        )
    }
    pub fn to_euler(self) -> [f32; 3] {
        let Self { w, x, y, z } = self;
        [
            libm::atan2f(2.0 * (w * x + y * z), 1.0 - 2.0 * (x * x + y * y)),
            libm::asinf((2.0 * (w * y - z * x)).clamp(-1.0, 1.0)),
            libm::atan2f(2.0 * (w * z + x * y), 1.0 - 2.0 * (y * y + z * z)),
        ]
    }
    pub fn rotation_matrix(self) -> Matrix<3, 3> {
        let Self { w, x, y, z } = self;
        Matrix::from([
            [
                1.0 - 2.0 * (y * y + z * z),
                2.0 * (x * y - w * z),
                2.0 * (x * z + w * y),
            ],
            [
                2.0 * (x * y + w * z),
                1.0 - 2.0 * (x * x + z * z),
                2.0 * (y * z - w * x),
            ],
            [
                2.0 * (x * z - w * y),
                2.0 * (y * z + w * x),
                1.0 - 2.0 * (x * x + y * y),
            ],
        ])
    }
    pub fn rotate(self, vector: [f32; 3]) -> [f32; 3] {
        let v = self.rotation_matrix() * Matrix::from(vector.map(|x| [x]));
        core::array::from_fn(|i| v[i][0])
    }
    /// Unit world gravity expressed in body coordinates.
    pub fn gravity(self) -> [f32; 3] {
        let Self { w, x, y, z } = self;
        [
            2.0 * (x * z - w * y),
            2.0 * (y * z + w * x),
            w * w - x * x - y * y + z * z,
        ]
    }
    pub(crate) fn gravity_jacobian(self) -> Matrix<3, 4> {
        let Self { w, x, y, z } = self;
        Matrix::from([
            [-2.0 * y, 2.0 * z, -2.0 * w, 2.0 * x],
            [2.0 * x, 2.0 * w, 2.0 * z, 2.0 * y],
            [2.0 * w, -2.0 * x, -2.0 * y, 2.0 * z],
        ])
    }
}
impl Mul for Quaternion {
    type Output = Self;
    fn mul(self, b: Self) -> Self {
        let a = self;
        Self::new(
            a.w * b.w - a.x * b.x - a.y * b.y - a.z * b.z,
            a.w * b.x + a.x * b.w + a.y * b.z - a.z * b.y,
            a.w * b.y - a.x * b.z + a.y * b.w + a.z * b.x,
            a.w * b.z + a.x * b.y - a.y * b.x + a.z * b.w,
        )
    }
}

/// Choose continuous 2π-equivalent Euler angles nearest the previous sample.
pub fn unwrap_euler(current: [f32; 3], previous: [f32; 3]) -> Result<[f32; 3], Error> {
    finite(&current)?;
    finite(&previous)?;
    let mut result = current;
    for i in 0..3 {
        result[i] = previous[i]
            + crate::math::normalize_angle(current[i] - previous[i], core::f32::consts::PI)?;
    }
    Ok(result)
}
