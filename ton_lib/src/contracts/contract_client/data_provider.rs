use crate::cell::ton_hash::TonHash;
use crate::clients::client_types::{TxId, TxIdLTHash};
use crate::contracts::contract_client::types::ContractState;
use crate::emulators::emul_bc_config::EmulatorBCConfig;
use crate::emulators::tvm::tvm_response::TVMRunGetMethodSuccess;
use crate::errors::TonlibError;
use crate::types::ton_address::TonAddress;
use async_trait::async_trait;

/// If Optional argument is not specified, returns data for the latest mc_seqno
#[async_trait]
pub trait DataProvider: Send + Sync {
    async fn get_latest_mc_seqno(&self) -> Result<u32, TonlibError>;
    /// returns latest state if tx_id is None
    async fn get_state(&self, address: &TonAddress, tx_id: Option<&TxId>) -> Result<ContractState, TonlibError>;
    /// return latest config if mc_seqno is not specified
    async fn get_config_boc(&self, mc_seqno: Option<u32>) -> Result<EmulatorBCConfig, TonlibError>;
    /// doesn't check if all required libraries was received
    async fn get_libs_boc(&self, lib_ids: &[TonHash], mc_seqno: Option<u32>) -> Result<Option<Vec<u8>>, TonlibError>;
    async fn get_last_tx_id_per_addr(&self, mc_seqno: u32) -> Result<Vec<(TonAddress, TxIdLTHash)>, TonlibError>;

    async fn run_get_method(
        &self,
        address: &TonAddress,
        method: &str,
        stack_boc: Vec<u8>,
    ) -> Result<TVMRunGetMethodSuccess, TonlibError>;
}
