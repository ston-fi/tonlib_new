use crate::cell::meta::cell_meta::CellMeta;
use crate::cell::meta::level_mask::LevelMask;
use crate::cell::ton_hash::TonHash;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TonCell {
    pub meta: CellMeta,
    pub data: Vec<u8>,
    pub data_bits_len: usize,
    pub refs: TonCellRefsStore,
}

impl TonCell {
    pub const EMPTY: Self = TonCell {
        meta: CellMeta::EMPTY_CELL_META,
        data: vec![],
        data_bits_len: 0,
        refs: TonCellRefsStore::new(),
    };

    pub fn hash_for_level(&self, level: LevelMask) -> &TonHash { &self.meta.hashes[level.mask() as usize] }
    pub fn hash(&self) -> &TonHash { self.hash_for_level(LevelMask::MAX_LEVEL) }
    pub fn into_ref(self) -> TonCellRef { TonCellRef(self.into()) }
}

unsafe impl Sync for TonCell {}
unsafe impl Send for TonCell {}

impl PartialEq for TonCell {
    fn eq(&self, other: &Self) -> bool { self.hash() == other.hash() }
}

impl Eq for TonCell {}

impl Display for TonCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write_cell_display(f, self, 0) }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TonCellRef(pub Arc<TonCell>);
impl Deref for TonCellRef {
    type Target = TonCell;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for TonCellRef {
    fn deref_mut(&mut self) -> &mut Self::Target { Arc::get_mut(&mut self.0).unwrap() }
}
impl AsRef<TonCell> for TonCellRef {
    fn as_ref(&self) -> &TonCell { &self.0 }
}
impl From<TonCell> for TonCellRef {
    fn from(value: TonCell) -> Self { value.into_ref() }
}

pub type TonCellRefsStore = Vec<TonCellRef>;

pub fn write_cell_display(f: &mut Formatter<'_>, cell: &TonCell, indent_level: usize) -> std::fmt::Result {
    use std::fmt::Write;
    let indent = "    ".repeat(indent_level);
    // Generate the data display string
    let mut data_display = cell.data.iter().fold(String::new(), |mut res, byte| {
        let _ = write!(res, "{byte:02x}");
        res
    });
    // completion tag
    if cell.data_bits_len % 8 != 0 {
        data_display.push('_');
    }

    if data_display.is_empty() {
        data_display.push_str("");
    };

    if cell.refs.is_empty() {
        // Compact format for cells without references
        writeln!(
            f,
            "{}Cell {{Type: {:?}, data: [{}], bit_len: {}}}",
            indent, cell.meta.cell_type, data_display, cell.data_bits_len,
        )
    } else {
        // Full format for cells with references
        writeln!(
            f,
            "{}Cell x{{Type: {:?}, data: [{}], bit_len: {}, references: [",
            indent, cell.meta.cell_type, data_display, cell.data_bits_len
        )?;
        for i in 0..cell.refs.len() {
            write_cell_display(f, cell.refs[i].deref(), indent_level + 1)?;
        }
        writeln!(f, "{}]}}", indent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_owned_create() {
        let child = TonCell {
            meta: CellMeta::EMPTY_CELL_META,
            data: vec![0x01, 0x02, 0x03],
            data_bits_len: 24,
            refs: TonCellRefsStore::new(),
        }
        .into_ref();

        let _cell = TonCell {
            meta: CellMeta::EMPTY_CELL_META,
            data: vec![0x04, 0x05, 0x06],
            data_bits_len: 24,
            refs: TonCellRefsStore::from([child]),
        };
    }
}
