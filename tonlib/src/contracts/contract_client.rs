use crate::error::TLError;
use std::collections::HashMap;
use std::sync::Arc;
use ton_lib_core::traits::contract_provider::{
    ContractMethodArgs, ContractMethodResponse, ContractProvider, ContractState,
};
use ton_lib_core::types::{TonAddress, TxIdLTHash};

// just a wrapper around ContractProvider for convenience
#[derive(Clone)]
pub struct ContractClient(Arc<dyn ContractProvider>);

impl ContractClient {
    pub fn new(data_provider: impl ContractProvider) -> Result<Self, TLError> {
        Ok(ContractClient(Arc::new(data_provider)))
    }

    pub async fn get_state(
        &self,
        address: &TonAddress,
        tx_id: Option<&TxIdLTHash>,
    ) -> Result<Arc<ContractState>, TLError> {
        Ok(self.0.get_contract(address, tx_id).await?)
    }

    pub async fn run_get_method(&self, args: ContractMethodArgs) -> Result<ContractMethodResponse, TLError> {
        Ok(self.0.run_get_method(args).await?)
    }

    pub async fn get_cache_stats(&self) -> Result<HashMap<String, usize>, TLError> {
        Ok(self.0.get_cache_stats().await?)
    }
}
