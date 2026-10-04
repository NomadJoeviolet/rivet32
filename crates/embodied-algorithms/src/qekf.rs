//! Quaternion EKFs: four-state normalized EKF and six-state bias EKF.
//! Accelerations are normalized internally; gyro is rad/s and dt is seconds.
use crate::{
    Error,
    math::{finite, positive_dt},
    matrix::Matrix,
    quaternion::Quaternion,
};

fn unit(a: [f32; 3]) -> Result<[f32; 3], Error> {
    finite(&a)?;
    let n = libm::sqrtf(a.iter().map(|x| x * x).sum());
    if n == 0.0 {
        return Err(Error::ZeroNorm);
    }
    finite(&[n])?;
    Ok(a.map(|x| x / n))
}
fn omega([x, y, z]: [f32; 3]) -> Matrix<4, 4> {
    Matrix::from([
        [0.0, -x, -y, -z],
        [x, 0.0, z, -y],
        [y, -z, 0.0, x],
        [z, y, -x, 0.0],
    ])
}
fn noise_jacobian(q: Quaternion) -> Matrix<4, 3> {
    let Quaternion { w, x, y, z } = q;
    Matrix::from([[-x, -y, -z], [w, -z, y], [z, w, -x], [-y, x, w]])
}
fn observed(accel: [f32; 3]) -> Result<[f32; 2], Error> {
    let a = unit(accel)?;
    Ok([
        libm::atan2f(a[1], a[2]),
        libm::asinf((-a[0]).clamp(-1.0, 1.0)),
    ])
}

#[derive(Clone, Copy, Debug)]
pub struct LiteConfig {
    pub process_noise: [f32; 3],
    pub accel_measurement_noise: [f32; 3],
    pub chi_square_threshold: f32,
}
impl Default for LiteConfig {
    fn default() -> Self {
        Self {
            process_noise: [1.0; 3],
            accel_measurement_noise: [1.0; 3],
            chi_square_threshold: 0.5,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct LiteQekf {
    state: Quaternion,
    pub covariance: Matrix<4, 4>,
    config: LiteConfig,
    converged: bool,
}
impl LiteQekf {
    pub fn new(config: LiteConfig) -> Result<Self, Error> {
        finite(&config.process_noise)?;
        finite(&config.accel_measurement_noise)?;
        finite(&[config.chi_square_threshold])?;
        if config.process_noise.iter().any(|&x| x < 0.0)
            || config.accel_measurement_noise.iter().any(|&x| x <= 0.0)
            || config.chi_square_threshold <= 0.0
        {
            return Err(Error::InvalidParameter);
        }
        Ok(Self {
            state: Quaternion::identity(),
            covariance: Matrix::identity(),
            config,
            converged: false,
        })
    }
    pub fn reset(&mut self) {
        self.state = Quaternion::identity();
        self.covariance = Matrix::identity();
        self.converged = false;
    }
    pub fn quaternion(&self) -> Quaternion {
        self.state
    }
    pub fn converged(&self) -> bool {
        self.converged
    }
    pub fn initialize_from_accel(&mut self, a: [f32; 3]) -> Result<(), Error> {
        let [r, p] = observed(a)?;
        self.reset();
        self.state = Quaternion::from_euler([r, p, 0.0]);
        Ok(())
    }
    pub fn rebuild_pitch_roll_from_accel(&mut self, a: [f32; 3]) -> Result<(), Error> {
        let [r, p] = observed(a)?;
        let y = self.state.to_euler()[2];
        self.reset();
        self.state = Quaternion::from_euler([r, p, y]);
        Ok(())
    }
    pub fn predict(&mut self, gyro: [f32; 3], dt: f32) -> Result<(), Error> {
        finite(&gyro)?;
        positive_dt(dt)?;
        let f = Matrix::identity() + omega(gyro) * (0.5 * dt);
        let raw = f * self.state.as_matrix();
        let state = Quaternion::from_matrix(raw).normalized()?;
        let inv_norm = 1.0 / raw.norm();
        let orthogonalization =
            (Matrix::identity() - state.as_matrix() * state.as_matrix().transpose()) * inv_norm;
        let fx = orthogonalization * f;
        let fw = orthogonalization * noise_jacobian(self.state) * (0.5 * dt);
        let p = (fx * self.covariance * fx.transpose()
            + fw * Matrix::diagonal(self.config.process_noise) * fw.transpose())
        .symmetrized();
        if !p.is_finite() {
            return Err(Error::NonFinite);
        }
        self.state = state;
        self.covariance = p;
        Ok(())
    }
    fn innovation(
        &self,
        accel: [f32; 3],
    ) -> Result<(Matrix<3, 1>, Matrix<3, 4>, Matrix<3, 3>), Error> {
        let a = unit(accel)?;
        let g = self.state.gravity();
        let e = Matrix::from(core::array::from_fn(|i| [a[i] - g[i]]));
        let h = self.state.gravity_jacobian();
        let s = h * self.covariance * h.transpose()
            + Matrix::diagonal(self.config.accel_measurement_noise);
        Ok((e, h, s.inverse()?))
    }
    pub fn correct_with_accel(&mut self, accel: [f32; 3], gain_scale: f32) -> Result<(), Error> {
        finite(&[gain_scale])?;
        let (e, h, s_inv) = self.innovation(accel)?;
        let k = (self.covariance * h.transpose() * s_inv) * gain_scale.clamp(0.0, 1.0);
        let state = Quaternion::from_matrix(self.state.as_matrix() + k * e).normalized()?;
        let a = Matrix::identity() - k * h;
        // Joseph form remains valid for adaptively scaled gain, unlike (I-KH)P.
        let p = (a * self.covariance * a.transpose()
            + k * Matrix::diagonal(self.config.accel_measurement_noise) * k.transpose())
        .symmetrized();
        if !p.is_finite() {
            return Err(Error::NonFinite);
        }
        self.state = state;
        self.covariance = p;
        Ok(())
    }
    pub fn accel_chi_square(&self, accel: [f32; 3]) -> Result<f32, Error> {
        let (e, _, s) = self.innovation(accel)?;
        let chi = (e.transpose() * s * e)[0][0];
        finite(&[chi])?;
        Ok(chi.max(0.0))
    }
    pub fn accel_direction_deviation(&self, accel: [f32; 3]) -> Result<f32, Error> {
        let a = unit(accel)?;
        let g = self.state.gravity();
        Ok(libm::acosf(
            a.iter()
                .zip(g)
                .map(|(x, y)| x * y)
                .sum::<f32>()
                .clamp(-1.0, 1.0),
        ))
    }
    pub fn adaptive_accel_gain(&mut self, chi: f32) -> Result<f32, Error> {
        finite(&[chi])?;
        if chi < 0.0 {
            return Err(Error::InvalidParameter);
        }
        let t = self.config.chi_square_threshold;
        if chi < 0.5 * t {
            self.converged = true;
        }
        Ok(if self.converged && chi > 0.1 * t && chi <= t {
            ((t - chi) / (0.9 * t)).clamp(0.0, 1.0)
        } else {
            1.0
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct QekfConfig {
    pub quaternion_process_noise: f32,
    pub bias_process_noise: f32,
    pub measurement_noise: f32,
    pub forgetting_factor: f32,
    pub accel_low_pass_time_constant: f32,
    pub chi_square_threshold: f32,
}
impl Default for QekfConfig {
    fn default() -> Self {
        Self {
            quaternion_process_noise: 0.01,
            bias_process_noise: 0.0001,
            measurement_noise: 1.0,
            forgetting_factor: 1.0,
            accel_low_pass_time_constant: 0.0,
            chi_square_threshold: 1e-8,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct QuaternionEkf {
    /// `[qw, qx, qy, qz, gyro_bias_x, gyro_bias_y]`; yaw bias is unobservable from gravity.
    pub state: Matrix<6, 1>,
    pub covariance: Matrix<6, 6>,
    config: QekfConfig,
    accel: [f32; 3],
    updates: u64,
    errors: u64,
    converged: bool,
    rejected: bool,
    chi: f32,
}
impl QuaternionEkf {
    pub fn new(config: QekfConfig) -> Result<Self, Error> {
        finite(&[
            config.quaternion_process_noise,
            config.bias_process_noise,
            config.measurement_noise,
            config.forgetting_factor,
            config.accel_low_pass_time_constant,
            config.chi_square_threshold,
        ])?;
        if config.quaternion_process_noise < 0.0
            || config.bias_process_noise < 0.0
            || config.measurement_noise <= 0.0
            || config.forgetting_factor <= 0.0
            || config.forgetting_factor > 1.0
            || config.accel_low_pass_time_constant < 0.0
            || config.chi_square_threshold <= 0.0
        {
            return Err(Error::InvalidParameter);
        }
        let mut p = Matrix::ones() * 0.1;
        for i in 0..6 {
            p[i][i] = if i < 4 { 1e5 } else { 1e2 };
        }
        Ok(Self {
            state: Matrix::from([[1.0], [0.0], [0.0], [0.0], [0.0], [0.0]]),
            covariance: p,
            config,
            accel: [0.0; 3],
            updates: 0,
            errors: 0,
            converged: false,
            rejected: false,
            chi: 0.0,
        })
    }
    pub fn reset(&mut self) {
        *self = Self::new(self.config).unwrap();
    }
    pub fn quaternion(&self) -> Quaternion {
        Quaternion::new(
            self.state[0][0],
            self.state[1][0],
            self.state[2][0],
            self.state[3][0],
        )
    }
    pub fn converged(&self) -> bool {
        self.converged
    }
    pub fn accel_rejected(&self) -> bool {
        self.rejected
    }
    pub fn chi_square(&self) -> f32 {
        self.chi
    }
    pub fn updates(&self) -> u64 {
        self.updates
    }
    /// Invalid inputs or singular innovations leave the entire filter unchanged.
    pub fn update(&mut self, gyro: [f32; 3], accel: [f32; 3], dt: f32) -> Result<(), Error> {
        let mut next = *self;
        next.step(gyro, accel, dt)?;
        *self = next;
        Ok(())
    }
    fn step(&mut self, gyro: [f32; 3], accel: [f32; 3], dt: f32) -> Result<(), Error> {
        finite(&gyro)?;
        unit(accel)?;
        positive_dt(dt)?;
        let gyro = [
            gyro[0] - self.state[4][0],
            gyro[1] - self.state[5][0],
            gyro[2],
        ];
        let block = Matrix::identity() + omega(gyro) * (0.5 * dt);
        let mut f = Matrix::<6, 6>::identity();
        for i in 0..4 {
            for j in 0..4 {
                f[i][j] = block[i][j];
            }
        }
        let mut prior = f * self.state;
        let q = Quaternion::from_matrix(prior.block::<4, 1>(0, 0)?).normalized()?;
        for (i, v) in q.as_array().iter().enumerate() {
            prior[i][0] = *v;
        }
        let nj = noise_jacobian(q) * (-0.5 * dt);
        for i in 0..4 {
            for j in 0..2 {
                f[i][j + 4] = nj[i][j];
            }
        }
        let c = self.config;
        if self.updates == 0 {
            self.accel = accel;
        }
        for (i, &a) in accel.iter().enumerate() {
            self.accel[i] = (self.accel[i] * c.accel_low_pass_time_constant + a * dt)
                / (dt + c.accel_low_pass_time_constant);
        }
        let z = unit(self.accel)?;
        let accel_norm = libm::sqrtf(self.accel.iter().map(|x| x * x).sum());
        let gyro_norm = libm::sqrtf(gyro.iter().map(|x| x * x).sum());
        let stable = gyro_norm < 0.3 && (accel_norm - 9.8).abs() < 0.5;
        let mut faded = self.covariance;
        for i in 4..6 {
            faded[i][i] = (faded[i][i] / c.forgetting_factor).min(10000.0);
        }
        let noise = Matrix::diagonal(core::array::from_fn(|i| {
            if i < 4 {
                c.quaternion_process_noise * dt
            } else {
                c.bias_process_noise * dt
            }
        }));
        let p = (f * faded * f.transpose() + noise).symmetrized();
        let mut h = Matrix::<3, 6>::zeros();
        let hq = q.gravity_jacobian();
        for i in 0..3 {
            for j in 0..4 {
                h[i][j] = hq[i][j];
            }
        }
        let r = Matrix::identity() * c.measurement_noise;
        let inverse = (h * p * h.transpose() + r).inverse()?;
        let g = q.gravity();
        let residual = Matrix::from(core::array::from_fn(|i| [z[i] - g[i]]));
        self.chi = (residual.transpose() * inverse * residual)[0][0].max(0.0);
        if self.chi < 0.5 * c.chi_square_threshold {
            self.converged = true;
        }
        self.rejected = false;
        let mut gain = 1.0;
        if self.chi > c.chi_square_threshold && self.converged {
            if stable {
                self.errors = self.errors.saturating_add(1);
            }
            if self.errors <= 50 {
                self.rejected = true;
                self.state = prior;
                self.covariance = p;
                self.updates = self.updates.saturating_add(1);
                return Ok(());
            }
            self.converged = false;
            self.errors = 0;
        } else {
            if self.converged && self.chi > 0.1 * c.chi_square_threshold {
                gain = ((c.chi_square_threshold - self.chi) / (0.9 * c.chi_square_threshold))
                    .clamp(0.0, 1.0);
            }
            self.errors = 0;
        }
        let mut k = (p * h.transpose() * inverse) * gain;
        for i in 4..6 {
            let scale = libm::acosf(g[i - 4].abs().clamp(0.0, 1.0)) / core::f32::consts::FRAC_PI_2;
            for j in 0..3 {
                k[i][j] *= scale;
            }
        }
        // Retain the reference yaw-channel suppression; gravity cannot observe yaw.
        for j in 0..3 {
            k[3][j] = 0.0;
        }
        let mut correction = k * residual;
        if self.converged {
            for i in 4..6 {
                correction[i][0] = correction[i][0].clamp(-0.01 * dt, 0.01 * dt);
            }
        }
        let mut state = prior + correction;
        let normalized = Quaternion::from_matrix(state.block::<4, 1>(0, 0)?).normalized()?;
        for (i, v) in normalized.as_array().iter().enumerate() {
            state[i][0] = *v;
        }
        let a = Matrix::identity() - k * h;
        let covariance = (a * p * a.transpose() + k * r * k.transpose()).symmetrized();
        if !state.is_finite() || !covariance.is_finite() {
            return Err(Error::NonFinite);
        }
        self.state = state;
        self.covariance = covariance;
        self.updates = self.updates.saturating_add(1);
        Ok(())
    }
}
