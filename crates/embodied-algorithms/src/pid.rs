//! Composable PID integrators/differentiators and seven-set fuzzy gain scheduling.
use crate::{
    Error,
    filter::Iir,
    math::{finite, positive_dt},
};
pub trait Integrator {
    fn update(&mut self, reference: f32, feedback: f32, ki: f32, dt: f32) -> f32;
    fn reset(&mut self);
}
pub trait Differentiator {
    fn update(&mut self, reference: f32, feedback: f32, kd: f32, dt: f32) -> f32;
    fn reset(&mut self);
}
#[derive(Clone, Copy, Debug)]
pub struct StandardI {
    sum: f32,
    limit: f32,
    last_error: f32,
}
impl StandardI {
    pub fn new(limit: f32) -> Result<Self, Error> {
        finite(&[limit])?;
        if limit < 0.0 {
            return Err(Error::InvalidParameter);
        }
        Ok(Self {
            sum: 0.0,
            limit,
            last_error: 0.0,
        })
    }
    pub fn value(&self) -> f32 {
        self.sum
    }
}
impl Integrator for StandardI {
    fn update(&mut self, r: f32, f: f32, ki: f32, dt: f32) -> f32 {
        let e = r - f;
        self.sum =
            (self.sum + ki * 0.5 * (e + self.last_error) * dt).clamp(-self.limit, self.limit);
        self.last_error = e;
        self.sum
    }
    fn reset(&mut self) {
        self.sum = 0.0;
        self.last_error = 0.0;
    }
}
pub struct VariableI<F> {
    sum: f32,
    limit: f32,
    scale: F,
}
impl<F: FnMut(f32) -> f32> VariableI<F> {
    pub fn new(limit: f32, scale: F) -> Result<Self, Error> {
        finite(&[limit])?;
        if limit < 0.0 {
            return Err(Error::InvalidParameter);
        }
        Ok(Self {
            sum: 0.0,
            limit,
            scale,
        })
    }
}
impl<F: FnMut(f32) -> f32> Integrator for VariableI<F> {
    fn update(&mut self, r: f32, f: f32, ki: f32, dt: f32) -> f32 {
        let e = r - f;
        let mut g = (self.scale)(e);
        if !g.is_finite() {
            g = 0.0;
        }
        self.sum = (self.sum + ki * g * e * dt).clamp(-self.limit, self.limit);
        self.sum
    }
    fn reset(&mut self) {
        self.sum = 0.0;
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct StandardD {
    last_error: f32,
}
impl Differentiator for StandardD {
    fn update(&mut self, r: f32, f: f32, kd: f32, dt: f32) -> f32 {
        let e = r - f;
        let d = kd * (e - self.last_error) / dt;
        self.last_error = e;
        d
    }
    fn reset(&mut self) {
        self.last_error = 0.0;
    }
}
#[derive(Clone, Copy, Debug)]
pub struct FilteredD<const N: usize> {
    filter: Iir<N>,
    previous: Option<f32>,
}
impl<const N: usize> FilteredD<N> {
    pub fn new(filter: Iir<N>) -> Self {
        Self {
            filter,
            previous: None,
        }
    }
}
impl<const N: usize> Differentiator for FilteredD<N> {
    fn update(&mut self, r: f32, f: f32, kd: f32, dt: f32) -> f32 {
        let Ok(e) = self.filter.update(r - f) else {
            return f32::NAN;
        };
        let d = self.previous.map_or(0.0, |p| kd * (e - p) / dt);
        self.previous = Some(e);
        d
    }
    fn reset(&mut self) {
        self.filter.reset();
        self.previous = None;
    }
}
#[derive(Clone, Copy, Debug)]
pub struct OnMeasurement<D>(pub D);
impl<D: Differentiator> Differentiator for OnMeasurement<D> {
    fn update(&mut self, _: f32, f: f32, kd: f32, dt: f32) -> f32 {
        self.0.update(0.0, f, kd, dt)
    }
    fn reset(&mut self) {
        self.0.reset();
    }
}
pub struct Pid<I = StandardI, D = StandardD> {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub output_limit: f32,
    pub integrator: I,
    pub differentiator: D,
    output: f32,
}
impl<I: Integrator, D: Differentiator> Pid<I, D> {
    pub fn new(
        kp: f32,
        ki: f32,
        kd: f32,
        output_limit: f32,
        integrator: I,
        differentiator: D,
    ) -> Result<Self, Error> {
        finite(&[kp, ki, kd, output_limit])?;
        if output_limit < 0.0 {
            return Err(Error::InvalidParameter);
        }
        Ok(Self {
            kp,
            ki,
            kd,
            output_limit,
            integrator,
            differentiator,
            output: 0.0,
        })
    }
    /// Explicit time step; unlike wall-clock wrappers there is no hidden minimum dt.
    pub fn update(&mut self, reference: f32, feedback: f32, dt: f32) -> Result<f32, Error> {
        finite(&[
            reference,
            feedback,
            self.kp,
            self.ki,
            self.kd,
            self.output_limit,
            reference - feedback,
        ])?;
        positive_dt(dt)?;
        if self.output_limit < 0.0 {
            return Err(Error::InvalidParameter);
        }
        let y = self.kp * (reference - feedback)
            + self.integrator.update(reference, feedback, self.ki, dt)
            + self.differentiator.update(reference, feedback, self.kd, dt);
        finite(&[y])?;
        self.output = y.clamp(-self.output_limit, self.output_limit);
        Ok(self.output)
    }
    pub fn reset(&mut self) {
        self.integrator.reset();
        self.differentiator.reset();
        self.output = 0.0;
    }
    pub fn value(&self) -> f32 {
        self.output
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FuzzyTable {
    rules: [[u8; 7]; 7],
    values: [f32; 7],
    error_range: [f32; 2],
    rate_range: [f32; 2],
}
impl FuzzyTable {
    pub fn new(
        rules: [[u8; 7]; 7],
        values: [f32; 7],
        error_range: [f32; 2],
        rate_range: [f32; 2],
    ) -> Result<Self, Error> {
        finite(&values)?;
        finite(&error_range)?;
        finite(&rate_range)?;
        if error_range[0] >= error_range[1]
            || rate_range[0] >= rate_range[1]
            || rules.iter().flatten().any(|&x| x > 6)
        {
            return Err(Error::InvalidParameter);
        }
        Ok(Self {
            rules,
            values,
            error_range,
            rate_range,
        })
    }
    fn membership(x: f32, range: [f32; 2]) -> (usize, usize, f32) {
        let x = (6.0 * (x - range[0]) / (range[1] - range[0])).clamp(0.0, 6.0);
        let i = x as usize;
        (i, (i + 1).min(6), x - i as f32)
    }
    pub fn lookup(&self, error: f32, rate: f32) -> Result<f32, Error> {
        finite(&[error, rate])?;
        let (a, b, u) = Self::membership(error, self.error_range);
        let (c, d, v) = Self::membership(rate, self.rate_range);
        Ok(
            self.values[self.rules[a][c] as usize] * (1.0 - u) * (1.0 - v)
                + self.values[self.rules[a][d] as usize] * (1.0 - u) * v
                + self.values[self.rules[b][c] as usize] * u * (1.0 - v)
                + self.values[self.rules[b][d] as usize] * u * v,
        )
    }
}
pub struct FuzzyPid<I = StandardI, D = StandardD> {
    pub controller: Pid<I, D>,
    tables: [FuzzyTable; 3],
    last_error: f32,
}
impl<I: Integrator, D: Differentiator> FuzzyPid<I, D> {
    pub fn new(controller: Pid<I, D>, tables: [FuzzyTable; 3]) -> Self {
        Self {
            controller,
            tables,
            last_error: 0.0,
        }
    }
    pub fn update(&mut self, reference: f32, feedback: f32, dt: f32) -> Result<f32, Error> {
        positive_dt(dt)?;
        let error = reference - feedback;
        let rate = (error - self.last_error) / dt;
        self.controller.kp = self.tables[0].lookup(error, rate)?;
        self.controller.ki = self.tables[1].lookup(error, rate)?;
        self.controller.kd = self.tables[2].lookup(error, rate)?;
        let y = self.controller.update(reference, feedback, dt)?;
        self.last_error = error;
        Ok(y)
    }
    pub fn reset(&mut self) {
        self.controller.reset();
        self.last_error = 0.0;
    }
}
