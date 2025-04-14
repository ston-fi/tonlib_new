use crate::boc::boc::BOC;
use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::cell::meta::cell_meta::CellMeta;
use crate::cell::meta::cell_type::CellType;
use crate::cell::ton_cell::{TonCell, TonCellRef};
use crate::cell::ton_hash::TonHash;
use crate::errors::TonLibError;
use crate::tlb::tlb_type::TLBType;
use std::sync::Arc;

impl TLBType for TonCell {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonLibError> {
        let bits_left = parser.data_bits_left()?;
        if parser.cell.data_bits_len == bits_left as usize && parser.next_ref_pos == 0 {
            let _data = parser.read_bits(bits_left)?; // drain data
            parser.next_ref_pos = parser.cell.refs.len(); // drain refs
            Ok(parser.cell.clone())
        } else {
            let data = parser.read_bits(bits_left)?;
            let refs = Vec::from(&parser.cell.refs[parser.next_ref_pos..]);
            parser.next_ref_pos = parser.cell.refs.len(); // drain refs
            let meta = CellMeta::new(CellType::Ordinary, &data, bits_left as usize, &refs)?;
            Ok(Self {
                meta,
                data,
                data_bits_len: bits_left as usize,
                refs,
            })
        }
    }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        builder.write_bits(&self.data, self.data_bits_len as u32)?;
        self.refs.iter().cloned().try_for_each(|r| builder.write_ref(r))
    }

    fn from_boc(boc: &[u8]) -> Result<Self, TonLibError> {
        // optimization - doesn't copy Cell, just takes ownership
        // unwrap is safe - no one own Reference expect this function
        Ok(Arc::try_unwrap(BOC::from_bytes(boc)?.single_root()?.0).unwrap())
    }

    fn to_cell(&self) -> Result<TonCell, TonLibError> { Ok(self.clone()) }
}

impl TLBType for TonCellRef {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonLibError> { parser.read_next_ref().cloned() }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> { builder.write_ref(self.clone()) }
}

impl TLBType for TonHash {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonLibError> {
        TonHash::from_vec(parser.read_bits(TonHash::BITS_LEN as u32)?)
    }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        builder.write_bits(self.as_slice(), TonHash::BITS_LEN as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tlb_cell() -> anyhow::Result<()> {
        let mut builder = CellBuilder::new();
        builder.write_num(&3u32, 32)?;
        let cell = builder.build()?;
        let parsed = TonCell::from_cell(&cell)?;
        assert_eq!(cell, parsed);
        Ok(())
    }

    #[test]
    fn test_tlb_cell_ref() -> anyhow::Result<()> {
        let mut ref_builder = CellBuilder::new();
        ref_builder.write_num(&3u32, 32)?;
        let cell_ref = ref_builder.build()?.into_ref();

        let mut cell_builder = CellBuilder::new();
        cell_builder.write_ref(cell_ref.clone())?;
        let cell = cell_builder.build()?;
        let parsed = TonCellRef::from_cell(&cell)?;

        assert_eq!(cell_ref, parsed);
        Ok(())
    }
}
