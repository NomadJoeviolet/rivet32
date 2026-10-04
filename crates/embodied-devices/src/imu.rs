//! HAL-independent BMI088 and ICM42688 register drivers. One SPI transaction
//! asserts the device's CS for all bytes and deasserts it on success AND error.
//! The adapter owns bus locking, mode/frequency selection and transfer timeout.
//! Initialization delays are explicit; periodic sampling is scheduled by the caller.
use crate::{Connection, Error, Freshness, be16, finite, le16};
pub trait SpiDevice {
    type Error;
    fn transfer_in_place(&mut self, bytes: &mut [u8]) -> Result<(), Self::Error>;
}
pub trait DelayUs {
    fn delay_us(&mut self, us: u32);
}
/// Async adapters must release CS/bus ownership if their future is cancelled.
/// Implement with the board's Embassy SPI device and an Embassy Timer delay.
#[allow(async_fn_in_trait)]
pub trait AsyncSpiDevice {
    type Error;
    async fn transfer_in_place(&mut self, bytes: &mut [u8]) -> Result<(), Self::Error>;
}
#[allow(async_fn_in_trait)]
pub trait AsyncDelayUs {
    async fn delay_us(&mut self, us: u32);
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IoError<E> {
    Bus(E),
    Protocol(Error),
    Identity {
        expected: u8,
        actual: u8,
    },
    Register {
        address: u8,
        expected: u8,
        actual: u8,
    },
    SelfTest,
    NotInitialized,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sample {
    pub acceleration: [f32; 3],
    pub angular_velocity: [f32; 3],
    pub temperature_c: f32,
    pub timestamp_us: u64,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Calibration {
    pub gyro_bias: [f32; 3],
    pub acceleration_bias: [f32; 3],
    pub acceleration_scale: f32,
    pub mounting: [[f32; 3]; 3],
}
impl Default for Calibration {
    fn default() -> Self {
        Self {
            gyro_bias: [0.0; 3],
            acceleration_bias: [0.0; 3],
            acceleration_scale: 1.0,
            mounting: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }
}
impl Calibration {
    pub fn validate(&self) -> Result<(), Error> {
        finite(&self.gyro_bias)?;
        finite(&self.acceleration_bias)?;
        finite(&[self.acceleration_scale])?;
        if self.acceleration_scale <= 0.0 {
            return Err(Error::Range);
        }
        for row in &self.mounting {
            finite(row)?;
        }
        Ok(())
    }
    pub fn apply(&self, mut sample: Sample) -> Sample {
        let mut accel = [0.0; 3];
        let mut gyro = [0.0; 3];
        for i in 0..3 {
            for j in 0..3 {
                accel[i] += self.mounting[i][j]
                    * (sample.acceleration[j] - self.acceleration_bias[j])
                    * self.acceleration_scale;
                gyro[i] += self.mounting[i][j] * (sample.angular_velocity[j] - self.gyro_bias[j]);
            }
        }
        sample.acceleration = accel;
        sample.angular_velocity = gyro;
        sample
    }
    /// Compose yaw-pitch-roll in radians into the sensor-to-body mounting matrix.
    pub fn set_mounting_euler(&mut self, roll: f32, pitch: f32, yaw: f32) -> Result<(), Error> {
        finite(&[roll, pitch, yaw])?;
        let (sr, cr) = (libm::sinf(roll), libm::cosf(roll));
        let (sp, cp) = (libm::sinf(pitch), libm::cosf(pitch));
        let (sy, cy) = (libm::sinf(yaw), libm::cosf(yaw));
        self.mounting = [
            [cy * cp, cy * sp * sr - sy * cr, cy * sp * cr + sy * sr],
            [sy * cp, sy * sp * sr + cy * cr, sy * sp * cr - cy * sr],
            [-sp, cp * sr, cp * cr],
        ];
        Ok(())
    }
}

/// Incremental stationary calibration; does not spin or hide a 100-second loop.
/// A motion rejection is sticky until reset. Only a complete stable window can finish.
pub struct StationaryCalibrator {
    required: u32,
    count: u32,
    gyro_sum: [f64; 3],
    norm_sum: f64,
    gyro_min: [f32; 3],
    gyro_max: [f32; 3],
    norm_min: f32,
    norm_max: f32,
    gyro_span: f32,
    norm_span: f32,
    rejected: bool,
    freshness: Freshness,
}
impl StationaryCalibrator {
    pub fn new(required: u32, gyro_span: f32, norm_span: f32) -> Result<Self, Error> {
        finite(&[gyro_span, norm_span])?;
        if required < 2 || gyro_span <= 0.0 || norm_span <= 0.0 {
            return Err(Error::Range);
        }
        Ok(Self {
            required,
            count: 0,
            gyro_sum: [0.0; 3],
            norm_sum: 0.0,
            gyro_min: [f32::INFINITY; 3],
            gyro_max: [f32::NEG_INFINITY; 3],
            norm_min: f32::INFINITY,
            norm_max: f32::NEG_INFINITY,
            gyro_span,
            norm_span,
            rejected: false,
            freshness: Freshness::new(),
        })
    }
    pub fn push(&mut self, s: Sample) -> Result<(), Error> {
        if self.rejected {
            return Err(Error::Range);
        }
        if self.count >= self.required {
            return Err(Error::Capacity);
        }
        finite(&s.acceleration)?;
        finite(&s.angular_velocity)?;
        self.freshness.validate(s.timestamp_us)?;
        let norm = libm::sqrtf(s.acceleration.iter().map(|x| x * x).sum());
        finite(&[norm])?;
        let norm_min = self.norm_min.min(norm);
        let norm_max = self.norm_max.max(norm);
        let mut min = self.gyro_min;
        let mut max = self.gyro_max;
        for i in 0..3 {
            min[i] = min[i].min(s.angular_velocity[i]);
            max[i] = max[i].max(s.angular_velocity[i]);
        }
        if norm_max - norm_min > self.norm_span || (0..3).any(|i| max[i] - min[i] > self.gyro_span)
        {
            self.rejected = true;
            return Err(Error::Range);
        }
        self.norm_min = norm_min;
        self.norm_max = norm_max;
        self.gyro_min = min;
        self.gyro_max = max;
        for i in 0..3 {
            self.gyro_sum[i] += s.angular_velocity[i] as f64;
        }
        self.norm_sum += norm as f64;
        self.count += 1;
        self.freshness.update(s.timestamp_us)
    }
    pub fn finish(&self) -> Result<Calibration, Error> {
        if self.rejected {
            return Err(Error::Range);
        }
        if self.count != self.required {
            return Err(Error::Length);
        }
        let norm = (self.norm_sum / self.count as f64) as f32;
        if !(8.80665..=10.80665).contains(&norm) {
            return Err(Error::Range);
        }
        let mut c = Calibration::default();
        for i in 0..3 {
            c.gyro_bias[i] = (self.gyro_sum[i] / self.count as f64) as f32;
        }
        c.acceleration_scale = 9.80665 / norm;
        Ok(c)
    }
    pub const fn count(&self) -> u32 {
        self.count
    }
    pub fn reset(&mut self) {
        self.count = 0;
        self.gyro_sum = [0.0; 3];
        self.norm_sum = 0.0;
        self.gyro_min = [f32::INFINITY; 3];
        self.gyro_max = [f32::NEG_INFINITY; 3];
        self.norm_min = f32::INFINITY;
        self.norm_max = f32::NEG_INFINITY;
        self.rejected = false;
        self.freshness = Freshness::new();
    }
}

fn write<S: SpiDevice>(spi: &mut S, reg: u8, value: u8) -> Result<(), IoError<S::Error>> {
    let mut d = [reg & 0x7f, value];
    spi.transfer_in_place(&mut d).map_err(IoError::Bus)
}
fn read<S: SpiDevice>(
    spi: &mut S,
    reg: u8,
    dummy: usize,
    out: &mut [u8],
) -> Result<(), IoError<S::Error>> {
    if out.is_empty() || out.len() > 14 || dummy > 1 || reg > 0x7f || reg as usize + out.len() > 128
    {
        return Err(IoError::Protocol(Error::Length));
    }
    let mut bytes = [0; 16];
    bytes[0] = reg | 0x80;
    let n = out.len() + 1 + dummy;
    spi.transfer_in_place(&mut bytes[..n])
        .map_err(IoError::Bus)?;
    out.copy_from_slice(&bytes[1 + dummy..n]);
    Ok(())
}
fn register<S: SpiDevice>(spi: &mut S, reg: u8, dummy: usize) -> Result<u8, IoError<S::Error>> {
    let mut d = [0];
    read(spi, reg, dummy, &mut d)?;
    Ok(d[0])
}
fn verified_write<S: SpiDevice, D: DelayUs>(
    spi: &mut S,
    delay: &mut D,
    reg: u8,
    value: u8,
    dummy: usize,
) -> Result<(), IoError<S::Error>> {
    write(spi, reg, value)?;
    delay.delay_us(450);
    let actual = register(spi, reg, dummy)?;
    if actual != value {
        return Err(IoError::Register {
            address: reg,
            expected: value,
            actual,
        });
    }
    Ok(())
}

async fn write_async<S: AsyncSpiDevice>(
    spi: &mut S,
    reg: u8,
    value: u8,
) -> Result<(), IoError<S::Error>> {
    let mut d = [reg & 0x7f, value];
    spi.transfer_in_place(&mut d).await.map_err(IoError::Bus)
}
async fn read_async<S: AsyncSpiDevice>(
    spi: &mut S,
    reg: u8,
    dummy: usize,
    out: &mut [u8],
) -> Result<(), IoError<S::Error>> {
    if out.is_empty() || out.len() > 14 || dummy > 1 || reg > 0x7f || reg as usize + out.len() > 128
    {
        return Err(IoError::Protocol(Error::Length));
    }
    let mut bytes = [0; 16];
    bytes[0] = reg | 0x80;
    let n = out.len() + 1 + dummy;
    spi.transfer_in_place(&mut bytes[..n])
        .await
        .map_err(IoError::Bus)?;
    out.copy_from_slice(&bytes[1 + dummy..n]);
    Ok(())
}
async fn register_async<S: AsyncSpiDevice>(
    spi: &mut S,
    reg: u8,
    dummy: usize,
) -> Result<u8, IoError<S::Error>> {
    let mut d = [0];
    read_async(spi, reg, dummy, &mut d).await?;
    Ok(d[0])
}
async fn verified_write_async<S: AsyncSpiDevice, D: AsyncDelayUs>(
    spi: &mut S,
    delay: &mut D,
    reg: u8,
    value: u8,
    dummy: usize,
) -> Result<(), IoError<S::Error>> {
    write_async(spi, reg, value).await?;
    delay.delay_us(450).await;
    let actual = register_async(spi, reg, dummy).await?;
    if actual != value {
        return Err(IoError::Register {
            address: reg,
            expected: value,
            actual,
        });
    }
    Ok(())
}

impl Bmi088 {
    /// Cancellation leaves the driver uninitialized until a full retry succeeds.
    pub async fn initialize_async<
        A: AsyncSpiDevice,
        G: AsyncSpiDevice<Error = A::Error>,
        D: AsyncDelayUs,
    >(
        &mut self,
        accel: &mut A,
        gyro: &mut G,
        delay: &mut D,
    ) -> Result<(), IoError<A::Error>> {
        self.initialized = false;
        self.sample = None;
        self.freshness = Freshness::new();
        let _ = register_async(accel, 0, 1).await?;
        write_async(accel, 0x7e, 0xb6).await?;
        write_async(gyro, 0x14, 0xb6).await?;
        delay.delay_us(80_000).await;
        let _ = register_async(accel, 0, 1).await?;
        delay.delay_us(1000).await;
        let actual = register_async(accel, 0, 1).await?;
        if actual != 0x1e {
            return Err(IoError::Identity {
                expected: 0x1e,
                actual,
            });
        }
        let actual = register_async(gyro, 0, 0).await?;
        if actual != 0x0f {
            return Err(IoError::Identity {
                expected: 0x0f,
                actual,
            });
        }
        for (reg, value) in [
            (0x7d, 4),
            (0x7c, 0),
            (0x40, 0xa0 | self.config.acceleration_odr),
            (0x41, self.config.acceleration_range as u8),
            (0x53, 8),
            (0x58, 4),
        ] {
            verified_write_async(accel, delay, reg, value, 1).await?;
        }
        for (reg, value) in [
            (0x0f, self.config.gyro_range as u8),
            (0x10, 0x80 | self.config.gyro_bandwidth),
            (0x11, 0),
            (0x15, 0x80),
            (0x16, 0),
            (0x18, 1),
        ] {
            verified_write_async(gyro, delay, reg, value, 0).await?;
        }
        delay.delay_us(80_000).await;
        self.initialized = true;
        Ok(())
    }
    /// A cancelled/failed transfer never publishes a partially updated sample.
    pub async fn read_async<A: AsyncSpiDevice, G: AsyncSpiDevice<Error = A::Error>>(
        &mut self,
        accel: &mut A,
        gyro: &mut G,
        now_us: u64,
    ) -> Result<Sample, IoError<A::Error>> {
        if !self.initialized {
            return Err(IoError::NotInitialized);
        }
        self.freshness.validate(now_us).map_err(IoError::Protocol)?;
        let mut a = [0; 6];
        let mut g = [0; 6];
        let mut t = [0; 2];
        read_async(accel, 0x12, 1, &mut a).await?;
        read_async(gyro, 0x02, 0, &mut g).await?;
        read_async(accel, 0x22, 1, &mut t).await?;
        let sample = self
            .calibration
            .apply(Self::decode_raw(&a, &g, &t, self.config, now_us).map_err(IoError::Protocol)?);
        self.freshness.update(now_us).map_err(IoError::Protocol)?;
        self.sample = Some(sample);
        Ok(sample)
    }
}
impl Icm42688 {
    pub async fn initialize_async<S: AsyncSpiDevice, D: AsyncDelayUs>(
        &mut self,
        spi: &mut S,
        delay: &mut D,
    ) -> Result<(), IoError<S::Error>> {
        self.initialized = false;
        self.sample = None;
        self.freshness = Freshness::new();
        write_async(spi, 0x76, 0).await?;
        write_async(spi, 0x11, 1).await?;
        delay.delay_us(1000).await;
        let actual = register_async(spi, 0x75, 0).await?;
        if actual != 0x47 {
            return Err(IoError::Identity {
                expected: 0x47,
                actual,
            });
        }
        verified_write_async(
            spi,
            delay,
            0x4f,
            (self.config.gyro_range as u8) << 5 | self.config.odr,
            0,
        )
        .await?;
        verified_write_async(
            spi,
            delay,
            0x50,
            (self.config.acceleration_range as u8) << 5 | self.config.odr,
            0,
        )
        .await?;
        verified_write_async(spi, delay, 0x4e, 0x0f, 0).await?;
        delay.delay_us(45_000).await;
        self.initialized = true;
        Ok(())
    }
    pub async fn read_async<S: AsyncSpiDevice>(
        &mut self,
        spi: &mut S,
        now_us: u64,
    ) -> Result<Sample, IoError<S::Error>> {
        if !self.initialized {
            return Err(IoError::NotInitialized);
        }
        self.freshness.validate(now_us).map_err(IoError::Protocol)?;
        let mut bytes = [0; 14];
        read_async(spi, 0x1d, 0, &mut bytes).await?;
        let sample = self
            .calibration
            .apply(Self::decode_raw(&bytes, self.config, now_us).map_err(IoError::Protocol)?);
        self.freshness.update(now_us).map_err(IoError::Protocol)?;
        self.sample = Some(sample);
        Ok(sample)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BmiAccelRange {
    G3 = 0,
    G6 = 1,
    G12 = 2,
    G24 = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BmiGyroRange {
    Dps2000 = 0,
    Dps1000 = 1,
    Dps500 = 2,
    Dps250 = 3,
    Dps125 = 4,
}
#[derive(Clone, Copy, Debug)]
pub struct BmiConfig {
    pub acceleration_range: BmiAccelRange,
    pub gyro_range: BmiGyroRange,
    pub acceleration_odr: u8,
    pub gyro_bandwidth: u8,
}
impl Default for BmiConfig {
    fn default() -> Self {
        Self {
            acceleration_range: BmiAccelRange::G6,
            gyro_range: BmiGyroRange::Dps2000,
            acceleration_odr: 0x0b,
            gyro_bandwidth: 2,
        }
    }
}
impl BmiConfig {
    fn validate(self) -> Result<(), Error> {
        if !(5..=12).contains(&self.acceleration_odr) || self.gyro_bandwidth > 7 {
            Err(Error::Range)
        } else {
            Ok(())
        }
    }
    fn accel_scale(self) -> f32 {
        (3u32 << (self.acceleration_range as u8)) as f32 * 9.8 / 32768.0
    }
    fn gyro_scale(self) -> f32 {
        2000.0 / (1u32 << (self.gyro_range as u8)) as f32 * core::f32::consts::PI / 180.0 / 32768.0
    }
}
pub struct Bmi088 {
    config: BmiConfig,
    calibration: Calibration,
    initialized: bool,
    sample: Option<Sample>,
    freshness: Freshness,
}
impl Bmi088 {
    pub fn new(config: BmiConfig) -> Result<Self, Error> {
        config.validate()?;
        Ok(Self {
            config,
            calibration: Calibration::default(),
            initialized: false,
            sample: None,
            freshness: Freshness::new(),
        })
    }
    pub fn set_calibration(&mut self, calibration: Calibration) -> Result<(), Error> {
        calibration.validate()?;
        self.calibration = calibration;
        Ok(())
    }
    pub fn initialize<A: SpiDevice, G: SpiDevice<Error = A::Error>, D: DelayUs>(
        &mut self,
        accel: &mut A,
        gyro: &mut G,
        delay: &mut D,
    ) -> Result<(), IoError<A::Error>> {
        self.initialized = false;
        self.sample = None;
        self.freshness = Freshness::new();
        // An initial CS edge switches the accelerometer from I2C to SPI.
        let _ = register(accel, 0, 1)?;
        write(accel, 0x7e, 0xb6)?;
        write(gyro, 0x14, 0xb6)?;
        delay.delay_us(80_000);
        let _ = register(accel, 0, 1)?;
        delay.delay_us(1000);
        let actual = register(accel, 0, 1)?;
        if actual != 0x1e {
            return Err(IoError::Identity {
                expected: 0x1e,
                actual,
            });
        }
        let actual = register(gyro, 0, 0)?;
        if actual != 0x0f {
            return Err(IoError::Identity {
                expected: 0x0f,
                actual,
            });
        }
        for (reg, value) in [
            (0x7d, 4),
            (0x7c, 0),
            (0x40, 0xa0 | self.config.acceleration_odr),
            (0x41, self.config.acceleration_range as u8),
            (0x53, 8),
            (0x58, 4),
        ] {
            verified_write(accel, delay, reg, value, 1)?;
        }
        for (reg, value) in [
            (0x0f, self.config.gyro_range as u8),
            (0x10, 0x80 | self.config.gyro_bandwidth),
            (0x11, 0),
            (0x15, 0x80),
            (0x16, 0),
            (0x18, 1),
        ] {
            verified_write(gyro, delay, reg, value, 0)?;
        }
        delay.delay_us(80_000);
        self.initialized = true;
        Ok(())
    }
    pub fn read<A: SpiDevice, G: SpiDevice<Error = A::Error>>(
        &mut self,
        accel: &mut A,
        gyro: &mut G,
        now_us: u64,
    ) -> Result<Sample, IoError<A::Error>> {
        if !self.initialized {
            return Err(IoError::NotInitialized);
        }
        self.freshness.validate(now_us).map_err(IoError::Protocol)?;
        let mut a = [0; 6];
        let mut g = [0; 6];
        let mut t = [0; 2];
        read(accel, 0x12, 1, &mut a)?;
        read(gyro, 0x02, 0, &mut g)?;
        read(accel, 0x22, 1, &mut t)?;
        let raw = Self::decode_raw(&a, &g, &t, self.config, now_us).map_err(IoError::Protocol)?;
        let sample = self.calibration.apply(raw);
        self.freshness.update(now_us).map_err(IoError::Protocol)?;
        self.sample = Some(sample);
        Ok(sample)
    }
    pub fn decode_raw(
        a: &[u8],
        g: &[u8],
        t: &[u8],
        config: BmiConfig,
        now_us: u64,
    ) -> Result<Sample, Error> {
        crate::exact(a, 6)?;
        crate::exact(g, 6)?;
        crate::exact(t, 2)?;
        config.validate()?;
        // BMI088 datasheet rev. 1.9, section 5.3.7: TEMP_MSB 0x80 is
        // invalid regardless of TEMP_LSB. Do not publish it as a temperature.
        if t[0] == 0x80 {
            return Err(Error::Range);
        }
        let mut acceleration = [0.0; 3];
        let mut angular_velocity = [0.0; 3];
        for i in 0..3 {
            acceleration[i] = le16(a, i * 2) as i16 as f32 * config.accel_scale();
            angular_velocity[i] = le16(g, i * 2) as i16 as f32 * config.gyro_scale();
        }
        let raw = ((t[0] as i16) << 3) | ((t[1] >> 5) as i16);
        let signed = if raw >= 1024 { raw - 2048 } else { raw };
        Ok(Sample {
            acceleration,
            angular_velocity,
            temperature_c: signed as f32 * 0.125 + 23.0,
            timestamp_us: now_us,
        })
    }
    pub fn data_ready<A: SpiDevice, G: SpiDevice<Error = A::Error>>(
        &self,
        accel: &mut A,
        gyro: &mut G,
    ) -> Result<(bool, bool), IoError<A::Error>> {
        Ok((
            register(accel, 3, 1)? & 0x80 != 0,
            register(gyro, 0x0a, 0)? & 0x80 != 0,
        ))
    }
    /// Runs accelerometer positive/negative excitation and bounded gyro BIST.
    /// Reinitializes the configured ranges even if self-test fails.
    pub fn self_test<A: SpiDevice, G: SpiDevice<Error = A::Error>, D: DelayUs>(
        &mut self,
        accel: &mut A,
        gyro: &mut G,
        delay: &mut D,
    ) -> Result<(), IoError<A::Error>> {
        self.initialized = false;
        let result = (|| {
            for (reg, value) in [(0x7d, 4), (0x7c, 0), (0x40, 0xac), (0x41, 3)] {
                verified_write(accel, delay, reg, value, 1)?;
            }
            delay.delay_us(50_000);
            let mut positive = [0; 6];
            let mut negative = [0; 6];
            write(accel, 0x6d, 0x0d)?;
            delay.delay_us(50_000);
            read(accel, 0x12, 1, &mut positive)?;
            write(accel, 0x6d, 0x09)?;
            delay.delay_us(50_000);
            read(accel, 0x12, 1, &mut negative)?;
            write(accel, 0x6d, 0)?;
            for (i, threshold) in [1365, 1365, 680].into_iter().enumerate() {
                let delta = (le16(&positive, i * 2) as i16 as i32
                    - le16(&negative, i * 2) as i16 as i32)
                    .abs();
                if delta < threshold {
                    return Err(IoError::SelfTest);
                }
            }
            write(gyro, 0x3c, 1)?;
            for _ in 0..20 {
                delay.delay_us(1000);
                let status = register(gyro, 0x3c, 0)?;
                if status & 2 != 0 {
                    return if status & 4 == 0 {
                        Ok(())
                    } else {
                        Err(IoError::SelfTest)
                    };
                }
            }
            Err(IoError::SelfTest)
        })();
        let restore = self.initialize(accel, gyro, delay);
        result.and(restore)
    }
    pub fn sample(&self) -> Option<&Sample> {
        self.sample.as_ref()
    }
    pub fn connection(&self, now_us: u64, timeout_us: u64) -> Connection {
        self.freshness.connection(now_us, timeout_us)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IcmAccelRange {
    G16 = 0,
    G8 = 1,
    G4 = 2,
    G2 = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IcmGyroRange {
    Dps2000 = 0,
    Dps1000 = 1,
    Dps500 = 2,
    Dps250 = 3,
    Dps125 = 4,
    Dps62_5 = 5,
    Dps31_25 = 6,
    Dps15_625 = 7,
}
#[derive(Clone, Copy, Debug)]
pub struct IcmConfig {
    pub acceleration_range: IcmAccelRange,
    pub gyro_range: IcmGyroRange,
    pub odr: u8,
}
impl Default for IcmConfig {
    fn default() -> Self {
        Self {
            acceleration_range: IcmAccelRange::G16,
            gyro_range: IcmGyroRange::Dps2000,
            odr: 6,
        }
    }
}
impl IcmConfig {
    fn validate(self) -> Result<(), Error> {
        if (1..=11).contains(&self.odr) || self.odr == 15 {
            Ok(())
        } else {
            Err(Error::Range)
        }
    }
}
pub struct Icm42688 {
    config: IcmConfig,
    calibration: Calibration,
    initialized: bool,
    sample: Option<Sample>,
    freshness: Freshness,
}
impl Icm42688 {
    pub fn new(config: IcmConfig) -> Result<Self, Error> {
        config.validate()?;
        Ok(Self {
            config,
            calibration: Calibration::default(),
            initialized: false,
            sample: None,
            freshness: Freshness::new(),
        })
    }
    pub fn set_calibration(&mut self, calibration: Calibration) -> Result<(), Error> {
        calibration.validate()?;
        self.calibration = calibration;
        Ok(())
    }
    pub fn initialize<S: SpiDevice, D: DelayUs>(
        &mut self,
        spi: &mut S,
        delay: &mut D,
    ) -> Result<(), IoError<S::Error>> {
        self.initialized = false;
        self.sample = None;
        self.freshness = Freshness::new();
        write(spi, 0x76, 0)?;
        write(spi, 0x11, 1)?;
        delay.delay_us(1000);
        let actual = register(spi, 0x75, 0)?;
        if actual != 0x47 {
            return Err(IoError::Identity {
                expected: 0x47,
                actual,
            });
        }
        verified_write(
            spi,
            delay,
            0x4f,
            (self.config.gyro_range as u8) << 5 | self.config.odr,
            0,
        )?;
        verified_write(
            spi,
            delay,
            0x50,
            (self.config.acceleration_range as u8) << 5 | self.config.odr,
            0,
        )?;
        verified_write(spi, delay, 0x4e, 0x0f, 0)?;
        delay.delay_us(45_000);
        self.initialized = true;
        Ok(())
    }
    pub fn read<S: SpiDevice>(
        &mut self,
        spi: &mut S,
        now_us: u64,
    ) -> Result<Sample, IoError<S::Error>> {
        if !self.initialized {
            return Err(IoError::NotInitialized);
        }
        self.freshness.validate(now_us).map_err(IoError::Protocol)?;
        let mut bytes = [0; 14];
        read(spi, 0x1d, 0, &mut bytes)?;
        let sample = self
            .calibration
            .apply(Self::decode_raw(&bytes, self.config, now_us).map_err(IoError::Protocol)?);
        self.sample = Some(sample);
        self.freshness.update(now_us).map_err(IoError::Protocol)?;
        Ok(sample)
    }
    pub fn decode_raw(d: &[u8], config: IcmConfig, now_us: u64) -> Result<Sample, Error> {
        crate::exact(d, 14)?;
        config.validate()?;
        let mut a = [0.0; 3];
        let mut g = [0.0; 3];
        let accel_scale =
            16.0 / (1u32 << (config.acceleration_range as u8)) as f32 * 9.80665 / 32768.0;
        let gyro_scale = 2000.0 / (1u32 << (config.gyro_range as u8)) as f32
            * core::f32::consts::PI
            / 180.0
            / 32768.0;
        for i in 0..3 {
            let accel = be16(d, 2 + i * 2) as i16;
            let gyro = be16(d, 8 + i * 2) as i16;
            if accel == i16::MIN || gyro == i16::MIN {
                return Err(Error::Range);
            }
            a[i] = accel as f32 * accel_scale;
            g[i] = gyro as f32 * gyro_scale;
        }
        Ok(Sample {
            acceleration: a,
            angular_velocity: g,
            temperature_c: be16(d, 0) as i16 as f32 / 132.48 + 25.0,
            timestamp_us: now_us,
        })
    }
    pub fn data_ready<S: SpiDevice>(&self, spi: &mut S) -> Result<bool, IoError<S::Error>> {
        Ok(register(spi, 0x2d, 0)? & 8 != 0)
    }
    pub fn sample(&self) -> Option<&Sample> {
        self.sample.as_ref()
    }
    pub fn connection(&self, now_us: u64, timeout_us: u64) -> Connection {
        self.freshness.connection(now_us, timeout_us)
    }
}
