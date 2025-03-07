use crate::cell_build_parse::builder::CellBuilder;
use crate::cell_build_parse::parser::CellParser;
use crate::errors::TonLibError;
use crate::tlb::tlb_type::TLBType;
use std::collections::HashMap;

impl<K: TLBType, V: TLBType> TLBType for HashMap<K, V> {
    fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
        if parser.read_bit()? {
            todo!()
        } else {
            Ok(HashMap::new())
        }
    }

    fn write_def(&self, dst: &mut CellBuilder) -> Result<(), TonLibError> {
        if self.is_empty() {
            dst.write_bit(false)?;
            return Ok(());
        }
        dst.write_bit(true)?;
        todo!()
    }
}
