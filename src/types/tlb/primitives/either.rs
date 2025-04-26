use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonlibError;
use crate::types::tlb::tlb_type::TLBType;
use std::ops::{Deref, DerefMut};

// https://github.com/ton-blockchain/ton/blob/2a68c8610bf28b43b2019a479a70d0606c2a0aa1/crypto/block/block.tlb#L11
#[derive(Clone, Debug, PartialEq)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

// Either X ^X
#[derive(Clone, Debug)]
pub struct EitherRef<T> {
    pub value: T,
    pub layout: EitherRefLayout,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum EitherRefLayout {
    ToCell,
    ToRef,
    Native,
}

impl<L: TLBType, R: TLBType> TLBType for Either<L, R> {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> {
        match parser.read_bit()? {
            false => Ok(Self::Left(L::read(parser)?)),
            true => Ok(Self::Right(R::read(parser)?)),
        }
    }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonlibError> {
        match self {
            Either::Left(left) => {
                builder.write_bit(false)?;
                left.write(builder)
            }
            Either::Right(right) => {
                builder.write_bit(true)?;
                right.write(builder)
            }
        }
    }
}

impl<T: TLBType> TLBType for EitherRef<T> {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> {
        let val = match parser.read_bit()? {
            false => EitherRef {
                value: TLBType::read(parser)?,
                layout: EitherRefLayout::ToCell,
            },
            true => EitherRef {
                value: TLBType::from_cell(parser.read_next_ref()?)?,
                layout: EitherRefLayout::ToRef,
            },
        };
        Ok(val)
    }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonlibError> {
        let cell = self.value.to_cell()?;
        let serial_layout = match self.layout {
            EitherRefLayout::ToCell => EitherRefLayout::ToCell,
            EitherRefLayout::ToRef => EitherRefLayout::ToRef,
            EitherRefLayout::Native => {
                if cell.data_bits_len < builder.data_bits_left() as usize {
                    EitherRefLayout::ToCell
                } else {
                    EitherRefLayout::ToRef
                }
            }
        };
        match serial_layout {
            EitherRefLayout::ToCell => {
                builder.write_bit(false)?;
                builder.write_cell(&cell)?;
            }
            EitherRefLayout::ToRef => {
                builder.write_bit(true)?;
                builder.write_ref(cell.into_ref())?;
            }
            _ => unreachable!("Invalid EitherRefLayout value"),
        };
        Ok(())
    }
}

impl<T> EitherRef<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            layout: EitherRefLayout::Native,
        }
    }
}
impl<T> Deref for EitherRef<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.value }
}
impl<T> DerefMut for EitherRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}
impl<T: PartialEq> PartialEq for EitherRef<T> {
    fn eq(&self, other: &Self) -> bool { self.value == other.value }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::ton_cell::TonCell;
    use crate::types::tlb::primitives::EitherRefLayout::{ToCell, ToRef};
    use crate::types::tlb::primitives::_test_types::{TestType1, TestType2};
    use tokio_test::assert_ok;
    use ton_lib_macros::TLBDerive;

    #[test]
    fn test_either_ref() -> anyhow::Result<()> {
        let obj1 = EitherRef {
            value: TestType1 { value: 1 },
            layout: ToCell,
        };

        let obj2 = EitherRef {
            value: TestType2 { value: 2 },
            layout: ToRef,
        };

        let obj3 = EitherRef {
            value: TestType1 { value: 3 },
            layout: EitherRefLayout::Native,
        };

        let mut builder = CellBuilder::new();
        obj1.write(&mut builder)?;
        obj2.write(&mut builder)?;
        obj3.write(&mut builder)?;
        let cell = builder.build()?;
        let mut parser = CellParser::new(&cell);
        let parsed_obj1 = EitherRef::<TestType1>::read(&mut parser)?;
        let parsed_obj2 = EitherRef::<TestType2>::read(&mut parser)?;
        let parsed_obj3 = EitherRef::<TestType1>::read(&mut parser)?;
        assert_eq!(obj1, parsed_obj1);
        assert_eq!(parsed_obj1.layout, ToCell);
        assert_eq!(obj2, parsed_obj2);
        assert_eq!(parsed_obj2.layout, ToRef);

        assert_eq!(obj3.value, parsed_obj3.value);
        assert_eq!(parsed_obj1.layout, ToCell);
        Ok(())
    }

    #[test]
    fn test_either() -> anyhow::Result<()> {
        let obj1: Either<TestType1, TestType2> = Either::Left(TestType1 { value: 1 });
        let obj2: Either<TestType1, TestType2> = Either::Right(TestType2 { value: 2 });
        let mut builder = CellBuilder::new();
        obj1.write(&mut builder)?;
        obj2.write(&mut builder)?;
        let cell = builder.build()?;
        let mut parser = CellParser::new(&cell);
        let parsed_obj1 = TLBType::read(&mut parser)?;
        let parsed_obj2 = TLBType::read(&mut parser)?;
        assert_eq!(obj1, parsed_obj1);
        assert_eq!(obj2, parsed_obj2);

        // check raw data
        let mut parser = CellParser::new(&cell);
        assert!(!parser.read_bit()?);
        assert_ok!(parser.read_bits(32)); // skipping
        assert!(parser.read_bit()?);
        Ok(())
    }

    #[test]
    fn test_either_recursive() -> anyhow::Result<()> {
        #[derive(Debug, PartialEq, Clone)]
        enum List {
            Empty,
            Some(Box<Item>),
        }

        #[derive(Debug, PartialEq, Clone, TLBDerive)]
        struct Item {
            next: EitherRef<TonCell>,
            number1: u128,
            number2: u128,
            number3: u128,
        }

        impl TLBType for List {
            fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> {
                match parser.data_bits_left()? {
                    0 => Ok(Self::Empty),
                    _ => Ok(Self::Some(TLBType::read(parser)?)),
                }
            }

            fn write_definition(&self, dst: &mut CellBuilder) -> Result<(), TonlibError> {
                match self {
                    List::Empty => {}
                    List::Some(item) => item.write(dst)?,
                }
                Ok(())
            }
        }

        let new_list = List::Some(Box::from(Item {
            next: EitherRef {
                value: List::Some(Box::from(Item {
                    next: EitherRef {
                        value: List::Empty.to_cell()?,
                        layout: EitherRefLayout::Native,
                    },
                    number1: 1,
                    number2: 1,
                    number3: 1,
                }))
                .to_cell()?,
                layout: EitherRefLayout::Native,
            },
            number1: 1,
            number2: 2,
            number3: 3,
        }));

        assert_ok!(new_list.to_cell());
        Ok(())
    }
}
