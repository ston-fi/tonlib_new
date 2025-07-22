use log::LevelFilter;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::config::{Appender, Root};
use log4rs::Config;
use std::sync::Once;
use std::time::Duration;
use tokio;
use ton_lib::block_tlb::{Coins, CommonMsgInfoInt, Msg};
use ton_lib::block_tlb::{CommonMsgInfo, CurrencyCollection};
use ton_lib::clients::tl_client::tl::client::TLClientTrait;
use ton_lib::clients::tl_client::TLClient;
use ton_lib::clients::tl_client::TLClientConfig;
use ton_lib::contracts::contract_client::ContractClient;
use ton_lib::contracts::tl_provider::provider::TLProvider;
use ton_lib::contracts::tl_provider::provider_config::TLProviderConfig;
use ton_lib::contracts::ton_contract::TonContract;
use ton_lib::contracts::ton_wallet::TonWalletContract;
use ton_lib::sys_utils::sys_tonlib_set_verbosity_level;
use ton_lib::wallet::KeyPair;
use ton_lib::wallet::Mnemonic;
use ton_lib::wallet::TonWallet;
use ton_lib::wallet::WalletVersion;
use ton_lib_core::boc::BOC;
use ton_lib_core::traits::tlb::TLB;
use ton_lib_core::types::tlb_core::MsgAddress;
use ton_lib_core::{cell::TonCell, types::tlb_core::EitherRefLayout, *};
//https://docs.ton.org/v3/guidelines/smart-contracts/howto/wallet#-external-and-internal-messages
/* Plan:
    - Ton transfer (We will use wallet v4)
        - make an internal message with empty sell. It will signal that it is transfer
        - make an correct external message, and put there an internal message
        - send message to ton blockchain
*/
static LOG: Once = Once::new();

pub(crate) fn init_logging() {
    LOG.call_once(|| {
        let stderr = ConsoleAppender::builder()
            .target(Target::Stderr)
            .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
                "{d(%Y-%m-%d %H:%M:%S%.6f)} {T:>15.15} {h({l:>5.5})} {t}:{L} - {m}{n}",
            )))
            .build();

        let config = Config::builder()
            .appender(Appender::builder().build("stderr", Box::new(stderr)))
            .build(Root::builder().appender("stderr").build(LevelFilter::Info))
            .unwrap();

        log4rs::init_config(config).unwrap();
    })
}

pub(crate) async fn make_tl_client(mainnet: bool, archive_only: bool) -> anyhow::Result<TLClient> {
    init_logging();
    log::info!("Initializing tl_client with mainnet={mainnet}...");
    let mut config = match mainnet {
        true => TLClientConfig::new_mainnet(archive_only),
        false => TLClientConfig::new_testnet(archive_only),
    };
    config.connections_count = 10;
    config.retry_strategy.retry_count = 10;
    let client = TLClient::new(config).await?;
    sys_tonlib_set_verbosity_level(0);
    Ok(client)
}

fn make_keypair(mnemonic_str: &str) -> KeyPair {
    let mnemonic = Mnemonic::from_str(mnemonic_str, None).unwrap();
    mnemonic.to_key_pair().unwrap()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // ---------- Wallet initial ----------
    let mnemonic_str: String = std::env::var("MNEMONIC_STR").unwrap();
    let key_pair = make_keypair(&mnemonic_str);
    // To create w5 testnet wallet you need to use TonWallet::new_with_params with WALLET_V5R1_DEFAULT_ID_TESTNET wallet_id
    let wallet = TonWallet::new(WalletVersion::V4R2, key_pair)?;

    // Make testnet client
    let tl_client = make_tl_client(false, false).await?;
    let head_seqno = tl_client.get_mc_info().await?.last.seqno;
    let provider_config = TLProviderConfig::new_no_cache(head_seqno);
    let data_provider = TLProvider::new(provider_config, tl_client.clone()).await?;
    let ctr_cli = ContractClient::new(data_provider)?;

    // ---------- Building internal message ----------
    let builder = TonCell::builder();
    let transfer_cell = builder.build()?;

    let internal_message_info = CommonMsgInfo::Int(CommonMsgInfoInt {
        ihr_disabled: false,
        bounce: false,
        bounced: false,
        src: MsgAddress::NONE,
        dst: wallet.address.to_msg_address_int().into(),
        value: CurrencyCollection::new(50010u128),
        ihr_fee: Coins::ZERO,
        fwd_fee: Coins::ZERO,
        created_lt: 0, // lt
        created_at: 0,
    });

    let internal_msg = Msg {
        info: internal_message_info,
        init: None,
        body: types::tlb_core::TLBEitherRef {
            value: transfer_cell,
            layout: EitherRefLayout::ToRef,
        },
    };
    let int_msg_cell_ref = internal_msg.to_cell_ref()?;

    let expired_at_time = std::time::SystemTime::now() + Duration::from_secs(600);
    let expire_at = expired_at_time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u32;

    // Get current wallet seqno
    let contract = TonWalletContract::new(&ctr_cli, wallet.address.clone(), None)?;
    let seqno = contract.seqno().await?;

    let ext_cell = wallet.create_ext_in_msg(vec![int_msg_cell_ref.clone()], seqno, expire_at, false)?;
    let bag_of_cells = BOC::new(ext_cell.into_ref());
    let _ = tl_client.send_msg(bag_of_cells.to_bytes(true).unwrap()).await?;

    Ok(())
}
