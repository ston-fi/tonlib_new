use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::cell::ton_cell_num::TonCellNum;
use crate::errors::TonLibError;
use std::marker::PhantomData;

/// Adaptor to write data with fixed length into a cell.
///
/// Usage: `#[tlb_derive(adapter="ConstLen", bits_len={BITS_LEN})]`
pub struct ConstLen<T>(PhantomData<T>);

impl<T: TonCellNum> ConstLen<T> {
    pub fn read(parser: &mut CellParser, bits_len: u32) -> Result<T, TonLibError> { parser.read_num(bits_len) }
    pub fn write(builder: &mut CellBuilder, val: &T, bits_len: u32) -> Result<(), TonLibError> {
        builder.write_num(val, bits_len)
    }
}

impl<T: TonCellNum> ConstLen<Option<T>> {
    pub fn read(parser: &mut CellParser, bits_len: u32) -> Result<Option<T>, TonLibError> {
        match parser.read_bit()? {
            true => Ok(Some(parser.read_num(bits_len)?)),
            false => Ok(None),
        }
    }
    pub fn write(builder: &mut CellBuilder, val: &Option<T>, bits_len: u32) -> Result<(), TonLibError> {
        if let Some(val) = val {
            builder.write_bit(true)?;
            builder.write_num(val, bits_len)
        } else {
            builder.write_bit(false)
        }
    }
}

impl ConstLen<Vec<u8>> {
    pub fn read(parser: &mut CellParser, bits_len: u32) -> Result<Vec<u8>, TonLibError> { parser.read_bits(bits_len) }
    pub fn write(builder: &mut CellBuilder, val: &Vec<u8>, bits_len: u32) -> Result<(), TonLibError> {
        builder.write_bits(val, bits_len)
    }
}

impl ConstLen<Option<Vec<u8>>> {
    pub fn read(parser: &mut CellParser, bits_len: u32) -> Result<Option<Vec<u8>>, TonLibError> {
        match parser.read_bit()? {
            true => Ok(Some(parser.read_bits(bits_len)?)),
            false => Ok(None),
        }
    }
    pub fn write(builder: &mut CellBuilder, val: &Option<Vec<u8>>, bits_len: u32) -> Result<(), TonLibError> {
        match val {
            Some(val) => {
                builder.write_bit(true)?;
                builder.write_bits(val, bits_len)
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
        ConstLen::<u32>::write(&mut builder, &1u32, 24)?;
        let cell = builder.build()?;
        assert_eq!(&cell.data, &[0, 0, 1]);
        let parsed = ConstLen::<u32>::read(&mut CellParser::new(&cell), 24)?;
        assert_eq!(parsed, 1u32);
        Ok(())
    }
}
