//! Algorithm implementations for semantic routing.
pub mod calibration;
pub mod hybrid_scoring;
pub mod vector_ops;

// Scalar / SIMD-ready ops kept internal — imported directly by the index.
pub use calibration::CalibrationManager;
pub use hybrid_scoring::HybridScorer;
