use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonLibError;
use crate::tlb::dict::adapters_key::DictKeyAdapter;
use crate::tlb::dict::adapters_val::DictValAdapter;
use crate::tlb::dict::data_builder::DictDataBuilder;
use crate::tlb::dict::data_parser::DictDataParser;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Dict<K, V, KA: DictKeyAdapter<K>, VA: DictValAdapter<V>> {
    _phantom: std::marker::PhantomData<(K, V, KA, VA)>,
}

impl<K, V, KA, VA> Dict<K, V, KA, VA>
where
    K: Eq + Hash + Ord,
    KA: DictKeyAdapter<K>,
    VA: DictValAdapter<V>,
{
    pub fn read(parser: &mut CellParser, key_bits_len: u32) -> Result<HashMap<K, V>, TonLibError> {
        if !parser.read_bit()? {
            return Ok(HashMap::default());
        }

        let data_cell = parser.read_next_ref()?;
        let mut data_parser = DictDataParser::new(key_bits_len as usize);
        let data_raw = data_parser.read::<V, VA>(&mut CellParser::new(data_cell))?;
        let data = data_raw
            .into_iter()
            .map(|(k, v)| Ok::<_, TonLibError>((KA::extract_key(&k)?, v)))
            .collect::<Result<HashMap<K, V>, _>>()?;
        Ok(data)
    }

    pub fn write(builder: &mut CellBuilder, key_bits_len: u32, data: &HashMap<K, V>) -> Result<(), TonLibError> {
        if data.is_empty() {
            builder.write_bit(false)?;
            return Ok(());
        }

        let mut keys: Vec<_> = data.keys().collect();
        keys.sort_unstable();
        let mut values_sorted = Vec::with_capacity(data.len());
        for key in &keys {
            let value = data.get(key).unwrap();
            values_sorted.push(value);
        }
        let keys_sorted = keys.into_iter().map(|k| KA::make_key(k)).collect::<Result<Vec<_>, TonLibError>>()?;
        let data_builder = DictDataBuilder::<V, VA>::new(key_bits_len as usize, keys_sorted, values_sorted.as_slice())?;
        let dict_data_cell = data_builder.build()?.into_ref();
        builder.write_bit(true)?;
        builder.write_ref(dict_data_cell)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::ton_cell::TonCell;
    use crate::tlb::dict::adapters_key::DictKeyAdapterInto;
    use crate::tlb::dict::adapters_val::DictValAdapterNum;
    use crate::tlb::tlb_type::TLBType;
    use num_bigint::BigUint;
    use std::collections::HashMap;

    #[test]
    fn test_dict_blockchain_data() -> anyhow::Result<()> {
        let expected_data = HashMap::from([
            (0u8, BigUint::from(25965603044000000000u128)),
            (1, BigUint::from(5173255344000000000u64)),
            (2, BigUint::from(344883687000000000u64)),
        ]);
        let boc_hex = "b5ee9c7241010601005a000119c70d3ca5000d99b931ea4e8cc0010201cd020302012004050027400000000000000000000001325178b51d9180200026000000000000000000000168585a65986be8000026000000000000000000000047cb18538782e000353c80b9";
        let dict_cell = TonCell::from_boc_hex(boc_hex)?;
        let mut parser = CellParser::new(&dict_cell);
        let some_data = parser.read_bits(96)?;

        let parsed_data = Dict::<_, _, DictKeyAdapterInto, DictValAdapterNum<150>>::read(&mut parser, 8)?;
        assert_eq!(expected_data, parsed_data);
        let mut builder = CellBuilder::new();
        builder.write_bits(&some_data, 96)?;
        Dict::<_, _, DictKeyAdapterInto, DictValAdapterNum<150>>::write(&mut builder, 8, &expected_data)?;
        let constructed_cell = builder.build()?;
        assert_eq!(dict_cell, constructed_cell);
        Ok(())
    }

    #[test]
    fn test_dict_key_bits_len_bigger_than_key() -> anyhow::Result<()> {
        let data = HashMap::from([
            (0u16, BigUint::from(4u32)),
            (1, BigUint::from(5u32)),
            (2, BigUint::from(6u32)),
            (10u16, BigUint::from(7u32)),
            (127, BigUint::from(8u32)),
        ]);

        for key_len_bits in [8u32, 16, 32, 64, 111] {
            let mut builder = CellBuilder::new();
            Dict::<_, _, DictKeyAdapterInto, DictValAdapterNum<150>>::write(&mut builder, key_len_bits, &data)?;
            let dict_cell = builder.build()?;
            let mut parser = CellParser::new(&dict_cell);
            let parsed = Dict::<_, _, DictKeyAdapterInto, DictValAdapterNum<150>>::read(&mut parser, key_len_bits)?;
            assert_eq!(data, parsed, "key_len_bits: {}", key_len_bits);
        }
        Ok(())
    }
}
