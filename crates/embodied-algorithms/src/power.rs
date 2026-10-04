//! Four-motor loss model and RLS identification: P = ωτ + k1|ω| + k2τ² + k3/4.
use crate::{
    Error,
    filter::{Iir, MovingAverage},
    math::finite,
    rls::Rls,
};
#[derive(Clone, Copy, Debug)]
pub struct PowerController {
    pub estimator: Rls<2>,
    speed: [MovingAverage<8>; 4],
    current: [Iir<2>; 4],
    power: Iir<2>,
    ratio: f32,
    limit: f32,
    static_power: f32,
}
impl PowerController {
    pub fn new(
        sample_rate: f32,
        torque_current_ratio: f32,
        max_current: f32,
        static_power: f32,
    ) -> Result<Self, Error> {
        finite(&[torque_current_ratio, max_current, static_power])?;
        if torque_current_ratio <= 0.0 || max_current <= 0.0 || static_power < 0.0 {
            return Err(Error::InvalidParameter);
        }
        let filter = Iir::butterworth(sample_rate, 50.0)?;
        Ok(Self {
            estimator: Rls::new(1e-5, 0.9999)?,
            speed: [MovingAverage::new()?; 4],
            current: [filter; 4],
            power: filter,
            ratio: torque_current_ratio,
            limit: max_current,
            static_power,
        })
    }
    pub fn set_parameters(&mut self, p: [f32; 2]) -> Result<(), Error> {
        finite(&p)?;
        if p.iter().any(|&x| x < 0.0) {
            return Err(Error::InvalidParameter);
        }
        self.estimator.set_parameters(p)
    }
    pub fn reset(&mut self) {
        self.estimator.reset();
        for f in &mut self.speed {
            f.reset();
        }
        for f in &mut self.current {
            f.reset();
        }
        self.power.reset();
    }
    pub fn predict_power(&self, speed: f32, current: f32) -> Result<f32, Error> {
        finite(&[speed, current])?;
        let [k1, k2] = self.estimator.parameters();
        let t = current * self.ratio;
        let p = t * speed + k1 * speed.abs() + k2 * t * t + self.static_power / 4.0;
        finite(&[p])?;
        Ok(p)
    }
    pub fn current_for_power(&self, power: f32, speed: f32, old: f32) -> Result<f32, Error> {
        finite(&[power, speed, old])?;
        let [k1, k2] = self.estimator.parameters();
        let a = k2 * self.ratio * self.ratio;
        let b = speed * self.ratio;
        let c = k1 * speed.abs() + self.static_power / 4.0 - power;
        let mut current = if a.abs() < 1e-6 {
            if b.abs() > 1e-6 { -c / b } else { 0.0 }
        } else {
            let delta = b * b - 4.0 * a * c;
            if delta < 0.0 {
                old
            } else {
                let root = libm::sqrtf(delta);
                // Rationalized roots avoid cancellation for small losses.
                let plus = if b >= 0.0 {
                    -2.0 * c / (b + root)
                } else {
                    (-b + root) / (2.0 * a)
                };
                let minus = if b <= 0.0 {
                    -2.0 * c / (b - root)
                } else {
                    (-b - root) / (2.0 * a)
                };
                let plus_valid = plus.is_finite() && plus * old >= 0.0;
                let minus_valid = minus.is_finite() && minus * old >= 0.0;
                match (plus_valid, minus_valid) {
                    // Regeneration can put both roots on the same side of zero.
                    // Preserve continuity instead of jumping to the distant root.
                    (true, true) => {
                        if (plus - old).abs() <= (minus - old).abs() {
                            plus
                        } else {
                            minus
                        }
                    }
                    (true, false) => plus,
                    (false, true) => minus,
                    (false, false) => old * 0.5,
                }
            }
        };
        if a != 0.0 && b == 0.0 && c == 0.0 {
            current = 0.0;
        }
        finite(&[current])?;
        Ok(current.clamp(-self.limit, self.limit))
    }
    /// Filters and model updates are committed only if the complete operation succeeds.
    pub fn update(
        &mut self,
        target: f32,
        command: &mut [f32; 4],
        speed: [f32; 4],
        measured_power: f32,
        measured_current: [f32; 4],
        identify: bool,
    ) -> Result<(), Error> {
        finite(&[target, measured_power])?;
        finite(command)?;
        finite(&speed)?;
        finite(&measured_current)?;
        if target < 0.0 {
            return Err(Error::InvalidParameter);
        }
        let mut next = *self;
        let mut out = *command;
        next.step(
            target,
            &mut out,
            speed,
            measured_power,
            measured_current,
            identify,
        )?;
        *self = next;
        *command = out;
        Ok(())
    }
    fn step(
        &mut self,
        target: f32,
        command: &mut [f32; 4],
        speed: [f32; 4],
        measured_power: f32,
        measured_current: [f32; 4],
        identify: bool,
    ) -> Result<(), Error> {
        let mut filtered = [0.0; 4];
        let mut predictions = [0.0; 4];
        for i in 0..4 {
            filtered[i] = self.speed[i].update(speed[i])?;
            command[i] = command[i].clamp(-self.limit, self.limit);
            predictions[i] = self.predict_power(filtered[i], command[i])?;
        }
        let sum: f32 = predictions.iter().sum();
        finite(&[sum])?;
        if sum > target {
            for i in 0..4 {
                // Total per-wheel target includes its static loss exactly once.
                command[i] =
                    self.current_for_power(target * predictions[i] / sum, filtered[i], command[i])?;
            }
        }
        if !identify {
            return Ok(());
        }
        let mut feature = [0.0; 2];
        let mut mechanical = 0.0;
        for i in 0..4 {
            let torque = self.current[i].update(measured_current[i])? * self.ratio;
            feature[0] += filtered[i].abs();
            feature[1] += torque * torque;
            mechanical += torque * filtered[i];
        }
        if measured_power <= 5.0 || measured_power - mechanical - self.static_power < 0.0 {
            return Ok(());
        }
        let measured = self.power.update(measured_power)?;
        let parameters = self
            .estimator
            .update(feature, measured - mechanical - self.static_power)?
            .map(|x| x.max(1e-5));
        self.estimator.set_parameters(parameters)?;
        Ok(())
    }
}
