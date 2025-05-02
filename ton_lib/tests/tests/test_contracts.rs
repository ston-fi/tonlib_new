use std::str::FromStr;
use tokio_test::assert_ok;
use ton_lib::clients::tonlib::TLClient;

use crate::tests::test_tl_client::make_tl_client_default;
use ton_lib::contracts::jetton_wallet::JettonWalletContract;
use ton_lib::contracts::ton_contract::TonContract;
use ton_lib::types::ton_address::TonAddress;

#[tokio::test]
async fn test_all() -> anyhow::Result<()> {
    let tl_client = make_tl_client_default(true, false).await?;
    assert_jetton_wallet(&tl_client).await?;
    Ok(())
}

async fn assert_jetton_wallet(tl_client: &TLClient) -> anyhow::Result<()> {
    let usdt_wallet_address = TonAddress::from_str("EQAmJs8wtwK93thF78iD76RQKf9Z3v2sxM57iwpZZtdQAiVM")?;
    let contract = JettonWalletContract::new(usdt_wallet_address, tl_client.clone(), None).await?;
    assert_ok!(contract.get_wallet_data().await);
    Ok(())
}
