// use std::sync::Arc;
// use smallvec::SmallVec;
// use crate::cell::meta::cell_meta::CellMeta;
// use crate::cell::ton_cell::{TonCellRef, TonCell, TonCellRefsStore};
//
// // Doesn't own the data - nice for reading
// #[derive(Debug, Clone)]
// pub struct CellSlice<'a> {
//     pub meta: &'a CellMeta,
//     pub data: &'a [u8],
//     pub data_bits_len: usize,
//     pub refs: TonCellRefsStore,
// }
//
// impl<'a> CellSlice<'a> {
//     pub fn new(
//         meta: &'a CellMeta,
//         data: &'a [u8],
//         data_bits_len: usize,
//         refs: TonCellRefsStore,
//     ) -> Self {
//         Self {
//             meta,
//             data,
//             data_bits_len,
//             refs,
//         }
//     }
//
//     pub fn from_cell(cell: &'a dyn TonCell) -> Self {
//         Self::new(cell.get_meta(), cell.get_data(), cell.get_data_bits_len(), SmallVec::from(cell.get_refs()))
//     }
// }
//
// impl TonCell for CellSlice<'_> {
//     fn get_meta(&self) -> &CellMeta { self.meta }
//     fn get_data(&self) -> &[u8] { self.data }
//     fn get_data_bits_len(&self) -> usize { self.data_bits_len }
//     fn get_refs(&self) -> &[TonCellRef] { &self.refs }
// }
