use super::TwoByteInt;
use super::{DataRepresentationError, FixedStr};
use core::{fmt, str};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utf8String<const N: usize> {
    value: FixedStr<N>,
    length: u16,
}

const MAX_STR_LEN: u16 = 65535;

impl<const N: usize> Utf8String<N> {
    /// Creates an empty utf-8 string
    pub const fn new() -> Self {
        Self {
            value: FixedStr::new(),
            length: 0,
        }
    }

    /// Sets the value of the string, enforcing utf-8 validation per the spec
    pub fn set(&mut self, value: &str) -> Result<(), DataRepresentationError> {
        if value.len() > N {
            return Err(DataRepresentationError::Utf8StringTooLong);
        }

        if value.contains('\0') {
            return Err(DataRepresentationError::NullTerminatorInString);
        }

        // UTF-8 encoding check is handled by Rust's str type
        self.value.clear();
        match self.value.push_str(value) {
            Ok(_) => {
                self.length = value.len() as u16;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Encodes the UTF-8 string into the MQTT-spec format
    /// Returns the length (including the 2 bytes of length data) of the encoded string
    pub fn encode(&self, buffer: &mut [u8]) -> Result<u16, DataRepresentationError> {
        // limit the string length to the maximum permitted by the spec
        if self.length > MAX_STR_LEN {
            return Err(DataRepresentationError::Utf8StringTooLong);
        }

        // Ensure the buffer is large enough (remember, we have 2 bytes of 'length' to encode)
        if buffer.len() < (self.length + 2) as usize {
            return Err(DataRepresentationError::Utf8BufferOverflow);
        }

        // Encode length as two-byte integer (in bytes)
        let length_bytes = TwoByteInt::from(self.length).to_bytes();

        // Copy the length bytes into the buffer
        buffer[..2].copy_from_slice(&length_bytes);

        // Copy the string data into the buffer
        // Use the length to only copy the necessary bytes; this allows the caller to
        // provide an oversized buffer, which avoids them knowing the internal representation.
        buffer[2..2 + self.length as usize].copy_from_slice(self.value.as_str().as_bytes());

        Ok(2 + self.length)
    }

    /// Decodes an MQTT UTF-8 string from a byte buffer
    pub fn decode(buffer: &[u8]) -> Result<Self, DataRepresentationError> {
        if buffer.len() < 2 {
            return Err(DataRepresentationError::Utf8MalformedBuffer);
        }

        // Read the length
        let len = TwoByteInt::from_bytes([buffer[0], buffer[1]]).value() as usize;

        // Ensure the buffer is large enough to hold the supposed number of bytes
        if len + 2 > buffer.len() {
            return Err(DataRepresentationError::Utf8MalformedBuffer);
        }

        let utf8_bytes = &buffer[2..2 + len];

        // Ensure valid UTF-8
        let utf8_str = core::str::from_utf8(utf8_bytes)
            .map_err(|_| DataRepresentationError::InvalidUTF8String)?;

        // Ensure no null-terminators
        if utf8_str.contains('\0') {
            return Err(DataRepresentationError::NullTerminatorInString);
        }

        let mut utf8_string = Utf8String::new();
        utf8_string.set(utf8_str)?;

        Ok(utf8_string)
    }
}

impl<const N: usize> fmt::Display for Utf8String<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.as_str())
    }
}

#[cfg(test)]
mod test_utf8_str {
    use super::*;

    #[test]
    fn encodes_simple_string() {
        let test_string = "AB";

        let mut utf8_str = Utf8String::<2>::new();
        utf8_str.set(test_string).unwrap();

        let mut buffer = [0; 4];

        let expected_buffer = [
            0x00, 0x02, // length of 2 utf-8 chars
            0x41, // A
            0x42, // B
        ];

        let _ = utf8_str.encode(&mut buffer).unwrap();

        assert_eq!(buffer, expected_buffer);
    }

    #[test]
    fn decodes_simple_string() {
        let buffer = [
            0x00, 0x02, // length of 2 utf-8 chars
            0x41, // A
            0x42, // B
        ];

        let utf8_str = Utf8String::<2>::decode(&buffer).unwrap();

        assert_eq!(utf8_str.value.as_str(), "AB");
    }

    #[test]
    fn encodes_example_from_spec() {
        let test_str = "A𪛔";

        let mut utf8_str = Utf8String::<16>::new();
        utf8_str.set(test_str).unwrap();

        let mut buffer = [0; 8];

        let expected_buffer = [
            0x00, 0x05, // length of 5 utf-8 chars
            0x41, // A
            0xF0, 0xAA, 0x9B, 0x94, // 𪛔
        ];

        let _ = utf8_str.encode(&mut buffer).unwrap();

        assert_eq!(&buffer[0..7], &expected_buffer);
    }

    #[test]
    fn decodes_example_from_spec() {
        let buffer = [
            0x00, 0x05, // length of 5 utf-8 chars
            0x41, // A
            0xF0, 0xAA, 0x9B, 0x94, // 𪛔
        ];

        let utf8_str = Utf8String::<16>::decode(&buffer).unwrap();

        assert_eq!(utf8_str.value.as_str(), "A𪛔");
    }
}
