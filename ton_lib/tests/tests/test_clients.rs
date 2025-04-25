use crate::tests::utils::{make_lite_client, make_tlj_client_default};
use std::str::FromStr;
use ton_lib::clients::tonlibjson::tlj_client::TLJClient;
use ton_lib::errors::TonlibError;
use ton_lib::types::tlb::block_tlb::account::MaybeAccount;
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
    let tlj_client = make_tlj_client_default(true, false).await?;

    let mc_info = tlj_client.get_mc_info().await?;
    assert_ne!(mc_info.last.seqno, 0);

    // another node may be behind
    let block = tlj_client.lookup_mc_block(mc_info.last.seqno - 100).await?;
    assert_eq!(block.seqno, mc_info.last.seqno - 100);

    Ok(())
}
