pub use bitstream_io::Numeric; // re-export
pub use num_traits::Num;
use std::fmt::Display;

pub trait TonNumber: Numeric + Display {
    type UnsignedType: Numeric;
    fn to_unsigned(&self) -> Self::UnsignedType;
}

pub trait TonBigNumber: Display + Sized {
    const SIGNED: bool;
    fn is_negative(&self) -> bool;
    fn is_zero(&self) -> bool;
    fn zero() -> Self;
    /// must includes sign bit if SIGNED=true
    fn min_bits_len(&self) -> u32;
    fn to_unsigned_bytes_be(&self) -> Vec<u8>;
    fn from_unsigned_bytes_be(negative: bool, bytes: &[u8]) -> Self;
    fn shr(&self, bits: u32) -> Self;
}
