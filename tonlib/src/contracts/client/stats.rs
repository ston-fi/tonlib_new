use std::sync::atomic::AtomicUsize;

#[derive(Default)]
pub(super) struct CacheStatsLocal {
    pub state_latest_req: AtomicUsize,
    pub state_latest_miss: AtomicUsize,
    pub state_by_tx_req: AtomicUsize,
    pub state_by_tx_miss: AtomicUsize,
}

pub struct CacheStats {
    pub state_latest_req: usize,
    pub state_latest_miss: usize,
    pub state_by_tx_req: usize,
    pub state_by_tx_miss: usize,
}
