use crate::cell::meta::cell_meta::CellMeta;
use crate::cell::ton_cell::{TonCellRef, TonCell, TonCellRefsStore};

// Doesn't own the data - nice for reading
#[derive(Debug, Clone)]
pub struct CellSlice<'a> {
    cell: &'a dyn TonCell,
    first_bit: usize,
    last_bit: usize,
    first_ref: usize,
    last_ref: usize,
}

impl<'a> CellSlice<'a> {
    pub fn from_cell(cell: &'a dyn TonCell, first_bit: usize, last_bit: usize, first_ref: usize, last_ref: usize) -> Self {
        Self {
            cell,
            first_bit,
            last_bit,
            first_ref,
            last_ref,
        }
    }
}
// 
// impl TonCell for CellSlice<'_> {
//     fn from_data(meta: CellMeta, data: Vec<u8>, data_bits_len: usize, refs: TonCellRefsStore) -> Self
//     where
//         Self: Sized
//     {
//         todo!()
//     }
//     fn get_meta(&self) -> &CellMeta { self.meta }
//     fn get_data(&self) -> &[u8] { self.data }
// 
//     fn get_data_bits_len(&self) -> usize { self.data_bits_len }
//     fn get_refs(&self) -> &[TonCellRef] { self.refs }
// }
