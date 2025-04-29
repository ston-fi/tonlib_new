use crate::cell::meta::cell_type::CellType;
use crate::cell::ton_cell::TonCell;
use crate::cell::ton_hash::TonHash;
use crate::errors::TonlibError;
use std::collections::{HashSet, VecDeque};
use std::ops::Deref;

pub struct TonCellUtils {}

impl TonCellUtils {
    /// Result vector will contains only unique hashes
    pub fn extract_lib_ids<'a, I>(cells_iter: I) -> Result<Vec<TonHash>, TonlibError>
    where
        I: IntoIterator<Item = &'a TonCell>,
    {
        let mut result = HashSet::new();
        let mut visited = HashSet::new();

        let mut queue = VecDeque::from_iter(cells_iter);

        while let Some(cell_ref) = queue.pop_front() {
            if !visited.insert(cell_ref.hash()) {
                continue;
            }
            if let Some(lib_id) = Self::read_lib_id(cell_ref)? {
                result.insert(lib_id);
            }
            queue.extend(cell_ref.refs.iter().map(|x| x.deref()));
        }
        Ok(result.into_iter().collect())
    }

    pub fn read_lib_id(cell: &TonCell) -> Result<Option<TonHash>, TonlibError> {
        if cell.meta.cell_type != CellType::Library {
            return Ok(None);
        }
        Ok(Some(TonHash::from_bytes(&cell.data[1..=32])?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::ton_hash::TonHash;
    use crate::types::tlb::tlb_type::TLBType;
    use std::str::FromStr;

    #[test]
    fn test_extract_lib_ids_stonfi_router() -> Result<(), TonlibError> {
        let expected_lib_id = TonHash::from_str("57DE63D28E4D3608E0C02D437A7B50EF5F28F36A4821A047FD663CE63F4597EC")?;
        let code = TonCell::from_boc_hex(
            "b5ee9c720101010100230008420257de63d28e4d3608e0c02d437a7b50ef5f28f36a4821a047fd663ce63f4597ec",
        )?;
        let cells = vec![&code];
        let lib_ids = TonCellUtils::extract_lib_ids(cells)?;
        assert_eq!(lib_ids, vec![expected_lib_id.clone()]);

        // check accepted formats
        let code_ref = code.clone().into_ref();
        let cells = vec![code_ref.deref(), &code];
        let lib_ids = TonCellUtils::extract_lib_ids(cells)?;
        assert_eq!(lib_ids, vec![expected_lib_id.clone()]);
        Ok(())
    }
}
