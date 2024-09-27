use crate::pool_object::PoolObject;
use crate::Config;
use parking_lot::{Condvar, Mutex};

/// A pool of objects.
/// After an object is taken from the pool, it is returned to the pool when it is dropped.
/// Pool items must be passed on creation by values:
/// # Examples
/// basic usage:
/// ```
/// let pool = autoreturn_pool::Pool::new([1, 2]);
/// let item = pool.take();
/// ```
/// with custom config:
/// ```
/// let config = autoreturn_pool::Config {
///    wait_duration: std::time::Duration::from_millis(5),
/// };
/// let pool = autoreturn_pool::Pool::with_config(config, [1, 2]);
/// let item = pool.take();
/// ```
pub struct Pool<T: Send> {
    config: Config,
    storage: Mutex<Vec<T>>,
    condvar: Condvar,
}

impl<T: Send + 'static> Pool<T> {
    pub fn new(items: impl IntoIterator<Item = T>) -> Self {
        Self::with_config(Config::default(), items)
    }

    pub fn with_config(config: Config, items: impl IntoIterator<Item = T>) -> Self {
        let objects = items.into_iter().collect();
        Self {
            config,
            storage: Mutex::new(objects),
            condvar: Condvar::new(),
        }
    }

    /// Take an object from the pool.
    /// If the pool is empty, the method will wait for the object to be returned to the pool.
    /// If the wait duration is exceeded, the method will return `None`.
    pub fn take(&self) -> Option<PoolObject<T>> {
        let mut lock = self.storage.lock();
        while lock.is_empty() {
            let wait_res = self.condvar.wait_for(&mut lock, self.config.wait_duration);
            if wait_res.timed_out() {
                return None;
            }
        }
        let inner = lock.pop().unwrap();
        Some(PoolObject::new(inner, self))
    }

    /// Allows to add new object to the pool.
    pub fn add(&self, item: T) {
        self.storage.lock().push(item);
    }

    /// Get the number of available objects in the pool.
    pub fn size(&self) -> usize {
        self.storage.lock().len()
    }

    /// Put an object back into the pool and notify one waiting thread.
    pub(crate) fn put(&self, item: T) {
        self.storage.lock().push(item);
        self.condvar.notify_one();
    }
}

#[cfg(test)]
mod tests {
    use crate::pool::Pool;
    use crate::Config;
    use std::ops::Deref;

    #[test]
    fn test_create() {
        let pool = Pool::new([1, 2, 3]);
        assert_eq!(pool.size(), 3);
    }

    #[test]
    fn test_take() {
        let pool = Pool::new([1, 2, 3]);
        let obj1 = pool.take();
        assert_eq!(pool.size(), 2);
        assert_eq!(*obj1.as_ref().unwrap().deref(), 3);
    }

    #[test]
    fn test_add() {
        let pool = Pool::new([1]);
        pool.add(2);
        assert_eq!(pool.size(), 2);
    }

    #[test]
    fn test_wait() {
        let wait_time = std::time::Duration::from_millis(20);
        let config = Config {
            wait_duration: wait_time,
        };
        let pool = Pool::with_config(config, [1]);
        let _obj1 = pool.take();
        assert_eq!(pool.size(), 0);
        let start_time = std::time::Instant::now();
        let obj2 = pool.take();
        assert!(start_time.elapsed() >= wait_time);
        assert!(obj2.is_none());
    }

    #[test]
    fn test_workflow() -> anyhow::Result<()> {
        let config = Config {
            wait_duration: std::time::Duration::from_millis(5),
        };
        let pool = Pool::with_config(config, [1, 2, 3]);
        assert_eq!(pool.size(), 3);

        let obj1 = pool.take();
        assert_eq!(pool.size(), 2);
        assert_eq!(*obj1.as_ref().unwrap().deref(), 3);

        let obj2 = pool.take();
        assert_eq!(*obj2.as_ref().unwrap().deref(), 2);
        let obj3 = pool.take();
        assert_eq!(pool.size(), 0);
        assert_eq!(*obj3.as_ref().unwrap().deref(), 1);

        let obj4 = pool.take();
        assert!(obj4.is_none());

        Ok(())
    }
}
