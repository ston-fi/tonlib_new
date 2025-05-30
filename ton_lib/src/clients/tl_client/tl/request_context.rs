use std::fmt::Display;
use std::time::Instant;

use tokio::sync::oneshot::Sender;

use crate::{clients::tl_client::tl::response::TLResponse, errors::TonlibError};

pub struct TLRequestCtx {
    pub id: u64,
    pub method: &'static str,
    pub send_time: Instant,
    pub sender: Sender<Result<TLResponse, TonlibError>>,
}

impl Display for TLRequestCtx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!("req_id: {}, method: {}, elapsed: {:?}", self.id, self.method, self.send_time.elapsed());
        write!(f, "{}", str)
    }
}
