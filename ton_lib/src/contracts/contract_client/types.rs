use crate::cell::ton_hash::TonHash;
use crate::clients::client_types::TxIdLTHash;
use crate::types::ton_address::TonAddress;

#[derive(Debug, Clone)]
pub struct ContractState {
    pub address: TonAddress,
    pub last_tx_id: TxIdLTHash,
    pub code_boc: Option<Vec<u8>>,
    pub data_boc: Option<Vec<u8>>,
    pub frozen_hash: Option<TonHash>,
    pub balance: i64,
}
