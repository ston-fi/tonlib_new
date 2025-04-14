use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonLibError;
use crate::tlb::tlb_type::TLBType;
use std::ops::{Deref, DerefMut};

// https://github.com/ton-blockchain/ton/blob/ed4682066978f69ffa38dd98912ca77d4f660f66/crypto/block/block.tlb#L33
// Optimized implementation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unary(pub u32);

impl TLBType for Unary {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonLibError> {
        let mut bits_len = 0;
        while parser.read_bit()? {
            bits_len += 1;
        }
        Ok(Unary(bits_len))
    }

    fn write_definition(&self, dst: &mut CellBuilder) -> Result<(), TonLibError> {
        for _ in 0..self.0 {
            dst.write_bit(true)?;
        }
        dst.write_bit(false)
    }
}

impl Deref for Unary {
    type Target = u32;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Unary {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::build_parse::builder::CellBuilder;
    use crate::cell::build_parse::parser::CellParser;

    #[test]
    fn test_unary() -> anyhow::Result<()> {
        let mut builder = CellBuilder::new();
        let unary = Unary(5);
        unary.write(&mut builder)?;
        let cell = builder.build()?;
        assert_eq!(cell.data_bits_len, 6);
        assert_eq!(cell.data, vec![0b11111000]);
        let mut parser = CellParser::new(&cell);
        let parsed_unary = Unary::read(&mut parser)?;
        assert_eq!(parsed_unary, unary);
        Ok(())
    }
}
