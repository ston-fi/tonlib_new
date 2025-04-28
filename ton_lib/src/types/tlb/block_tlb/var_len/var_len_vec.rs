use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::var_len::VarLen;
use crate::types::tlb::tlb_type::TLBType;

impl<const BITS_LEN_LEN: usize, const BL: bool> TLBType for VarLen<Vec<u8>, BITS_LEN_LEN, BL> {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> {
        let len = parser.read_num(BITS_LEN_LEN)?;
        let bits_len = if BL { len * 8 } else { len };
        let data = parser.read_bits(bits_len)?;
        Ok(Self { data, len })
    }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonlibError> {
        builder.write_num(&self.len, BITS_LEN_LEN)?;
        let bits_len = if BL { self.len * 8 } else { self.len };
        builder.write_bits(&self.data, bits_len)?;
        Ok(())
    }
}
