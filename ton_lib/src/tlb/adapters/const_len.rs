use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::cell::ton_cell_num::TonCellNum;
use crate::errors::TonLibError;
use std::marker::PhantomData;

/// Adaptor to write data with fixed length into a cell.
///
/// Usage: `#[tlb_derive(adapter="ConstLen::<_>::new({BITS_LEN})")]`
pub struct ConstLen<T> {
    bits_len: u32,
    _phantom: PhantomData<T>,
}

impl<T> ConstLen<T> {
    pub fn new(bits_len: u32) -> Self {
        Self {
            bits_len,
            _phantom: PhantomData,
        }
    }
}

impl<T: TonCellNum> ConstLen<T> {
    pub fn read(&self, parser: &mut CellParser) -> Result<T, TonLibError> { parser.read_num(self.bits_len) }
    pub fn write(&self, builder: &mut CellBuilder, val: &T) -> Result<(), TonLibError> {
        builder.write_num(val, self.bits_len)
    }
}

impl<T: TonCellNum> ConstLen<Option<T>> {
    pub fn read(&self, parser: &mut CellParser) -> Result<Option<T>, TonLibError> {
        match parser.read_bit()? {
            true => Ok(Some(parser.read_num(self.bits_len)?)),
            false => Ok(None),
        }
    }
    pub fn write(&self, builder: &mut CellBuilder, val: &Option<T>) -> Result<(), TonLibError> {
        builder.write_bit(val.is_some())?;
        if let Some(val) = val {
            return builder.write_num(val, self.bits_len);
        };
        Ok(())
    }
}

impl ConstLen<Vec<u8>> {
    pub fn read(&self, parser: &mut CellParser) -> Result<Vec<u8>, TonLibError> { parser.read_bits(self.bits_len) }
    pub fn write(&self, builder: &mut CellBuilder, val: &Vec<u8>) -> Result<(), TonLibError> {
        builder.write_bits(val, self.bits_len)
    }
}

impl ConstLen<Option<Vec<u8>>> {
    pub fn read(&self, parser: &mut CellParser) -> Result<Option<Vec<u8>>, TonLibError> {
        match parser.read_bit()? {
            true => Ok(Some(parser.read_bits(self.bits_len)?)),
            false => Ok(None),
        }
    }
    pub fn write(&self, builder: &mut CellBuilder, val: &Option<Vec<u8>>) -> Result<(), TonLibError> {
        match val {
            Some(val) => {
                builder.write_bit(true)?;
                builder.write_bits(val, self.bits_len)
            }
            None => builder.write_bit(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_len() -> anyhow::Result<()> {
        let mut builder = CellBuilder::new();
        ConstLen::<u32>::new(24).write(&mut builder, &1u32)?;
        let cell = builder.build()?;
        assert_eq!(&cell.data, &[0, 0, 1]);
        let parsed = ConstLen::<u32>::new(24).read(&mut CellParser::new(&cell))?;
        assert_eq!(parsed, 1u32);
        Ok(())
    }
}
