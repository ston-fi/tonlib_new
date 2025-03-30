use crate::cell::ton_number::traits::TonBigNumber;
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::Zero;

impl TonBigNumber for BigInt {
    const SIGNED: bool = true;
    fn is_negative(&self) -> bool { num_traits::Signed::is_negative(self) }
    fn is_zero(&self) -> bool { Zero::is_zero(self) }
    fn zero() -> Self { Zero::zero() }
    fn min_bits_len(&self) -> u32 { self.bits() as u32 + 1 } // extra bit for sign
    fn to_unsigned_bytes_be(&self) -> Vec<u8> { BigInt::to_bytes_be(self).1 }

    fn from_unsigned_bytes_be(negative: bool, bytes: &[u8]) -> BigInt {
        let sign = if negative { Sign::Minus } else { Sign::Plus };
        BigInt::from_bytes_be(sign, bytes)
    }
    fn shr(&self, bits: u32) -> Self { self >> bits }
}

impl TonBigNumber for BigUint {
    const SIGNED: bool = false;
    fn is_negative(&self) -> bool { false }
    fn is_zero(&self) -> bool { Zero::is_zero(self) }
    fn zero() -> Self { Zero::zero() }
    fn min_bits_len(&self) -> u32 { self.bits() as u32 }
    fn to_unsigned_bytes_be(&self) -> Vec<u8> { BigUint::to_bytes_be(self) }

    fn from_unsigned_bytes_be(_negative: bool, bytes: &[u8]) -> BigUint { BigUint::from_bytes_be(bytes) }
    fn shr(&self, bits: u32) -> Self { self >> bits }
}
