//! DMA UART byte streams. Construct the HAL UART with board-specific pins,
//! DMA channels and IRQ bindings, then move it into these adapters.
//!
//! RX uses idle framing and may return fewer bytes than requested. UART must be
//! asynchronous full-duplex, with at most eight data bits. Cancelled RX may discard bytes
//! already written by DMA; cancelled TX may have sent a prefix. HAL drop guards
//! stop the DMA transfer before releasing the borrowed buffer. Neither operation
//! promises transactional delivery. For continuous reception use the HAL's
//! ring-buffered UART and an explicit overflow policy instead.

use embassy_stm32::{mode::Async, usart};
use embedded_io::{ErrorKind, ErrorType};
use embedded_io_async::{Read, Write};

/// Conservative transfer limit shared by the STM32 DMA families used here.
pub const MAX_DMA_BYTES: usize = u16::MAX as usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UartError(pub usart::Error);
impl core::fmt::Display for UartError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}
impl core::error::Error for UartError {}
impl embedded_io::Error for UartError {
    fn kind(&self) -> ErrorKind {
        match self.0 {
            usart::Error::Framing | usart::Error::Noise | usart::Error::Parity => {
                ErrorKind::InvalidData
            }
            usart::Error::BufferTooLong => ErrorKind::InvalidInput,
            _ => ErrorKind::Other,
        }
    }
}

pub struct UartStream<'d>(usart::Uart<'d, Async>);
pub struct UartTx<'d>(usart::UartTx<'d, Async>);
pub struct UartRx<'d>(usart::UartRx<'d, Async>);

impl<'d> UartStream<'d> {
    pub fn new(uart: usart::Uart<'d, Async>) -> Self {
        Self(uart)
    }
    pub fn into_inner(self) -> usart::Uart<'d, Async> {
        self.0
    }
    pub fn split(self) -> (UartTx<'d>, UartRx<'d>) {
        let (tx, rx) = self.0.split();
        (UartTx(tx), UartRx(rx))
    }
}
impl<'d> UartTx<'d> {
    pub fn new(tx: usart::UartTx<'d, Async>) -> Self {
        Self(tx)
    }
    pub fn into_inner(self) -> usart::UartTx<'d, Async> {
        self.0
    }
}
impl<'d> UartRx<'d> {
    pub fn new(rx: usart::UartRx<'d, Async>) -> Self {
        Self(rx)
    }
    pub fn into_inner(self) -> usart::UartRx<'d, Async> {
        self.0
    }
}

macro_rules! error_type {
    ($($name:ident),+) => { $(impl ErrorType for $name<'_> { type Error = UartError; })+ };
}
error_type!(UartStream, UartTx, UartRx);

macro_rules! read_impl {
    ($($name:ident),+) => { $(impl Read for $name<'_> {
        async fn read(&mut self, bytes: &mut [u8]) -> Result<usize, Self::Error> {
            if bytes.is_empty() { return Ok(0); }
            // Idle without payload is not stream EOF; wait for real data.
            let capacity = bytes.len().min(MAX_DMA_BYTES);
            loop {
                let count = self.0.read_until_idle(&mut bytes[..capacity]).await.map_err(UartError)?;
                if count != 0 { return Ok(count); }
            }
        }
    })+ };
}
read_impl!(UartStream, UartRx);

macro_rules! write_impl {
    ($($name:ident),+) => { $(impl Write for $name<'_> {
        async fn write(&mut self, bytes: &[u8]) -> Result<usize, Self::Error> {
            if bytes.is_empty() { return Ok(0); }
            let count = bytes.len().min(MAX_DMA_BYTES);
            self.0.write(&bytes[..count]).await.map_err(UartError)?;
            Ok(count)
        }
        async fn flush(&mut self) -> Result<(), Self::Error> {
            self.0.flush().await.map_err(UartError)
        }
    })+ };
}
write_impl!(UartStream, UartTx);
