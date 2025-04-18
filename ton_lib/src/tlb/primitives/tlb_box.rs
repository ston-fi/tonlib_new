use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonLibError;
use crate::tlb::tlb_type::TLBType;
use std::ops::Deref;

impl<T: TLBType> TLBType for Box<T> {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonLibError> { Ok(Box::new(T::read(parser)?)) }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> { self.deref().write(builder) }
}
