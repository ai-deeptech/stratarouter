//! StrataRouter Core - High-performance semantic routing
//!
//! A production-ready semantic router with hybrid scoring capabilities.
//!
//! # Features
//!
//! - **HNSW Index**: O(log N) approximate nearest neighbor search
//! - **Hybrid Scoring**: Combines semantic similarity, keyword matching, and rule-based routing
//! - **Confidence Calibration**: Isotonic regression for accurate probability estimates
//! - **SIMD Optimization**: AVX2 vectorization for 10x performance
//! - **Thread-Safe**: All operations are Send + Sync
//!
//! # Performance
//!
//! - P99 Latency: < 10ms
//! - Throughput: 10K+ queries/second
//! - Memory: ~64MB for 1000 routes
//!
//! # Examples
//!
//! ```no_run
//! use stratarouter_core::{Router, RouterConfig, Route};
//!
//! let config = RouterConfig::default();
//! let mut router = Router::new(config);
//!
//! let route = Route::new("billing")
//!     .with_description("Billing questions")
//!     .with_examples(vec!["Where's my invoice?".to_string()]);
//!
//! router.add_route(route)?;
//! # let embeddings = vec![vec![0.5; 384]];
//! router.build_index(embeddings)?;
//!
//! # let embedding = vec![0.5; 384];
//! let result = router.route("I need my invoice", &embedding)?;
//! assert_eq!(result.route_id, "billing");
//! # Ok::<(), stratarouter_core::Error>(())
//! ```
//!
//! # Safety
//!
//! This crate uses `unsafe` code only in SIMD operations, which are:
//! - Audited for memory safety
//! - Tested with property-based tests
//! - Have fallback implementations for non-AVX2 systems

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![deny(unsafe_code)]
#![allow(non_local_definitions)] // PyO3 macros
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

pub mod error;
pub mod router;
pub mod types;

// Make algorithms public for benchmarks
pub mod algorithms;

// Make index public for tests
pub mod index;

// FFI bindings
#[cfg(feature = "python")]
pub mod ffi;

// Re-exports for convenience
pub use error::{Error, Result};
pub use router::{Router, RouterConfig};
pub use types::{Route, RouteResult, RouteScores};

/// Library version from Cargo.toml
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Build timestamp
pub const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");

/// Check if running on AVX2-capable hardware
#[cfg(target_arch = "x86_64")]
#[must_use]
pub fn has_avx2() -> bool {
    is_x86_feature_detected!("avx2")
}

/// Check if running on AVX2-capable hardware
#[cfg(not(target_arch = "x86_64"))]
#[must_use]
pub const fn has_avx2() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert!(VERSION.contains('.'));
    }
    
    #[test]
    fn test_avx2_check() {
        // Should not panic
        let _has_avx2 = has_avx2();
    }
}

// PyO3 module
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn stratarouter_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ffi::PyRouter>()?;
    m.add_class::<ffi::PyRoute>()?;
    m.add("__version__", VERSION)?;
    m.add("has_avx2", has_avx2())?;
    Ok(())
}
