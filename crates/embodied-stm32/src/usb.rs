//! CDC byte streams, including short reads and explicit bulk transfer termination.
//! Run the separate `UsbDevice::run` future continuously. Endpoint enable and DTR
//! are different facts: `last_io` is an observation, while DTR/RTS are current
//! host control signals. It is not a USB cable-presence detector.

use embassy_usb::class::cdc_acm::{CdcAcmClass, Receiver, Sender};
use embassy_usb::driver::{Driver, EndpointError};
use embedded_io::{ErrorKind, ErrorType};
use embedded_io_async::{Read, Write};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbError {
    Disconnected,
    BufferOverflow,
}
impl From<EndpointError> for UsbError {
    fn from(error: EndpointError) -> Self {
        match error {
            EndpointError::Disabled => Self::Disconnected,
            EndpointError::BufferOverflow => Self::BufferOverflow,
        }
    }
}
impl core::fmt::Display for UsbError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl core::error::Error for UsbError {}
impl embedded_io::Error for UsbError {
    fn kind(&self) -> ErrorKind {
        match self {
            Self::Disconnected => ErrorKind::NotConnected,
            Self::BufferOverflow => ErrorKind::InvalidInput,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkObservation {
    Unknown,
    Ready,
    Disconnected,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Connection {
    pub last_io: LinkObservation,
    pub dtr: bool,
    pub rts: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RxBufferTooSmall {
    pub required: usize,
    pub actual: usize,
}

pub struct UsbTx<'d, D: Driver<'d>> {
    stream: PacketWriter<Sender<'d, D>>,
}
pub struct UsbRx<'d, D: Driver<'d>> {
    stream: PacketReader<'d, Receiver<'d, D>>,
}

/// Split the CDC class into independent byte-stream halves. RX storage must hold
/// at least one max-size packet (64 bytes for full-speed, 512 for high-speed).
/// The supplied class is consumed; validate storage before constructing the class
/// if ownership must be retained on a configuration error.
pub fn split<'d, D: Driver<'d>>(
    class: CdcAcmClass<'d, D>,
    storage: &'d mut [u8],
) -> Result<(UsbTx<'d, D>, UsbRx<'d, D>), RxBufferTooSmall> {
    let packet_size = class.max_packet_size() as usize;
    if storage.len() < packet_size {
        return Err(RxBufferTooSmall {
            required: packet_size,
            actual: storage.len(),
        });
    }
    let (tx, rx) = class.split();
    Ok((
        UsbTx {
            stream: PacketWriter {
                inner: tx,
                packet_size,
                needs_zlp: false,
                state: LinkObservation::Unknown,
            },
        },
        UsbRx {
            stream: PacketReader {
                inner: rx,
                storage,
                start: 0,
                end: 0,
                state: LinkObservation::Unknown,
            },
        },
    ))
}

impl<'d, D: Driver<'d>> UsbTx<'d, D> {
    pub async fn wait_connection(&mut self) {
        self.stream.inner.wait_connection().await;
        self.stream.state = LinkObservation::Ready;
        self.stream.needs_zlp = false;
    }
    pub fn connection(&self) -> Connection {
        Connection {
            last_io: self.stream.state,
            dtr: self.stream.inner.dtr(),
            rts: self.stream.inner.rts(),
        }
    }
}
impl<'d, D: Driver<'d>> UsbRx<'d, D> {
    /// Start/restart a session and discard any unread bytes from the old session.
    pub async fn wait_connection(&mut self) {
        self.stream.inner.wait_connection().await;
        self.stream.start = 0;
        self.stream.end = 0;
        self.stream.state = LinkObservation::Ready;
    }
    pub fn connection(&self) -> Connection {
        Connection {
            last_io: self.stream.state,
            dtr: self.stream.inner.dtr(),
            rts: self.stream.inner.rts(),
        }
    }
}

impl<'d, D: Driver<'d>> ErrorType for UsbTx<'d, D> {
    type Error = UsbError;
}
impl<'d, D: Driver<'d>> ErrorType for UsbRx<'d, D> {
    type Error = UsbError;
}
impl<'d, D: Driver<'d>> Write for UsbTx<'d, D> {
    /// Accept at most one packet. Use `write_all` for longer buffers, then flush.
    /// Cancellation follows the USB driver's semantics and is not transactional.
    async fn write(&mut self, bytes: &[u8]) -> Result<usize, Self::Error> {
        self.stream.write(bytes).await
    }
    /// Send a ZLP if the preceding write ended at a full packet boundary. This
    /// makes finite exact-packet-length output visible to the host, not an ACK
    /// that a host application consumed it.
    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.stream.flush().await
    }
}
impl<'d, D: Driver<'d>> Read for UsbRx<'d, D> {
    async fn read(&mut self, bytes: &mut [u8]) -> Result<usize, Self::Error> {
        self.stream.read(bytes).await
    }
}

trait PacketRead {
    async fn receive(&mut self, bytes: &mut [u8]) -> Result<usize, UsbError>;
}
trait PacketWrite {
    async fn send(&mut self, bytes: &[u8]) -> Result<(), UsbError>;
}
impl<'d, D: Driver<'d>> PacketRead for Receiver<'d, D> {
    async fn receive(&mut self, bytes: &mut [u8]) -> Result<usize, UsbError> {
        self.read_packet(bytes).await.map_err(Into::into)
    }
}
impl<'d, D: Driver<'d>> PacketWrite for Sender<'d, D> {
    async fn send(&mut self, bytes: &[u8]) -> Result<(), UsbError> {
        self.write_packet(bytes).await.map_err(Into::into)
    }
}

struct PacketReader<'d, R> {
    inner: R,
    storage: &'d mut [u8],
    start: usize,
    end: usize,
    state: LinkObservation,
}
impl<R: PacketRead> PacketReader<'_, R> {
    async fn read(&mut self, bytes: &mut [u8]) -> Result<usize, UsbError> {
        if bytes.is_empty() {
            return Ok(0);
        }
        while self.start == self.end {
            match self.inner.receive(self.storage).await {
                Ok(count) => {
                    if count > self.storage.len() {
                        return Err(UsbError::BufferOverflow);
                    }
                    // No await between receipt and publishing the buffered range.
                    self.start = 0;
                    self.end = count;
                    self.state = LinkObservation::Ready;
                    // USB ZLP terminates a transfer; it does not close a byte stream.
                }
                Err(error) => {
                    observe_error(&mut self.state, error);
                    return Err(error);
                }
            }
        }
        let count = bytes.len().min(self.end - self.start);
        bytes[..count].copy_from_slice(&self.storage[self.start..self.start + count]);
        self.start += count;
        Ok(count)
    }
}
struct PacketWriter<W> {
    inner: W,
    packet_size: usize,
    needs_zlp: bool,
    state: LinkObservation,
}
impl<W: PacketWrite> PacketWriter<W> {
    async fn write(&mut self, bytes: &[u8]) -> Result<usize, UsbError> {
        if bytes.is_empty() {
            return Ok(0);
        }
        let count = bytes.len().min(self.packet_size);
        // Conservatively retain the termination obligation if this future is
        // cancelled after the driver queues a full packet.
        if count == self.packet_size {
            self.needs_zlp = true;
        }
        match self.inner.send(&bytes[..count]).await {
            Ok(()) => {
                self.needs_zlp = count == self.packet_size;
                self.state = LinkObservation::Ready;
                Ok(count)
            }
            Err(error) => {
                observe_error(&mut self.state, error);
                Err(error)
            }
        }
    }
    async fn flush(&mut self) -> Result<(), UsbError> {
        if self.needs_zlp {
            if let Err(error) = self.inner.send(&[]).await {
                observe_error(&mut self.state, error);
                return Err(error);
            }
            self.needs_zlp = false;
            self.state = LinkObservation::Ready;
        }
        Ok(())
    }
}
fn observe_error(state: &mut LinkObservation, error: UsbError) {
    if error == UsbError::Disconnected {
        *state = LinkObservation::Disconnected;
    }
}
