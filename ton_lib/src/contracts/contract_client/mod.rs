use crate::cell::ton_hash::TonHash;
use crate::clients::client_types::TxId;
use crate::contracts::contract_client::contract_client_cache::{ContractClientCache, ContractClientCacheConfig};
use crate::contracts::contract_client::data_provider::DataProvider;
use crate::contracts::contract_client::types::ContractState;
use crate::emulators::tvm::{TVMMethodId, TVMRunMethodSuccess};
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::tvm::TVMStack;
use crate::types::tlb::tlb_type::TLBType;
use crate::types::ton_address::TonAddress;
use std::sync::Arc;

mod contract_client_cache;
pub mod data_provider;
pub mod types;

pub struct ContractClient {
    inner: Arc<Inner>,
}

impl ContractClient {
    pub async fn new(
        cache_config: ContractClientCacheConfig,
        data_provider: Arc<dyn DataProvider>,
    ) -> Result<Self, TonlibError> {
        let latest_mc_seqno = data_provider.get_latest_mc_seqno().await?;
        Ok(Self {
            inner: Arc::new(Inner {
                data_provider: data_provider.clone(),
                cache: ContractClientCache::new(cache_config, data_provider, latest_mc_seqno)?,
            }),
        })
    }

    pub async fn get_state(
        &self,
        address: &TonAddress,
        tx_id: Option<&TxId>,
    ) -> Result<Arc<ContractState>, TonlibError> {
        self.inner.cache.get_state(address, tx_id).await
    }

    pub async fn get_config_boc(&self, mc_seqno: Option<u32>) -> Result<Vec<u8>, TonlibError> {
        self.inner.data_provider.get_config_boc(mc_seqno).await
    }

    pub async fn get_libs_boc(&self, lib_ids: &[TonHash]) -> Result<Option<Vec<u8>>, TonlibError> {
        self.inner.data_provider.get_libs_boc(lib_ids).await
    }

    pub async fn run_method<M>(
        &self,
        address: &TonAddress,
        method: M,
        stack: &TVMStack,
    ) -> Result<TVMRunMethodSuccess, TonlibError>
    where
        M: Into<TVMMethodId> + Send,
    {
        self.inner.data_provider.run_method(address, &method.into().as_str(), stack.to_boc(false)?).await
    }

    // pub async fn get_account_state_raw(&self, address: &TonAddress) -> Result<TLRawFullAccountState, TonlibError> {
    //     self.inner.data_provider.get_account_state_raw(address).await
    // }
}

struct Inner {
    data_provider: Arc<dyn DataProvider>,
    cache: ContractClientCache,
}
