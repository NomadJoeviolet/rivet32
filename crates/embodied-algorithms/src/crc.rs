//! Width-parameterized CRC, with streaming and non-byte-aligned input.
use crate::Error;

#[derive(Clone, Copy, Debug)]
pub struct CrcParameters {
    pub width: u8,
    /// Polynomial without its leading x^width term, in normal (MSB) notation.
    pub polynomial: u64,
    pub initial: u64,
    pub xor_out: u64,
    pub reflect_in: bool,
    pub reflect_out: bool,
}
const fn params(
    width: u8,
    polynomial: u64,
    initial: u64,
    xor_out: u64,
    reflect_in: bool,
    reflect_out: bool,
) -> CrcParameters {
    CrcParameters {
        width,
        polynomial,
        initial,
        xor_out,
        reflect_in,
        reflect_out,
    }
}
pub const CRC8_MAXIM: CrcParameters = params(8, 0x31, 0, 0, true, true);
pub const CRC8_DJI: CrcParameters = params(8, 0x31, 0xff, 0, true, true);
pub const CRC16_IBM_3740: CrcParameters = params(16, 0x1021, 0xffff, 0, false, false);
pub const CRC16_MODBUS: CrcParameters = params(16, 0x8005, 0xffff, 0, true, true);
pub const CRC16_X25: CrcParameters = params(16, 0x1021, 0xffff, 0xffff, true, true);
pub const CRC16_DJI: CrcParameters = params(16, 0x1021, 0xffff, 0, true, true);
pub const CRC32_ISO_HDLC: CrcParameters =
    params(32, 0x04c11db7, 0xffffffff, 0xffffffff, true, true);
pub const CRC64_ECMA: CrcParameters = params(64, 0x42f0e1eba9ea3693, 0, 0, false, false);

#[derive(Clone, Copy, Debug)]
pub struct Crc {
    params: CrcParameters,
    remainder: u64,
    mask: u64,
}
impl Crc {
    pub fn new(params: CrcParameters) -> Result<Self, Error> {
        if !(1..=64).contains(&params.width) {
            return Err(Error::InvalidParameter);
        }
        let mask = u64::MAX >> (64 - params.width);
        if params.polynomial == 0
            || (params.polynomial | params.initial | params.xor_out) & !mask != 0
        {
            return Err(Error::InvalidParameter);
        }
        Ok(Self {
            params,
            remainder: params.initial,
            mask,
        })
    }
    pub fn reset(&mut self) {
        self.remainder = self.params.initial;
    }
    fn bit(&mut self, input: bool) {
        let feedback = (self.remainder >> (self.params.width - 1) != 0) ^ input;
        self.remainder = (self.remainder << 1) & self.mask;
        if feedback {
            self.remainder ^= self.params.polynomial;
        }
    }
    pub fn update(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            for bit in 0..8 {
                let pos = if self.params.reflect_in { bit } else { 7 - bit };
                self.bit(byte & (1 << pos) != 0);
            }
        }
    }
    /// Process `bit_count` bits, taking low bits first for reflected input.
    pub fn update_bits(&mut self, bytes: &[u8], bit_count: usize) -> Result<(), Error> {
        if bit_count / 8 > bytes.len()
            || (bit_count / 8 == bytes.len() && !bit_count.is_multiple_of(8))
        {
            return Err(Error::InvalidParameter);
        }
        for i in 0..bit_count {
            let pos = if self.params.reflect_in {
                i % 8
            } else {
                7 - i % 8
            };
            self.bit(bytes[i / 8] & (1 << pos) != 0);
        }
        Ok(())
    }
    pub fn finish(&self) -> u64 {
        let r = if self.params.reflect_out {
            self.remainder.reverse_bits() >> (64 - self.params.width)
        } else {
            self.remainder
        };
        (r ^ self.params.xor_out) & self.mask
    }
    pub fn checksum(mut self, bytes: &[u8]) -> u64 {
        self.reset();
        self.update(bytes);
        self.finish()
    }
}
pub fn crc8_dji(bytes: &[u8]) -> u8 {
    Crc::new(CRC8_DJI).unwrap().checksum(bytes) as u8
}
pub fn crc16_dji(bytes: &[u8]) -> u16 {
    Crc::new(CRC16_DJI).unwrap().checksum(bytes) as u16
}
