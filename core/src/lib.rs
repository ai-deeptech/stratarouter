//! StrataRouter Core — high-performance semantic routing engine.
//!
//! This crate provides the Rust core that powers the StrataRouter Python SDK.
//! It can also be used directly as a standalone Rust library.
//!
//! # Architecture
//!
//! - **[`Router`]** — the main entry point. Holds routes and the search index.
//! - **[`LinearIndex`]** — brute-force cosine-distance index (O(N) scan).
//!   A graph-based HNSW replacement is planned; see ROADMAP.md.
//! - **[`HybridScorer`]** — combines dense semantic similarity, BM25 keyword
//!   matching, and pattern rules into a single confidence score.
//! - **[`CalibrationManager`]** — piecewise-linear score normalisation per route.
//!
//! # Quick start
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
//!
//! let embeddings = vec![vec![0.5_f32; 384]];
//! router.build_index(embeddings)?;
//!
//! let embedding = vec![0.5_f32; 384];
//! let result = router.route("I need my invoice", &embedding)?;
//! assert_eq!(result.route_id, "billing");
//! # Ok::<(), stratarouter_core::Error>(())
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
// clippy::pedantic is intentionally NOT enabled in CI — it fires on
// legitimate numeric casts (cast_precision_loss, cast_possible_truncation)
// that are correct in this codebase. Re-enable selectively when each
// pedantic lint has been individually reviewed and suppressed.
#![deny(unsafe_code)]
#![allow(non_local_definitions)] // PyO3 macros
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

pub mod algorithms;
pub mod cache;
pub mod error;
pub mod index;
pub mod router;
pub mod types;

/// Python FFI bindings (compiled only with `--features python`).
#[cfg(feature = "python")]
pub mod ffi;

// Top-level re-exports for ergonomic use.
pub use algorithms::{CalibrationManager, HybridScorer};
pub use error::{Error, Result};
pub use index::LinearIndex;
pub use router::{Router, RouterConfig};
pub use types::{Route, RouteResult, RouteScores};

/// Crate version, taken from `Cargo.toml` at compile time.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Unix timestamp (seconds) of the current build, if set at build time.
/// Falls back to `"unknown"` in environments without a build script.
pub const BUILD_TIMESTAMP: Option<&str> = option_env!("BUILD_TIMESTAMP");

/// Return `true` if the current CPU supports AVX2 instructions.
///
/// Used to decide whether to enable SIMD-accelerated paths in future
/// versions of the vector operations module.
#[cfg(target_arch = "x86_64")]
#[must_use]
pub fn has_avx2() -> bool {
    is_x86_feature_detected!("avx2")
}

/// Return `false` on non-x86_64 platforms (AVX2 is x86-only).
#[cfg(not(target_arch = "x86_64"))]
#[must_use]
pub const fn has_avx2() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_is_semver() {
        assert!(!VERSION.is_empty());
        assert!(VERSION.contains('.'), "VERSION must be a semver string");
    }

    #[test]
    fn test_avx2_check_does_not_panic() {
        let _ = has_avx2();
    }
}

// ── PyO3 module registration ──────────────────────────────────────────────────

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn _core(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<ffi::PyRouter>()?;
    m.add_class::<ffi::PyRoute>()?;
    m.add("__version__", VERSION)?;
    m.add("has_avx2", has_avx2())?;
    Ok(())
}
