//! Sampled logistic recurrence from SigmoidIter; deterministic explicit elapsed seconds.
use crate::{
    Error,
    math::{finite, positive_dt},
};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SigmoidState {
    Start,
    Iterating,
    Finished,
}
#[derive(Clone, Copy, Debug)]
pub struct Sigmoid {
    stride: f32,
    elapsed: f32,
    start: f32,
    target: f32,
    fraction: f32,
    k: f32,
    output: f32,
    state: SigmoidState,
}
impl Sigmoid {
    pub fn new(frequency: f32) -> Result<Self, Error> {
        positive_dt(frequency)?;
        Ok(Self {
            stride: 1.0 / frequency,
            elapsed: 0.0,
            start: 0.0,
            target: 0.0,
            fraction: 0.0,
            k: 0.0,
            output: 0.0,
            state: SigmoidState::Finished,
        })
    }
    pub fn set(&mut self, start: f32, target: f32, duration: f32) -> Result<(), Error> {
        finite(&[start, target, target - start])?;
        positive_dt(duration)?;
        let k = self.stride / duration * 14.0;
        finite(&[k])?;
        self.start = start;
        self.target = target;
        self.k = k;
        self.fraction = 0.009588;
        self.elapsed = 0.0;
        self.output = start;
        self.state = if start == target {
            SigmoidState::Finished
        } else {
            SigmoidState::Start
        };
        Ok(())
    }
    pub fn set_target(&mut self, target: f32, duration: f32) -> Result<(), Error> {
        self.set(self.output, target, duration)
    }
    pub fn value(&self) -> f32 {
        self.output
    }
    pub fn state(&self) -> SigmoidState {
        self.state
    }
    pub fn advance(&mut self, elapsed: f32) -> Result<f32, Error> {
        finite(&[elapsed])?;
        if elapsed < 0.0 {
            return Err(Error::InvalidParameter);
        }
        self.elapsed += elapsed;
        if self.elapsed >= self.stride && self.state != SigmoidState::Finished {
            // Match source scheduling: one iteration per call; missed ticks are skipped.
            self.elapsed = 0.0;
            match self.state {
                SigmoidState::Start => self.state = SigmoidState::Iterating,
                SigmoidState::Iterating => {
                    let remaining = 1.0 - self.fraction;
                    if self.fraction >= 1.0
                        || (remaining < 0.001
                            && ((self.target - self.start) * remaining).abs() < 0.1)
                    {
                        self.fraction = 1.0;
                        self.state = SigmoidState::Finished;
                    } else {
                        self.fraction =
                            (self.fraction + self.k * self.fraction * remaining).min(1.0);
                    }
                }
                SigmoidState::Finished => {}
            }
            self.output = self.start + (self.target - self.start) * self.fraction;
        }
        Ok(self.output)
    }
}
pub fn sigmoid(value: f32) -> f32 {
    if value >= 0.0 {
        1.0 / (1.0 + libm::expf(-value))
    } else {
        let e = libm::expf(value);
        e / (1.0 + e)
    }
}
