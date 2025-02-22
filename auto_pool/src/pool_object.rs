use std::ops::{Deref, DerefMut};

use super::pool::AutoPool;

/// Wrapper allows to send object back to the pool when it's dropped
pub struct PoolObject<'a, T: Send + 'static> {
    inner: Option<T>,
    pool: &'a AutoPool<T>,
}

impl<'a, T: Send + 'static> PoolObject<'a, T> {
    pub(crate) fn new(inner: T, pool: &'a AutoPool<T>) -> Self {
        Self {
            inner: Some(inner),
            pool,
        }
    }

    /// Release inner object from the pool.
    /// It won't be put back to the pool when wrapper is dropped
    pub fn release(mut self) -> T {
        self.inner.take().unwrap()
    }
}

impl<T: Send + 'static> Deref for PoolObject<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

impl<T: Send + 'static> DerefMut for PoolObject<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}

impl<T: Send + 'static> Drop for PoolObject<'_, T> {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            self.pool.add(inner);
        }
    }
}
