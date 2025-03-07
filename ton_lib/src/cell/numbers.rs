pub use bitstream_io::Numeric; // re-export
use std::fmt::Display;

pub trait TonNumber: Numeric + Display {
    type DstType: Numeric;
    fn to_unsigned(&self) -> Self::DstType;
}

macro_rules! impl_ton_number {
    ($src:ty, $dst:ty) => {
        impl TonNumber for $src {
            type DstType = $dst;
            fn to_unsigned(&self) -> Self::DstType { *self as $dst }
        }
    };
}

impl_ton_number!(i8, u8);
impl_ton_number!(u8, u8);
impl_ton_number!(i16, u16);
impl_ton_number!(u16, u16);
impl_ton_number!(i32, u32);
impl_ton_number!(u32, u32);
impl_ton_number!(i64, u64);
impl_ton_number!(u64, u64);
impl_ton_number!(i128, u128);
impl_ton_number!(u128, u128);
