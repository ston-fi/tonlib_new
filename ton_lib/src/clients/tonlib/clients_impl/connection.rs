use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak};
use std::thread;
use std::time::Instant;

use crate::bc_constants::{TON_MASTERCHAIN_ID, TON_SHARD_FULL};
use crate::clients::tonlib::clients_impl::client_raw::TLClientRaw;
use crate::clients::tonlib::tl_api::tl_req_ctx::TLRequestCtx;
use crate::clients::tonlib::tl_api::tl_request::TLRequest;
use crate::clients::tonlib::tl_api::tl_response::TLResponse;
use crate::clients::tonlib::tl_api::tl_types::{TLBlockId, TLOptions, TLOptionsInfo};
use crate::clients::tonlib::tl_callback::{TLCallback, TLCallbacksStore};
use crate::clients::tonlib::tl_client::TLClient;
use crate::clients::tonlib::tl_client_config::{LiteNodeFilter, TLClientConfig};
use crate::errors::TonlibError;
use crate::unwrap_tl_response;
use async_trait::async_trait;
use tokio::sync::{oneshot, Mutex, Semaphore};

static CONNECTION_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
pub struct TLConnection {
    inner: Arc<Inner>,
}

impl TLConnection {
    pub async fn new(config: &TLClientConfig, semaphore: Arc<Semaphore>) -> Result<TLConnection, TonlibError> {
        new_connection_checked(config, semaphore).await
    }

    async fn init(&self, options: TLOptions) -> Result<TLOptionsInfo, TonlibError> {
        let req = TLRequest::Init { options };
        unwrap_tl_response!(self.exec_impl(&req).await?, TLOptionsInfo)
    }

    pub async fn exec_impl(&self, req: &TLRequest) -> Result<TLResponse, TonlibError> {
        self.inner.exec_impl(req).await
    }
}

#[async_trait]
impl TLClient for TLConnection {
    async fn get_connection(&self) -> Result<&TLConnection, TonlibError> { Ok(self) }
}

async fn new_connection_checked(
    config: &TLClientConfig,
    semaphore: Arc<Semaphore>,
) -> Result<TLConnection, TonlibError> {
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

async fn new_connection(config: &TLClientConfig, semaphore: Arc<Semaphore>) -> Result<TLConnection, TonlibError> {
    let conn_id = CONNECTION_COUNTER.fetch_add(1, Ordering::Relaxed);
    let tag = format!("ton-conn-{conn_id}");

    let inner = Inner {
        client_raw: TLClientRaw::new(tag.clone()),
        active_requests: Mutex::new(HashMap::new()),
        semaphore,
        next_request_id: AtomicU64::new(0),
        callbacks: config.callbacks.clone(),
    };
    let inner_arc = Arc::new(inner);
    let inner_weak = Arc::downgrade(&inner_arc);
    let thread_builder = thread::Builder::new().name(tag.clone());
    let callbacks = config.callbacks.clone();
    let _join_handle = thread_builder.spawn(|| run_loop(tag, inner_weak, callbacks))?;

    let conn = TLConnection { inner: inner_arc };
    let _info = conn.init(config.init_opts.clone()).await?;
    Ok(conn)
}

struct Inner {
    client_raw: TLClientRaw,
    active_requests: Mutex<HashMap<u64, TLRequestCtx>>,
    semaphore: Arc<Semaphore>,
    next_request_id: AtomicU64,
    callbacks: TLCallbacksStore,
}

impl Inner {
    pub async fn exec_impl(&self, req: &TLRequest) -> Result<TLResponse, TonlibError> {
        let _permit = self.semaphore.acquire().await;
        let req_id = self.next_request_id.fetch_add(1, Ordering::Relaxed);
        let tag = self.client_raw.tag();

        let (sender, receiver) = oneshot::channel();
        let req_ctx = TLRequestCtx {
            req_id,
            req_method: req.into(),
            send_time: Instant::now(),
            sender,
        };

        self.callbacks.before_send(tag, &req_ctx, req);
        self.active_requests.lock().await.insert(req_id, req_ctx);

        let extra = req_id.to_string();
        if let Err(err) = self.client_raw.send(req, &extra) {
            let req_ctx = self.active_requests.lock().await.remove(&req_id).unwrap();
            self.callbacks.on_send_error(tag, &req_ctx, &err);
            return Err(err);
        }
        receiver.await?
    }
}

// receiving updates from tonlib
fn run_loop(tag: String, weak_inner: Weak<Inner>, callbacks: TLCallbacksStore) {
    callbacks.on_loop_enter(&tag);
    while let Some(inner) = weak_inner.upgrade() {
        let tag = inner.client_raw.tag();
        let result = match inner.client_raw.receive(1.0) {
            Some(res) => res,
            None => {
                callbacks.on_idle(tag);
                continue;
            }
        };
        callbacks.on_result(tag, &result);
        let (response, maybe_extra) = match result {
            Ok(res) => res,
            Err(_) => continue,
        };

        let maybe_req_ctx = maybe_extra
            .and_then(|extra| extra.parse::<u64>().ok())
            .and_then(|req_id| inner.active_requests.blocking_lock().remove(&req_id));

        callbacks.on_response(tag, &response, maybe_req_ctx.as_ref());

        if let Some(req_ctx) = maybe_req_ctx {
            if let Err(val) = req_ctx.sender.send(Ok(response)) {
                callbacks.on_notify_error(tag, &val);
            }
        }
    }
    callbacks.on_loop_enter(&tag);
}
