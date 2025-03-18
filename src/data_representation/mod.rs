mod errors;
mod four_byte_int;
mod two_byte_int;
mod variable_byte_int;

pub(crate) use errors::DataRepresentationError;
pub(crate) use four_byte_int::FourByteInt;
pub(crate) use two_byte_int::TwoByteInt;
pub(crate) use variable_byte_int::VariableByteInt;
