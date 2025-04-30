pub mod data_builder;
pub mod data_parser;
pub mod dict_key_adapters;
pub mod dict_val_adapters;
pub mod label_type;
pub mod leading_bit_utils;

use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonlibError;
use crate::types::tlb::adapters::dict::data_builder::DictDataBuilder;
use crate::types::tlb::adapters::dict::data_parser::DictDataParser;
use crate::types::tlb::adapters::dict::dict_key_adapters::DictKeyAdapter;
use crate::types::tlb::adapters::dict::dict_val_adapters::DictValAdapter;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

/// Adapter to write HashMap with arbitrary key/values into a cell.
/// Usage: `#[tlb_derive(adapter = "Dict::<DictKeyAdapterTonHash, DictValAdapterTLB, _, _>::new(256)")]` instead
pub struct Dict<KA: DictKeyAdapter<K>, VA: DictValAdapter<V>, K, V> {
    key_bits_len: u32,
    _phantom: PhantomData<(KA, VA, K, V)>,
}

impl<KA, VA, K, V> Dict<KA, VA, K, V>
where
    KA: DictKeyAdapter<K>,
    VA: DictValAdapter<V>,
    K: Eq + Hash + Ord,
{
    pub fn new(key_bits_len: u32) -> Self {
        Self {
            key_bits_len,
            _phantom: PhantomData,
        }
    }

    pub fn read(&self, parser: &mut CellParser) -> Result<HashMap<K, V>, TonlibError> {
        let mut data_parser = DictDataParser::new(self.key_bits_len as usize);
        let data_raw = data_parser.read::<V, VA>(parser)?;
        let data = data_raw
            .into_iter()
            .map(|(k, v)| Ok::<_, TonlibError>((KA::extract_key(&k)?, v)))
            .collect::<Result<HashMap<K, V>, _>>()?;
        Ok(data)
    }

    pub fn write(&self, builder: &mut CellBuilder, data: &HashMap<K, V>) -> Result<(), TonlibError> {
        let mut keys: Vec<_> = data.keys().collect();
        keys.sort_unstable();
        let mut values_sorted = Vec::with_capacity(data.len());
        for key in &keys {
            let value = data.get(key).unwrap();
            values_sorted.push(value);
        }
        let keys_sorted = keys.into_iter().map(|k| KA::make_key(k)).collect::<Result<Vec<_>, TonlibError>>()?;
        let data_builder =
            DictDataBuilder::<V, VA>::new(self.key_bits_len as usize, keys_sorted, values_sorted.as_slice())?;
        let dict_data_cell = data_builder.build()?;
        builder.write_cell(&dict_data_cell)
    }
}

#[cfg(test)]
mod tests {
    // tested in LibsDict
}
