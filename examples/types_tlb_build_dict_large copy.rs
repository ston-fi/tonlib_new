use num_bigint::BigUint;
use std::collections::HashMap;
use ton_lib::types::tlb::adapters::dict_key_adapters::DictKeyAdapterInto;
use ton_lib::types::tlb::adapters::dict_val_adapters::DictValAdapterNum;
use ton_lib::types::tlb::adapters::Dict;
use ton_lib::{cell::ton_cell::TonCell, types::tlb::TLB};

use ton_lib_macros::TLBDerive;

// const ITEMS_COUNT: usize = 40000000;
const ITEMS_COUNT: usize = 4000000;

struct MyDict {
    pub data: HashMap<usize, BigUint>,
}
impl ton_lib::types::tlb::TLB for MyDict {
    const PREFIX: ton_lib::types::tlb::TLBPrefix = ton_lib::types::tlb::TLBPrefix::new(0usize, 0usize);
    fn read_definition(
        parser: &mut ton_lib::cell::build_parse::parser::CellParser,
    ) -> Result<Self, ton_lib::errors::TonlibError> {
        use ton_lib::types::tlb::TLB;
        let data = Dict::<DictKeyAdapterInto, DictValAdapterNum<256>, _, _>::new(256).read(parser)?.into();
        Ok(Self { data })
    }
    fn write_definition(
        &self,
        builder: &mut ton_lib::cell::build_parse::builder::CellBuilder,
    ) -> Result<(), ton_lib::errors::TonlibError> {
        Dict::<DictKeyAdapterInto, DictValAdapterNum<256>, _, _>::new(256).write(builder, &self.data)?;
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let mut data = HashMap::new();
    for i in 0..ITEMS_COUNT {
        data.insert(i, BigUint::from(i));
    }
    let mut builder = TonCell::builder();

    MyDict { data }.write(&mut builder)?;
    // Dict::<DictKeyAdapterInto, DictValAdapterNum<256>, _, _>::new(256).write(&mut builder, &data)?;
    let cell = builder.build()?;
    println!("{}", cell.hash());
    Ok(())
}
