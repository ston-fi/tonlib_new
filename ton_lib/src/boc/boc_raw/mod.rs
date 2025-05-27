mod from_bytes_impl;
mod from_ton_cells_impl;
mod into_ton_cell_impl;
mod to_bytes_impl;

use crate::cell::meta::cell_type::CellType;
use crate::cell::meta::level_mask::LevelMask;

pub const GENERIC_BOC_MAGIC: u32 = 0xb5ee9c72;
/// `cells` must be topologically sorted.
#[derive(PartialEq, Debug, Clone)]
pub(crate) struct BOCRaw {
    pub(crate) cells: Vec<CellRaw>,
    pub(crate) roots_position: Vec<usize>,
}

/// References are stored as indices in BagOfCells.
#[derive(PartialEq, Debug, Clone)]
pub(crate) struct CellRaw {
    pub(crate) cell_type: CellType,
    pub(crate) data: Vec<u8>,
    pub(crate) data_bits_len: usize,
    pub(crate) refs_positions: Vec<usize>,
    pub(crate) level_mask: LevelMask,
}
