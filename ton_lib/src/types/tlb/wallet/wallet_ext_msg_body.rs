use crate::cell::ton_cell::TonCellRef;

/// https://docs.ton.org/participate/wallets/contracts#wallet-v2
#[derive(Debug, PartialEq, Clone)]
pub struct WalletV2ExtMsgBody {
    pub msg_seqno: u32,
    pub valid_until: u32,
    pub msgs_modes: Vec<u8>,
    pub msgs: Vec<TonCellRef>,
}

/// https://docs.ton.org/participate/wallets/contracts#wallet-v3
/// signature is not considered as part of msg body
/// documentation is not correct about body layout
#[derive(Debug, PartialEq, Clone)]
pub struct WalletV3ExtMsgBody {
    pub subwallet_id: i32,
    pub valid_until: u32,
    pub msg_seqno: u32,
    pub msgs_modes: Vec<u8>,
    pub msgs: Vec<TonCellRef>,
}

/// https://docs.ton.org/participate/wallets/contracts#wallet-v4
/// signature is not considered as part of msg body
#[derive(Debug, PartialEq, Clone)]
pub struct WalletV4ExtMsgBody {
    pub subwallet_id: i32,
    pub valid_until: u32,
    pub msg_seqno: u32,
    pub opcode: u32,
    pub msgs_modes: Vec<u8>,
    pub msgs: Vec<TonCellRef>,
}

/// https://docs.ton.org/participate/wallets/contracts#wallet-v5
/// signature is not considered as part of msg body
/// https://github.com/ton-blockchain/wallet-contract-v5/blob/main/types.tlb
/// This implementation support only jetton transfer messages
#[derive(Debug, PartialEq, Clone)]
pub struct WalletV5ExtMsgBody {
    pub wallet_id: i32,
    pub valid_until: u32,
    pub msg_seqno: u32,
    pub msgs_modes: Vec<u8>,
    pub msgs: Vec<TonCellRef>,
}

mod msgs_adapter {}
