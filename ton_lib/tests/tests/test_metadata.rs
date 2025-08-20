use crate::tests::utils::init_logging;
use crate::tests::utils::make_tl_client;
use sha2::Digest;
use sha2::Sha256;
use std::str::FromStr;
use std::time::Duration;
use tokio_test::assert_ok;
use ton_lib::contracts::client::contract_client::{ContractClient, ContractClientConfig};
use ton_lib::contracts::client::tl_provider::TLProvider;
use ton_lib::contracts::jetton_master::JettonMaster;
use ton_lib::contracts::methods::get_jetton_data::GetJettonData;
use ton_lib::contracts::methods::get_wallet_address::GetWalletAddress;
use ton_lib::contracts::ton_contract::TonContract;
use ton_lib::tep::IpfsLoaderConfig;
use ton_lib::tep::JettonMetaData;
use ton_lib::tep::LoadMeta;
use ton_lib::tep::MetaDataContent;
use ton_lib::tep::MetaDataExternal;
use ton_lib::tep::MetaLoader;
use ton_lib::tep::SnakeData;
use ton_lib_core::cell::TonHash;
use ton_lib_core::types::TonAddress;

#[tokio::test]
async fn test_get_jetton_content_uri() -> anyhow::Result<()> {
    init_logging();
    let tl_client = make_tl_client(true, true).await?;
    let config = ContractClientConfig::new_no_cache(Duration::from_millis(100));
    let data_provider = TLProvider::new(tl_client.clone());
    let ctr_cli = ContractClient::new(config, data_provider)?;

    let moon_jetton = TonAddress::from_str("EQDk2VTvn04SUKJrW7rXahzdF8_Qi6utb0wj43InCu9vdjrR")?;
    let contract = JettonMaster::new(&ctr_cli, moon_jetton, None).await?;

    let res = assert_ok!(contract.get_jetton_data().await);
    assert_eq!(
        res.content,
        MetaDataContent::External(MetaDataExternal {
            uri: SnakeData::<false>::from_str("https://tarantini.dev/ston/moon.json")?
        })
    );
    let meta_loader = MetaLoader::<JettonMetaData>::default();
    let content_res: JettonMetaData = assert_ok!(meta_loader.load(&res.content).await);
    assert_eq!(content_res.symbol.as_ref().unwrap(), &String::from("MOON"));
    assert_eq!(content_res.decimals.unwrap(), 0x9);
    Ok(())
}

#[tokio::test]
async fn test_get_jetton_content_internal_uri() -> anyhow::Result<()> {
    init_logging();
    let tl_client = make_tl_client(true, true).await?;
    let config = ContractClientConfig::new_no_cache(Duration::from_millis(100));
    let data_provider = TLProvider::new(tl_client.clone());
    let ctr_cli = ContractClient::new(config, data_provider)?;

    let fnz_jetton = TonAddress::from_str("EQDCJL0iQHofcBBvFBHdVG233Ri2V4kCNFgfRT-gqAd3Oc86")?;
    let contract = JettonMaster::new(&ctr_cli, fnz_jetton, None).await?;

    let res = assert_ok!(contract.get_jetton_data().await);
    let meta_loader = MetaLoader::<JettonMetaData>::default();
    let content_res: JettonMetaData = assert_ok!(meta_loader.load(&res.content).await);
    assert_eq!(content_res.symbol.as_ref().unwrap(), &String::from("FNZ"));
    assert_eq!(content_res.decimals.unwrap(), 0x9);
    Ok(())
}

#[tokio::test]
async fn test_get_jetton_content_internal_uri_jusdt() -> anyhow::Result<()> {
    init_logging();
    let tl_client = make_tl_client(true, true).await?;
    let config = ContractClientConfig::new_no_cache(Duration::from_millis(100));
    let data_provider = TLProvider::new(tl_client.clone());
    let ctr_cli = ContractClient::new(config, data_provider)?;

    let jusdt_jetton = TonAddress::from_str("EQBynBO23ywHy_CgarY9NK9FTz0yDsG82PtcbSTQgGoXwiuA")?;
    let contract = JettonMaster::new(&ctr_cli, jusdt_jetton, None).await?;

    let res = assert_ok!(contract.get_jetton_data().await);
    let meta_loader = MetaLoader::<JettonMetaData>::default();
    let content_res: JettonMetaData = assert_ok!(meta_loader.load(&res.content).await);
    assert_eq!(content_res.symbol.as_ref().unwrap(), &String::from("jUSDT"));
    assert_eq!(content_res.decimals, Some(6));
    Ok(())
}

#[tokio::test]
async fn test_get_jetton_content_empty_external_meta() -> anyhow::Result<()> {
    init_logging();
    let tl_client = make_tl_client(true, true).await?;
    let config = ContractClientConfig::new_no_cache(Duration::from_millis(100));
    let data_provider = TLProvider::new(tl_client.clone());
    let ctr_cli = ContractClient::new(config, data_provider)?;

    let jusdt_jetton = TonAddress::from_str("EQD-J6UqYQezuUm6SlPDnHwTxXqo4uHys_fle_zKvM5nYJkA")?;
    let contract = JettonMaster::new(&ctr_cli, jusdt_jetton, None).await?;

    let res = assert_ok!(contract.get_jetton_data().await);
    let meta_loader = MetaLoader::<JettonMetaData>::default();
    let content_res: JettonMetaData = assert_ok!(meta_loader.load(&res.content).await);
    assert_eq!(content_res.symbol.as_ref().unwrap(), &String::from("BLKC"));
    assert_eq!(content_res.decimals, Some(8));
    Ok(())
}

// this test is ignored due restrictions of cloudflare-ipfs.com
#[tokio::test]
#[ignore]
async fn test_get_jetton_content_ipfs_uri() -> anyhow::Result<()> {
    init_logging();
    let tl_client = make_tl_client(true, true).await?;
    let config = ContractClientConfig::new_no_cache(Duration::from_millis(100));
    let data_provider = TLProvider::new(tl_client.clone());
    let ctr_cli = ContractClient::new(config, data_provider)?;

    let bolt_jetton = TonAddress::from_str("EQD0vdSA_NedR9uvbgN9EikRX-suesDxGeFg69XQMavfLqIw")?;
    let contract = JettonMaster::new(&ctr_cli, bolt_jetton, None).await?;

    let res = assert_ok!(contract.get_jetton_data().await);
    let meta_loader = MetaLoader::<JettonMetaData>::default();
    let content_res: JettonMetaData = assert_ok!(meta_loader.load(&res.content).await);
    assert_eq!(content_res.symbol.as_ref().unwrap(), &String::from("BOLT"));
    log::info!("{:?}", content_res);
    log::info!("{:?}", content_res.image_data);
    assert_eq!(content_res.decimals.unwrap(), 0x9);

    log::info!("{:?}", res);
    Ok(())
}

#[tokio::test]
async fn test_get_semi_chain_layout_jetton_content() -> anyhow::Result<()> {
    init_logging();
    let tl_client = make_tl_client(true, true).await?;
    let config = ContractClientConfig::new_no_cache(Duration::from_millis(100));
    let data_provider = TLProvider::new(tl_client.clone());
    let ctr_cli = ContractClient::new(config, data_provider)?;

    let jusdc_jetton = TonAddress::from_str("EQB-MPwrd1G6WKNkLz_VnV6WqBDd142KMQv-g1O-8QUA3728")?;
    let contract = JettonMaster::new(&ctr_cli, jusdc_jetton, None).await?;

    let res = assert_ok!(contract.get_jetton_data().await);
    let meta_loader = MetaLoader::<JettonMetaData>::default();
    let content_res: JettonMetaData = assert_ok!(meta_loader.load(&res.content).await);
    log::info!("content_res: {:?}", content_res);
    assert_eq!(content_res.symbol.as_ref().unwrap(), &String::from("jUSDC"));
    assert_eq!(content_res.name.as_ref().unwrap(), &String::from("USD Coin"));
    assert_eq!(content_res.decimals.unwrap(), 0x6);
    Ok(())
}

#[tokio::test]
async fn test_get_wallet_address() -> anyhow::Result<()> {
    init_logging();
    let tl_client = make_tl_client(true, true).await?;
    let config = ContractClientConfig::new_no_cache(Duration::from_millis(100));
    let data_provider = TLProvider::new(tl_client.clone());
    let ctr_cli = ContractClient::new(config, data_provider)?;

    let wallet_addr = TonAddress::from_str("EQDk2VTvn04SUKJrW7rXahzdF8_Qi6utb0wj43InCu9vdjrR")?;
    let contract = JettonMaster::new(&ctr_cli, wallet_addr, None).await?;

    let owner_address = assert_ok!(TonAddress::from_str("EQB2BtXDXaQuIcMYW7JEWhHmwHfPPwa-eoCdefiAxOhU3pQg"));
    let wallet_address = assert_ok!(contract.get_wallet_address(&owner_address).await);
    assert_eq!("EQCGY3OVLtD9KRcOsP2ldQDtuY0FMzV7wPoxjrFbayBXc23c", wallet_address.to_string());
    Ok(())
}

#[tokio::test]
async fn test_get_jetton_data_invalid_utf8_sequence() -> anyhow::Result<()> {
    init_logging();
    let tl_client = make_tl_client(true, true).await?;
    let config = ContractClientConfig::new_no_cache(Duration::from_millis(100));
    let data_provider = TLProvider::new(tl_client.clone());
    let ctr_cli = ContractClient::new(config, data_provider)?;

    let addr = TonAddress::from_str("EQDX__KZ7A--poP3Newpo_zx4tQ-yl9yzRwlmg_vifxMEA8m")?; // Maybe parse
    let contract = JettonMaster::new(&ctr_cli, addr, None).await?;

    let res = assert_ok!(contract.get_jetton_data().await);
    log::info!("DATA: {:?}", res);
    let meta_loader = MetaLoader::<JettonMetaData>::default();
    let content_res: JettonMetaData = assert_ok!(meta_loader.load(&res.content).await);
    assert_eq!(content_res.symbol.as_ref().unwrap(), &String::from("DuRove's"));
    assert_eq!(content_res.decimals.unwrap(), 0x9);

    let addr = TonAddress::from_str("EQDoEAodkem9PJdk3W1mqjnkvRphNaWu0glIRzxQBNZuOIbP")?; // Maybe parse
    let contract = JettonMaster::new(&ctr_cli, addr, None).await?;

    let res = assert_ok!(contract.get_jetton_data().await);
    log::info!("DATA: {:?}", res);
    let meta_loader = MetaLoader::<JettonMetaData>::default();
    let content_res: JettonMetaData = assert_ok!(meta_loader.load(&res.content).await);
    assert_eq!(content_res.symbol.as_ref().unwrap(), &String::from("TFH"));
    assert_eq!(content_res.decimals.unwrap(), 0x9);
    Ok(())
}

#[tokio::test]
async fn test_jetton_image_data() -> anyhow::Result<()> {
    init_logging();
    let tl_client = make_tl_client(true, true).await?;
    let config = ContractClientConfig::new_no_cache(Duration::from_millis(100));
    let data_provider = TLProvider::new(tl_client.clone());
    let ctr_cli = ContractClient::new(config, data_provider)?;

    let jusdc_jetton = TonAddress::from_str("EQCTzzFI_I9OmQu5CfiK1k2TIV8ZvMtIWRkMjHdRrpfhJuBX")?;
    let contract = JettonMaster::new(&ctr_cli, jusdc_jetton, None).await?;

    let res = assert_ok!(contract.get_jetton_data().await);
    let meta_loader = MetaLoader::<JettonMetaData>::default();
    let content_res: JettonMetaData = assert_ok!(meta_loader.load(&res.content).await);

    let target_image_hash: TonHash = TonHash::from([
        45, 186, 67, 118, 224, 166, 76, 84, 0, 203, 69, 175, 47, 34, 164, 184, 36, 229, 51, 193, 17, 18, 84, 70, 179,
        240, 137, 163, 42, 147, 119, 220,
    ]);
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(content_res.image_data.unwrap());
    let img_hash = hasher.finalize()[..].to_vec();
    assert_eq!(target_image_hash.as_slice(), img_hash.as_slice());

    Ok(())
}
