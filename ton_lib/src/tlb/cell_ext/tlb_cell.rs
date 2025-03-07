use crate::cell::cell_owned::CellOwned;
use crate::cell::meta::cell_meta::CellMeta;
use crate::cell::meta::cell_type::CellType;
use crate::cell::ton_cell::{TonCell, TonCellRef};
use crate::cell::ton_hash::TonHash;
use crate::cell_build_parse::builder::CellBuilder;
use crate::cell_build_parse::parser::CellParser;
use crate::errors::TonLibError;
use crate::tlb::tlb_type::TLBType;

impl TLBType for CellOwned {
    fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
        let bits_left = parser.data_bits_left()?;

        if parser.cell.get_data_bits_len() == bits_left as usize && parser.next_ref_pos == 0 {
            let mut data = Vec::with_capacity((bits_left as usize) / 8 + 1);
            parser.read_bits(bits_left, &mut data)?;

            let mut refs = Vec::with_capacity(parser.cell.refs_count());
            for i in 0..parser.cell.refs_count() {
                refs.push(parser.cell.get_ref(i).unwrap().clone());
            }

            Ok(Self::new(
                parser.cell.get_meta().clone(),
                parser.cell.get_data().to_vec(),
                parser.cell.get_data_bits_len(),
                refs,
            ))
        } else {
            let mut data = Vec::with_capacity((bits_left as usize) / 8 + 1);
            parser.read_bits(bits_left, &mut data)?;

            let mut refs = vec![];
            while let Ok(ref_cell) = parser.read_next_ref() {
                refs.push(ref_cell.clone());
            }
            let meta = CellMeta::new(CellType::Ordinary, &data, bits_left as usize, &refs)?;
            Ok(Self::new(meta, data, bits_left as usize, refs))
        }
    }

    fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        builder.write_bits(self.get_data(), self.get_data_bits_len() as u32)?;
        for i in 0..self.refs_count() {
            builder.write_ref(self.get_ref(i).unwrap().clone())?;
        }
        Ok(())
    }
}

impl TLBType for TonCellRef {
    fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> { Ok(parser.read_next_ref()?.clone()) }

    fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        builder.write_ref(self.clone())?;
        Ok(())
    }
}

impl TLBType for TonHash {
    fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
        let mut data = [0; TonHash::BYTES_LEN];
        parser.read_bytes(&mut data)?;
        Ok(TonHash::from(data))
    }

    fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        builder.write_bytes(self.as_slice())?;
        Ok(())
    }
}
