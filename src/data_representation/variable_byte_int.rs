use crate::data_representation::DataRepresentationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VariableByteInt {
    value: u32,    // per the spec, there is a maximum of 4 bytes permitted
    length: usize, // length of the encoded value, in bytes
}

impl VariableByteInt {
    // Maximum value allowed (28-bit maximum)
    const MAX_VALUE: u32 = 0x0FFF_FFFF;

    /// Creates a new Variable Byte Integer from a u32 value
    fn new(value: u32) -> Result<Self, DataRepresentationError> {
        if value > Self::MAX_VALUE {
            return Err(DataRepresentationError::MalformedVariableByteInteger);
        }

        let mut x = value;
        let mut length = 1; // minimum of 1 byte to represent anything meaningful

        // calculate required number of bytes to hold this value
        while x >= 128 {
            x /= 128;
            length += 1
        }

        if length > 4 {
            return Err(DataRepresentationError::VariableByteIntegerOutOfRange);
        }

        Ok(VariableByteInt { value, length })
    }

    /// Getter for the value
    fn value(self) -> u32 {
        self.value
    }

    /// Getter for the length
    fn length(self) -> usize {
        self.length
    }

    /// Encodes the value into a `VariableByteInt` format
    fn encode(self) -> [u8; 4] {
        let mut x = self.value;
        let mut output = [0u8; 4];
        let mut i = 0;

        loop {
            let mut byte = (x % 128) as u8; // least-significant 7 bits
            x /= 128;

            if x > 0 {
                byte |= 0b1000_0000; // set the continuation bit; more data follows
            }

            output[i] = byte;
            i += 1;

            if x == 0 {
                break;
            }
        }

        output // return the full buffer; the caller must know the actual length
    }

    /// Decodes from a Variable Byte Integer byte sequence
    fn decode(input: &[u8]) -> Result<Self, DataRepresentationError> {
        let mut multiplier = 1;
        let mut value: u32 = 0;
        let mut length = 0;

        // silently ignore extra bytes, length must be 4 or fewer
        for &byte in input.iter().take(4) {
            let digit = (byte & 127) as u32;
            value += digit * multiplier;

            if multiplier > 128 * 128 * 128 {
                return Err(DataRepresentationError::MalformedVariableByteInteger);
            }

            multiplier *= 128;
            length += 1;

            if (byte & 128) == 0 {
                return Ok(VariableByteInt { value, length }); // no more data
            }
        }

        Err(DataRepresentationError::MalformedVariableByteInteger)
    }
}

#[cfg(test)]
mod test_variable_byte_int {
    use super::*;

    #[test]
    fn test_simple_encode() {
        let variable_byte_int = VariableByteInt::new(25).unwrap();
        let encoded = variable_byte_int.encode();
        let expected = [25, 0, 0, 0]; // 25 in the least significant byte

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_simple_decode() {
        let encoded = [25, 0, 0, 0];
        let decoded = VariableByteInt::decode(&encoded).unwrap();
        let expected = VariableByteInt::new(25).unwrap();

        assert_eq!(decoded, expected);
    }

    #[test]
    fn test_reversibility() {
        let original = VariableByteInt::new(0x69420).unwrap();
        let encoded = original.encode();
        let decoded = VariableByteInt::decode(&encoded).unwrap();

        assert_eq!(original, decoded);
        assert_eq!(decoded.length(), 3); // should be the full length
    }

    // #[test]
    // fn test_max_value() {
    //     let max_value = VariableByteInt::new(VariableByteInt::MAX_VALUE).unwrap();
    //     let encoded = max_value.encode();
    //     let (decoded, length) = VariableByteInt::decode(&encoded).unwrap();

    //     assert_eq!(max_value, decoded);
    //     assert_eq!(length, 4); // should be the full length
    // }
}
