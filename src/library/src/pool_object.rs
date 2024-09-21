use crate::pool::Pool;
use std::ops::{Deref, DerefMut};

/// Wrapper allows to send object back to the pool when it's dropped
pub struct PoolObject<'a, T: Send + 'static> {
    inner: Option<T>,
    parent: &'a Pool<T>,
}

impl<'a, T: Send + 'static> PoolObject<'a, T> {
    pub(crate) fn new(inner: T, parent: &'a Pool<T>) -> Self {
        Self {
            inner: Some(inner),
            parent,
        }
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
        let inner = self.inner.take().unwrap();
        if let Err(err) = self.parent.put(inner) {
            log::error!("Failed to put object back to the pool: {}", err);
        }
    }
}
