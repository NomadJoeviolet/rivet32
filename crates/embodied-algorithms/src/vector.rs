//! Small spatial vectors. Linear quantities use caller-selected consistent units.
use crate::{
    Error,
    math::{finite, sin_cos},
};
use core::ops::{Add, Div, Mul, Sub};
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
impl Vector3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn dot(self, b: Self) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
    pub fn cross(self, b: Self) -> Self {
        Self::new(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x,
        )
    }
    pub fn magnitude(self) -> f32 {
        libm::sqrtf(self.dot(self))
    }
    pub fn normalized(self) -> Result<Self, Error> {
        finite(&[self.x, self.y, self.z])?;
        let n = self.magnitude();
        if n == 0.0 {
            return Err(Error::ZeroNorm);
        }
        if !n.is_finite() {
            return Err(Error::NonFinite);
        }
        Ok(self / n)
    }
    pub fn project_onto(self, b: Self) -> Result<Self, Error> {
        let b = b.normalized()?;
        Ok(b * self.dot(b))
    }
    pub fn rotate_axis(self, axis: Self, angle: f32) -> Result<Self, Error> {
        finite(&[angle])?;
        let k = axis.normalized()?;
        let (s, c) = sin_cos(angle);
        Ok(self * c + k.cross(self) * s + k * (k.dot(self) * (1.0 - c)))
    }
    pub fn rotate_x(self, a: f32) -> Self {
        let (s, c) = sin_cos(a);
        Self::new(self.x, self.y * c - self.z * s, self.y * s + self.z * c)
    }
    pub fn rotate_y(self, a: f32) -> Self {
        let (s, c) = sin_cos(a);
        Self::new(self.x * c + self.z * s, self.y, self.z * c - self.x * s)
    }
    pub fn rotate_z(self, a: f32) -> Self {
        let (s, c) = sin_cos(a);
        Self::new(self.x * c - self.y * s, self.x * s + self.y * c, self.z)
    }
    pub fn rotate_euler(self, [r, p, y]: [f32; 3]) -> Self {
        self.rotate_x(r).rotate_y(p).rotate_z(y)
    }
    pub fn to_2d(self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
}
impl Vector2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn from_polar(radius: f32, angle: f32) -> Self {
        let (s, c) = sin_cos(angle);
        Self::new(radius * c, radius * s)
    }
    pub fn dot(self, b: Self) -> f32 {
        self.x * b.x + self.y * b.y
    }
    pub fn cross(self, b: Self) -> f32 {
        self.x * b.y - self.y * b.x
    }
    pub fn magnitude(self) -> f32 {
        libm::sqrtf(self.dot(self))
    }
    pub fn angle(self) -> f32 {
        libm::atan2f(self.y, self.x)
    }
    pub fn normalized(self) -> Result<Self, Error> {
        let v = self.to_3d().normalized()?;
        Ok(v.to_2d())
    }
    pub fn project_onto(self, b: Self) -> Result<Self, Error> {
        let b = b.normalized()?;
        Ok(b * self.dot(b))
    }
    pub fn rotate(self, a: f32) -> Self {
        let (s, c) = sin_cos(a);
        Self::new(self.x * c - self.y * s, self.x * s + self.y * c)
    }
    pub const fn perpendicular(self) -> Self {
        Self::new(-self.y, self.x)
    }
    pub const fn to_3d(self) -> Vector3 {
        Vector3::new(self.x, self.y, 0.0)
    }
}
macro_rules! ops {($t:ident,$($v:ident),+) => {
    impl Add for $t {type Output=Self;fn add(self,b:Self)->Self{Self{$($v:self.$v+b.$v),+}}}
    impl Sub for $t {type Output=Self;fn sub(self,b:Self)->Self{Self{$($v:self.$v-b.$v),+}}}
    impl Mul<f32> for $t {type Output=Self;fn mul(self,b:f32)->Self{Self{$($v:self.$v*b),+}}}
    impl Div<f32> for $t {type Output=Self;fn div(self,b:f32)->Self{Self{$($v:self.$v/b),+}}}
};}
ops!(Vector3, x, y, z);
ops!(Vector2, x, y);
