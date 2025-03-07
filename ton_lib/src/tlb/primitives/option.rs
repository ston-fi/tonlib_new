use crate::cell_build_parse::builder::CellBuilder;
use crate::cell_build_parse::parser::CellParser;
use crate::errors::TonLibError;
use crate::tlb::tlb_type::TLBType;

// Maybe X
impl<T: TLBType> TLBType for Option<T> {
    fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
        match parser.read_bit()? {
            false => Ok(None),
            true => Ok(Some(T::read(parser)?)),
        }
    }

    fn write_def(&self, dst: &mut CellBuilder) -> Result<(), TonLibError> {
        match self {
            None => {
                dst.write_bit(false)?;
            }
            Some(value) => {
                dst.write_bit(true)?;
                value.write(dst)?;
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tlb::primitives::_test_types::TestType1;
    use tokio_test::*;

    #[test]
    fn test_option() -> anyhow::Result<()> {
        let obj1 = Some(TestType1 { value: 1 });
        let obj2: Option<TestType1> = None;
        let mut builder = CellBuilder::new();
        obj1.write(&mut builder)?;
        obj2.write(&mut builder)?;

        let cell = builder.build()?;
        let mut parser = CellParser::new(&cell);
        let parsed_obj1: Option<TestType1> = TLBType::read(&mut parser)?;
        let parsed_obj2: Option<TestType1> = TLBType::read(&mut parser)?;
        assert_eq!(obj1, parsed_obj1);
        assert_eq!(None, parsed_obj2);

        // check layout
        let mut parser = CellParser::new(&cell);
        assert!(parser.read_bit()?); // Some
        assert_ok!(parser.read_bits(32, &mut [0; 32])); // skipping
        assert!(!parser.read_bit()?); // None
        Ok(())
    }
}
