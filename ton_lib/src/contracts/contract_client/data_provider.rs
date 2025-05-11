use crate::cell::ton_hash::TonHash;
use crate::clients::client_types::{TxId, TxIdLTHash};
use crate::contracts::contract_client::types::ContractState;
use crate::emulators::tvm::TVMRunMethodSuccess;
use crate::errors::TonlibError;
use crate::types::ton_address::TonAddress;
use async_trait::async_trait;

#[async_trait]
pub trait DataProvider: Send + Sync {
    async fn get_latest_mc_seqno(&self) -> Result<u32, TonlibError>;
    /// returns latest state if tx_id is None
    async fn get_state(&self, address: &TonAddress, tx_id: Option<&TxId>) -> Result<ContractState, TonlibError>;
    async fn get_config_boc(&self, mc_seqno: Option<u32>) -> Result<Vec<u8>, TonlibError>;
    async fn get_libs_boc(&self, lib_ids: &[TonHash]) -> Result<Option<Vec<u8>>, TonlibError>;
    async fn get_block_tx_ids(&self, mc_seqno: u32) -> Result<Vec<(TonAddress, TxIdLTHash)>, TonlibError>;

    async fn run_method(
        &self,
        address: &TonAddress,
        method: &str,
        stack_boc: Vec<u8>,
    ) -> Result<TVMRunMethodSuccess, TonlibError>;
}
