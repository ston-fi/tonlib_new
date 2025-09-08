use crate::cell::TonHash;
use crate::error::TLCoreError;
use crate::types::{TonAddress, TxIdLTHash};
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
#[rustfmt::skip]
pub trait ContractProvider: Send + Sync + 'static {
    async fn last_mc_seqno(&self) -> Result<u32, TLCoreError>;
    /// if tx_id is None, returns latest state
    async fn load_state(&self, address: TonAddress, tx_id: Option<TxIdLTHash>) -> Result<ContractState, TLCoreError>;
    /// load latest blockchain config if mc_seqno is None
    async fn load_bc_config(&self, mc_seqno: Option<u32>) -> Result<Vec<u8>, TLCoreError>;
    
    async fn load_libs(&self, lib_ids: Vec<TonHash>, mc_seqno: Option<u32>) -> Result<Vec<(TonHash, Vec<u8>)>, TLCoreError>;
    
    async fn load_latest_tx_per_address(&self, mc_seqno: u32) -> Result<HashMap<TonAddress, TxIdLTHash>, TLCoreError>;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ContractState {
    pub mc_seqno: Option<u32>,
    pub address: TonAddress,
    pub last_tx_id: TxIdLTHash,
    pub code_boc: Option<Vec<u8>>,
    pub data_boc: Option<Vec<u8>>,
    pub frozen_hash: Option<TonHash>,
    pub balance: i64,
}
