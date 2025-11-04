use pyo3::prelude::*;

pub mod error;
pub mod router;
pub mod route;
pub mod similarity;
pub mod cache;

pub use error::{StrataError, Result};
pub use router::Router;
pub use route::{Route, RouteMatch};
pub use similarity::{cosine_similarity, cosine_similarity_batch};

/// Python module initialization
#[pymodule]
fn stratarouter_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Router>()?;
    m.add_class::<Route>()?;
    m.add_class::<RouteMatch>()?;
    
    m.add_function(wrap_pyfunction!(cosine_similarity, m)?)?;
    m.add_function(wrap_pyfunction!(cosine_similarity_batch, m)?)?;
    
    Ok(())
}
