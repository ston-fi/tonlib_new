use super::pool_object::PoolObject;
use crate::config::AutoPoolConfig;
use parking_lot::{Condvar, Mutex};
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
        self.get_impl(self.config.wait_duration)
    }

    /// Async version - tries to get object, sleep if fails until timeout
    #[cfg(feature = "async")]
    pub async fn get_async(&self) -> Option<PoolObject<T>> {
        if self.config.wait_duration.is_zero() {
            return self.get();
        }

        let start_time = Instant::now();
        while Instant::now() - start_time < self.config.wait_duration {
            if let Some(obj) = self.get_impl(self.config.lock_duration) {
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

    fn get_impl(&self, timeout: Duration) -> Option<PoolObject<T>> {
        if timeout.is_zero() {
            return self.storage.lock().pop().map(|x| PoolObject::new(x, self));
        }

        let mut lock = self.storage.lock();
        while lock.is_empty() {
            let wait_res = self.condvar.wait_for(&mut lock, timeout);
            if wait_res.timed_out() {
                return None;
            }
        }
        let inner = lock.pop().unwrap();
        Some(PoolObject::new(inner, self))
    }
}

#[cfg(test)]
mod tests {
    use crate::config::AutoPoolConfig;
    use crate::pool::AutoPool;
    use std::ops::Deref;

    #[test]
    fn test_create() {
        let pool = AutoPool::new([1, 2, 3]);
        assert_eq!(pool.size(), 3);
    }

    #[test]
    fn test_take() {
        let pool = AutoPool::new([1, 2, 3]);
        let obj1 = pool.get();
        assert_eq!(pool.size(), 2);
        assert_eq!(*obj1.as_ref().unwrap().deref(), 3);
    }

    #[tokio::test]
    #[cfg(feature = "async")]
    async fn test_take_async() {
        let pool = AutoPool::new([1, 2, 3]);
        let obj1 = pool.get_async().await;
        assert_eq!(pool.size(), 2);
        assert_eq!(*obj1.as_ref().unwrap().deref(), 3);
    }

    #[test]
    fn test_add() {
        let pool = AutoPool::new([1]);
        pool.add(2);
        assert_eq!(pool.size(), 2);
    }

    #[test]
    fn test_wait() {
        let wait_time = std::time::Duration::from_millis(20);
        let config = AutoPoolConfig {
            wait_duration: wait_time,
            ..Default::default()
        };
        let pool = AutoPool::new_with_config(config, [1]);
        let _obj1 = pool.get();
        assert_eq!(pool.size(), 0);
        let start_time = std::time::Instant::now();
        let obj2 = pool.get();
        assert!(start_time.elapsed() >= wait_time);
        assert!(obj2.is_none());
    }

    #[test]
    fn test_workflow() {
        let config = AutoPoolConfig {
            wait_duration: std::time::Duration::from_millis(5),
            ..Default::default()
        };
        let pool = AutoPool::new_with_config(config, [1, 2, 3]);
        assert_eq!(pool.size(), 3);

        let obj1 = pool.get();
        assert_eq!(pool.size(), 2);
        assert_eq!(*obj1.as_ref().unwrap().deref(), 3);

        let obj2 = pool.get();
        assert_eq!(*obj2.as_ref().unwrap().deref(), 2);
        let obj3 = pool.get();
        assert_eq!(pool.size(), 0);
        assert_eq!(*obj3.as_ref().unwrap().deref(), 1);

        let obj4 = pool.get();
        assert!(obj4.is_none());
    }
}
