use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

#[derive(Default)]
pub(crate) struct CacheStats {
    pub state_latest_req: AtomicUsize,
    pub state_latest_miss: AtomicUsize,
    pub state_by_tx_req: AtomicUsize,
    pub state_by_tx_miss: AtomicUsize,
}

impl CacheStats {
    pub(crate) fn to_hashmap(&self) -> HashMap<String, usize> {
        HashMap::from([
            ("state_latest_req".to_string(), self.state_latest_req.load(Relaxed)),
            ("state_latest_miss".to_string(), self.state_latest_miss.load(Relaxed)),
            ("state_by_tx_req".to_string(), self.state_by_tx_req.load(Relaxed)),
            ("state_by_tx_miss".to_string(), self.state_by_tx_miss.load(Relaxed)),
        ])
    }
}
