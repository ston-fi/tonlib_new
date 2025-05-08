use super::connection::Connection;
use crate::bc_constants::{TON_MASTERCHAIN_ID, TON_SHARD_FULL};
use crate::clients::client_types::MasterchainInfo;
use crate::clients::lite::config::LiteClientConfig;
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::account::MaybeAccount;
use crate::types::tlb::block_tlb::block::BlockIdExt;
use crate::types::tlb::tlb_type::TLBType;
use crate::types::ton_address::TonAddress;
use auto_pool::config::{AutoPoolConfig, PickStrategy};
use auto_pool::pool::AutoPool;
use std::cmp::max;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use ton_liteapi::tl::common::AccountId;
use ton_liteapi::tl::request::{GetAccountState, GetBlock, LookupBlock, Request, WaitMasterchainSeqno, WrappedRequest};
use ton_liteapi::tl::response::{BlockData, Response};

const WAIT_MC_SEQNO_MS: u32 = 5000;
const WAIT_CONNECTION_MS: u64 = 5;

#[macro_export]
macro_rules! unwrap_lite_response {
    ($result:expr, $variant:ident) => {
        match $result {
            Response::$variant(inner) => Ok(inner),
            _ => {
                Err(TonlibError::TonLiteClientWrongResponse(stringify!($variant).to_string(), format!("{:?}", $result)))
            }
        }
    };
}

#[derive(Clone)]
pub struct LiteClient {
    inner: Arc<Inner>,
}

// converts ton_block -> ton_liteapi objects under the hood
impl LiteClient {
    pub fn new(config: LiteClientConfig) -> Result<Self, TonlibError> {
        Ok(Self {
            inner: Arc::new(Inner::new(config)?),
        })
    }

    pub async fn get_mc_info(&self) -> Result<MasterchainInfo, TonlibError> {
        let rsp = self.exec(Request::GetMasterchainInfo, None).await?;
        let mc_info = unwrap_lite_response!(rsp, MasterchainInfo)?;
        Ok(mc_info.into())
    }

    pub async fn lookup_mc_block(&self, seqno: u32) -> Result<BlockIdExt, TonlibError> {
        self.lookup_block(TON_MASTERCHAIN_ID, TON_SHARD_FULL, seqno).await
    }

    pub async fn lookup_block(&self, wc: i32, shard: u64, seqno: u32) -> Result<BlockIdExt, TonlibError> {
        let req = Request::LookupBlock(LookupBlock {
            mode: (),
            id: ton_liteapi::tl::common::BlockId {
                workchain: wc,
                shard,
                seqno,
            },
            seqno: Some(()),
            lt: None,
            utime: None,
            with_state_update: None,
            with_value_flow: None,
            with_extra: None,
            with_shard_hashes: None,
            with_prev_blk_signatures: None,
        });
        let rsp = self.exec(req, Some(seqno)).await?;
        let lite_id = unwrap_lite_response!(rsp, BlockHeader)?.id;
        Ok(lite_id.into())
    }

    pub async fn get_block(&self, block_id: BlockIdExt) -> Result<BlockData, TonlibError> {
        let seqno = block_id.seqno;
        let req = Request::GetBlock(GetBlock { id: block_id.into() });
        let rsp = self.exec(req, Some(seqno)).await?;
        unwrap_lite_response!(rsp, BlockData)
    }

    pub async fn get_account_state(&self, address: &TonAddress, mc_seqno: u32) -> Result<MaybeAccount, TonlibError> {
        let req = Request::GetAccountState(GetAccountState {
            id: self.lookup_mc_block(mc_seqno).await?.into(),
            account: AccountId {
                workchain: address.wc,
                id: address.hash.clone().into(),
            },
        });
        let rsp = self.exec(req, Some(mc_seqno)).await?;
        let account_state_rsp = unwrap_lite_response!(rsp, AccountState)?;
        MaybeAccount::from_boc(&account_state_rsp.state)
    }

    pub async fn exec(&self, req: Request, wait_mc_seqno: Option<u32>) -> Result<Response, TonlibError> {
        self.exec_with_timeout(req, self.inner.config.query_timeout, wait_mc_seqno).await
    }

    pub async fn exec_with_timeout(
        &self,
        request: Request,
        timeout: Duration,
        wait_mc_seqno: Option<u32>,
    ) -> Result<Response, TonlibError> {
        self.inner.exec_with_retries(request, timeout, wait_mc_seqno).await
    }
}

struct Inner {
    config: LiteClientConfig,
    conn_pool: AutoPool<Connection>,
    global_req_id: AtomicU64,
}

impl Inner {
    fn new(config: LiteClientConfig) -> Result<Self, TonlibError> {
        let conn_per_node = max(1, config.connections_per_node);
        log::info!(
            "Creating LiteClient with {} conns per node; nodes_cnt: {}, query_timeout: {:?}",
            conn_per_node,
            config.net_config.lite_endpoints.len(),
            config.query_timeout,
        );

        let mut connections = Vec::new();
        for _ in 0..conn_per_node {
            for endpoint in &config.net_config.lite_endpoints {
                let conn = Connection::new(endpoint.clone(), config.conn_timeout)?;
                connections.push(conn);
            }
        }
        let ap_config = AutoPoolConfig {
            wait_duration: Duration::MAX,
            lock_duration: Duration::from_millis(2),
            sleep_duration: Duration::from_millis(WAIT_CONNECTION_MS),
            pick_strategy: PickStrategy::RANDOM,
        };

        let connection_pool = AutoPool::new_with_config(ap_config, connections);

        Ok(Self {
            config,
            conn_pool: connection_pool,
            global_req_id: AtomicU64::new(0),
            // metrics,
        })
    }

    async fn exec_with_retries(
        &self,
        request: Request,
        req_timeout: Duration,
        wait_seqno: Option<u32>,
    ) -> Result<Response, TonlibError> {
        let wrap_req = WrappedRequest {
            wait_masterchain_seqno: wait_seqno.map(|seqno| WaitMasterchainSeqno {
                seqno,
                timeout_ms: WAIT_MC_SEQNO_MS,
            }),
            request,
        };
        let req_id = self.global_req_id.fetch_add(1, Relaxed);
        let max_ret_count = self.config.retry_count;
        let mut ret_num = 0;

        let retry_duration = self.config.retry_waiting;
        loop {
            log::trace!("[req_id={req_id} ret_num={ret_num}/{max_ret_count}] send req: {wrap_req:?}");
            let execute_result = self.exec_impl(req_id, ret_num, wrap_req.clone(), req_timeout).await;

            match execute_result {
                // case got response of error type
                Ok(Response::Error(err)) => {
                    log::trace!("[req_id={req_id} ret_num={ret_num}/{max_ret_count}] got response with error: {err:?}");
                    if ret_num == max_ret_count {
                        return Ok(Response::Error(err));
                    }
                }
                // case got response of any other type
                Ok(response) => break Ok(response),

                // case failed to get response with retryable error
                Err(err) => {
                    if ret_num == max_ret_count {
                        return Err(err);
                    }
                    log::warn!("[req_id={req_id} ret_num={ret_num}/{max_ret_count}] got error: {err:?}");
                }
            };
            ret_num += 1;
            sleep(retry_duration).await;
        }
    }

    async fn exec_impl(
        &self,
        _req_id: u64,
        _retry_num: u32,
        req: WrappedRequest,
        req_timeout: Duration,
    ) -> Result<Response, TonlibError> {
        // pool is configured to spin until get connection
        let mut conn = self.conn_pool.get_async().await.unwrap();
        conn.exec(req, req_timeout).await
    }
}
