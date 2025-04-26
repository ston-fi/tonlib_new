use crate::clients::tonlibjson::tl_api::tl_response::TLResponse;
use crate::errors::TonlibError;
use std::fmt::Display;
use std::time::Instant;
use tokio::sync::oneshot;

pub struct TLRequestCtx {
    pub req_id: u64,
    pub req_method: &'static str,
    pub send_time: Instant,
    pub sender: oneshot::Sender<Result<TLResponse, TonlibError>>,
}

impl Display for TLRequestCtx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str =
            format!("req_id: {}, method: {}, elapsed: {:?}", self.req_id, self.req_method, self.send_time.elapsed());
        write!(f, "{}", str)
    }
}
