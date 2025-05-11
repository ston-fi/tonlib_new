use crate::clients::client_types::{TxId, TxIdLTHash};
use crate::contracts::contract_client::data_provider::DataProvider;
use crate::contracts::contract_client::types::ContractState;
use crate::errors::TonlibError;
use crate::types::ton_address::TonAddress;
use moka::future::Cache;
use std::sync::{Arc, Weak};
use std::time::Duration;

pub struct ContractClientCacheConfig {
    pub loop_error_sleep_duration: Duration,
    pub tx_ids_capacity: u64,
    pub tx_ids_ttl: Duration,
    pub states_capacity: u64,
    pub states_ttl: Duration,
}

#[allow(unused)]
pub struct ContractClientCacheStats {
    pub tx_ids_request: usize,
    pub tx_ids_miss: usize,
    pub tx_ids_size: usize,
    pub states_ids_request: usize,
    pub states_ids_miss: usize,
    pub states_ids_size: usize,
}

/// Except regular cache mechanic, maintain latest tx_ids using BlockStream
/// If cached entity tx_id doesn't match latest tx_id, update it
pub(super) struct ContractClientCache {
    inner: Arc<Inner>,
}

impl ContractClientCache {
    pub(crate) fn new(
        config: ContractClientCacheConfig,
        data_provider: Arc<dyn DataProvider>,
        start_mc_seqno: u32,
    ) -> Result<Self, TonlibError> {
        let loop_error_sleep_duration = config.loop_error_sleep_duration;

        let inner = Arc::new(Inner::new(config, data_provider.clone()));
        let weak = Arc::downgrade(&inner);
        tokio::spawn(run_loop(weak, data_provider, start_mc_seqno, loop_error_sleep_duration));
        Ok(Self { inner })
    }

    pub(crate) async fn get_state(
        &self,
        address: &TonAddress,
        tx_id: Option<&TxId>,
    ) -> Result<Arc<ContractState>, TonlibError> {
        if tx_id.is_some() {
            match self.inner.latest_states.try_get_with_by_ref(address, self.load_state(address, tx_id)).await {
                Ok(state) => return Ok(state),
                Err(err_ptr) => {
                    if err_ptr.to_string().contains("transaction hash mismatch") {
                        log::warn!("Fail to get state for address={address}, tx_id={tx_id:?}: {err_ptr}. Fallback to latest state.");
                        return self.load_state(address, tx_id).await;
                    }
                }
            }
        };
        todo!()
    }

    async fn load_state(&self, address: &TonAddress, tx_id: Option<&TxId>) -> Result<Arc<ContractState>, TonlibError> {
        self.inner.data_provider.get_state(address, tx_id).await.map(Arc::new)
    }

    // pub fn get_stats(&self) -> ContractClientCacheStats {
    //     let tx_ids_size = self.inner.latest_tx_ids.entry_count();
    //     let states_size = self.inner.latest_states.entry_count();
    //     ContractClientCacheStats {
    //         tx_ids_request: 1,
    //         tx_ids_miss: s2,
    //         tx_ids_size,
    //         states_ids_request: self.inner.latest_states.request_count(),
    //         states_ids_miss: self.inner.latest_states.miss_count(),
    //         states_ids_size,
    //     }
    // }
}

// invalidates cache for address if see new tx_id for this address
async fn run_loop(
    weak: Weak<Inner>,
    data_provider: Arc<dyn DataProvider>,
    start_mc_seqno: u32,
    error_sleep_duration: Duration,
) -> Result<(), TonlibError> {
    log::info!("[ContractClientCache] run_loop started with mc_seqno: {start_mc_seqno}");
    let mut cur_mc_seqno = start_mc_seqno;
    loop {
        let tx_ids = match data_provider.get_block_tx_ids(cur_mc_seqno).await {
            Ok(tx_ids) => tx_ids,
            Err(err) => {
                log::error!(
                    "[ContractClientCache] run_loop: fail to get block_tx_ids for mc_seqno={cur_mc_seqno}: {err}"
                );
                tokio::time::sleep(error_sleep_duration).await;
                continue;
            }
        };
        let Some(cache_data) = weak.upgrade() else { break };

        for (address, tx_id) in tx_ids {
            cache_data.latest_tx_ids.insert(address.clone(), tx_id).await;
            cache_data.latest_states.invalidate(&address).await;
        }
        cur_mc_seqno += 1;
    }
    log::info!("[ContractClientCache] run_loop completed: cache_data is dropped");
    Ok(())
}

struct Inner {
    data_provider: Arc<dyn DataProvider>,
    latest_tx_ids: Cache<TonAddress, TxIdLTHash>,
    latest_states: Cache<TonAddress, Arc<ContractState>>,
}

impl Inner {
    fn new(config: ContractClientCacheConfig, data_provider: Arc<dyn DataProvider>) -> Self {
        let latest_tx_ids =
            Cache::builder().max_capacity(config.tx_ids_capacity).time_to_live(config.tx_ids_ttl).build();

        let latest_states =
            Cache::builder().max_capacity(config.states_capacity).time_to_live(config.states_ttl).build();

        Self {
            data_provider,
            latest_tx_ids,
            latest_states,
        }
    }
}
