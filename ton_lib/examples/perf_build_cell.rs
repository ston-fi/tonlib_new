use ton_lib::cell::build_parse::builder::CellBuilder;

fn main() -> anyhow::Result<()> {
    for _ in 0..10000000 {
        let mut builder1 = CellBuilder::new();
        builder1.write_bit(true)?;
        builder1.write_bytes([1, 2, 3])?;
        builder1.write_num(&4, 4)?;

        let mut builder2 = CellBuilder::new();
        builder2.write_bytes([10, 20, 30])?;

        let mut builder3 = CellBuilder::new();
        builder3.write_bytes([100, 200, 255])?;

        let mut builder = CellBuilder::new();
        builder.write_ref(builder1.build()?.into())?;
        builder.write_ref(builder2.build()?.into())?;
        builder.write_ref(builder3.build()?.into())?;

        #[allow(unused)]
        let cell = builder.build()?;
        // println!("{cell}");
    }
    Ok(())
}
