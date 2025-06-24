mod tl_provider_config;

use crate::block_tlb::TVMStack;
use crate::clients::block_stream::BlockStream;
use crate::clients::tl_client::tl::client::TLClientTrait;
use crate::clients::tl_client::TLClient;
use crate::clients::tl_provider::tl_provider_config::TLProviderConfig;
use crate::emulators::emul_bc_config::EmulBCConfig;
use crate::emulators::tvm::tvm_c7::TVMEmulatorC7;
use crate::emulators::tvm::tvm_emulator::TVMEmulator;
use crate::emulators::tvm::tvm_response::TVMGetMethodSuccess;
use crate::error::TLError;
use async_trait::async_trait;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use futures_util::future::{join_all, try_join_all};
use moka::future::Cache;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Weak};
use std::time::Duration;
use ton_lib_core::cell::{TonCell, TonCellUtils, TonHash};
use ton_lib_core::error::TLCoreError;
use ton_lib_core::traits::contract_provider::{
    ContractMethodArgs, ContractMethodResponse, ContractMethodState, ContractProvider, ContractState,
};
use ton_lib_core::traits::tlb::TLB;
use ton_lib_core::types::{TonAddress, TxIdLTHash};
use ton_lib_core::types::{TxId, TxIdLTAddress};

pub struct TLProvider(Arc<Inner>);

impl TLProvider {
    pub async fn new(config: TLProviderConfig, tl_client: TLClient) -> Result<Self, TLError> {
        Ok(Self(Inner::new(config, tl_client).await?))
    }
}

#[async_trait]
impl ContractProvider for TLProvider {
    async fn get_contract(
        &self,
        address: &TonAddress,
        tx_id: Option<&TxId>,
    ) -> Result<Arc<ContractState>, TLCoreError> {
        if let Some(id) = tx_id {
            self.0.cache_stats.state_by_tx_req.fetch_add(1, Relaxed);
            return Ok(self
                .0
                .state_by_tx_cache
                .try_get_with_by_ref(id, self.0.load_contract(address, Some(id)))
                .await?);
        }

        self.0.cache_stats.state_latest_req.fetch_add(1, Relaxed);
        let state = if let Some(id) = self.0.latest_tx_cache.get(address).await {
            self.0.state_latest_cache.try_get_with_by_ref(address, self.0.load_contract(address, Some(&id))).await?
        } else {
            self.0.state_latest_cache.try_get_with_by_ref(address, self.0.load_contract(address, None)).await?
        };
        Ok(state)
    }

    async fn run_get_method(&self, args: ContractMethodArgs) -> Result<ContractMethodResponse, TLCoreError> {
        let state = match args.method_state {
            ContractMethodState::Latest => self.get_contract(&args.address, None).await?,
            ContractMethodState::TxId(id) => self.get_contract(&args.address, Some(&id)).await?,
            ContractMethodState::Custom {
                mc_seqno,
                code_boc,
                data_boc,
                balance,
            } => Arc::new(ContractState {
                address: args.address.clone(),
                mc_seqno,
                last_tx_id: TxId::LTAddress(TxIdLTAddress {
                    lt: 0,
                    address: args.address.clone(),
                }),
                code_boc: Some(code_boc),
                data_boc,
                frozen_hash: None,
                balance,
            }),
        };
        let success = self.0.emulate_get_method(&state, args.method_id, args.stack_boc.as_deref()).await?;
        Ok(ContractMethodResponse {
            exit_code: success.vm_exit_code,
            stack_boc: BASE64_STANDARD.decode(success.stack_boc_base64)?,
        })
    }

    async fn get_cache_stats(&self) -> Result<HashMap<String, usize>, TLCoreError> {
        Ok(self.0.cache_stats.to_hashmap())
    }
}

struct Inner {
    tl_client: TLClient,
    bc_config: EmulBCConfig,
    latest_tx_cache: Cache<TonAddress, TxId>,
    state_latest_cache: Cache<TonAddress, Arc<ContractState>>,
    state_by_tx_cache: Cache<TxId, Arc<ContractState>>,
    cache_stats: CacheStats,
}

impl Inner {
    async fn new(config: TLProviderConfig, tl_client: TLClient) -> Result<Arc<Inner>, TLError> {
        let bc_config = tl_client.get_config_boc_all(0).await?;
        let inner = Arc::new(Inner {
            tl_client,
            bc_config: EmulBCConfig::from_boc(&bc_config)?,
            latest_tx_cache: init_cache(config.cache_capacity, config.cache_ttl),
            state_latest_cache: init_cache(config.cache_capacity, config.cache_ttl),
            state_by_tx_cache: init_cache(config.cache_capacity, config.cache_ttl),
            cache_stats: CacheStats::default(),
        });
        let block_stream = BlockStream::new(inner.tl_client.clone(), config.stream_from_seqno, None).await?;
        let weak = Arc::downgrade(&inner);
        tokio::spawn(recent_tx_loop(weak, block_stream, config.idle_on_error));
        Ok(inner)
    }

    async fn load_contract(
        &self,
        address: &TonAddress,
        tx_id: Option<&TxId>,
    ) -> Result<Arc<ContractState>, TLCoreError> {
        let account_state = match tx_id {
            Some(tx_id) => {
                self.cache_stats.state_by_tx_miss.fetch_add(1, Relaxed);
                self.tl_client.get_account_state_raw_by_tx(address.clone(), tx_id.clone().try_into()?).await?
            }
            None => {
                self.cache_stats.state_latest_miss.fetch_add(1, Relaxed);
                self.tl_client.get_account_state_raw(address.clone()).await?
            }
        };
        let code_boc = match account_state.code.is_empty() {
            true => None,
            false => Some(account_state.code),
        };

        let data_boc = match account_state.data.is_empty() {
            true => None,
            false => Some(account_state.data),
        };

        let frozen_hash = match account_state.frozen_hash.is_empty() {
            true => None,
            false => Some(TonHash::from_vec(account_state.frozen_hash)?),
        };

        Ok(Arc::new(ContractState {
            address: address.clone(),
            mc_seqno: account_state.block_id.seqno,
            last_tx_id: account_state.last_tx_id.into(),
            code_boc,
            data_boc,
            frozen_hash,
            balance: account_state.balance,
        }))
    }

    async fn emulate_get_method(
        &self,
        state: &ContractState,
        method: i32,
        stack: Option<&[u8]>,
    ) -> Result<TVMGetMethodSuccess, TLError> {
        let code_boc = match &state.code_boc {
            Some(boc) => boc,
            None => return Err(TLCoreError::ContractError("code is None at state: {state:?}".to_string()).into()),
        };

        let data_boc = state.data_boc.as_deref().unwrap_or(&[]);

        let c7 = TVMEmulatorC7 {
            address: state.address.clone(),
            unix_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(TLCoreError::from)?
                .as_secs() as u32,
            balance: state.balance as u64,
            rand_seed: TonHash::ZERO,
            config: self.bc_config.clone(),
        };

        let mut emulator = TVMEmulator::new(code_boc, data_boc, &c7)?;
        if let Some(libs) = self.get_libs_boc(code_boc, data_boc).await? {
            emulator.set_libs(&libs)?;
        }
        emulator.run_get_method(method, stack.unwrap_or(TVMStack::EMPTY_BOC))
    }

    async fn get_libs_boc(&self, code_boc: &[u8], data_boc: &[u8]) -> Result<Option<Vec<u8>>, TLCoreError> {
        let code = TonCell::from_boc(code_boc)?;
        let data = if data_boc.is_empty() {
            None
        } else {
            Some(TonCell::from_boc(data_boc)?)
        };
        let cells = [Some(&code), data.as_ref()].into_iter().flatten();
        let lib_ids = TonCellUtils::extract_lib_ids(cells)?;
        let libs = if lib_ids.is_empty() {
            return Ok(None);
        } else {
            self.tl_client.get_libs(lib_ids.into_iter().collect()).await?
        };
        libs.map(|x| x.to_boc()).transpose()
    }
}

async fn recent_tx_loop(weak_inner: Weak<Inner>, mut stream: BlockStream, idle_on_error: Duration) {
    log::info!("[recent_tx_loop] started with last_mc_seqno: {}", stream.last_mc_seqno());

    loop {
        let inner = match weak_inner.upgrade() {
            Some(inner) => inner,
            None => {
                log::warn!("[recent_tx_loop] inner is dropped");
                break;
            }
        };
        let inner_ref = &inner;

        let block_ids = match stream.next().await {
            Ok(Some(block_ids)) => block_ids,
            Ok(None) => {
                log::warn!("[tx_update_loop] got None from stream");
                break;
            }
            Err(err) => {
                log::warn!("[tx_update_loop] error getting next block: {:?}", err);
                tokio::time::sleep(idle_on_error).await;
                continue;
            }
        };

        let txs_by_addr_futs = block_ids.into_iter().map(|block_id| async move {
            let txs = inner_ref.tl_client.get_block_txs(&block_id).await?;
            let mut txs_by_addr = HashMap::new();
            for tx in txs {
                let address = TonAddress::new(block_id.shard_ident.wc, tx.address_hash);
                // in shard scope we always need the latest tx
                txs_by_addr.insert(address, TxIdLTHash::new(tx.lt, tx.tx_hash));
                // we always interesting in
            }
            Ok::<_, TLError>(txs_by_addr)
        });
        let txs_by_addr_vec = match try_join_all(txs_by_addr_futs).await {
            Ok(res) => res,
            Err(err) => {
                log::warn!("[recent_tx_loop] error getting shards_txs: {:?}", err);
                tokio::time::sleep(idle_on_error).await;
                continue;
            }
        };

        let mut txs_by_addr_latest = HashMap::<TonAddress, TxIdLTHash>::with_capacity(txs_by_addr_vec.len());
        for txs_by_addr in txs_by_addr_vec {
            for (address, tx_id) in txs_by_addr {
                if let Some(cur_tx_id) = txs_by_addr_latest.get(&address) {
                    if cur_tx_id.lt < tx_id.lt {
                        txs_by_addr_latest.insert(address, tx_id);
                    }
                } else {
                    txs_by_addr_latest.insert(address, tx_id);
                }
            }
        }

        let update_cache_futs = txs_by_addr_latest.into_iter().map(|(address, tx_id)| async move {
            inner_ref.latest_tx_cache.insert(address.clone(), TxId::LTHash(tx_id)).await;
            inner_ref.state_latest_cache.invalidate(&address).await;
        });
        join_all(update_cache_futs).await;
    }
    log::info!("[recent_tx_loop] completed");
}

#[derive(Default)]
struct CacheStats {
    pub state_latest_req: AtomicUsize,
    pub state_latest_miss: AtomicUsize,
    pub state_by_tx_req: AtomicUsize,
    pub state_by_tx_miss: AtomicUsize,
}

impl CacheStats {
    fn to_hashmap(&self) -> HashMap<String, usize> {
        HashMap::from([
            ("state_latest_req".to_string(), self.state_latest_req.load(Relaxed)),
            ("state_latest_miss".to_string(), self.state_latest_miss.load(Relaxed)),
            ("state_by_tx_req".to_string(), self.state_by_tx_req.load(Relaxed)),
            ("state_by_tx_miss".to_string(), self.state_by_tx_miss.load(Relaxed)),
        ])
    }
}

fn init_cache<K, V>(capacity: u64, ttl: Duration) -> Cache<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Send + Sync + Clone + 'static,
{
    Cache::builder().max_capacity(capacity).time_to_live(ttl).build()
}
