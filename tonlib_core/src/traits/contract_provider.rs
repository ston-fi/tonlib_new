use crate::cell::TonHash;
use crate::error::TLCoreError;
use crate::types::{TonAddress, TxId};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait]
pub trait ContractProvider: Send + Sync {
    /// if tx_id is None, returns latest state
    async fn get_state(&self, address: &TonAddress, tx_id: Option<&TxId>) -> Result<Arc<ContractState>, TLCoreError>;
    async fn run_get_method(&self, args: GetMethodArgs) -> Result<GetMethodResponse, TLCoreError>;
    async fn get_cache_state(&self) -> Result<HashMap<String, usize>, TLCoreError>;
}

#[derive(Debug, Clone)]
pub struct GetMethodArgs {
    pub address: TonAddress,
    pub state: GetMethodState,
    pub method_id: i32,
    pub stack_boc: Option<Vec<u8>>,
}

impl GetMethodArgs {
    /// uses latest state if tx_id is None
    pub fn new(address: TonAddress, tx_id: Option<TxId>, method_id: i32, stack_boc: Option<Vec<u8>>) -> Self {
        let state = match tx_id {
            Some(tx_id) => GetMethodState::TxId(tx_id),
            None => GetMethodState::Latest,
        };
        Self {
            address,
            state,
            method_id,
            stack_boc,
        }
    }

    pub fn with_stack(mut self, stack_boc: Vec<u8>) -> Self {
        self.stack_boc = Some(stack_boc);
        self
    }
}

#[derive(Debug, Clone)]
pub enum GetMethodState {
    Latest,
    TxId(TxId),
    Custom {
        code_boc: Vec<u8>,
        data_boc: Option<Vec<u8>>,
        balance: i64,
    },
}

#[derive(Debug, Clone)]
pub struct GetMethodResponse {
    pub exit_code: i32,
    pub stack_boc: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ContractState {
    pub address: TonAddress,
    pub mc_seqno: u32,
    pub last_tx_id: TxId,
    pub code_boc: Option<Vec<u8>>,
    pub data_boc: Option<Vec<u8>>,
    pub frozen_hash: Option<TonHash>,
    pub balance: i64,
}
