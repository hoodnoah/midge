// simple heapless string implementation

use super::DataRepresentationError;
use core::{fmt, str};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedStr<const N: usize> {
    buffer: [u8; N],
    len: usize,
}

impl<const N: usize> FixedStr<N> {
    /// Creates a new, empty FixedStr
    pub const fn new() -> Self {
        Self {
            buffer: [0; N],
            len: 0,
        }
    }

    /// Appends a string slice to the buffer, if space allows
    pub fn push_str(&mut self, s: &str) -> Result<(), DataRepresentationError> {
        let bytes = s.as_bytes();

        // ensure buffer is large enough
        if bytes.len() > N - self.len {
            return Err(DataRepresentationError::FixedStrBufferOverflow);
        }

        // add string bytes to the buffer
        self.buffer[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();

        Ok(())
    }

    /// Returns the string slice of the currently stored UTF-8 data
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buffer[..self.len]).unwrap()
    }

    /// clears the buffer
    pub fn clear(&mut self) {
        self.len = 0;
    }
}

/// Implementing the `fmt::Write` trait allows us to use the `write!` macro
impl<const N: usize> fmt::Write for FixedStr<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s).map_err(|_| fmt::Error)
    }
}
