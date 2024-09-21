use std::fmt::{Debug, Display, Formatter};
use std::sync::PoisonError;

/// There is only one error type in this library:
/// PoisonError from std::sync.
#[derive(Debug)]
pub enum Error {
    LockError(String),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(err: PoisonError<T>) -> Self {
        Self::LockError(err.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LockError(err) => write!(f, "Lock error: {}", err),
        }
    }
}

impl std::error::Error for Error {}
