use pyo3::{exceptions::PyException, PyErr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StrataError {
    #[error("Invalid embedding dimension: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
    
    #[error("Route not found: {0}")]
    RouteNotFound(String),
    
    #[error("Invalid threshold: {0} (must be between 0.0 and 1.0)")]
    InvalidThreshold(f32),
    
    #[error("Empty route: {0}")]
    EmptyRoute(String),
    
    #[error("Invalid route name: {0}")]
    InvalidRouteName(String),
    
    #[error("Duplicate route: {0}")]
    DuplicateRoute(String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<StrataError> for PyErr {
    fn from(err: StrataError) -> PyErr {
        PyException::new_err(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, StrataError>;
