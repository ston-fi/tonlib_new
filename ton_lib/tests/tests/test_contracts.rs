use crate::tests::utils::make_tl_client;
use std::str::FromStr;
use std::time::Duration;
use tokio_test::assert_ok;
use ton_lib::contracts::client::contract_client::{ContractClient, ContractClientConfig};
use ton_lib::contracts::client::tl_provider::TLProvider;
use ton_lib::contracts::jetton_master::JettonMaster;
use ton_lib::contracts::jetton_wallet::JettonWallet;
use ton_lib::contracts::methods::get_collection_data::GetCollectionData;
use ton_lib::contracts::methods::get_jetton_data::GetJettonData;
use ton_lib::contracts::methods::get_nft_address_by_index::GetNftAddressByIndex;
use ton_lib::contracts::methods::get_nft_content::GetNftContent;
use ton_lib::contracts::methods::get_nft_data::GetNftData;
use ton_lib::contracts::methods::get_wallet_address::GetWalletAddress;
use ton_lib::contracts::methods::get_wallet_data::GetWalletData;
use ton_lib::contracts::nft_collection_contract::NftCollectionContract;
use ton_lib::contracts::nft_item_contract::NftItemContract;
use ton_lib::contracts::ton_contract::TonContract;
use ton_lib::contracts::ton_wallet::TonWalletContract;
use ton_lib_core::cell::TonHash;
use ton_lib_core::types::TonAddress;

#[tokio::test]
async fn test_contracts() -> anyhow::Result<()> {
    let tl_client = make_tl_client(true, true).await?;
    let config = ContractClientConfig::new_no_cache(Duration::from_millis(100));
    let data_provider = TLProvider::new(tl_client.clone());
    let ctr_cli = ContractClient::new(config, data_provider)?;

    assert_jetton_wallet(&ctr_cli).await?;
    assert_jetton_master(&ctr_cli).await?;
    assert_wallet_contract(&ctr_cli).await?;
    assert_nft_item_contract(&ctr_cli).await?;
    assert_nft_collection_contract(&ctr_cli).await?;
    Ok(())
}

async fn assert_jetton_wallet(ctr_cli: &ContractClient) -> anyhow::Result<()> {
    let usdt_wallet = TonAddress::from_str("EQAmJs8wtwK93thF78iD76RQKf9Z3v2sxM57iwpZZtdQAiVM")?;
    let contract = JettonWallet::new(ctr_cli, usdt_wallet, None).await?;
    assert_ok!(contract.get_wallet_data().await);
    Ok(())
}

async fn assert_jetton_master(ctr_cli: &ContractClient) -> anyhow::Result<()> {
    let usdt_master = TonAddress::from_str("EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs")?;
    let contract = JettonMaster::new(ctr_cli, usdt_master, None).await?;
    assert_ok!(contract.get_jetton_data().await);
    let owner = TonAddress::from_str("UQAj-peZGPH-cC25EAv4Q-h8cBXszTmkch6ba6wXC8BM40qt")?;
    let wallet = assert_ok!(contract.get_wallet_address(&owner).await);
    assert_eq!(wallet.address.to_string(), "EQAmJs8wtwK93thF78iD76RQKf9Z3v2sxM57iwpZZtdQAiVM");
    Ok(())
}

async fn assert_wallet_contract(ctr_cli: &ContractClient) -> anyhow::Result<()> {
    let wallet = TonAddress::from_str("UQAj-peZGPH-cC25EAv4Q-h8cBXszTmkch6ba6wXC8BM40qt")?;
    let contract = TonWalletContract::new(ctr_cli, wallet, None).await?;
    let seqno = contract.seqno().await?;
    assert!(seqno > 0);
    let public_key = contract.get_public_key().await?;
    assert_ne!(public_key, TonHash::ZERO);
    Ok(())
}

async fn assert_nft_collection_contract(ctr_cli: &ContractClient) -> anyhow::Result<()> {
    let collection = TonAddress::from_str("EQC3dNlesgVD8YbAazcauIrXBPfiVhMMr5YYk2in0Mtsz0Bz")?;
    let contract = NftCollectionContract::new(ctr_cli, collection, None).await?;

    let data = contract.get_collection_data().await?;
    dbg!(&data);

    Ok(())
}

async fn assert_nft_item_contract(ctr_cli: &ContractClient) -> anyhow::Result<()> {
    let item = TonAddress::from_str("EQB43-VCmf17O7YMd51fAvOjcMkCw46N_3JMCoegH_ZDo40e")?;
    let onchain = TonAddress::from_str("EQBq5z4N_GeJyBdvNh4tPjMpSkA08p8vWyiAX6LNbr3aLjI0")?; // TODO SUCCESS
    let semichain = TonAddress::from_str("EQB2NJFK0H5OxJTgyQbej0fy5zuicZAXk2vFZEDrqbQ_n5YW")?;
    let contract = NftItemContract::new(ctr_cli, item, None).await?;

    let data = contract.get_nft_data().await?;
    dbg!(data);
    panic!();

    Ok(())
}
