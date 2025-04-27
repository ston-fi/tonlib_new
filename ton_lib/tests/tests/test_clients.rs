use crate::tests::utils::make_lite_client;
use std::str::FromStr;
use tokio_test::assert_ok;
#[cfg(feature = "sys")]
use ton_lib::clients::tonlibjson::tlj_client::TLJClient;

use ton_lib::cell::build_parse::parser::CellParser;
use ton_lib::cell::ton_cell::TonCell;
use ton_lib::errors::TonlibError;
use ton_lib::types::tlb::block_tlb::account::MaybeAccount;
use ton_lib::types::tlb::tlb_type::TLBType;
use ton_lib::types::ton_address::TonAddress;
use ton_lib::unwrap_lite_response;
use ton_liteapi::tl::request::Request;
use ton_liteapi::tl::response::Response;

#[tokio::test]
async fn test_lite_client() -> anyhow::Result<()> {
    let lite_client = make_lite_client(true).await?;

    // generic interface
    let mc_info_rsp = lite_client.exec(Request::GetMasterchainInfo, None).await?;
    let mc_info_generic = unwrap_lite_response!(mc_info_rsp, MasterchainInfo)?;
    assert_ne!(mc_info_generic.last.seqno, 0);

    // === specialized interface ===
    let mc_info = lite_client.get_mc_info().await?;
    assert_ne!(mc_info_generic.last.seqno, 0);

    let block_id = lite_client.lookup_mc_block(mc_info.last.seqno).await?;
    assert_eq!(block_id, mc_info.last);

    let usdt_addr = TonAddress::from_str("EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs")?;
    let account_boc = lite_client.get_account_state(&usdt_addr, mc_info.last.seqno).await?;
    assert!(matches!(account_boc, MaybeAccount::Some(_)));

    Ok(())
}

#[tokio::test]
#[cfg(feature = "sys")]
async fn test_tlj_client_default() -> anyhow::Result<()> {
    let tlj_client = crate::tests::utils::make_tlj_client_default(true, false).await?;
    // ton_lib::utils::tonlib_set_verbosity_level(4);

    let mc_info = tlj_client.get_mc_info().await?;
    assert_ne!(mc_info.last.seqno, 0);

    // another node may be behind
    let block = tlj_client.lookup_mc_block(mc_info.last.seqno - 100).await?;
    assert_eq!(block.seqno, mc_info.last.seqno - 100);

    let config = tlj_client.get_config_all(0).await?;
    assert_ok!(TonCell::from_boc(&config.config.bytes));

    let config = tlj_client.get_config_param(0, 34).await?;
    let cell = assert_ok!(TonCell::from_boc(&config.config.bytes));
    let mut parser = CellParser::new(&cell);
    let value: u8 = TLBType::read(&mut parser)?;
    assert_eq!(value, 0x12);

    // https://tonviewer.com/EQCGScrZe1xbyWqWDvdI6mzP-GAcAWFv6ZXuaJOuSqemxku4
    // let lib_id = TonHash::from_hex("A9338ECD624CA15D37E4A8D9BF677DDC9B84F0E98F05F2FB84C7AFE332A281B4")?;
    // let lib_result = tlj_client.get_libs(vec![lib_id.clone()]).await?;
    // assert_eq!(lib_result.result.len(), 1);
    // assert_eq!(lib_result.result[0].hash.as_slice(), lib_id.as_slice());
    // let lib_cell = assert_ok!(TonCell::from_boc(&lib_result.result[0].data));
    // assert_eq!(lib_cell.hash(), &lib_id);
    Ok(())
}
