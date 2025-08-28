use crate::tests::utils::make_tl_client;
use num_bigint::BigInt;
use sha2::{Digest, Sha256};
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
use ton_lib::meta_loader::MetaLoader;
use ton_lib::tep::metadata::metadata_content::{MetadataContent, MetadataInternal};
use ton_lib::tep::metadata::nft_item_metadata::NftItemMetadata;
use ton_lib::tep::metadata::snake_data::SnakeData;
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
    assert_get_nft_data_semichain(&ctr_cli).await?;
    assert_get_nft_address_by_index_is_valid(&ctr_cli).await?;
    assert_get_collection_data_is_valid(&ctr_cli).await?;
    assert_get_nft_data_internal(&ctr_cli).await?;
    assert_get_nft_data_is_valid(&ctr_cli).await?;
    assert_get_nft_address_by_index(&ctr_cli).await?;
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

    assert_eq!(data.collection_content.as_external().unwrap().uri.as_str(), "https://dns.ton.org/collection.json");
    assert_eq!(data.next_item_index, -1);
    assert_eq!(data.owner_address, TonAddress::from_str("EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c")?);

    Ok(())
}

async fn assert_get_nft_data_semichain(ctr_cli: &ContractClient) -> anyhow::Result<()> {
    let semichain = TonAddress::from_str("EQAbNqfCuv4Chy6D-2UBKzi3qYvVPrB-STOzBGQo5AKh4P9u")?;
    let contract = NftItemContract::new(ctr_cli, semichain, None).await?;

    let data = contract.get_nft_data().await?;
    if let MetadataContent::Unsupported(meta) = data.individual_content {
        let collection_contract = NftCollectionContract::new(ctr_cli, data.collection_address, None).await?;
        let full_content = collection_contract.get_nft_content(data.index, meta.cell.into_ref()).await?;
        let meta_loader = MetaLoader::builder().build()?;
        let content_res: NftItemMetadata = meta_loader.load(&full_content.full_content).await?;
        let expected = NftItemMetadata {
            name: Some(
                String::from("Season 2 Airdrop Member"),
            ),
            description: Some(
                String::from("This SBT confirms that you have completed the Season 2 checklist and claimed the official airdrop, verifying your daily logins, partner game plays, and event participation. Holders earn community recognition and gain early access to benefits from future drops."),
            ),
            image: Some(
                String::from("https://static.sidusheroes.com/prod/tonstation/nft/Season%202%20Airdrop%20Participant.png"),
            ),
            content_url: Some(
                String::from("https://static.sidusheroes.com/prod/tonstation/nft/Season%202%20Airdrop%20Participant.png"),
            ),
            attributes: None,
        };
        assert_eq!(expected, content_res);
    } else {
        unreachable!("The metadata must be in unparseble format");
    }

    Ok(())
}

async fn assert_get_nft_data_is_valid(ctr_cli: &ContractClient) -> anyhow::Result<()> {
    let address = TonAddress::from_str("EQCGZEZZcYO9DK877fJSIEpYMSvfui7zmTXGhq0yq1Ce1Mb6")?;
    let contract = NftItemContract::new(ctr_cli, address, None).await?;
    let res = assert_ok!(contract.get_nft_data().await);

    let expected_collection_address =
        assert_ok!(TonAddress::from_str("EQAOQdwdw8kGftJCSFgOErM1mBjYPe4DBPq8-AhF6vr9si5N"));
    let expected_index =
        assert_ok!(BigInt::from_str("15995005474673311991943775795727481451058346239240361725119718297821926435889",));

    assert!(res.init);
    assert_eq!(res.index, expected_index);
    assert_eq!(res.collection_address, expected_collection_address);
    assert_eq!(
        res.individual_content.as_external().unwrap().uri.data,
        SnakeData::from_str("https://nft.fragment.com/number/88805397120.json")?.data
    );

    Ok(())
}

async fn assert_get_nft_address_by_index(ctr_cli: &ContractClient) -> anyhow::Result<()> {
    let address = TonAddress::from_str("EQB2iHQ9lmJ9zvYPauxN9hVOfHL3c_fuN5AyRq5Pm84UH6jC")?;
    let contract = NftCollectionContract::new(ctr_cli, address, None).await?;
    assert_ok!(contract.get_nft_address_by_index(2).await);
    Ok(())
}

async fn assert_get_nft_data_internal(ctr_cli: &ContractClient) -> anyhow::Result<()> {
    let address = TonAddress::from_str("EQDUF9cLVBH3BgziwOAIkezUdmfsDxxJHd6WSv0ChIUXYwCx")?;
    let contract = NftItemContract::new(ctr_cli, address, None).await?;
    let res = contract.get_nft_data().await?;

    let internal = match res.individual_content {
        MetadataContent::Internal(MetadataInternal { data: dict }) => dict,
        _ => panic!("Expected internal content"),
    };

    let expected_key = {
        let mut hasher: Sha256 = Sha256::new();
        hasher.update("public_keys");
        let slice = &hasher.finalize()[..];
        TonHash::from_slice(slice)?
    };
    assert!(internal.contains_key(&expected_key));
    Ok(())
}

async fn assert_get_collection_data_is_valid(ctr_cli: &ContractClient) -> anyhow::Result<()> {
    let address = TonAddress::from_str("EQAOQdwdw8kGftJCSFgOErM1mBjYPe4DBPq8-AhF6vr9si5N")?;
    let contract = NftCollectionContract::new(ctr_cli, address, None).await?;

    let res = assert_ok!(contract.get_collection_data().await);

    assert_eq!(res.next_item_index, -1);
    assert_eq!(
        res.collection_content.as_external().unwrap().uri.data,
        SnakeData::from_str("https://nft.fragment.com/numbers.json")?.data,
    );
    Ok(())
}

async fn assert_get_nft_address_by_index_is_valid(ctr_cli: &ContractClient) -> anyhow::Result<()> {
    let address = TonAddress::from_str("EQB2iHQ9lmJ9zvYPauxN9hVOfHL3c_fuN5AyRq5Pm84UH6jC")?;
    let contract = NftCollectionContract::new(ctr_cli, address, None).await?;

    let res_0 = assert_ok!(contract.get_nft_address_by_index(0).await);
    let res_2 = assert_ok!(contract.get_nft_address_by_index(2).await);
    let res_1 = assert_ok!(contract.get_nft_address_by_index(1).await);

    let expected_addr_0 = assert_ok!(TonAddress::from_str("EQBKwtMZSZurMxGp7FLZ_lM9t54_ECEsS46NLR3qfIwwTnKW"));
    let expected_addr_1 = assert_ok!(TonAddress::from_str("EQB6rnPIZr8dXmLy0xVp4lTe1AlYRwOUghEG9zzCcCkCp8IS"));
    let expected_addr_2 = assert_ok!(TonAddress::from_str("EQD0VQNu41wZmWMQjXfifnljGR0vOAULh0stBLItskMavwH0"));
    assert_eq!(res_0.nft_address, expected_addr_0);
    assert_eq!(res_1.nft_address, expected_addr_1);
    assert_eq!(res_2.nft_address, expected_addr_2);
    Ok(())
}
