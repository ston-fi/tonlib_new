// use std::ops::{Deref, DerefMut};
// use crate::build_parse::builder::CellBuilder;
// use crate::build_parse::parser::CellParser;
// use crate::errors::TonLibError;
// use crate::tlb::tlb_type::{TLBPrefix, TLBType};
//
// pub struct SnakeData<T: AsRef<[u8]> + From<&[u8]>>(T);
//
// impl<T: AsRef<[u8]>> AsRef<[u8]> for SnakeData<T> {
//     fn as_ref(&self) -> &[u8] {
//         self.0.as_ref()
//     }
// }
//
// impl<T> Deref for SnakeData<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// impl<T> DerefMut for SnakeData<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
//
// impl<T: AsRef<[u8]> + From<&[u8]>> TLBType for SnakeData<T> {
//     fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
//         let mut bits_left = parser.data_bits_left()?;
//         let mut data = vec![0; bits_left as usize];
//         while bits_left > 0 {
//             parser.read_bits(bits_left, &mut data)?;
//         }
//         let data = parser.read_
//         let mut cur_parser = parser.clone();
//         todo!()
//     }
//
//     fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
//         todo!()
//     }
//
//     fn prefix() -> &'static TLBPrefix {
//         const PREFIX = TLBPrefix {
//             value: 0,
//             bit_len: 8,
//         };
//     }
// }
