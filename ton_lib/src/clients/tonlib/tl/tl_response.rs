use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::clients::tonlib::tl::stack::TvmCell;
use crate::clients::tonlib::tl::types::{
    BlockIdExt, BlocksHeader, BlocksMasterchainInfo, BlocksShards, BlocksTransactions, BlocksTransactionsExt,
    ConfigInfo, FullAccountState, LiteServerInfo, LogVerbosityLevel, OptionsInfo, RawExtMessageInfo,
    RawFullAccountState, RawTransactions, SmcInfo, SmcLibraryResult, SmcLibraryResultExt, SmcRunResult,
    UpdateSyncState,
};
use crate::errors::TonLibError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(tag = "@type", rename_all = "camelCase")]
pub enum TLResponse {
    // tonlib_api.tl, line 20
    Error {
        code: i32,
        message: String,
    },
    // tonlib_api.tl, line 21
    Ok {},
    // tonlib_api.tl, line 30
    #[serde(rename = "options.info")]
    OptionsInfo(OptionsInfo),
    // tonlib_api.tl, line 51
    #[serde(rename = "ton.blockIdExt")]
    BlockIdExt(BlockIdExt),
    // tonlib_api.tl, line 53
    #[serde(rename = "raw.fullAccountState")]
    RawFullAccountState(RawFullAccountState),
    // tonlib_api.tl, line 56
    #[serde(rename = "raw.transactions")]
    RawTransactions(RawTransactions),
    // tonlib_api.tl, line 58
    #[serde(rename = "raw.extMessageInfo")]
    RawExtMessageInfo(RawExtMessageInfo),
    // tonlib_api.tl, line 90
    #[serde(rename = "fullAccountState")]
    FullAccountState(FullAccountState),
    // tonlib_api.tl, line 167
    #[serde(rename = "tvm.cell")]
    TvmCell(TvmCell),
    // tonlib_api.tl, line 179
    #[serde(rename = "smc.info")]
    SmcInfo(SmcInfo),
    // tonlib_api.tl, line 184
    #[serde(rename = "smc.runResult")]
    SmcRunResult(SmcRunResult),
    // tonlib_api.tl, line 187
    #[serde(rename = "smc.libraryResult")]
    SmcLibraryResult(SmcLibraryResult),
    // tonlib_api.tl, line 191
    #[serde(rename = "smc.libraryResultExt")]
    SmcLibraryResultExt(SmcLibraryResultExt),
    // tonlib_api.tl, line 194
    #[serde(rename = "updateSyncState")]
    UpdateSyncState(UpdateSyncState),
    // tonlib_api.tl, line 203
    #[serde(rename = "liteServer.info")]
    LiteServerInfo(LiteServerInfo),
    // tonlib_api.tl, line 216
    #[serde(rename = "logVerbosityLevel")]
    LogVerbosityLevel(LogVerbosityLevel),
    // tonlib_api.tl, line 219
    #[serde(rename = "blocks.masterchainInfo")]
    BlocksMasterchainInfo(BlocksMasterchainInfo),
    // tonlib_api.tl, line 220
    #[serde(rename = "blocks.shards")]
    BlocksShards(BlocksShards),
    // tonlib_api.tl, line 223
    #[serde(rename = "blocks.transactions")]
    BlocksTransactions(BlocksTransactions),
    // tonlib_api.tl, line 224
    #[serde(rename = "blocks.transactionsExt")]
    BlocksTransactionsExt(BlocksTransactionsExt),
    // tonlib_api.tl, line 225
    #[serde(rename = "blocks.header")]
    BlocksHeader(BlocksHeader),
    // tonlib_api.tl, line 243
    #[serde(rename = "configInfo")]
    ConfigInfo(ConfigInfo),
}

impl TLResponse {
    pub fn expect_ok(&self) -> Result<(), TonLibError> {
        match self {
            TLResponse::Ok {} => Ok(()),
            r => Err(TonLibError::TonlibClientUnexpectedResult("OK".to_string(), "NE_OK".to_string())),
        }
    }
}
