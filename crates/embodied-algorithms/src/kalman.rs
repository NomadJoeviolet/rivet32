//! Linear and extended Kalman recursions with Joseph covariance correction.
use crate::{Error, math::positive_dt, matrix::Matrix};

#[derive(Clone, Copy, Debug)]
pub struct Kalman<const N: usize, const M: usize> {
    pub state: Matrix<N, 1>,
    pub covariance: Matrix<N, N>,
    pub process_noise: Matrix<N, N>,
    pub measurement_noise: Matrix<M, M>,
}
impl<const N: usize, const M: usize> Kalman<N, M> {
    pub const fn new(
        state: Matrix<N, 1>,
        covariance: Matrix<N, N>,
        process_noise: Matrix<N, N>,
        measurement_noise: Matrix<M, M>,
    ) -> Self {
        Self {
            state,
            covariance,
            process_noise,
            measurement_noise,
        }
    }
    /// `control` is the already projected control vector `B * u`.
    pub fn predict(
        &mut self,
        transition: Matrix<N, N>,
        control: Matrix<N, 1>,
    ) -> Result<(), Error> {
        self.commit(
            transition * self.state + control,
            transition * self.covariance * transition.transpose() + self.process_noise,
        )
    }
    pub fn predict_extended<F, J>(
        &mut self,
        dt: f32,
        transition: F,
        jacobian: J,
    ) -> Result<(), Error>
    where
        F: FnOnce(Matrix<N, 1>, f32) -> Matrix<N, 1>,
        J: FnOnce(Matrix<N, 1>, f32) -> Matrix<N, N>,
    {
        positive_dt(dt)?;
        // Both evaluations use the previous posterior, not the already predicted state.
        let f = jacobian(self.state, dt);
        let state = transition(self.state, dt);
        self.commit(
            state,
            f * self.covariance * f.transpose() + self.process_noise,
        )
    }
    pub fn correct(&mut self, measurement: Matrix<M, 1>, h: Matrix<M, N>) -> Result<(), Error> {
        self.correct_residual(measurement - h * self.state, h)
    }
    pub fn correct_extended<H, J>(
        &mut self,
        measurement: Matrix<M, 1>,
        model: H,
        jacobian: J,
    ) -> Result<(), Error>
    where
        H: FnOnce(Matrix<N, 1>) -> Matrix<M, 1>,
        J: FnOnce(Matrix<N, 1>) -> Matrix<M, N>,
    {
        self.correct_residual(measurement - model(self.state), jacobian(self.state))
    }
    fn correct_residual(&mut self, residual: Matrix<M, 1>, h: Matrix<M, N>) -> Result<(), Error> {
        let s = h * self.covariance * h.transpose() + self.measurement_noise;
        let k = self.covariance * h.transpose() * s.inverse()?;
        let a = Matrix::identity() - k * h;
        self.commit(
            self.state + k * residual,
            a * self.covariance * a.transpose() + k * self.measurement_noise * k.transpose(),
        )
    }
    fn commit(&mut self, state: Matrix<N, 1>, covariance: Matrix<N, N>) -> Result<(), Error> {
        if !state.is_finite() || !covariance.is_finite() {
            return Err(Error::NonFinite);
        }
        self.state = state;
        self.covariance = covariance.symmetrized();
        Ok(())
    }
}
