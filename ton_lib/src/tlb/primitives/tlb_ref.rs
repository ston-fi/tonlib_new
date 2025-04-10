use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonLibError;
use crate::tlb::TLBType;
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, Clone)]
pub struct TLBRef<T>(pub T);

impl<T> TLBRef<T> {
    pub const fn new(value: T) -> Self { TLBRef(value) }
}

impl<T> Deref for TLBRef<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> DerefMut for TLBRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<T: TLBType> TLBType for TLBRef<T> {
    fn read_def(parser: &mut CellParser) -> Result<TLBRef<T>, TonLibError> {
        Ok(TLBRef(T::from_cell(parser.read_next_ref()?)?))
    }

    fn write_def(&self, dst: &mut CellBuilder) -> Result<(), TonLibError> {
        dst.write_ref(self.0.to_cell()?.into_ref())?;
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use crate::tlb::primitives::_test_types::TestType1;
    use crate::tlb::primitives::tlb_ref::CellBuilder;
    use crate::tlb::primitives::tlb_ref::TLBRef;
    use crate::tlb::TLBType;

    #[test]
    fn test_ref() -> anyhow::Result<()> {
        let obj = TLBRef::new(TestType1 { value: 1 });
        let mut builder = CellBuilder::new();
        obj.write(&mut builder)?;
        let cell = builder.build()?;
        assert_eq!(cell.refs.len(), 1);
        let parsed_back = TLBRef::<TestType1>::from_cell(&cell)?;
        assert_eq!(obj, parsed_back);

        // parse ref directly
        let parsed_from_ref = TestType1::from_cell(&cell.refs[0])?;
        assert_eq!(obj.0, parsed_from_ref);
        Ok(())
    }
}
