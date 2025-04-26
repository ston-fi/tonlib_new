use num_bigint::BigUint;
use std::collections::HashMap;
use ton_lib::cell::build_parse::builder::CellBuilder;
use ton_lib::types::tlb::adapters::dict_key_adapters::DictKeyAdapterInto;
use ton_lib::types::tlb::adapters::dict_val_adapters::DictValAdapterNum;
use ton_lib::types::tlb::adapters::Dict;

// const ITEMS_COUNT: usize = 40000000;
const ITEMS_COUNT: usize = 4000000;

fn main() -> anyhow::Result<()> {
    let mut data = HashMap::new();
    for i in 0..ITEMS_COUNT {
        data.insert(i, BigUint::from(i));
    }
    let mut builder = CellBuilder::new();
    Dict::<DictKeyAdapterInto, DictValAdapterNum<256>, _, _>::new(256).write(&mut builder, &data)?;
    let cell = builder.build()?;
    println!("{}", cell.hash());
    Ok(())
}
