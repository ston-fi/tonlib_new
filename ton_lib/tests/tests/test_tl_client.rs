use tokio_test::assert_ok;
use ton_lib::clients::tonlib::tl_client::TLClient;

use ton_lib::cell::build_parse::parser::CellParser;
use ton_lib::cell::ton_cell::TonCell;
use ton_lib::types::tlb::tlb_type::TLBType;

#[tokio::test]
async fn test_tl_client_default() -> anyhow::Result<()> {
    let tl_client = crate::tests::utils::make_tl_client_default(true, false).await?;
    // ton_lib::utils::tonlib_set_verbosity_level(4);

    let mc_info = tl_client.get_mc_info().await?;
    assert_ne!(mc_info.last.seqno, 0);

    // another node may be behind
    let block = tl_client.lookup_mc_block(mc_info.last.seqno - 100).await?;
    assert_eq!(block.seqno, mc_info.last.seqno - 100);

    let config = tl_client.get_config_all(0).await?;
    assert_ok!(TonCell::from_boc(&config.config.bytes));

    let config = tl_client.get_config_param(0, 34).await?;
    let cell = assert_ok!(TonCell::from_boc(&config.config.bytes));
    let mut parser = CellParser::new(&cell);
    let value: u8 = TLBType::read(&mut parser)?;
    assert_eq!(value, 0x12);

    // https://tonviewer.com/EQCGScrZe1xbyWqWDvdI6mzP-GAcAWFv6ZXuaJOuSqemxku4
    // let lib_id = TonHash::from_hex("A9338ECD624CA15D37E4A8D9BF677DDC9B84F0E98F05F2FB84C7AFE332A281B4")?;
    // let lib_result = tl_client.get_libs(vec![lib_id.clone()]).await?;
    // assert_eq!(lib_result.result.len(), 1);
    // assert_eq!(lib_result.result[0].hash.as_slice(), lib_id.as_slice());
    // let lib_cell = assert_ok!(TonCell::from_boc(&lib_result.result[0].data));
    // assert_eq!(lib_cell.hash(), &lib_id);
    Ok(())
}
