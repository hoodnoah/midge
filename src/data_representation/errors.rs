#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataRepresentationError {
    // variable-byte integer errors
    MalformedVariableByteInteger,
    VariableByteIntegerOutOfRange,

    // fixed string errors
    FixedStrBufferOverflow,

    // utf-8 string errors
    Utf8StringTooLong,
    NullTerminatorInString,
    Utf8BufferOverflow,
    Utf8MalformedBuffer,
    InvalidUTF8String,
}
