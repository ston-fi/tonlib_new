use crate::block_tlb::TVMStack;
use crate::contracts::client::stats::{CacheStats, CacheStatsLocal};
use crate::emulators::tvm::tvm_method_id::TVMGetMethodID;
use crate::emulators::tvm::tvm_response::TVMGetMethodSuccess;
use crate::error::TLError;
use moka::future::Cache;
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use std::time::Duration;
use ton_lib_core::traits::contract_provider::{ContractProvider, ContractState, ContractMethodArgs};
use ton_lib_core::traits::tlb::TLB;
use ton_lib_core::types::{TonAddress, TxId};

#[derive(Clone)]
pub struct ContractClient(Arc<dyn ContractProvider>);

impl ContractClient {
    pub fn new(data_provider: Arc<dyn ContractProvider>) -> Result<Self, TLError> { Ok(ContractClient(data_provider)) }

    pub async fn get_state(&self, address: &TonAddress, tx_id: Option<&TxId>) -> Result<Arc<ContractState>, TLError> {
        Ok(self.0.get_state(address, tx_id).await?)
    }

    pub async fn run_get_method<M, S, SO>(
        &self,
        address: &TonAddress,
        tx_id: Option<&TxId>,
        method: M,
        stack: SO,
    ) -> Result<TVMStack, TLError>
    where
        M: Into<TVMGetMethodID> + Send,
        S: AsRef<TVMStack>,
        SO: Into<Option<S>>,
    {
        let method_id = method.into().to_id();
        let stack_boc = stack.into().as_ref().map(|s| s.as_ref().to_boc()).transpose()?;
        let args = ContractMethodArgs::new(address.clone(), tx_id.cloned(), method_id, stack_boc);
        let rsp = self.0.run_get_method(args).await?;
        Ok(TVMStack::from_boc(&rsp.stack_boc)?)
    }

    pub async fn get_cache_stats(&self) -> Result<HashMap<String, usize>, TLError> {
        Ok(self.0.get_cache_stats().await?)
    }
}

async fn latest_tx_updater_loop(
    weak: Weak<Inner>,
    data_provider: Arc<dyn ContractProvider>,
    start_mc_seqno: u32,
    error_sleep_duration: Duration,
) -> Result<(), TLError> {
    log::info!("[ContractClient][run_loop] started with mc_seqno: {start_mc_seqno}");
    let mut cur_mc_seqno = start_mc_seqno;
    loop {
        let tx_ids = match data_provider.get_latest_txs(cur_mc_seqno).await {
            Ok(tx_ids) => tx_ids,
            Err(err) => {
                log::error!("[ContractClient][run_loop] get_latest_txs error for mc_seqno={cur_mc_seqno}: {err}");
                tokio::time::sleep(error_sleep_duration).await;
                continue;
            }
        };
        let Some(inner) = weak.upgrade() else { break };

        for (address, tx_id) in tx_ids {
            inner.state_by_address.invalidate(&address).await;
            inner.latest_tx_cache.insert(address, tx_id).await;
        }
        cur_mc_seqno += 1;
    }
    log::info!("[ContractClient] run_loop completed: inner is dropped");
    Ok(())
}

struct Inner {
    data_provider: Arc<dyn ContractProvider>,
    latest_tx_cache: Cache<TonAddress, TxId>,
    state_by_tx: Cache<TxId, Arc<ContractState>>,
    state_by_address: Cache<TonAddress, Arc<ContractState>>,
    cache_stats: CacheStatsLocal,
}
