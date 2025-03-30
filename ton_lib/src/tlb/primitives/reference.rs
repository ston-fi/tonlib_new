// use crate::errors::TLBResult;
// use crate::tlb_type::TLBType;
// use std::ops::{Deref, DerefMut};
// use ton_lib_cell::build_parse::builder::TonCellBuilder;
// use ton_lib_cell::build_parse::parser::TonCellParser;
//
// #[derive(Debug, PartialEq, Clone)]
// pub struct Ref<T>(pub T);
//
// impl<T> Ref<T> {
//     pub const fn new(value: T) -> Self { Ref(value) }
// }
//
// impl<T> Deref for Ref<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target { &self.0 }
// }
//
// impl<T> DerefMut for Ref<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
// }
//
// impl<T: TLBType> TLBType for Ref<T> {
//     fn read_def(parser: &mut TonCellParser) -> TLBResult<Ref<T>> {
//         Ok(Ref(T::from_cell(parser.read_next_ref()?)?))
//     }
//
//     fn write_def(&self, dst: &mut TonCellBuilder) -> TLBResult<()> {
//         dst.write_ref(self.0.to_cell()?.into_ref())?;
//         Ok(())
//     }
// }
//
// #[cfg(test)]
// mod test {
//     use crate::primitives::_test_types::TestType1;
//     use crate::primitives::reference::Ref;
//     use crate::tlb_type::TLBType;
//     use ton_lib_cell::build_parse::builder::TonCellBuilder;
//     use ton_lib_cell::cell::ton_cell::TonCell;
//
//     #[test]
//     fn test_ref() -> anyhow::Result<()> {
//         let obj = Ref::new(TestType1 { value: 1 });
//         let mut builder = TonCellBuilder::new();
//         obj.write(&mut builder)?;
//         let cell = builder.build()?;
//         assert_eq!(cell.refs_count(), 1);
//         let parsed_back = Ref::<TestType1>::from_cell(&cell)?;
//         assert_eq!(obj, parsed_back);
//         Ok(())
//     }
// }
