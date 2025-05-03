use crate::tests::test_tl_client::make_tl_client_default;
use std::str::FromStr;
use tokio_test::assert_ok;
use ton_lib::clients::tonlib::TLClient;
use ton_lib::contracts::jetton_master::JettonMasterContract;
use ton_lib::contracts::jetton_wallet::JettonWalletContract;
use ton_lib::contracts::ton_contract::TonContract;
use ton_lib::types::ton_address::TonAddress;

#[tokio::test]
async fn test_contracts_all() -> anyhow::Result<()> {
    let tl_client = make_tl_client_default(true, false).await?;
    assert_jetton_wallet(&tl_client).await?;
    assert_jetton_master(&tl_client).await?;
    Ok(())
}

async fn assert_jetton_wallet(tl_client: &TLClient) -> anyhow::Result<()> {
    let usdt_wallet = TonAddress::from_str("EQAmJs8wtwK93thF78iD76RQKf9Z3v2sxM57iwpZZtdQAiVM")?;
    let contract = JettonWalletContract::new(usdt_wallet, tl_client.clone(), None).await?;
    assert_ok!(contract.get_wallet_data().await);
    Ok(())
}

async fn assert_jetton_master(tl_client: &TLClient) -> anyhow::Result<()> {
    let usdt_master = TonAddress::from_str("EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs")?;
    let contract = JettonMasterContract::new(usdt_master, tl_client.clone(), None).await?;
    assert_ok!(contract.get_jetton_data().await);
    let owner = TonAddress::from_str("UQAj-peZGPH-cC25EAv4Q-h8cBXszTmkch6ba6wXC8BM40qt")?;
    let wallet = assert_ok!(contract.get_wallet_address(&owner).await);
    assert_eq!(wallet.to_string(), "EQAmJs8wtwK93thF78iD76RQKf9Z3v2sxM57iwpZZtdQAiVM");
    Ok(())
}
