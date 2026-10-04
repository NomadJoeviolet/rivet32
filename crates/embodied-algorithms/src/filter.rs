//! Butterworth low pass (transposed direct form II) and moving mean.
use crate::{Error, math::finite};
#[derive(Clone, Copy, Debug)]
pub struct MovingAverage<const N: usize> {
    buffer: [f32; N],
    index: usize,
    count: usize,
    sum: f32,
}
impl<const N: usize> MovingAverage<N> {
    pub fn new() -> Result<Self, Error> {
        if N == 0 {
            return Err(Error::InvalidParameter);
        }
        Ok(Self {
            buffer: [0.0; N],
            index: 0,
            count: 0,
            sum: 0.0,
        })
    }
    pub fn update(&mut self, value: f32) -> Result<f32, Error> {
        finite(&[value])?;
        let sum = self.sum - self.buffer[self.index] + value;
        finite(&[sum])?;
        self.sum = sum;
        self.buffer[self.index] = value;
        self.index = (self.index + 1) % N;
        self.count = (self.count + 1).min(N);
        Ok(self.value())
    }
    pub fn value(&self) -> f32 {
        if self.count == 0 {
            0.0
        } else {
            self.sum / self.count as f32
        }
    }
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.index = 0;
        self.count = 0;
        self.sum = 0.0;
    }
}

/// `N` is the coefficient count; filter order is `N - 1`.
#[derive(Clone, Copy, Debug)]
pub struct Iir<const N: usize> {
    b: [f32; N],
    a: [f32; N],
    state: [f32; N],
    output: f32,
}
impl<const N: usize> Iir<N> {
    pub fn from_coefficients(mut b: [f32; N], mut a: [f32; N]) -> Result<Self, Error> {
        if N < 2 {
            return Err(Error::InvalidParameter);
        }
        finite(&a)?;
        finite(&b)?;
        if a[0] == 0.0 {
            return Err(Error::InvalidParameter);
        }
        let a0 = a[0];
        for v in &mut a {
            *v /= a0;
        }
        for v in &mut b {
            *v /= a0;
        }
        finite(&a)?;
        finite(&b)?;
        Ok(Self {
            a,
            b,
            state: [0.0; N],
            output: 0.0,
        })
    }
    /// Bilinear transform of analog Butterworth poles, calculated in f64.
    /// Single-precision direct forms are recommended only through order four.
    pub fn butterworth(sample_rate: f32, cutoff: f32) -> Result<Self, Error> {
        finite(&[sample_rate, cutoff])?;
        if N < 2 || sample_rate <= 0.0 || cutoff <= 0.0 || cutoff >= sample_rate * 0.5 {
            return Err(Error::InvalidParameter);
        }
        let order = N - 1;
        let k = libm::tan(core::f64::consts::PI * cutoff as f64 / sample_rate as f64);
        let mut a = [(0.0_f64, 0.0_f64); N];
        a[0] = (1.0, 0.0);
        for m in 0..order {
            let theta = core::f64::consts::PI * (2 * m + 1 + order) as f64 / (2 * order) as f64;
            let pr = k * libm::cos(theta);
            let pi = k * libm::sin(theta);
            let denom = (1.0 - pr) * (1.0 - pr) + pi * pi;
            let zr = (1.0 - pr * pr - pi * pi) / denom;
            let zi = 2.0 * pi / denom;
            for r in (1..=m + 1).rev() {
                let prev = a[r - 1];
                a[r].0 -= zr * prev.0 - zi * prev.1;
                a[r].1 -= zr * prev.1 + zi * prev.0;
            }
        }
        let mut b = [0.0_f64; N];
        b[0] = 1.0;
        for i in 1..N {
            b[i] = b[i - 1] * (N - i) as f64 / i as f64;
        }
        let gain = a.iter().map(|x| x.0).sum::<f64>() / b.iter().sum::<f64>();
        Self::from_coefficients(
            core::array::from_fn(|i| (b[i] * gain) as f32),
            core::array::from_fn(|i| a[i].0 as f32),
        )
    }
    pub fn coefficients(&self) -> (&[f32; N], &[f32; N]) {
        (&self.b, &self.a)
    }
    pub fn update(&mut self, input: f32) -> Result<f32, Error> {
        finite(&[input])?;
        let y = self.b[0] * input + self.state[0];
        let mut s = self.state;
        for i in 0..N - 2 {
            s[i] = s[i + 1] + self.b[i + 1] * input - self.a[i + 1] * y;
        }
        s[N - 2] = self.b[N - 1] * input - self.a[N - 1] * y;
        finite(&s)?;
        finite(&[y])?;
        self.state = s;
        self.output = y;
        Ok(y)
    }
    pub fn reset(&mut self) {
        self.state.fill(0.0);
        self.output = 0.0;
    }
    pub fn reset_to(&mut self, output: f32) -> Result<(), Error> {
        finite(&[output])?;
        let mut s = [0.0; N];
        for i in (0..N - 1).rev() {
            s[i] = s[i + 1] + (self.b[i + 1] - self.a[i + 1]) * output;
        }
        finite(&s)?;
        self.state = s;
        self.output = output;
        Ok(())
    }
    pub fn value(&self) -> f32 {
        self.output
    }
}
