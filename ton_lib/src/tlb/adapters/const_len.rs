use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::cell::ton_cell_num::TonCellNum;
use crate::errors::TonLibError;
use crate::tlb::tlb_type::TLBType;
use std::fmt::Debug;

/// ConstLen - length is known at compile time. It's not supposed to be used directly.
///
/// use `#[tlb_derive(bits_len = {BITS_LEN})]` instead
/// Should be for reading (owning underlying data)
#[derive(Debug, PartialEq)]
pub struct ConstLen<T, const BITS_LEN: u32>(pub T);

/// Should be used for writing (borrowing underlying data)
#[derive(Debug, PartialEq)]
pub struct ConstLenRef<'a, T, const BITS_LEN: u32>(pub &'a T);

// === TLBType for TonCellNum ===
impl<T: TonCellNum, const L: u32> TLBType for ConstLen<T, L> {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonLibError> {
        let data = parser.read_num(L)?;
        Ok(Self(data))
    }
    fn write_definition(&self, _builder: &mut CellBuilder) -> Result<(), TonLibError> {
        unimplemented!("use ConstLenRef for writing")
    }
}

impl<T: TonCellNum, const L: u32> TLBType for ConstLenRef<'_, T, L> {
    fn read_definition(_parser: &mut CellParser) -> Result<Self, TonLibError> {
        unimplemented!("use ConstLen for reading")
    }
    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        builder.write_num(self.0, L)?;
        Ok(())
    }
}

// === TLBType for Vec<u8> ===
impl<const BITS_LEN: u32> TLBType for ConstLen<Vec<u8>, BITS_LEN> {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonLibError> {
        let data: Vec<u8> = parser.read_bits(BITS_LEN)?;
        Ok(Self(data))
    }
    fn write_definition(&self, _builder: &mut CellBuilder) -> Result<(), TonLibError> {
        unimplemented!("use ConstLenRef for writing")
    }
}

// === TLBType for &Vec<u8> ===
impl<const BITS_LEN: u32> TLBType for ConstLenRef<'_, Vec<u8>, BITS_LEN> {
    fn read_definition(_parser: &mut CellParser) -> Result<Self, TonLibError> {
        unimplemented!("use ConstLen for reading")
    }
    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        builder.write_bits(self.0, BITS_LEN)?;
        Ok(())
    }
}

// === TLBType for Option<T> ===
impl<T, const BITS_LEN: u32> TLBType for ConstLen<Option<T>, BITS_LEN>
where
    ConstLen<T, BITS_LEN>: TLBType,
{
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonLibError> {
        let val = Option::<ConstLen<T, BITS_LEN>>::read(parser)?;
        Ok(Self(val.map(|v| v.0)))
    }
    fn write_definition(&self, _builder: &mut CellBuilder) -> Result<(), TonLibError> {
        unimplemented!("use ConstLenRef for writing")
    }
}

// === TLBType for &Option<T> ===
impl<'a, T, const BITS_LEN: u32> TLBType for ConstLenRef<'a, Option<T>, BITS_LEN>
where
    ConstLenRef<'a, T, BITS_LEN>: TLBType,
{
    fn read_definition(_parser: &mut CellParser) -> Result<Self, TonLibError> {
        unimplemented!("use ConstLen for reading")
    }
    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        let val = self.0.as_ref().map(|v| ConstLenRef::<T, BITS_LEN>(v));
        val.write(builder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tlb::tlb_type::TLBType;

    #[test]
    fn test_const_len() -> anyhow::Result<()> {
        let obj = ConstLenRef::<_, 24>(&1u32);
        let cell = obj.to_cell()?;
        assert_eq!(&cell.data, &[0, 0, 1]);
        let parsed = ConstLen::<u32, 24>::from_cell(&cell)?;
        assert_eq!(obj.0, &parsed.0);
        Ok(())
    }
}
