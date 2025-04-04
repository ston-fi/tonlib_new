use crate::cell::num::traits::TonCellNum;
use num_bigint::{BigInt, BigUint};
use num_traits::Zero;

impl TonCellNum for BigInt {
    const SIGNED: bool = true;
    type UnsignedPrimitive = u128;
    type Primitive = i128;
    fn from_bytes(bytes: &[u8]) -> Self { BigInt::from_signed_bytes_be(bytes) }
    fn to_bytes(&self) -> Vec<u8> { BigInt::to_signed_bytes_be(self) }

    fn from_primitive(value: Self::Primitive) -> Self { value.into() }
    fn to_unsigned_primitive(&self) -> Option<Self::UnsignedPrimitive> { None }

    fn is_zero(&self) -> bool { Zero::is_zero(self) }
    fn min_bits_len(&self) -> u32 { self.bits() as u32 + 1 } // extra bit for sign
    fn shr(&self, bits: u32) -> Self { self >> bits }
}

impl TonCellNum for BigUint {
    const SIGNED: bool = false;
    type UnsignedPrimitive = u128;
    type Primitive = u128;
    fn from_bytes(bytes: &[u8]) -> Self { BigUint::from_bytes_be(bytes) }
    fn to_bytes(&self) -> Vec<u8> { BigUint::to_bytes_be(self) }

    fn from_primitive(value: Self::Primitive) -> Self { value.into() }
    fn to_unsigned_primitive(&self) -> Option<Self::UnsignedPrimitive> { None }

    fn is_zero(&self) -> bool { Zero::is_zero(self) }
    fn min_bits_len(&self) -> u32 { self.bits() as u32 }
    fn shr(&self, bits: u32) -> Self { self >> bits }
}
