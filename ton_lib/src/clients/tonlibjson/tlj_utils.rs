use crate::cell::build_parse::parser::CellParser;
use crate::cell::ton_cell::TonCell;
use crate::clients::lite::config::LiteClientConfig;
use crate::clients::lite::lite_client::LiteClient;
use crate::clients::tonlibjson::tl_api::tl_types::TLKeyStoreType;
use crate::clients::tonlibjson::tlj_config::TLJClientConfig;
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::block::BlockIdExt;
use crate::types::tlb::tlb_type::TLBType;
use crate::utils::tonlib_set_verbosity_level;
use futures_util::future::join_all;
use std::time::Duration;
use ton_liteapi::tl::response::BlockData;

const BLOCK_INFO_TAG: u32 = 0x9bc7a987;

pub async fn prepare_client_env(config: &mut TLJClientConfig) -> Result<(), TonlibError> {
    if config.update_init_block {
        update_init_block(config).await?;
    }

    if let TLKeyStoreType::Directory { directory } = &config.init_opts.keystore_type {
        std::fs::create_dir_all(directory)?
    }
    tonlib_set_verbosity_level(config.tonlib_verbosity_level);
    Ok(())
}

async fn update_init_block(config: &mut TLJClientConfig) -> Result<(), TonlibError> {
    log::info!("Updating init_block...");
    let lite_config = LiteClientConfig::new(&config.init_opts.config.net_config)?;
    let cur_init_seqno = lite_config.net_config.get_init_block_seqno();
    let lite_client = LiteClient::new(lite_config.clone())?;
    let lite_client_ref = &lite_client;
    let mut futs = vec![];
    for _ in lite_config.net_config.lite_endpoints.iter() {
        let future = async {
            let mc_info = lite_client_ref.get_mc_info().await?;
            let block = lite_client_ref.get_block(mc_info.last).await?;
            let seqno = parse_key_block_seqno(&block)?;
            lite_client_ref.lookup_mc_block(seqno).await
        };
        futs.push(future);
    }
    let exec_timeout = Duration::from_secs(config.update_init_block_timeout_sec.saturating_sub(1));
    let results = tokio::time::timeout(exec_timeout, join_all(futs)).await?;
    let mut max_block: Option<BlockIdExt> = None;
    for res in &results {
        match res {
            Ok(block) => {
                if max_block.is_none() || max_block.as_ref().unwrap().seqno < block.seqno {
                    max_block = Some(block.clone());
                }
            }
            Err(err) => log::warn!("Failed to get recent_init_block from particular node: {err:?}"),
        }
    }

    if let Some(block) = max_block {
        log::info!("init_block updated: {} -> {}", cur_init_seqno, block.seqno);
        let mut net_conf = lite_config.net_config.clone();
        net_conf.set_init_block(&block);
        config.init_opts.config.net_config = net_conf.to_json()?;
    }
    Ok(())
}

fn parse_key_block_seqno(block: &BlockData) -> Result<u32, TonlibError> {
    let block_cell = TonCell::from_boc(&block.data)?;
    if block_cell.refs.is_empty() {
        return Err(TonlibError::CustomError("No refs in block cell".to_string()));
        // TODO make proper block parser
    }
    let mut parser = CellParser::new(&block_cell.refs[0]);
    let tag: u32 = parser.read_num(32)?;
    if tag != BLOCK_INFO_TAG {
        return Err(TonlibError::CustomError("Invalid block tag".to_string())); // TODO make proper block parser
    }
    // version(32), merge_info(8), flags(8), seqno(32), vert_seqno(32), shard(104), utime(32), start/end lt(128),
    // validator_list_hash(32), catchain_seqno(32), min_ref_mc_seqno(32)
    parser.read_bits(32 + 8 + 8 + 32 + 32 + 104 + 32 + 128 + 32 + 32 + 32)?;
    let key_block_seqno = parser.read_num(32)?;
    Ok(key_block_seqno)
}
