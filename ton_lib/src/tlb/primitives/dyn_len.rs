use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::cell::ton_cell_num::TonCellNum;
use crate::errors::TonLibError;
use crate::tlb::tlb_type::TLBType;

/// ConstLen - length is known at compile time. It's not supposed to be used directly.
///
/// use `#[tlb_derive(bits_len = {BITS_LEN})]` instead
#[derive(Debug, Clone, PartialEq)]
pub struct ConstLen<T, const BITS_LEN: u32>(pub T);

/// VarLen: store data len, and then data itself
///
/// BITS_LEN_LEN - number of bits used to store length
///
/// LEN_IN_BYTES - if true, data len is specified in bytes. Otherwise - in bits
#[derive(Debug, Clone, PartialEq)]
pub struct VarLen<T, const BITS_LEN_LEN: u32, const LEN_IN_BYTES: bool = false> {
    pub data: T,
    pub len: u32,
}

// === new ===
impl<T, const L: u32> ConstLen<T, L> {
    pub fn new<D: Into<T>>(data: D) -> Self { Self(data.into()) }
}

impl<T, const L: u32, const BL: bool> VarLen<T, L, BL> {
    pub fn new<D: Into<T>>(data: D, len: u32) -> Self { Self { len, data: data.into() } }
}

// === TLBType for Vec<u8> ===
impl<const BITS_LEN: u32> TLBType for ConstLen<Vec<u8>, BITS_LEN> {
    fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
        let data: Vec<u8> = parser.read_bits(BITS_LEN)?;
        Ok(Self(data))
    }

    fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        builder.write_bits(&self.0, BITS_LEN)?;
        Ok(())
    }
}

impl<const BITS_LEN_LEN: u32, const BL: bool> TLBType for VarLen<Vec<u8>, BITS_LEN_LEN, BL> {
    fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
        let len = parser.read_num(BITS_LEN_LEN)?;
        let bits_len = if BL { len * 8 } else { len };
        let data = parser.read_bits(bits_len)?;
        Ok(Self { data, len })
    }

    fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        builder.write_num(&self.len, BITS_LEN_LEN)?;
        let bits_len = if BL { self.len * 8 } else { self.len };
        builder.write_bits(&self.data, bits_len)?;
        Ok(())
    }
}

// === TLBType for &Vec<u8> ===
impl<const BITS_LEN: u32> TLBType for ConstLen<&Vec<u8>, BITS_LEN> {
    fn read_def(_parser: &mut CellParser) -> Result<Self, TonLibError> {
        unimplemented!("ConstLen::read() can't be called on ref internal type")
    }

    fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        builder.write_bits(self.0, BITS_LEN)?;
        Ok(())
    }
}

// === TLBType for TonCellNum ===
impl<T: TonCellNum, const L: u32> TLBType for ConstLen<T, L> {
    fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
        let data = parser.read_num(L)?;
        Ok(Self(data))
    }

    fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        builder.write_num(&self.0, L)?;
        Ok(())
    }
}

impl<T: TonCellNum, const L: u32, const BL: bool> TLBType for VarLen<T, L, BL> {
    fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
        let len = parser.read_num(L)?;
        let bits_len = if BL { len * 8 } else { len };
        let data = parser.read_num(bits_len)?;
        Ok(Self { data, len })
    }

    fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        builder.write_num(&self.len, L)?;
        let bits_len = if BL { self.len * 8 } else { self.len };
        builder.write_num(&self.data, bits_len)?;
        Ok(())
    }
}

#[rustfmt::skip]
mod core_traits_impl {
    use std::ops::{Deref, DerefMut};
    use crate::tlb::primitives::dyn_len::{ConstLen, VarLen};

    // From
    impl<T, const L: u32> From<T> for ConstLen<T, L> { fn from(value: T) -> Self { Self(value) } }
    impl<T, const L: u32, const LB: bool> From<(u32, T)> for VarLen<T, L, LB> { fn from(value: (u32, T)) -> Self { Self { len: value.0, data: value.1} } }
    
    // Deref
    impl<T, const L: u32> Deref for ConstLen<T, L> { type Target = T; fn deref(&self) -> &Self::Target { &self.0 }}
    impl<T, const L: u32, const BL: bool> Deref for VarLen<T, L, BL> { type Target = T; fn deref(&self) -> &Self::Target { &self.data } }
    
    // DerefMut
    impl<T, const L: u32> DerefMut for ConstLen<T, L> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 } }
    impl<T, const L: u32, const BL: bool> DerefMut for VarLen<T, L, BL> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.data } }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tlb::tlb_type::TLBType;

    #[test]
    fn test_const_len() -> anyhow::Result<()> {
        let obj = ConstLen::<u32, 24>::new(1u8);
        let cell = obj.to_cell()?;
        assert_eq!(&cell.data, &[0, 0, 1]);
        let parsed = ConstLen::<u32, 24>::from_cell(&cell)?;
        assert_eq!(obj, parsed);
        Ok(())
    }

    #[test]
    fn test_var_len() -> anyhow::Result<()> {
        // len in bits
        let obj = VarLen::<u32, 8>::new(1u8, 4);
        let cell = obj.to_cell()?;
        // 8 bits of length (value = 4) + 4 bits of data (value = 1)
        assert_eq!(&cell.data, &[0b00000100, 0b00010000]);
        let parsed = VarLen::<u32, 8>::from_cell(&cell)?;
        assert_eq!(obj, parsed);

        // len in bytes
        let obj = VarLen::<u32, 16, true>::new(1u8, 2);
        let cell = obj.to_cell()?;
        // 16 bits of length (value = 2), and then 16 (value * 8) bits of data (value = 1)
        assert_eq!(&cell.data, &[0b00000000, 0b00000010, 0b00000000, 0b00000001]);
        let parsed = VarLen::<u32, 16, true>::from_cell(&cell)?;
        assert_eq!(obj, parsed);

        Ok(())
    }
}
