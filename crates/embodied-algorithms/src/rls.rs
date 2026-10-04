//! Recursive least squares with explicit forgetting factor and covariance scale.
use crate::{Error, math::finite, matrix::Matrix};
#[derive(Clone, Copy, Debug)]
pub struct Rls<const N: usize> {
    parameters: [f32; N],
    initial: [f32; N],
    pub covariance: Matrix<N, N>,
    delta: f32,
    lambda: f32,
    pub updates: u64,
}
impl<const N: usize> Rls<N> {
    /// Initial covariance is `delta * I`, matching both reference RLS variants.
    pub fn new(delta: f32, lambda: f32) -> Result<Self, Error> {
        finite(&[delta, lambda])?;
        if N == 0 || delta <= 0.0 || lambda <= 0.0 || lambda > 1.0 {
            return Err(Error::InvalidParameter);
        }
        Ok(Self {
            parameters: [0.0; N],
            initial: [0.0; N],
            covariance: Matrix::identity() * delta,
            delta,
            lambda,
            updates: 0,
        })
    }
    pub fn parameters(&self) -> [f32; N] {
        self.parameters
    }
    pub fn set_parameters(&mut self, parameters: [f32; N]) -> Result<(), Error> {
        finite(&parameters)?;
        self.parameters = parameters;
        Ok(())
    }
    pub fn set_initial_parameters(&mut self, parameters: [f32; N]) -> Result<(), Error> {
        self.set_parameters(parameters)?;
        self.initial = parameters;
        Ok(())
    }
    pub fn reset(&mut self) {
        self.parameters = self.initial;
        self.covariance = Matrix::identity() * self.delta;
        self.updates = 0;
    }
    pub fn predict(&self, sample: [f32; N]) -> Result<f32, Error> {
        finite(&sample)?;
        let y = sample.iter().zip(self.parameters).map(|(x, p)| x * p).sum();
        finite(&[y])?;
        Ok(y)
    }
    pub fn update(&mut self, sample: [f32; N], actual: f32) -> Result<[f32; N], Error> {
        finite(&sample)?;
        finite(&[actual])?;
        let x = Matrix::from(sample.map(|x| [x]));
        let px = self.covariance * x;
        let denominator = self.lambda + (x.transpose() * px)[0][0];
        if !denominator.is_finite() {
            return Err(Error::NonFinite);
        }
        if denominator <= 0.0 {
            return Err(Error::Singular);
        }
        let gain = px / denominator;
        let residual = actual - self.predict(sample)?;
        let p = core::array::from_fn(|i| self.parameters[i] + gain[i][0] * residual);
        let covariance = ((self.covariance - gain * x.transpose() * self.covariance) / self.lambda)
            .symmetrized();
        finite(&p)?;
        if !covariance.is_finite() {
            return Err(Error::NonFinite);
        }
        self.parameters = p;
        self.covariance = covariance;
        self.updates = self.updates.saturating_add(1);
        Ok(p)
    }
}
