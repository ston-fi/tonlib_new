use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak};
use std::thread;
use std::time::Instant;

use crate::bc_constants::{TON_MASTERCHAIN_ID, TON_SHARD_FULL};
use crate::clients::tonlibjson::clients_impl::client_raw::TLClientRaw;
use crate::clients::tonlibjson::tl_api::tl_request::TLRequest;
use crate::clients::tonlibjson::tl_api::tl_response::TLResponse;
use crate::clients::tonlibjson::tl_api::tl_types::{TLBlockId, TLConfig, TLOptions, TLOptionsInfo};
use crate::clients::tonlibjson::tlj_client::TLJClient;
use crate::clients::tonlibjson::tlj_config::{LiteNodeFilter, TLJClientConfig};
use crate::errors::TonlibError;
use crate::unwrap_tlj_response;
use async_trait::async_trait;
use tokio::sync::{oneshot, Mutex, Semaphore};

#[derive(Clone)]
pub struct TLJConnection {
    inner: Arc<Inner>,
}

struct RequestCtx {
    method: &'static str,
    send_time: Instant,
    sender: oneshot::Sender<Result<TLResponse, TonlibError>>,
}

struct Inner {
    client_raw: TLClientRaw,
    active_requests: Mutex<HashMap<u64, RequestCtx>>,
    semaphore: Arc<Semaphore>,
    next_request_id: AtomicU64,
}

static CONNECTION_COUNTER: AtomicU64 = AtomicU64::new(0);

impl TLJConnection {
    pub async fn new(config: &TLJClientConfig, semaphore: Arc<Semaphore>) -> Result<TLJConnection, TonlibError> {
        new_connection_checked(config, semaphore).await
    }

    async fn init(&self, options: TLOptions) -> Result<TLOptionsInfo, TonlibError> {
        let req = TLRequest::Init { options };
        unwrap_tlj_response!(self.exec_impl(&req).await?, TLOptionsInfo)
    }

    pub async fn exec_impl(&self, req: &TLRequest) -> Result<TLResponse, TonlibError> {
        let _permit = self.inner.semaphore.acquire().await;
        let req_id = self.inner.next_request_id.fetch_add(1, Ordering::Relaxed);

        let (sender, receiver) = oneshot::channel();
        let req_ctx = RequestCtx {
            method: req.into(),
            send_time: Instant::now(),
            sender,
        };

        let extra = req_id.to_string();
        self.inner.active_requests.lock().await.insert(req_id, req_ctx);
        // callback on_exec

        match self.inner.client_raw.send(req, &extra) {
            Ok(()) => {}
            Err(err) => {
                let req_ctx = self.inner.active_requests.lock().await.remove(&req_id).unwrap();
                let duration = req_ctx.send_time.elapsed();
                // on_error callback
                let result = Err(TonlibError::TLJSendError(err.to_string()));
                req_ctx.sender.send(result).unwrap(); // Send should always succeed, so something went terribly wrong
            }
        }
        receiver.await.unwrap_or_else(|err| Err(TonlibError::UnexpectedError(err.into())))
    }

    // pub async fn smc_run_get_method(
    //     &self,
    //     id: i64,
    //     method: &TonMethodId,
    //     stack: &[TvmStackEntry],
    // ) -> Result<SmcRunResult, TonClientError> {
    //     let func = TonFunction::SmcRunGetMethod {
    //         id,
    //         method: method.into(),
    //         stack: stack.to_vec(),
    //     };
    //     let result = self.invoke(&func).await?;
    //     match result {
    //         TonResult::SmcRunResult(result) => Ok(result),
    //         r => Err(TonClientError::unexpected_ton_result(
    //             TonResultDiscriminants::SmcRunResult,
    //             r,
    //         )),
    //     }
    // }
}

async fn new_connection_checked(
    config: &TLJClientConfig,
    semaphore: Arc<Semaphore>,
) -> Result<TLJConnection, TonlibError> {
    loop {
        let conn = new_connection(config, semaphore.clone()).await?;
        match config.connection_check {
            LiteNodeFilter::Health => match conn.get_mc_info().await {
                Ok(info) => match conn.get_block_header(info.last).await {
                    Ok(_) => break Ok(conn),
                    Err(err) => {
                        log::info!("Dropping connection to unhealthy node: {:?}", err);
                    }
                },
                Err(err) => {
                    log::info!("Dropping connection to unhealthy node: {:?}", err);
                }
            },
            LiteNodeFilter::Archive => {
                let info = TLBlockId {
                    workchain: TON_MASTERCHAIN_ID,
                    shard: TON_SHARD_FULL as i64,
                    seqno: 1,
                };
                conn.sync().await?;
                match conn.lookup_block(1, info, 0, 0).await {
                    Ok(_) => break Ok(conn),
                    Err(err) => log::info!("Dropping connection to unhealthy node: {:?}", err),
                }
            }
        }
    }
}

async fn new_connection(config: &TLJClientConfig, semaphore: Arc<Semaphore>) -> Result<TLJConnection, TonlibError> {
    let conn_id = CONNECTION_COUNTER.fetch_add(1, Ordering::Relaxed);
    let tag = format!("ton-conn-{conn_id}");

    let inner = Inner {
        client_raw: TLClientRaw::new(tag.clone()),
        active_requests: Mutex::new(HashMap::new()),
        semaphore,
        next_request_id: AtomicU64::new(0),
    };
    let inner_arc = Arc::new(inner);
    let inner_weak = Arc::downgrade(&inner_arc);
    let thread_builder = thread::Builder::new().name(tag.clone());
    let _join_handle = thread_builder.spawn(|| run_loop(tag, inner_weak))?;

    let conn = TLJConnection { inner: inner_arc };
    let _info = conn.init(config.init_opts.clone()).await?;
    Ok(conn)
}

#[async_trait]
impl TLJClient for TLJConnection {
    async fn get_connection(&self) -> Result<&TLJConnection, TonlibError> { Ok(self) }
}

/// Client run loop
fn run_loop(tag: String, weak_inner: Weak<Inner>) {
    // callback.on_connection_loop_start(&tag);
    while let Some(inner) = weak_inner.upgrade() {
        let (response, maybe_extra) = match inner.client_raw.receive(1.0) {
            Some(Ok(update)) => update,
            Some(Err(err)) => {
                // callback
                continue;
            }
            None => {
                // callback.on_idle(&tag);
                continue;
            }
        };
        // callback.on_update(&tag, &update);
        let maybe_req_id = match &maybe_extra {
            Some(s) => s.parse::<u64>().ok(),
            None => None,
        };
        let maybe_req_ctx = match maybe_req_id {
            Some(req_id) => inner.active_requests.blocking_lock().remove(&req_id),
            None => None,
        };
        match maybe_req_ctx {
            Some(req_ctx) => match req_ctx.sender.send(Ok(response)) {
                Ok(()) => {}
                Err(err) => {
                    log::error!("Fail to send response: {err:?}");
                }
            },
            None => {
                log::error!("Fail to find corresponding req_ctx for req_id ??? TODO");
            }
        }
    }
    // callback.on_connection_loop_exit(tag.as_str());
}
