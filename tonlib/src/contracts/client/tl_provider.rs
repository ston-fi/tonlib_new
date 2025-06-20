use crate::clients::block_stream::BlockStream;
use crate::clients::tl_client::tl::client::TLClientTrait;
use crate::clients::tl_client::TLClient;
use crate::contracts::client::config::CacheUnit;
use crate::emulators::tvm::tvm_c7::TVMEmulatorC7;
use crate::emulators::tvm::tvm_emulator::TVMEmulator;
use crate::error::TLError;
use async_trait::async_trait;
use moka::future::Cache;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ton_lib_core::cell::{TonCell, TonHash};
use ton_lib_core::error::TLCoreError;
use ton_lib_core::traits::contract_provider::{ContractProvider, ContractState};
use ton_lib_core::traits::tlb::TLB;
use ton_lib_core::types::{TonAddress, TxId};

pub struct TLProvider {
    tl_client: TLClient,
    max_seen_mc_seqno: RwLock<u32>,
    block_stream: BlockStream,
}

impl TLProvider {
    pub async fn new(tl_client: TLClient, stream_from_seqno: u32) -> Result<Self, TLError> {
        let block_stream = BlockStream::new(tl_client.clone(), stream_from_seqno, None).await?;
        Self {
            tl_client,
            max_seen_mc_seqno: RwLock::new(0),
            block_stream: BlockStream::new(),
        }
    }
}

#[async_trait]
impl ContractProvider for TLDataProvider {
    async fn get_latest_mc_seqno(&self) -> Result<u32, TLCoreError> {
        let last_seqno = self.tl_client.get_mc_info().await?.last.seqno;
        if last_seqno > *self.max_seen_mc_seqno.read() {
            *self.max_seen_mc_seqno.write() = last_seqno;
            return Ok(last_seqno);
        }
        Ok(*self.max_seen_mc_seqno.read())
    }

    async fn get_state(&self, address: &TonAddress, tx_id: Option<&TxId>) -> Result<ContractState, TLCoreError> {
        let state_raw = match tx_id {
            Some(id) => self.tl_client.get_account_state_raw_by_tx(address.clone(), id.clone().try_into()?).await,
            None => self.tl_client.get_account_state_raw(address.clone()).await,
        }?;

        let code_boc = match state_raw.code.is_empty() {
            true => Some(state_raw.code),
            _ => None,
        };

        let data_boc = match state_raw.data.is_empty() {
            true => Some(state_raw.data),
            _ => None,
        };

        let frozen_hash = match state_raw.frozen_hash.is_empty() {
            true => None,
            _ => Some(TonHash::from_vec(state_raw.frozen_hash)?),
        };

        Ok(ContractState {
            address: address.clone(),
            mc_seqno: state_raw.block_id.seqno,
            last_tx_id: state_raw.last_tx_id.into(),
            code_boc,
            data_boc,
            frozen_hash,
            balance: state_raw.balance,
        })
    }

    async fn get_latest_txs(&self, _mc_seqno: u32) -> Result<HashMap<TonAddress, TxId>, TLCoreError> {
        // let block_txs = self.0.get_b
        todo!()
    }

    async fn run_get_method(
        &self,
        address: &TonAddress,
        tx_id: Option<&TxId>,
        _method: i32,
        _stack_boc: Vec<u8>,
    ) -> Result<String, TLCoreError> {
        let state = self.get_state(address, tx_id).await?;
        let code_boc = state.code_boc.as_deref().unwrap_or(&[]);
        let code_cell = state.code_boc.as_ref().map(|x| TonCell::from_boc(x)).transpose()?;

        let data_boc = state.data_boc.as_deref().unwrap_or(&[]);
        let data_cell = state.data_boc.as_ref().map(|x| TonCell::from_boc(x)).transpose()?;

        let mut emulator = match c7 {
            Some(c7) => TVMEmulator::new(code_boc, data_boc, c7)?,
            None => {
                let bc_config = ctx.client.get_config_boc(None).await?;
                let c7 = TVMEmulatorC7::new(ctx.address.clone(), bc_config)?;
                TVMEmulator::new(code_boc, data_boc, &c7)?
            }
        };
        let cells = [code_cell.as_ref(), data_cell.as_ref()].into_iter().flatten();
        let lib_ids = TonCellUtils::extract_lib_ids(cells)?;
        if !lib_ids.is_empty() {
            if let Some(libs_boc) = ctx.client.get_libs_boc(&lib_ids.into_iter().collect::<Vec<_>>()).await? {
                emulator.set_libs(&libs_boc)?;
            }
        }
        todo!()
    }
}

impl TLDataProvider {
    async fn get_config_boc(&self) -> Result<Vec<u8>, TLCoreError> { Ok(self.tl_client.get_config_boc_all(0).await?) }

    async fn get_libs_boc(&self, lib_ids: &[TonHash]) -> Result<Option<Vec<u8>>, TLCoreError> {
        let libs_dict = self.tl_client.get_libs(lib_ids.to_vec()).await?;
        libs_dict.map(|x| x.to_boc()).transpose()
    }
}

#[derive(Default)]
pub(super) struct CacheStatsLocal {
    pub state_latest_req: AtomicUsize,
    pub state_latest_miss: AtomicUsize,
    pub state_by_tx_req: AtomicUsize,
    pub state_by_tx_miss: AtomicUsize,
}

pub struct CacheStats {
    pub state_latest_req: usize,
    pub state_latest_miss: usize,
    pub state_by_tx_req: usize,
    pub state_by_tx_miss: usize,
}

impl From<&CacheStatsLocal> for CacheStats {
    fn from(stats: &CacheStatsLocal) -> Self {
        Self {
            state_latest_req: stats.state_latest_req.load(Ordering::Relaxed),
            state_latest_miss: stats.state_latest_miss.load(Ordering::Relaxed),
            state_by_tx_req: stats.state_by_tx_req.load(Ordering::Relaxed),
            state_by_tx_miss: stats.state_by_tx_miss.load(Ordering::Relaxed),
        }
    }
}

fn init_cache<K, V>(config: CacheUnit) -> Cache<K, V> {
    Cache::builder().max_capacity(config.capacity).time_to_live(config.ttl).build()
}

//
// match &tx_id {
// Some(_) => self.inner.cache_stats.state_by_tx_req.fetch_add(1, Ordering::Relaxed),
// None => self.inner.cache_stats.state_latest_req.fetch_add(1, Ordering::Relaxed),
// };
//
// let load_arc_state = async |tx_id_int: Option<&TxId>| {
// match &tx_id_int {
// Some(_) => self.inner.cache_stats.state_latest_miss.fetch_add(1, Ordering::Relaxed),
// None => self.inner.cache_stats.state_latest_miss.fetch_add(1, Ordering::Relaxed),
// };
// self.inner.data_provider.get_state(address, tx_id_int).await.map(Arc::new)
// };
//
// let required_tx = match tx_id {
// Some(tx_id) => Some(tx_id.clone()),
// None => self.inner.latest_tx_cache.get(address).await,
// };
//
// self.inner.state_by_tx.
//
// match tx_id {
// Some(tx_id) => return Ok(self.inner.state_cache.try_get_with_by_ref(tx_id, load_arc_state).await?),
// None => {
// self.inner.latest_tx_cache
// .get_or_try_insert_with(address, || self.inner.data_provider.get_state(address, None).await.map(Arc::new))
// .await?
// }
// }
//
// self.inner.state_by_tx
// self.inner.latest_tx_cache.get_state(address, tx_id).await

// pub(crate) async fn get_state(
//     &self,
//     address: &TonAddress,
//     tx_id: Option<&TxId>,
// ) -> Result<Arc<ContractState>, TLError> {
//     let load_arc_state = async |tx_id_int: Option<&TxId>| {
//         match &tx_id_int {
//             Some(_) => self.inner.state_by_tx_miss.fetch_add(1, Ordering::Relaxed),
//             None => self.inner.state_latest_req.fetch_add(1, Ordering::Relaxed),
//         };
//         self.inner.data_provider.get_state(address, tx_id_int).await.map(Arc::new)
//     };
//
//     match &tx_id {
//         Some(_) => self.inner.state_latest_req.fetch_add(1, Ordering::Relaxed),
//         None => self.inner.state_latest_miss.fetch_add(1, Ordering::Relaxed),
//     };
//
//     if let Some(tx_id) = tx_id {
//         return match self.inner.latest_states.get(address).await {
//             Some(state) if &state.last_tx_id == tx_id => Ok(state.clone()),
//             _ => Ok(load_arc_state(Some(tx_id)).await?),
//         };
//     }
//
//     let recent_tx = self.inner.latest_txs.get(address).await;
//     Ok(self.inner.latest_states.try_get_with(address.clone(), load_arc_state(recent_tx.as_ref())).await?)
// }
