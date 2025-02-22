use super::pool_object::PoolObject;
use crate::config::{AutoPoolConfig, PickStrategy};
use parking_lot::lock_api::{MutexGuard, RawMutex};
use parking_lot::{Condvar, Mutex};
use rand::RngCore;
use std::time::{Duration, Instant};

/// A pool of objects.
/// After an object is taken from the pool, it is returned to the pool when it is dropped.
/// Pool items must be passed on creation or added later:
/// # Examples
/// Basic usage:
/// ```
/// async fn test() {
///     use auto_pool::pool::AutoPool;
///     let pool = AutoPool::new([1, 2]);
///     let object1 = pool.get();
///     let object2 = pool.get_async().await;
///     pool.add(3);
///     let inner1 = object1.unwrap().release(); // won't be returned back
/// }
/// ```
///
/// Create with custom config:
/// ```
///     let config = auto_pool::config::AutoPoolConfig {
///         wait_duration: std::time::Duration::from_millis(5),
///         ..Default::default()
///     };
///     let pool = auto_pool::pool::AutoPool::new_with_config(config, [1, 2]);
///     let item = pool.get();
/// ```
pub struct AutoPool<T: Send> {
    config: AutoPoolConfig,
    storage: Mutex<Vec<T>>,
    condvar: Condvar,
}

impl<T: Send + 'static> AutoPool<T> {
    pub fn new(items: impl IntoIterator<Item = T>) -> Self {
        Self::new_with_config(AutoPoolConfig::default(), items)
    }

    pub fn new_with_config(config: AutoPoolConfig, items: impl IntoIterator<Item = T>) -> Self {
        let objects = items.into_iter().collect();
        Self {
            config,
            storage: Mutex::new(objects),
            condvar: Condvar::new(),
        }
    }

    /// Take an object from the pool.
    pub fn get(&self) -> Option<PoolObject<T>> {
        self.get_with_timeout(self.config.wait_duration)
    }

    /// Async version - tries to get object, sleep if fails until timeout
    #[cfg(feature = "async")]
    pub async fn get_async(&self) -> Option<PoolObject<T>> {
        if self.config.wait_duration.is_zero() {
            return self.get();
        }

        let start_time = Instant::now();
        while Instant::now() - start_time < self.config.wait_duration {
            if let Some(obj) = self.get_with_timeout(self.config.lock_duration) {
                return Some(obj);
            }
            async_std::task::sleep(self.config.sleep_duration).await;
        }
        None
    }

    /// Is used to return item back
    /// Also allows to add new item to the pool
    pub fn add(&self, item: T) {
        self.storage.lock().push(item);
        self.condvar.notify_one();
    }

    /// Get the number of available items
    pub fn size(&self) -> usize {
        self.storage.lock().len()
    }

    /// Shrink the pool to fit current number of items
    pub fn shrink_to_fit(&self) {
        self.storage.lock().shrink_to_fit();
    }

    fn get_with_timeout(&self, timeout: Duration) -> Option<PoolObject<T>> {
        let mut locked_storage = self.storage.lock();
        while locked_storage.is_empty() {
            let wait_res = self.condvar.wait_for(&mut locked_storage, timeout);
            if wait_res.timed_out() {
                return None;
            }
        }
        self.extract_object(locked_storage)
    }

    fn extract_object<R>(&self, mut locked_storage: MutexGuard<R, Vec<T>>) -> Option<PoolObject<T>>
    where
        R: RawMutex,
    {
        let inner = match self.config.pick_strategy {
            PickStrategy::LIFO => locked_storage.pop(),
            PickStrategy::RANDOM => match locked_storage.len() {
                0 => None,
                1 => locked_storage.pop(),
                items_cnt => {
                    let index = rand::rng().next_u64() as usize % items_cnt;
                    locked_storage.swap(index, items_cnt - 1);
                    locked_storage.pop()
                }
            },
        };
        inner.map(|inner| PoolObject::new(inner, self))
    }
}
