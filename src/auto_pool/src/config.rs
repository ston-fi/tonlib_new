use std::time::Duration;

/// Configuration for the pool.
/// Wait indefinitely by default.
#[derive(Clone, Debug)]
pub struct Config {
    pub wait_duration: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            wait_duration: Duration::MAX,
        }
    }
}
