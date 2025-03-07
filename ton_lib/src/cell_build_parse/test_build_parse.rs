use crate::cell_build_parse::builder::CellBuilder;
use crate::cell_build_parse::parser::CellParser;
// use crate::cell::cell_slice::CellSlice;

#[test]
fn test_build_parse_bit() -> anyhow::Result<()> {
    let mut writer = CellBuilder::new();
    writer.write_bit(true)?;
    writer.write_bit(false)?;
    writer.write_bit(true)?;
    writer.write_bit(false)?;
    let cell = writer.build()?;
    // let cell_slice = CellSlice::from_cell(&cell);
    let mut reader = CellParser::new(&cell);
    assert!(reader.read_bit()?);
    assert!(!reader.read_bit()?);
    assert!(reader.read_bit()?);
    assert!(!reader.read_bit()?);
    Ok(())
}

#[test]
fn test_build_parse_bits() -> anyhow::Result<()> {
    let mut writer = CellBuilder::new();
    writer.write_bit(true)?;
    writer.write_bits([0b1010_1010], 8)?;
    writer.write_bits([0b0101_0101], 4)?;
    let cell = writer.build()?;
    // let cell_slice = CellSlice::from_cell(&cell);
    let mut reader = CellParser::new(&cell);
    assert!(reader.read_bit()?);
    let mut dst = vec![0; 1];
    reader.read_bits(8, &mut dst)?;
    assert_eq!(dst, vec![0b1010_1010]);
    let mut dst = vec![0; 1];
    reader.read_bits(4, &mut dst)?;
    assert_eq!(dst, vec![0b0101_0000]);
    Ok(())
}

#[test]
fn test_build_parse_num() -> anyhow::Result<()> {
    let mut writer = CellBuilder::new();
    writer.write_num(1u8, 4)?;
    writer.write_num(2u16, 5)?;
    writer.write_num(5u32, 10)?;
    writer.write_num(-1i8, 8)?;
    writer.write_num(-2i16, 16)?;
    writer.write_num(-5i32, 32)?;
    let cell = writer.build()?;
    // let cell_slice = CellSlice::from_cell(&cell);
    let mut reader = CellParser::new(&cell);
    assert_eq!(reader.read_num::<u8>(4)?, 1);
    assert_eq!(reader.read_num::<u16>(5)?, 2);
    assert_eq!(reader.read_num::<u32>(10)?, 5);
    assert_eq!(reader.read_num::<i8>(8)?, -1);
    assert_eq!(reader.read_num::<i16>(16)?, -2);
    assert_eq!(reader.read_num::<i32>(32)?, -5);
    Ok(())
}
