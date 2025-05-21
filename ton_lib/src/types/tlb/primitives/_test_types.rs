use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonlibError;
use crate::types::tlb::TLB;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct TestType1 {
    pub(crate) value: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct TestType2 {
    pub(crate) value: i64,
}

impl TLB for TestType1 {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> {
        Ok(TestType1 {
            value: parser.read_num(32)?,
        })
    }

    fn write_definition(&self, dst: &mut CellBuilder) -> Result<(), TonlibError> {
        dst.write_num(&self.value, 32)?;
        Ok(())
    }
}

impl TLB for TestType2 {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> {
        Ok(TestType2 {
            value: parser.read_num(64)?,
        })
    }

    fn write_definition(&self, dst: &mut CellBuilder) -> Result<(), TonlibError> {
        dst.write_num(&self.value, 64)?;
        Ok(())
    }
}
