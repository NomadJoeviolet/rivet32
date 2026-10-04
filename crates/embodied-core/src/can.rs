//! Validated CAN data frames. Remote frames and controller-specific flags belong to the HAL.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id {
    Standard(u16),
    Extended(u32),
}

impl Id {
    pub const fn is_valid(self) -> bool {
        match self {
            Self::Standard(value) => value <= 0x7ff,
            Self::Extended(value) => value <= 0x1fff_ffff,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameError {
    InvalidId,
    InvalidLength,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CanFrame {
    id: Id,
    data: [u8; 64],
    len: u8,
    fd: bool,
}

impl CanFrame {
    pub fn new(id: Id, data: &[u8]) -> Result<Self, FrameError> {
        Self::build(id, data, false)
    }

    /// Requires an exact representable DLC length; never silently pads a payload.
    pub fn new_fd(id: Id, data: &[u8]) -> Result<Self, FrameError> {
        Self::build(id, data, true)
    }

    fn build(id: Id, data: &[u8], fd: bool) -> Result<Self, FrameError> {
        if !id.is_valid() {
            return Err(FrameError::InvalidId);
        }
        let valid_length = if fd {
            matches!(data.len(), 0..=8 | 12 | 16 | 20 | 24 | 32 | 48 | 64)
        } else {
            data.len() <= 8
        };
        if !valid_length {
            return Err(FrameError::InvalidLength);
        }
        let mut frame = Self {
            id,
            data: [0; 64],
            len: data.len() as u8,
            fd,
        };
        frame.data[..data.len()].copy_from_slice(data);
        Ok(frame)
    }

    pub const fn id(&self) -> Id {
        self.id
    }
    pub fn data(&self) -> &[u8] {
        &self.data[..self.len as usize]
    }
    pub const fn is_fd(&self) -> bool {
        self.fd
    }
}
