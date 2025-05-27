use crate::cell::ton_cell::TonCell;
use crate::cell::{meta::cell_meta::CellMeta, ton_cell::TonCellStorage};
use crate::errors::TonlibError;

use super::BOCRaw;

// Based on https://github.com/toncenter/tonweb/blob/c2d5d0fc23d2aec55a0412940ce6e580344a288c/src/boc/Cell.js#L198
impl TryFrom<BOCRaw> for TonCellStorage {
    type Error = TonlibError;

    fn try_from(boc_raw: BOCRaw) -> Result<TonCellStorage, Self::Error> {
        let num_cells = boc_raw.cells.len();
        let mut cells: TonCellStorage = Vec::with_capacity(num_cells);

        for (cell_index, cell_raw) in boc_raw.cells.into_iter().enumerate().rev() {
            let mut refs = Vec::with_capacity(cell_raw.refs_positions.len());
            for ref_index in &cell_raw.refs_positions {
                if *ref_index <= cell_index {
                    return Err(TonlibError::BOCCustom("refs to previous cells are not supported".to_string()));
                }
                refs.push(cells[num_cells - 1 - ref_index].clone());
            }

            let meta = CellMeta::new(cell_raw.cell_type, &cell_raw.data, cell_raw.data_bits_len, &refs)?;

            let cell_ref = TonCell {
                meta,
                data: cell_raw.data,
                data_bits_len: cell_raw.data_bits_len,
                refs,
            }
            .into_ref();
            cells.push(cell_ref);
        }

        let mut roots = Vec::with_capacity(boc_raw.roots_position.len());
        for root_index in boc_raw.roots_position {
            roots.push(cells[num_cells - 1 - root_index].clone());
        }
        Ok(roots)
    }
}
