use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonlibError;
use crate::types::tlb::TLB;
use std::ops::Deref;
use std::sync::Arc;

impl<T: TLB> TLB for Arc<T> {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> { Ok(Arc::new(T::read(parser)?)) }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonlibError> { self.deref().write(builder) }
}

impl<T: TLB> TLB for Box<T> {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> { Ok(Box::new(T::read(parser)?)) }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonlibError> { self.deref().write(builder) }
}
