pub use bitstream_io::Numeric; // re-export
pub use num_traits::Num;
use num_traits::Zero;
use std::fmt::Display;

pub trait TonCellNum: Display + Sized {
    const SIGNED: bool;
    const IS_PRIMITIVE: bool = false;
    type UnsignedPrimitive: Numeric;
    type Primitive: Zero + Numeric;

    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_bytes(&self) -> Vec<u8>;

    fn from_primitive(value: Self::Primitive) -> Self;
    fn to_unsigned_primitive(&self) -> Option<Self::UnsignedPrimitive>;

    fn is_zero(&self) -> bool;
    fn min_bits_len(&self) -> u32; // must includes sign bit if SIGNED=true
    fn shr(&self, bits: u32) -> Self;
}
