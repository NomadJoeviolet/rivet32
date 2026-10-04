//! HAL-independent asynchronous communication contracts.
use crate::can::CanFrame;
use core::future::Future;

/// Portable CAN errors. Platform adapters translate their native error values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanError {
    UnsupportedFd,
    RemoteFrameUnsupported,
    InvalidFrame,
    Stuff,
    Form,
    Acknowledge,
    BitRecessive,
    BitDominant,
    Crc,
    Software,
    BusOff,
    Overrun,
}

/// The driver has accepted the frame; this does not prove physical transmission
/// or acknowledgement. Some controllers replace a queued, lower-priority frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CanTxOutcome {
    /// A previously accepted frame evicted by the driver's priority policy.
    pub replaced: Option<CanFrame>,
    /// A replaced hardware frame could not be represented (for example a remote
    /// frame queued before this adapter took ownership). The new frame is still
    /// accepted. Set this or `replaced`, never both; do not report this as a failed
    /// transmit, which would make callers retry an already accepted frame.
    pub replaced_error: Option<CanError>,
}

pub trait AsyncCanTx {
    /// Wait until the driver accepts this exact data frame.
    ///
    /// Cancellation contract: while the returned future is Pending, dropping it
    /// must leave this frame unaccepted and unsent, so retrying cannot duplicate
    /// it. Acceptance and Ready(Ok) must happen in the same poll. Err means this
    /// attempt did not accept the frame. Implementations must not await physical
    /// completion after acceptance. Driver-side progress on older frames is
    /// permitted. The borrowed frame must not be retained after completion/Drop.
    ///
    /// Exact classic/FD payload length and ID are preserved. Adapters must reject
    /// unsupported formats rather than truncating or silently converting them.
    fn transmit(
        &mut self,
        frame: &CanFrame,
    ) -> impl Future<Output = Result<CanTxOutcome, CanError>>;
}

pub trait AsyncCanRx {
    /// Receive an owned, validated frame with its exact classic/FD payload length.
    ///
    /// A Pending future must not consume a frame on Drop. Consumption and Ready
    /// must happen in the same poll. Ready(Err) may report/discard an unsupported
    /// or malformed received item, or report a controller fault. No HAL-owned
    /// buffer or native controller handle escapes through this interface.
    fn receive(&mut self) -> impl Future<Output = Result<CanFrame, CanError>>;
}
