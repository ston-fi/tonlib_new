use std::time::Duration;

#[derive(Clone, Debug, Copy)]
pub struct AutoPoolConfig {
    /// Duration to wait for an object to be available
    pub wait_duration: Duration,
    /// For async operations, how long to keep the lock on the pool
    pub lock_duration: Duration,
    /// For async operations, how long to sleep between retries
    pub sleep_duration: Duration,
}

impl Default for AutoPoolConfig {
    fn default() -> Self {
        Self {
            wait_duration: Duration::MAX,
            lock_duration: Duration::from_millis(1),
            sleep_duration: Duration::from_millis(5),
        }
    }
}
