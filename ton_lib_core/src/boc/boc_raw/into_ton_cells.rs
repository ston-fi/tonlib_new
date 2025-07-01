use crate::cell::{CellMeta, TonCell, TonCellRef};
use crate::error::TLCoreError;

use super::BOCRaw;

impl BOCRaw {
    //Based on https://github.com/toncenter/tonweb/blob/c2d5d0fc23d2aec55a0412940ce6e580344a288c/src/boc/Cell.js#L198
    pub fn into_ton_cells(self) -> Result<Vec<TonCellRef>, TLCoreError> {
        let num_cells = self.cells.len();
        let mut cells: Vec<TonCellRef> = Vec::with_capacity(num_cells);

        for (cell_index, cell_raw) in self.cells.into_iter().enumerate().rev() {
            let mut refs = Vec::with_capacity(cell_raw.refs_positions.len());
            for ref_index in &cell_raw.refs_positions {
                if *ref_index <= cell_index {
                    return Err(TLCoreError::Custom("refs to previous cells are not supported".to_string()));
                }
                refs.push(cells[num_cells - 1 - ref_index].clone());
            }

            let cell_ref = TonCell {
                cell_type: cell_raw.cell_type,
                data: cell_raw.data,
                data_bits_len: cell_raw.data_bits_len,
                refs,
                meta: CellMeta::default(),
            }
            .into_ref();
            cells.push(cell_ref);
        }

        let mut roots = Vec::with_capacity(self.roots_position.len());
        for root_index in self.roots_position {
            roots.push(cells[num_cells - 1 - root_index].clone());
        }
        Ok(roots)
    }
}
