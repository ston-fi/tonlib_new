use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonLibError;
use crate::tlb::tlb_type::TLBType;

impl TLBType for bool {
    fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> { parser.read_bit() }

    fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        builder.write_bit(*self)?;
        Ok(())
    }
}
