use std::time::Duration;

pub struct TLProviderConfig {
    pub stream_from_seqno: u32,
    pub idle_on_error: Duration,
    pub cache_capacity: u64,
    pub cache_ttl: Duration,
}

impl TLProviderConfig {
    pub fn new_no_cache(stream_from_seqno: u32) -> Self { Self::new(stream_from_seqno, 0, Duration::from_secs(0)) }

    pub fn new(stream_from_seqno: u32, cache_capacity: u64, cache_ttl: Duration) -> Self {
        Self {
            stream_from_seqno,
            idle_on_error: Duration::from_millis(100),
            cache_capacity,
            cache_ttl,
        }
    }
}
