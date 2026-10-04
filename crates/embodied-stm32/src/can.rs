//! Validated CAN conversions and adapters for the selected STM32 controller.
//! Classic CAN/FD payload lengths are never padded or truncated. Remote frames
//! have no representation in core and produce `RemoteFrameUnsupported` on RX.

use embodied_core::can::{CanFrame, Id};
use embodied_core::communication::CanError;
#[cfg(any(test, stm32_can_bxcan, stm32_can_fdcan))]
use embodied_core::communication::CanTxOutcome;

pub fn to_embedded_id(id: Id) -> Result<embedded_can::Id, CanError> {
    match id {
        Id::Standard(value) => embedded_can::StandardId::new(value).map(Into::into),
        Id::Extended(value) => embedded_can::ExtendedId::new(value).map(Into::into),
    }
    .ok_or(CanError::InvalidFrame)
}
pub fn from_embedded_id(id: embedded_can::Id) -> Id {
    match id {
        embedded_can::Id::Standard(value) => Id::Standard(value.as_raw()),
        embedded_can::Id::Extended(value) => Id::Extended(value.as_raw()),
    }
}
/// Shared receive validation, also useful when bridging a non-STM32 controller.
pub fn decode_data_frame(
    id: embedded_can::Id,
    remote: bool,
    fd: bool,
    data: &[u8],
) -> Result<CanFrame, CanError> {
    if remote {
        return Err(CanError::RemoteFrameUnsupported);
    }
    let id = from_embedded_id(id);
    if fd {
        CanFrame::new_fd(id, data)
    } else {
        CanFrame::new(id, data)
    }
    .map_err(|_| CanError::InvalidFrame)
}

#[cfg(any(test, stm32_can_bxcan, stm32_can_fdcan))]
fn decode_hal_frame<'a>(
    id: embedded_can::Id,
    remote: bool,
    fd: bool,
    len: usize,
    data: impl FnOnce() -> &'a [u8],
) -> Result<CanFrame, CanError> {
    if remote {
        return Err(CanError::RemoteFrameUnsupported);
    }
    // Pinned HAL Frame::data() trusts the header length. bxCAN reads the
    // received four-bit DLC verbatim, including values above its 8-byte storage.
    // Validate before invoking that accessor; never clamp or silently truncate.
    if len > if fd { 64 } else { 8 } {
        return Err(CanError::InvalidFrame);
    }
    decode_data_frame(id, false, fd, data())
}

#[cfg(any(test, stm32_can_bxcan, stm32_can_fdcan))]
fn accepted(replaced: Option<Result<CanFrame, CanError>>) -> CanTxOutcome {
    // New TX already belongs to the HAL. An unrepresentable evicted old frame
    // must not turn this result into Err and cause the caller to resend new TX.
    match replaced {
        Some(Ok(frame)) => CanTxOutcome {
            replaced: Some(frame),
            replaced_error: None,
        },
        Some(Err(error)) => CanTxOutcome {
            replaced: None,
            replaced_error: Some(error),
        },
        None => CanTxOutcome::default(),
    }
}

#[cfg(any(stm32_can_bxcan, stm32_can_fdcan))]
mod hardware {
    use super::*;
    use embassy_stm32::can as hal;
    use embodied_core::communication::{AsyncCanRx, AsyncCanTx};

    #[cfg(stm32_can_fdcan)]
    #[derive(Clone, Copy, Default)]
    struct FdPolicy {
        enabled: bool,
        brs: bool,
    }

    /// Owns a running HAL controller. `new` deliberately allows only classic
    /// TX, since a running HAL controller does not expose its FD configuration.
    /// FDCAN callers use `start` to derive FD permission from its configurator.
    pub struct CanAdapter<'d> {
        inner: hal::Can<'d>,
        #[cfg(stm32_can_fdcan)]
        fd: FdPolicy,
    }
    pub struct CanTxAdapter<'d> {
        inner: hal::CanTx<'d>,
        #[cfg(stm32_can_fdcan)]
        fd: FdPolicy,
    }
    pub struct CanRxAdapter<'d> {
        inner: hal::CanRx<'d>,
    }

    impl<'d> CanAdapter<'d> {
        pub fn new(inner: hal::Can<'d>) -> Self {
            Self {
                inner,
                #[cfg(stm32_can_fdcan)]
                fd: FdPolicy::default(),
            }
        }
        pub fn into_inner(self) -> hal::Can<'d> {
            self.inner
        }

        /// Consume a configured FDCAN and start it in the chosen operating mode.
        /// Preserves nominal/data bit timings, filters and controller policy.
        #[cfg(stm32_can_fdcan)]
        pub fn start(config: hal::CanConfigurator<'d>, mode: hal::OperatingMode) -> Self {
            use hal::config::FrameTransmissionConfig;
            let fd = match config.config().frame_transmit {
                FrameTransmissionConfig::ClassicCanOnly => FdPolicy::default(),
                FrameTransmissionConfig::AllowFdCan => FdPolicy {
                    enabled: true,
                    brs: false,
                },
                FrameTransmissionConfig::AllowFdCanAndBRS => FdPolicy {
                    enabled: true,
                    brs: true,
                },
            };
            Self {
                inner: config.start(mode),
                fd,
            }
        }

        /// FDCAN HAL transfers ownership to its halves and Properties object.
        #[cfg(stm32_can_fdcan)]
        pub fn split(self) -> (CanTxAdapter<'d>, CanRxAdapter<'d>, hal::Properties) {
            let (tx, rx, properties) = self.inner.split();
            (
                CanTxAdapter {
                    inner: tx,
                    fd: self.fd,
                },
                CanRxAdapter { inner: rx },
                properties,
            )
        }

        /// bxCAN HAL resets the peripheral when its owner is dropped. Tie the
        /// returned adapters to this borrow so that the owner stays alive.
        #[cfg(stm32_can_bxcan)]
        pub fn split(&mut self) -> (CanTxAdapter<'_>, CanRxAdapter<'_>) {
            let (tx, rx) = self.inner.split();
            (CanTxAdapter { inner: tx }, CanRxAdapter { inner: rx })
        }
    }

    #[cfg(stm32_can_bxcan)]
    fn decode_classic(frame: &hal::frame::Frame) -> Result<CanFrame, CanError> {
        decode_hal_frame(
            *frame.id(),
            frame.header().rtr(),
            frame.header().fdcan(),
            usize::from(frame.header().len()),
            || frame.data(),
        )
    }
    #[cfg(stm32_can_fdcan)]
    fn decode_fd(frame: &hal::frame::FdFrame) -> Result<CanFrame, CanError> {
        decode_hal_frame(
            *frame.id(),
            frame.header().rtr(),
            frame.header().fdcan(),
            usize::from(frame.header().len()),
            || frame.data(),
        )
    }
    #[cfg(stm32_can_fdcan)]
    fn encode_fd(frame: &CanFrame, policy: FdPolicy) -> Result<hal::frame::FdFrame, CanError> {
        if frame.is_fd() && !policy.enabled {
            return Err(CanError::UnsupportedFd);
        }
        let id = to_embedded_id(frame.id())?;
        let header = if frame.is_fd() {
            hal::frame::Header::new_fd(id, frame.data().len() as u8, false, policy.brs)
        } else {
            hal::frame::Header::new(id, frame.data().len() as u8, false)
        };
        hal::frame::FdFrame::new(header, frame.data()).map_err(|_| CanError::InvalidFrame)
    }
    #[cfg(stm32_can_bxcan)]
    fn encode_classic(frame: &CanFrame) -> Result<hal::frame::Frame, CanError> {
        if frame.is_fd() {
            return Err(CanError::UnsupportedFd);
        }
        hal::frame::Frame::new_data(to_embedded_id(frame.id())?, frame.data())
            .map_err(|_| CanError::InvalidFrame)
    }
    fn bus_error(error: hal::enums::BusError) -> CanError {
        use hal::enums::BusError;
        match error {
            BusError::Stuff => CanError::Stuff,
            BusError::Form => CanError::Form,
            BusError::Acknowledge => CanError::Acknowledge,
            BusError::BitRecessive => CanError::BitRecessive,
            BusError::BitDominant => CanError::BitDominant,
            BusError::Crc => CanError::Crc,
            BusError::Software => CanError::Software,
        }
    }

    // The pinned HAL poll_fn completes in the same poll that accepts/dequeues
    // a frame. Do not insert a flush, yield, timer or other await after that point.
    macro_rules! transmit {
        ($($name:ident),+) => { $(impl AsyncCanTx for $name<'_> {
            async fn transmit(&mut self, frame: &CanFrame) -> Result<CanTxOutcome, CanError> {
                #[cfg(stm32_can_fdcan)] {
                    let frame = encode_fd(frame, self.fd)?;
                    // Use the 64-byte HAL representation even for classic frames,
                    // preserving a replaced FD frame instead of truncating it.
                    let replaced = self.inner.write_fd(&frame).await;
                    Ok(accepted(replaced.as_ref().map(decode_fd)))
                }
                #[cfg(stm32_can_bxcan)] {
                    let frame = encode_classic(frame)?;
                    let status = self.inner.write(&frame).await;
                    Ok(accepted(status.dequeued_frame().map(decode_classic)))
                }
            }
        })+ };
    }
    transmit!(CanAdapter, CanTxAdapter);
    macro_rules! receive {
        ($($name:ident),+) => { $(impl AsyncCanRx for $name<'_> {
            async fn receive(&mut self) -> Result<CanFrame, CanError> {
                #[cfg(stm32_can_fdcan)] {
                    let (frame, _) = self.inner.read_fd().await.map_err(bus_error)?.parts();
                    decode_fd(&frame)
                }
                #[cfg(stm32_can_bxcan)] {
                    let (frame, _) = self.inner.read().await.map_err(bus_error)?.parts();
                    decode_classic(&frame)
                }
            }
        })+ };
    }
    receive!(CanAdapter, CanRxAdapter);
}

#[cfg(any(stm32_can_bxcan, stm32_can_fdcan))]
pub use hardware::{CanAdapter, CanRxAdapter, CanTxAdapter};
