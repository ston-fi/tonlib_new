use crate::cell::meta::cell_meta::CellMeta;
use crate::cell::meta::level_mask::LevelMask;
use crate::cell::ton_hash::TonHash;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::sync::Arc;

pub type TonCellRef = Arc<dyn TonCell>;
pub type TonCellRefsStore = Vec<TonCellRef>;

pub trait TonCell: Debug + Send + Sync {
    fn from_data(meta: CellMeta, data: Vec<u8>, data_bits_len: usize, refs: TonCellRefsStore) -> Self
    where
        Self: Sized;
    // raw data access
    fn get_meta(&self) -> &CellMeta;
    fn get_data(&self) -> &[u8];
    fn get_data_bits_len(&self) -> usize;
    fn get_ref(&self, index: usize) -> Option<&TonCellRef>;
    fn into_ref(self) -> TonCellRef
    where
        Self: Sized + 'static,
    {
        Arc::new(self)
    }

    // handy wrappers over meta
    fn refs_count(&self) -> usize { self.get_meta().refs_count }
    fn hash(&self) -> &TonHash { self.hash_for_level(LevelMask::MAX_LEVEL) }
    fn hash_for_level(&self, level: LevelMask) -> &TonHash { &self.get_meta().hashes[level.mask() as usize] }
}

impl PartialEq for dyn TonCell {
    fn eq(&self, other: &Self) -> bool { self.hash() == other.hash() }
}

impl Display for dyn TonCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write_cell_display(f, self, 0) }
}

pub fn write_cell_display(f: &mut Formatter<'_>, cell: &dyn TonCell, indent_level: usize) -> std::fmt::Result {
    use std::fmt::Write;
    let indent = "    ".repeat(indent_level);
    // Generate the data display string
    let mut data_display = cell.get_data().iter().fold(String::new(), |mut res, byte| {
        let _ = write!(res, "{byte:02x}");
        res
    });
    // completion tag
    if cell.get_data_bits_len() % 8 != 0 {
        data_display.push('_');
    }

    if data_display.is_empty() {
        data_display.push_str("");
    };

    if cell.refs_count() == 0 {
        // Compact format for cells without references
        writeln!(
            f,
            "{}Cell {{Type: {:?}, data: [{}], bit_len: {}}}",
            indent,
            cell.get_meta().cell_type,
            data_display,
            cell.get_data_bits_len()
        )
    } else {
        // Full format for cells with references
        writeln!(
            f,
            "{}Cell x{{Type: {:?}, data: [{}], bit_len: {}, references: [",
            indent,
            cell.get_meta().cell_type,
            data_display,
            cell.get_data_bits_len()
        )?;
        for i in 0..cell.refs_count() {
            let ref_cell = cell.get_ref(i).unwrap();
            write_cell_display(f, ref_cell.deref(), indent_level + 1)?;
        }
        writeln!(f, "{}]}}", indent)
    }
}
