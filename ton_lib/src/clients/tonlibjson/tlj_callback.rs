use std::time::Duration;

use crate::clients::tonlibjson::tl_api::tl_request::TLRequest;
use crate::clients::tonlibjson::tl_api::tl_response::TLResponse;
use crate::clients::tonlibjson::tl_api::tl_types::TLUpdate;
use crate::errors::TonlibError;

/// Check connection.rs (mostly `run_loop` method) for method execution flow
pub trait TLJCallback: Send + Sync {
    /// called **before** invoking tonlib.
    fn before_exec(&self, tag: &str, req_id: u32, req: &TLRequest) {}
    fn on_conn_loop_start(&self, tag: &str) {}
    fn on_conn_loop_end(&self, tag: &str) {}
    fn on_result(
        &self,
        tag: &str,
        req_id: u32,
        method: &str,
        duration: &Duration,
        result: &Result<TLResponse, TonlibError>,
    ) {
    }
    fn on_cancel(&self, tag: &str, req_id: u32, method: &str, duration: &Duration) {}
    fn on_update(&self, tag: &str, notification: &TLUpdate) {}
    fn on_parser_response_error(&self, tag: &str, req_extra: Option<&str>, result: &TLResponse) {}
    fn on_idle(&self, tag: &str) {}
}

pub struct TLJCallbackNoop {}
impl TLJCallback for TLJCallbackNoop {}

pub struct TLJCallbackLogTrace {}
impl TLJCallback for TLJCallbackLogTrace {
    fn on_conn_loop_start(&self, tag: &str) {
        log::info!("[{tag}] Starting event loop");
    }
    fn on_conn_loop_end(&self, tag: &str) {
        log::info!("[{tag}] Exiting event loop");
    }

    fn on_result(
        &self,
        tag: &str,
        req_id: u32,
        method: &str,
        duration: &Duration,
        result: &Result<TLResponse, TonlibError>,
    ) {
        match result {
            Ok(rsp) => {
                let rsp_str: &'static str = rsp.into();
                log::trace!(
                    "[{tag}] Invoke successful, req_id: {req_id}, method: {method}, elapsed: {duration:?}: {rsp_str}",
                );
            }
            Err(err) => {
                log::warn!(
                    "[{tag}] Invocation error: req_id: {req_id}, method: {method}, elapsed: {duration:?}: {err}",
                );
            }
        }
    }

    fn on_cancel(&self, tag: &str, req_id: u32, method: &str, duration: &Duration) {
        log::warn!(
            "[{tag}] Can't send exec result: receiver is closed. method: {method} req_id: {req_id}, elapsed: {duration:?}",
       );
    }

    fn on_update(&self, tag: &str, update: &TLUpdate) {
        log::trace!("[{tag}] Sending notification: {update:?}");
    }

    fn on_parser_response_error(&self, tag: &str, req_extra: Option<&str>, response: &TLResponse) {
        let rsp_str: &'static str = response.into();
        log::error!("[{tag}] Fail to parse response (req_extra: {req_extra:?}): {rsp_str}");
    }
}
