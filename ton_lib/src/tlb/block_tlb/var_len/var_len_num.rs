use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::cell::ton_cell_num::TonCellNum;
use crate::errors::TonLibError;
use crate::tlb::block_tlb::var_len::var_len::VarLen;
use crate::tlb::tlb_type::TLBType;

impl<T: TonCellNum, const L: u32, const LEN_IN_BYTES: bool> TLBType for VarLen<T, L, LEN_IN_BYTES> {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonLibError> {
        let len = parser.read_num(L)?;
        let bits_len = if LEN_IN_BYTES { len * 8 } else { len };
        let data = parser.read_num(bits_len)?;
        Ok(Self { data, len })
    }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        builder.write_num(&self.len, L)?;
        let bits_len = if LEN_IN_BYTES { self.len * 8 } else { self.len };
        builder.write_num(&self.data, bits_len)?;
        Ok(())
    }
}
