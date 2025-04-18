use ton_lib::errors::TonLibError;
use ton_lib::lite_client::client::LiteClient;
use ton_lib::lite_client::config::LiteClientConfig;
use ton_lib::net_config::TON_NET_CONF_MAINNET;
use ton_lib::unwrap_lite_rsp;
use ton_liteapi::tl::request::Request;
use ton_liteapi::tl::response::Response;

#[tokio::test]
async fn test_lite_client() -> anyhow::Result<()> {
    let config = LiteClientConfig::new(TON_NET_CONF_MAINNET)?;
    let lite_client = LiteClient::new(config)?;

    // generic function
    let mc_info_rsp = lite_client.exec(Request::GetMasterchainInfo, None).await?;
    let mc_info_generic = unwrap_lite_rsp!(mc_info_rsp, MasterchainInfo)?;
    assert_ne!(mc_info_generic.last.seqno, 0);

    let mc_info = lite_client.get_mc_info().await?;
    assert_ne!(mc_info_generic.last.seqno, 0);

    let block_id = lite_client.lookup_mc_block(mc_info.last.seqno).await?;
    assert_eq!(block_id, mc_info.last);

    let account_boc =
        lite_client.get_account_boc("EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs", mc_info.last.seqno).await?;
    assert_ne!(account_boc.len(), 0);

    Ok(())
}
