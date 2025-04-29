use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonlibError;
use crate::types::tlb::tlb_type::TLBType;
use std::marker::PhantomData;

/// TLBRef - allows to save object in a reference cell.
///
/// use `#[tlb_derive(adapter="TLBRef")]` to apply in automatically in TLBDerive macro
#[derive(Debug, Clone, PartialEq)]
pub struct TLBRef<T: TLBType>(PhantomData<T>);

impl<T: TLBType> TLBRef<T> {
    pub fn new() -> Self { TLBRef(PhantomData) }

    pub fn read(&self, parser: &mut CellParser) -> Result<T, TonlibError> { T::from_cell(parser.read_next_ref()?) }

    pub fn write(&self, builder: &mut CellBuilder, val: &T) -> Result<(), TonlibError> {
        builder.write_ref(val.to_cell_ref()?)
    }
}

impl<T: TLBType> Default for TLBRef<T> {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ton_lib_macros::TLBDerive;

    #[test]
    fn test_tlb_ref() -> anyhow::Result<()> {
        let mut builder = CellBuilder::new();
        TLBRef::<bool>::new().write(&mut builder, &true)?;
        let cell = builder.build()?;
        assert_eq!(cell.refs.len(), 1);
        assert_eq!(cell.refs[0].data, vec![0b10000000]);

        let parsed = TLBRef::<bool>::new().read(&mut CellParser::new(&cell))?;
        assert!(parsed);
        Ok(())
    }

    #[derive(TLBDerive, PartialEq, Debug)]
    struct TestStruct {
        #[tlb_derive(adapter = "TLBRef::<u8>::new()")]
        pub a: u8,
        #[tlb_derive(adapter = "TLBRef")]
        pub b: u8,
    }

    #[test]
    fn test_tlb_ref_derive() -> anyhow::Result<()> {
        let expected = TestStruct { a: 255, b: 255 };
        let cell = expected.to_cell()?;
        assert_eq!(cell.refs.len(), 2);
        assert_eq!(cell.refs[0].data, vec![255]);
        assert_eq!(cell.refs[1].data, vec![255]);

        let parsed = TestStruct::from_cell(&cell)?;
        assert_eq!(parsed, expected);
        Ok(())
    }
}
