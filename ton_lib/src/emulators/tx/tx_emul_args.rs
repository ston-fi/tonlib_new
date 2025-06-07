use crate::cell::ton_hash::TonHash;
use crate::emulators::emul_bc_config::EmulatorBCConfig;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct EmulOrdinalArgs {
    pub in_msg_boc: Vec<u8>,
    pub emul_args: EmulArgs,
}

pub struct EmulTickTockArgs {
    pub is_tock: bool,
    pub emul_args: EmulArgs,
}

#[derive(Debug, Clone)]
pub struct EmulArgs {
    pub shard_account_boc: Vec<u8>,
    pub bc_config: EmulatorBCConfig,
    pub rand_seed: TonHash,
    pub utime: u32,
    pub lt: u64,
    pub ignore_chksig: bool,
    pub prev_blocks_boc: Option<Vec<u8>>,
    pub libs_boc: Option<Vec<u8>>,
}

impl Display for EmulArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let shard_acc_str = hex::encode(&self.shard_account_boc);

        let prev_blocks_str = match &self.prev_blocks_boc {
            None => "None",
            Some(boc) => &hex::encode(boc),
        };

        let libs_str = match &self.libs_boc {
            None => "None",
            Some(boc) => &hex::encode(boc),
        };

        f.write_fmt(format_args!(
            "shard_account_boc: {}, bc_config: {}, rand_seed: {}, utime: {}, lt: {}, ignore_chksig: {}, prev_blocks_boc: {}, libs_boc: {}",
            shard_acc_str, self.bc_config.to_string_lossy(), self.rand_seed, self.utime, self.lt, self.ignore_chksig, prev_blocks_str, libs_str
        ))
    }
}

impl Display for EmulOrdinalArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("in_msg_boc : {}, emul_args: {}", hex::encode(&self.in_msg_boc), &self.emul_args))
    }
}

impl Display for EmulTickTockArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("is_tock: {}, emul_args: {}", self.is_tock, &self.emul_args))
    }
}
