use crate::cell::ton_number::traits::TonNumber;

macro_rules! ton_number_primitive_impl {
    ($src:ty, $dst:ty) => {
        impl TonNumber for $src {
            type UnsignedType = $dst;
            fn to_unsigned(&self) -> Self::UnsignedType { *self as $dst }
        }
    };
}

ton_number_primitive_impl!(i8, u8);
ton_number_primitive_impl!(u8, u8);
ton_number_primitive_impl!(i16, u16);
ton_number_primitive_impl!(u16, u16);
ton_number_primitive_impl!(i32, u32);
ton_number_primitive_impl!(u32, u32);
ton_number_primitive_impl!(i64, u64);
ton_number_primitive_impl!(u64, u64);
ton_number_primitive_impl!(i128, u128);
ton_number_primitive_impl!(u128, u128);
