use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonLibError;
use crate::tlb::tlb_type::TLBType;

#[derive(Debug, PartialEq, Clone)]
pub(super) struct TestType1 {
    pub(super) value: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub(super) struct TestType2 {
    pub(super) value: i64,
}

impl TLBType for TestType1 {
    fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
        Ok(TestType1 {
            value: parser.read_num(32)?,
        })
    }

    fn write_def(&self, dst: &mut CellBuilder) -> Result<(), TonLibError> {
        dst.write_num(&self.value, 32)?;
        Ok(())
    }
}

impl TLBType for TestType2 {
    fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
        Ok(TestType2 {
            value: parser.read_num(64)?,
        })
    }

    fn write_def(&self, dst: &mut CellBuilder) -> Result<(), TonLibError> {
        dst.write_num(&self.value, 64)?;
        Ok(())
    }
}
