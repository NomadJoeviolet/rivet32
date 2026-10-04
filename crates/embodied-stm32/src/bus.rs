//! Device adapters for embedded-hal buses. HAL instances retain peripheral and pin ownership.

use embodied_devices::imu;

/// Adapts an already configured, synchronous SPI device (including its CS policy).
pub struct SpiDeviceAdapter<D>(pub D);

impl<D: embedded_hal::spi::SpiDevice<u8>> imu::SpiDevice for SpiDeviceAdapter<D> {
    type Error = D::Error;
    fn transfer_in_place(&mut self, bytes: &mut [u8]) -> Result<(), Self::Error> {
        self.0.transfer_in_place(bytes)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpiError<B, C> {
    Bus(B),
    ChipSelect(C),
}

/// Exclusive SPI bus plus active-low CS for IMU drivers.
///
/// The caller configures SPI mode/rate before construction and supplies DMA-visible
/// buffers with the appropriate cache policy. No allocation, bus sharing or hidden
/// timeout is introduced. Dropping an async transfer cancels the HAL bus future
/// before restoring CS high; GPIO errors on cancellation cannot be reported.
pub struct SpiBusDevice<B, C: embedded_hal::digital::OutputPin> {
    bus: B,
    cs: C,
}

impl<B, C: embedded_hal::digital::OutputPin> SpiBusDevice<B, C> {
    pub fn new(bus: B, mut cs: C) -> Result<Self, C::Error> {
        cs.set_high()?;
        Ok(Self { bus, cs })
    }
    pub fn into_parts(self) -> (B, C) {
        (self.bus, self.cs)
    }
}

struct Selected<'a, C: embedded_hal::digital::OutputPin> {
    cs: &'a mut C,
    active: bool,
}

impl<'a, C: embedded_hal::digital::OutputPin> Selected<'a, C> {
    fn new(cs: &'a mut C) -> Result<Self, C::Error> {
        // Install the guard before asserting CS, so even a GPIO error gets a
        // best-effort high transition on drop.
        let selected = Self { cs, active: true };
        selected.cs.set_low()?;
        Ok(selected)
    }
    fn finish(mut self) -> Result<(), C::Error> {
        let result = self.cs.set_high();
        self.active = false;
        result
    }
}

impl<C: embedded_hal::digital::OutputPin> Drop for Selected<'_, C> {
    fn drop(&mut self) {
        if self.active {
            let _ = self.cs.set_high();
        }
    }
}

impl<B: embedded_hal::spi::SpiBus<u8>, C: embedded_hal::digital::OutputPin> imu::SpiDevice
    for SpiBusDevice<B, C>
{
    type Error = SpiError<B::Error, C::Error>;
    fn transfer_in_place(&mut self, bytes: &mut [u8]) -> Result<(), Self::Error> {
        let selected = Selected::new(&mut self.cs).map_err(SpiError::ChipSelect)?;
        let transfer = self.bus.transfer_in_place(bytes).map_err(SpiError::Bus);
        let flush = self.bus.flush().map_err(SpiError::Bus);
        let deselect = selected.finish().map_err(SpiError::ChipSelect);
        transfer.and(flush).and(deselect)
    }
}

impl<B: embedded_hal_async::spi::SpiBus<u8>, C: embedded_hal::digital::OutputPin>
    imu::AsyncSpiDevice for SpiBusDevice<B, C>
{
    type Error = SpiError<B::Error, C::Error>;
    async fn transfer_in_place(&mut self, bytes: &mut [u8]) -> Result<(), Self::Error> {
        let selected = Selected::new(&mut self.cs).map_err(SpiError::ChipSelect)?;
        let transfer = self
            .bus
            .transfer_in_place(bytes)
            .await
            .map_err(SpiError::Bus);
        let flush = self.bus.flush().await.map_err(SpiError::Bus);
        let deselect = selected.finish().map_err(SpiError::ChipSelect);
        transfer.and(flush).and(deselect)
    }
}

/// Accepts e.g. `embassy_time::Delay`, or a board's blocking delay implementation.
pub struct DelayAdapter<D>(pub D);
impl<D: embedded_hal::delay::DelayNs> imu::DelayUs for DelayAdapter<D> {
    fn delay_us(&mut self, us: u32) {
        self.0.delay_us(us);
    }
}
impl<D: embedded_hal_async::delay::DelayNs> imu::AsyncDelayUs for DelayAdapter<D> {
    async fn delay_us(&mut self, us: u32) {
        self.0.delay_us(us).await;
    }
}

/// embedded-hal intentionally separates duty from output enable. This extension
/// makes that control explicit; setting duty never enables the channel.
pub trait PwmEnable: embedded_hal::pwm::SetDutyCycle {
    fn enable_output(&mut self) -> Result<(), Self::Error>;
    fn disable_output(&mut self) -> Result<(), Self::Error>;
    /// Refuse wider HAL timer configurations that cannot be represented by
    /// embedded-hal's u16 duty API, instead of calling a panicking conversion.
    fn checked_max_duty(&self) -> Option<u16> {
        Some(self.max_duty_cycle())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwmError<E> {
    InvalidDuty,
    UnsupportedResolution,
    Driver(E),
}

pub struct PwmAdapter<P>(P);
impl<P> PwmAdapter<P> {
    pub const fn new(pin: P) -> Self {
        Self(pin)
    }
    pub fn inner(&self) -> &P {
        &self.0
    }
    pub fn into_inner(self) -> P {
        self.0
    }
}
impl<P: PwmEnable> PwmAdapter<P> {
    pub fn disable(&mut self) -> Result<(), PwmError<P::Error>> {
        self.0.disable_output().map_err(PwmError::Driver)
    }
}
fn duty_count(duty: f32, max: u16) -> Option<u16> {
    if !duty.is_finite() || !(0.0..=1.0).contains(&duty) {
        return None;
    }
    Some((duty * max as f32 + 0.5) as u16)
}
impl<P: PwmEnable> PwmAdapter<P> {
    /// Explicitly enable the channel after the application configures duty.
    pub fn enable(&mut self) -> Result<(), PwmError<P::Error>> {
        self.0.enable_output().map_err(PwmError::Driver)
    }
    /// Set a finite duty fraction in 0..=1 without enabling the channel.
    pub fn set_duty(&mut self, duty: f32) -> Result<(), PwmError<P::Error>> {
        let max = self
            .0
            .checked_max_duty()
            .ok_or(PwmError::UnsupportedResolution)?;
        let count = duty_count(duty, max).ok_or(PwmError::InvalidDuty)?;
        self.0.set_duty_cycle(count).map_err(PwmError::Driver)
    }
}

#[cfg(all(feature = "hal", stm32_has_timer))]
impl<T: embassy_stm32::timer::GeneralInstance4Channel> PwmEnable
    for embassy_stm32::timer::simple_pwm::SimplePwmChannel<'_, T>
{
    fn enable_output(&mut self) -> Result<(), Self::Error> {
        self.enable();
        Ok(())
    }
    fn disable_output(&mut self) -> Result<(), Self::Error> {
        self.disable();
        Ok(())
    }
    fn checked_max_duty(&self) -> Option<u16> {
        self.max_duty_cycle().try_into().ok()
    }
}
