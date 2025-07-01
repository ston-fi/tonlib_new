use crate::cell::meta::cell_meta_builder::CellMetaBuilder;
use crate::cell::meta::level_mask::LevelMask;
use crate::cell::ton_cell::TonCell;
use crate::cell::ton_hash::TonHash;
use crate::error::TLCoreError;
use once_cell;
use once_cell::sync::OnceCell;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum LazyHashesDepths {
    Static(([TonHash; 4], [u16; 4])),
    Dynamic(OnceCell<([TonHash; 4], [u16; 4])>),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum LazyLevelMask {
    Static(LevelMask),
    Dynamic(OnceCell<LevelMask>),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CellMeta {
    level_mask: LazyLevelMask,
    hashes_depths: LazyHashesDepths,
}

impl CellMeta {
    pub(crate) const DEPTH_BYTES: usize = 2;

    pub(crate) const EMPTY_CELL_META: CellMeta = CellMeta {
        level_mask: LazyLevelMask::Static(LevelMask::new(0)),
        hashes_depths: LazyHashesDepths::Static(([TonCell::EMPTY_CELL_HASH; 4], [0; 4])),
    };

    pub(crate) fn validate(&self, cell: &TonCell) -> Result<(), TLCoreError> { self.make_builder(cell).validate() }

    pub(crate) fn level_mask(&self, cell: &TonCell) -> LevelMask {
        match &self.level_mask {
            LazyLevelMask::Static(mask) => *mask,
            LazyLevelMask::Dynamic(once) => *once.get_or_init(|| self.make_builder(cell).calc_level_mask()),
        }
    }

    pub(crate) fn hash(&self, cell: &TonCell, level: LevelMask) -> Result<&TonHash, TLCoreError> {
        let hashes = &self.get_hashes_depths(cell)?.0;
        Ok(&hashes[level.mask() as usize])
    }
    pub(crate) fn depth(&self, cell: &TonCell, level: LevelMask) -> Result<u16, TLCoreError> {
        let depths = &self.get_hashes_depths(cell)?.1;
        Ok(depths[level.mask() as usize])
    }

    fn get_hashes_depths(&self, cell: &TonCell) -> Result<&([TonHash; 4], [u16; 4]), TLCoreError> {
        match &self.hashes_depths {
            LazyHashesDepths::Static(hashes_depths) => Ok(hashes_depths),
            LazyHashesDepths::Dynamic(once) => once.get_or_try_init(|| {
                let level_mask = self.level_mask(cell);
                self.make_builder(cell).calc_hashes_and_depths(level_mask)
            }),
        }
    }

    fn make_builder<'a>(&self, cell: &'a TonCell) -> CellMetaBuilder<'a> {
        CellMetaBuilder::new(cell.cell_type, &cell.data, cell.data_bits_len, &cell.refs)
    }
}

impl Default for CellMeta {
    fn default() -> Self {
        Self {
            level_mask: LazyLevelMask::Dynamic(OnceCell::new()),
            hashes_depths: LazyHashesDepths::Dynamic(OnceCell::new()),
        }
    }
}
