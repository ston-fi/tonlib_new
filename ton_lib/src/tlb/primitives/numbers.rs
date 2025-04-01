use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonLibError;
use crate::tlb::tlb_type::TLBType;
#[cfg(feature = "num-bigint")]
use num_bigint::{BigInt, BigUint};
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TLBNumber<T, const BITS_LEN: u32>(T);

impl<T, const BITS_LEN: u32> TLBNumber<T, BITS_LEN> {
    pub fn new(value: T) -> Self { TLBNumber(value) }
}

impl<T, const BITS_LEN: u32> From<T> for TLBNumber<T, BITS_LEN> {
    fn from(value: T) -> Self { TLBNumber(value) }
}

impl<T, const BITS_SIZE: u32> Deref for TLBNumber<T, BITS_SIZE> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T, const BITS_SIZE: u32> DerefMut for TLBNumber<T, BITS_SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

macro_rules! tlb_number_primitive_impl {
    ($t:ty) => {
        impl<const BITS_LEN: u32> TLBType for TLBNumber<$t, BITS_LEN> {
            fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
                Ok(TLBNumber(parser.read_num(BITS_LEN)?))
            }

            fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
                builder.write_num(self.0, BITS_LEN)
            }
        }
    };
}

tlb_number_primitive_impl!(i8);
tlb_number_primitive_impl!(u8);
tlb_number_primitive_impl!(i16);
tlb_number_primitive_impl!(u16);
tlb_number_primitive_impl!(i32);
tlb_number_primitive_impl!(u32);
tlb_number_primitive_impl!(i64);
tlb_number_primitive_impl!(u64);
tlb_number_primitive_impl!(i128);
tlb_number_primitive_impl!(u128);

#[cfg(any(feature = "num-bigint", feature = "fastnum"))]
macro_rules! tlb_big_number_impl {
    ($t:ty) => {
        impl<const BITS_LEN: u32> TLBType for TLBNumber<$t, BITS_LEN> {
            fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
                Ok(TLBNumber(parser.read_big_num(BITS_LEN)?))
            }

            fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
                builder.write_big_num(&self.0, BITS_LEN)
            }
        }
    };
}

#[cfg(feature = "num-bigint")]
tlb_big_number_impl!(BigInt);
#[cfg(feature = "num-bigint")]
tlb_big_number_impl!(BigUint);
