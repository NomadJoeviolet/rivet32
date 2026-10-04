//! Rigid transforms map child coordinates into parent coordinates: R*x + t.
use crate::{Error, math::finite, matrix::Matrix};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub rotation: Matrix<3, 3>,
    pub translation: [f32; 3],
}
impl Transform {
    pub const fn identity() -> Self {
        Self {
            rotation: Matrix::identity(),
            translation: [0.0; 3],
        }
    }
    pub fn validate(&self) -> Result<(), Error> {
        finite(&self.translation)?;
        if !self.rotation.is_finite() {
            return Err(Error::NonFinite);
        }
        let p = self.rotation.transpose() * self.rotation;
        for i in 0..3 {
            for j in 0..3 {
                if (p[i][j] - if i == j { 1.0 } else { 0.0 }).abs() > 1e-4 {
                    return Err(Error::InvalidParameter);
                }
            }
        }
        if (self.rotation.determinant()? - 1.0).abs() > 1e-4 {
            return Err(Error::InvalidParameter);
        }
        Ok(())
    }
    pub fn apply(self, point: [f32; 3]) -> [f32; 3] {
        let p = self.rotation * Matrix::from(point.map(|x| [x]));
        core::array::from_fn(|i| p[i][0] + self.translation[i])
    }
    pub fn inverse(self) -> Self {
        let r = self.rotation.transpose();
        let t = r * Matrix::from(self.translation.map(|x| [-x]));
        Self {
            rotation: r,
            translation: core::array::from_fn(|i| t[i][0]),
        }
    }
    /// Compose `self` after `child`.
    pub fn compose(self, child: Self) -> Self {
        Self {
            rotation: self.rotation * child.rotation,
            translation: self.apply(child.translation),
        }
    }
}
#[derive(Clone, Copy, Debug)]
struct Frame {
    id: u32,
    parent: Option<usize>,
    transform: Transform,
}
pub struct TransformTree<const N: usize> {
    frames: [Option<Frame>; N],
    len: usize,
}
impl<const N: usize> Default for TransformTree<N> {
    fn default() -> Self {
        Self::new()
    }
}
impl<const N: usize> TransformTree<N> {
    pub const fn new() -> Self {
        Self {
            frames: [None; N],
            len: 0,
        }
    }
    pub fn find(&self, id: u32) -> Option<usize> {
        (0..self.len).find(|&i| self.frames[i].is_some_and(|f| f.id == id))
    }
    pub fn add_root(&mut self, id: u32) -> Result<usize, Error> {
        if self.len != 0 {
            return Err(Error::InvalidParameter);
        }
        self.insert(id, None, Transform::identity())
    }
    pub fn add(&mut self, id: u32, parent: usize, transform: Transform) -> Result<usize, Error> {
        self.frame(parent)?;
        self.insert(id, Some(parent), transform)
    }
    fn insert(
        &mut self,
        id: u32,
        parent: Option<usize>,
        transform: Transform,
    ) -> Result<usize, Error> {
        if self.find(id).is_some() {
            return Err(Error::InvalidParameter);
        }
        if self.len == N {
            return Err(Error::Full);
        }
        transform.validate()?;
        let index = self.len;
        self.frames[index] = Some(Frame {
            id,
            parent,
            transform,
        });
        self.len += 1;
        Ok(index)
    }
    fn frame(&self, index: usize) -> Result<Frame, Error> {
        self.frames
            .get(index)
            .copied()
            .flatten()
            .ok_or(Error::NotFound)
    }
    pub fn update(&mut self, index: usize, transform: Transform) -> Result<(), Error> {
        let f = self.frame(index)?;
        if f.parent.is_none() {
            return Err(Error::InvalidParameter);
        }
        transform.validate()?;
        self.frames[index] = Some(Frame { transform, ..f });
        Ok(())
    }
    pub fn to_root(&self, mut index: usize) -> Result<Transform, Error> {
        let mut t = Transform::identity();
        loop {
            let f = self.frame(index)?;
            t = f.transform.compose(t);
            match f.parent {
                Some(p) => index = p,
                None => return Ok(t),
            }
        }
    }
    pub fn transform_point(
        &self,
        point: [f32; 3],
        source: usize,
        target: usize,
    ) -> Result<[f32; 3], Error> {
        finite(&point)?;
        let result = self
            .to_root(target)?
            .inverse()
            .compose(self.to_root(source)?)
            .apply(point);
        finite(&result)?;
        Ok(result)
    }
}
