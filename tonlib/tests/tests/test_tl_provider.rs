use crate::tests::utils::make_tl_client;
use ton_lib::clients::tl_client::tl::client::TLClientTrait;
use ton_lib::contracts::contract_client::ContractClient;
use ton_lib::contracts::tl_provider::provider::TLProvider;
use ton_lib::contracts::tl_provider::provider_config::TLProviderConfig;

#[tokio::test]
async fn test_tl_provider() -> anyhow::Result<()> {
    let tl_client = make_tl_client(true, true).await?;

    let head_seqno = tl_client.get_mc_info().await?.last.seqno;

    let provider_config = TLProviderConfig::new_no_cache(head_seqno);
    let tl_provider = TLProvider::new(provider_config, tl_client.clone()).await?;
    let _ctr_cli = ContractClient::new(tl_provider)?;
    Ok(())
}
