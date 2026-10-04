//! Stack-allocated row-major f32 matrices; dimensions are checked by the type system.
use crate::Error;
use core::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub, SubAssign};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix<const R: usize, const C: usize> {
    pub data: [[f32; C]; R],
}
impl<const R: usize, const C: usize> Matrix<R, C> {
    pub const fn zeros() -> Self {
        Self {
            data: [[0.0; C]; R],
        }
    }
    pub const fn ones() -> Self {
        Self {
            data: [[1.0; C]; R],
        }
    }
    pub fn transpose(self) -> Matrix<C, R> {
        let mut m = Matrix::zeros();
        for i in 0..R {
            for j in 0..C {
                m[j][i] = self[i][j];
            }
        }
        m
    }
    pub fn is_finite(&self) -> bool {
        self.data.iter().flatten().all(|x| x.is_finite())
    }
    pub fn norm(&self) -> f32 {
        libm::sqrtf(self.data.iter().flatten().map(|x| x * x).sum())
    }
    pub fn block<const A: usize, const B: usize>(
        &self,
        row: usize,
        col: usize,
    ) -> Result<Matrix<A, B>, Error> {
        if row.checked_add(A).is_none_or(|v| v > R) || col.checked_add(B).is_none_or(|v| v > C) {
            return Err(Error::InvalidParameter);
        }
        Ok(Matrix::from(core::array::from_fn(|i| {
            core::array::from_fn(|j| self[row + i][col + j])
        })))
    }
}
impl<const N: usize> Matrix<N, N> {
    pub const fn identity() -> Self {
        let mut m = Self::zeros();
        let mut i = 0;
        while i < N {
            m.data[i][i] = 1.0;
            i += 1;
        }
        m
    }
    pub fn diagonal(values: [f32; N]) -> Self {
        let mut m = Self::zeros();
        for i in 0..N {
            m[i][i] = values[i];
        }
        m
    }
    pub fn trace(&self) -> f32 {
        (0..N).map(|i| self[i][i]).sum()
    }
    /// Gauss-Jordan inversion with scaled partial pivoting. Singular inputs return an error.
    pub fn inverse(self) -> Result<Self, Error> {
        if !self.is_finite() {
            return Err(Error::NonFinite);
        }
        let mut a = self;
        let mut inv = Self::identity();
        let scales: [f32; N] =
            core::array::from_fn(|i| a[i].iter().fold(0.0_f32, |m, x| m.max(x.abs())));
        let mut scales = scales;
        for c in 0..N {
            let mut pivot = c;
            let mut best = 0.0;
            for r in c..N {
                let score = if scales[r] > 0.0 {
                    a[r][c].abs() / scales[r]
                } else {
                    0.0
                };
                if score > best {
                    best = score;
                    pivot = r;
                }
            }
            if best <= f32::EPSILON * N.max(1) as f32 {
                return Err(Error::Singular);
            }
            a.data.swap(c, pivot);
            inv.data.swap(c, pivot);
            scales.swap(c, pivot);
            let d = a[c][c];
            for j in 0..N {
                a[c][j] /= d;
                inv[c][j] /= d;
            }
            for r in 0..N {
                if r == c {
                    continue;
                }
                let k = a[r][c];
                for j in 0..N {
                    a[r][j] -= k * a[c][j];
                    inv[r][j] -= k * inv[c][j];
                }
            }
        }
        if inv.is_finite() {
            Ok(inv)
        } else {
            Err(Error::NonFinite)
        }
    }
    pub fn determinant(self) -> Result<f32, Error> {
        if !self.is_finite() {
            return Err(Error::NonFinite);
        }
        let mut a = self;
        let mut det = 1.0;
        for c in 0..N {
            let mut p = c;
            for r in c + 1..N {
                if a[r][c].abs() > a[p][c].abs() {
                    p = r;
                }
            }
            if a[p][c] == 0.0 {
                return Ok(0.0);
            }
            if p != c {
                a.data.swap(p, c);
                det = -det;
            }
            let d = a[c][c];
            det *= d;
            for r in c + 1..N {
                let k = a[r][c] / d;
                for j in c + 1..N {
                    a[r][j] -= k * a[c][j];
                }
            }
        }
        if det.is_finite() {
            Ok(det)
        } else {
            Err(Error::NonFinite)
        }
    }
    pub fn cholesky(self) -> Result<Self, Error> {
        if !self.is_finite() {
            return Err(Error::NonFinite);
        }
        let mut l = Self::zeros();
        for i in 0..N {
            for j in 0..=i {
                if (self[i][j] - self[j][i]).abs() > 1e-5 * (1.0 + self[i][j].abs()) {
                    return Err(Error::InvalidParameter);
                }
                let mut sum = self[i][j];
                for k in 0..j {
                    sum -= l[i][k] * l[j][k];
                }
                if i == j {
                    if sum <= 0.0 {
                        return Err(Error::Singular);
                    }
                    l[i][j] = libm::sqrtf(sum);
                } else {
                    l[i][j] = sum / l[j][j];
                }
            }
        }
        Ok(l)
    }
    pub fn powi(self, power: i32) -> Result<Self, Error> {
        let mut base = if power < 0 { self.inverse()? } else { self };
        let mut n = power.unsigned_abs();
        let mut r = Self::identity();
        while n > 0 {
            if n & 1 != 0 {
                r = r * base;
            }
            n >>= 1;
            if n > 0 {
                base = base * base;
            }
        }
        if r.is_finite() {
            Ok(r)
        } else {
            Err(Error::NonFinite)
        }
    }
    pub fn symmetrized(self) -> Self {
        (self + self.transpose()) * 0.5
    }
}
impl<const R: usize, const C: usize> From<[[f32; C]; R]> for Matrix<R, C> {
    fn from(data: [[f32; C]; R]) -> Self {
        Self { data }
    }
}
impl<const R: usize, const C: usize> Default for Matrix<R, C> {
    fn default() -> Self {
        Self::zeros()
    }
}
impl<const R: usize, const C: usize> Index<usize> for Matrix<R, C> {
    type Output = [f32; C];
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}
impl<const R: usize, const C: usize> IndexMut<usize> for Matrix<R, C> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.data[i]
    }
}
impl<const R: usize, const C: usize> Add for Matrix<R, C> {
    type Output = Self;
    fn add(mut self, b: Self) -> Self {
        for i in 0..R {
            for j in 0..C {
                self[i][j] += b[i][j];
            }
        }
        self
    }
}
impl<const R: usize, const C: usize> Sub for Matrix<R, C> {
    type Output = Self;
    fn sub(mut self, b: Self) -> Self {
        for i in 0..R {
            for j in 0..C {
                self[i][j] -= b[i][j];
            }
        }
        self
    }
}
impl<const R: usize, const C: usize> Neg for Matrix<R, C> {
    type Output = Self;
    fn neg(self) -> Self {
        self * (-1.0)
    }
}
impl<const R: usize, const C: usize> AddAssign for Matrix<R, C> {
    fn add_assign(&mut self, b: Self) {
        *self = *self + b;
    }
}
impl<const R: usize, const C: usize> SubAssign for Matrix<R, C> {
    fn sub_assign(&mut self, b: Self) {
        *self = *self - b;
    }
}
impl<const R: usize, const C: usize> Mul<f32> for Matrix<R, C> {
    type Output = Self;
    fn mul(mut self, b: f32) -> Self {
        for row in &mut self.data {
            for v in row {
                *v *= b;
            }
        }
        self
    }
}
impl<const R: usize, const C: usize> Div<f32> for Matrix<R, C> {
    type Output = Self;
    fn div(mut self, b: f32) -> Self {
        for row in &mut self.data {
            for v in row {
                *v /= b;
            }
        }
        self
    }
}
impl<const R: usize, const C: usize, const K: usize> Mul<Matrix<C, K>> for Matrix<R, C> {
    type Output = Matrix<R, K>;
    fn mul(self, b: Matrix<C, K>) -> Self::Output {
        let mut out = Matrix::zeros();
        for i in 0..R {
            for j in 0..K {
                for k in 0..C {
                    out[i][j] += self[i][k] * b[k][j];
                }
            }
        }
        out
    }
}
