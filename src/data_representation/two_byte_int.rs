#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TwoByteInt(u16); // wrapper around a 16-bit integer; unsigned per the spec

impl TwoByteInt {
    pub fn from_bytes(bytes: [u8; 2]) -> Self {
        Self(u16::from_be_bytes(bytes)) // big-endian
    }

    pub fn to_bytes(self) -> [u8; 2] {
        self.0.to_be_bytes() // big-endian
    }

    pub fn value(self) -> u16 {
        self.0
    }
}

impl From<u16> for TwoByteInt {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod test_two_byte_int {
    use super::*;

    #[test]
    fn test_reversibility() {
        let original = TwoByteInt(0x1234);
        let bytes = original.to_bytes();
        let reconstructed = TwoByteInt::from_bytes(bytes);

        assert_eq!(original, reconstructed);
    }
}
