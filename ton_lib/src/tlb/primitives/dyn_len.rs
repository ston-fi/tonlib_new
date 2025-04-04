use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonLibError;
use crate::tlb::tlb_type::TLBType;
use std::ops::{Deref, DerefMut};

/// ConstLen - length is fixed and known at compile time
#[derive(Debug, Clone, PartialEq)]
pub struct ConstLen<T, const BITS_LEN: u32> {
    pub data: T,
}

/// VerLen: BITS_LEN_LEN specifies the number of bits used to store the length of the data
#[derive(Debug, Clone, PartialEq)]
pub struct VarLen<T, const BITS_LEN_LEN: u32, const LEN_IN_BYTES: bool = false> {
    pub len: u32,
    pub data: T,
}

/// new
impl<T, const L: u32> ConstLen<T, L> {
    pub fn new<D: Into<T>>(data: D) -> Self { Self { data: data.into() } }
}

impl<T, const L: u32, const BL: bool> VarLen<T, L, BL> {
    pub fn new<D: Into<T>>(len: u32, data: D) -> Self { Self { len, data: data.into() } }
}

// From
impl<T, const L: u32> From<T> for ConstLen<T, L> {
    fn from(value: T) -> Self { Self { data: value } }
}

impl<T, const L: u32, const LB: bool> From<(u32, T)> for VarLen<T, L, LB> {
    fn from(value: (u32, T)) -> Self {
        Self {
            len: value.0,
            data: value.1,
        }
    }
}

// Deref, DeferMut
impl<T, const L: u32> Deref for ConstLen<T, L> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.data }
}

impl<T, const L: u32> DerefMut for ConstLen<T, L> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.data }
}

impl<T, const L: u32, const BL: bool> Deref for VarLen<T, L, BL> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.data }
}

impl<T, const L: u32, const BL: bool> DerefMut for VarLen<T, L, BL> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.data }
}

/// Impl for Vec<u8>
impl<const BITS_LEN: u32> TLBType for ConstLen<Vec<u8>, BITS_LEN> {
    fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
        let data: Vec<u8> = parser.read_bits(BITS_LEN)?;
        Ok(Self { data })
    }

    fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        builder.write_bits(&self.data, BITS_LEN)?;
        Ok(())
    }
}

impl<const BITS_LEN_LEN: u32, const BL: bool> TLBType for VarLen<Vec<u8>, BITS_LEN_LEN, BL> {
    fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
        let len = parser.read_num(BITS_LEN_LEN)?;
        let data = if BL {
            parser.read_bytes(len)?
        } else {
            parser.read_bits(len)?
        };
        Ok(Self { len, data })
    }

    fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        builder.write_num(&self.len, BITS_LEN_LEN)?;
        let bits_len = if BL { self.len * 8 } else { self.len };
        builder.write_bits(&self.data, bits_len)?;
        Ok(())
    }
}

/// Implementations for TonCellNum
macro_rules! dyn_len_num_impl {
    ($t:ty) => {
        impl<const L: u32> TLBType for ConstLen<$t, L> {
            fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
                let data = parser.read_num(L)?;
                Ok(Self { data })
            }

            fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
                builder.write_num(&self.data, L)?;
                Ok(())
            }
        }

        impl<const L: u32, const BL: bool> TLBType for VarLen<$t, L, BL> {
            fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
                let len = parser.read_num(L)?;
                let data = if BL {
                    parser.read_num(len * 8)?
                } else {
                    parser.read_num(len)?
                };
                Ok(Self { len, data })
            }

            fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
                builder.write_num(&self.len, L)?;
                if BL {
                    builder.write_num(&self.data, self.len * 8)?;
                } else {
                    builder.write_num(&self.data, self.len)?;
                }
                Ok(())
            }
        }
    };
}

dyn_len_num_impl!(i8);
dyn_len_num_impl!(i16);
dyn_len_num_impl!(i32);
dyn_len_num_impl!(i64);
dyn_len_num_impl!(i128);
dyn_len_num_impl!(u8);
dyn_len_num_impl!(u16);
dyn_len_num_impl!(u32);
dyn_len_num_impl!(u64);
dyn_len_num_impl!(u128);

#[cfg(feature = "num-bigint")]
dyn_len_num_impl!(num_bigint::BigUint);
#[cfg(feature = "num-bigint")]
dyn_len_num_impl!(num_bigint::BigInt);
