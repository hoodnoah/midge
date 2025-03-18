#[cfg_attr(not(test), no_std)]
#[cfg(test)]
extern crate std;

mod data_representation; // data representations per the spec
pub mod error;
pub mod fixed_header;
