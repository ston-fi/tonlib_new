use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonLibError;
use crate::tlb::tlb_type::TLBType;

/// TLBRef - allows to save object in a reference cell.
///
/// use `#[tlb_derive(TLBRef)]` to apply in automatically in TLBDerive macro
#[derive(Debug, Clone, PartialEq)]
pub struct TLBRef<T: TLBType>(pub T);

impl<T: TLBType> TLBType for TLBRef<T> {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonLibError> {
        let cell_ref = parser.read_next_ref()?;
        Ok(Self(TLBType::from_cell(cell_ref)?))
    }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        let cell_ref = self.0.to_cell()?.into_ref();
        builder.write_ref(cell_ref)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tlb_ref() -> anyhow::Result<()> {
        let mut builder = CellBuilder::new();
        let val = TLBRef(true);
        val.write(&mut builder)?;
        let cell = builder.build()?;
        assert_eq!(cell.refs.len(), 1);
        assert_eq!(cell.refs[0].data, vec![0b10000000]);

        let parsed = TLBRef::<bool>::from_cell(&cell)?;
        assert_eq!(parsed, val);
        Ok(())
    }
}
