use crate::cell::num::traits::TonCellNum;

macro_rules! ton_cell_num_primitive_impl {
    ($src:ty, $sign:tt, $unsign:ty) => {
        impl TonCellNum for $src {
            const SIGNED: bool = $sign;
            const IS_PRIMITIVE: bool = true;
            type Primitive = $src;
            type UnsignedPrimitive = $unsign;
            fn from_bytes(_bytes: &[u8]) -> Self { unreachable!() }
            fn to_bytes(&self) -> Vec<u8> { unreachable!() }

            fn from_primitive(value: Self::Primitive) -> Self { value }
            fn to_unsigned_primitive(&self) -> Option<Self::UnsignedPrimitive> { Some(*self as $unsign) }

            fn is_zero(&self) -> bool { *self == 0 }
            fn min_bits_len(&self) -> u32 { unreachable!() }
            fn shr(&self, _bits: u32) -> Self { unreachable!() }
        }
    };
}

ton_cell_num_primitive_impl!(i8, true, u8);
ton_cell_num_primitive_impl!(u8, false, u8);
ton_cell_num_primitive_impl!(i16, true, u16);
ton_cell_num_primitive_impl!(u16, false, u16);
ton_cell_num_primitive_impl!(i32, true, u32);
ton_cell_num_primitive_impl!(u32, false, u32);
ton_cell_num_primitive_impl!(i64, true, u64);
ton_cell_num_primitive_impl!(u64, false, u64);
ton_cell_num_primitive_impl!(i128, true, u128);
ton_cell_num_primitive_impl!(u128, false, u128);
