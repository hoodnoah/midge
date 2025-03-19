#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FourByteInt(u32); // wrapper around a 32-bit integer; unsigned per the spec

impl FourByteInt {
    pub fn from_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(bytes)) // big-endian
    }

    pub fn to_bytes(self) -> [u8; 4] {
        self.0.to_be_bytes() // big-endian
    }
}

#[cfg(test)]
mod test_four_byte_int {
    use super::*;

    #[test]
    fn test_reversibility() {
        let original = FourByteInt(0x12345678);
        let bytes = original.to_bytes();
        let reconstructed = FourByteInt::from_bytes(bytes);

        assert_eq!(original, reconstructed);
    }
}
