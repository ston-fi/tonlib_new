use crate::cell::meta::cell_meta::CellMeta;
use crate::cell::ton_cell::{write_cell_display, TonCell, TonCellRef, TonCellRefsStore};
use std::fmt::Display;
use std::sync::Arc;

/// Owns the data - must be used for writing
#[derive(Debug, Clone)]
pub struct CellOwned {
    meta: CellMeta,
    data: Vec<u8>,
    data_bits_len: usize,
    refs: TonCellRefsStore,
}

impl CellOwned {
    pub const EMPTY: CellOwned = CellOwned::new(CellMeta::EMPTY_CELL_META, vec![], 0, TonCellRefsStore::new());

    pub const fn new(meta: CellMeta, data: Vec<u8>, data_bits_len: usize, refs: TonCellRefsStore) -> Self {
        Self {
            meta,
            data,
            data_bits_len,
            refs,
        }
    }
}

unsafe impl Sync for CellOwned {}
unsafe impl Send for CellOwned {}

impl TonCell for CellOwned {
    fn from_data(meta: CellMeta, data: Vec<u8>, data_bits_len: usize, refs: TonCellRefsStore) -> Self
    where
        Self: Sized,
    {
        Self::new(meta, data, data_bits_len, refs)
    }
    fn get_meta(&self) -> &CellMeta { &self.meta }
    fn get_data(&self) -> &[u8] { &self.data }
    fn get_data_bits_len(&self) -> usize { self.data_bits_len }
    fn get_ref(&self, index: usize) -> Option<&TonCellRef> { self.refs.get(index) }
}

impl PartialEq for CellOwned {
    fn eq(&self, other: &Self) -> bool { self.hash() == other.hash() }
}

impl Eq for CellOwned {}

impl From<CellOwned> for TonCellRef {
    fn from(value: CellOwned) -> Self { Arc::new(value) }
}

impl Display for CellOwned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write_cell_display(f, self, 0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_owned_create() {
        let child = CellOwned {
            meta: CellMeta::EMPTY_CELL_META,
            data: vec![0x01, 0x02, 0x03],
            data_bits_len: 24,
            refs: TonCellRefsStore::new(),
        }
        .into_ref();

        let _cell = CellOwned {
            meta: CellMeta::EMPTY_CELL_META,
            data: vec![0x04, 0x05, 0x06],
            data_bits_len: 24,
            refs: TonCellRefsStore::from([child]),
        };
    }
}
