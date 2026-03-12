//! Vector operations for embedding computation.
//!
//! Currently uses optimised scalar operations. An AVX2/SIMD path via the
//! `wide` crate is planned for a future release and will be a transparent
//! drop-in — callers do not need to change any code.

/// Cosine similarity between two embedding vectors.
///
/// Returns a value in `[-1.0, 1.0]` where `1.0` means identical direction
/// and `-1.0` means opposite direction. Returns `0.0` for empty or
/// zero-norm vectors.
///
/// # Examples
///
/// ```
/// use stratarouter_core::algorithms::vector_ops::cosine_similarity;
///
/// let a = vec![1.0_f32, 0.0, 0.0];
/// let b = vec![0.0_f32, 1.0, 0.0];
/// assert_eq!(cosine_similarity(&a, &b), 0.0); // orthogonal
/// assert_eq!(cosine_similarity(&a, &a), 1.0); // identical
/// ```
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    debug_assert_eq!(a.len(), b.len(), "Vector lengths must match");

    if a.is_empty() {
        return 0.0;
    }

    let mut dot = 0.0_f32;
    let mut norm_a = 0.0_f32;
    let mut norm_b = 0.0_f32;

    for i in 0..a.len() {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    (dot / (norm_a.sqrt() * norm_b.sqrt())).clamp(-1.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similar_vectors() {
        let a = vec![1.0_f32, 0.0, 0.0];
        let b = vec![0.5_f32, 0.5, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 0.707).abs() < 0.01);
    }

    #[test]
    fn test_cosine_identity() {
        let a = vec![1.0_f32, 2.0, 3.0];
        let sim = cosine_similarity(&a, &a);
        assert!((sim - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_cosine_orthogonal() {
        let a = vec![1.0_f32, 0.0, 0.0];
        let b = vec![0.0_f32, 1.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!(sim.abs() < 0.001);
    }

    #[test]
    fn test_cosine_opposite() {
        let a = vec![1.0_f32, 0.0];
        let b = vec![-1.0_f32, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - (-1.0)).abs() < 0.001);
    }

    #[test]
    fn test_cosine_empty() {
        let a: Vec<f32> = vec![];
        let b: Vec<f32> = vec![];
        assert_eq!(cosine_similarity(&a, &b), 0.0);
    }

    #[test]
    fn test_cosine_zero_vector() {
        let a = vec![0.0_f32, 0.0, 0.0];
        let b = vec![1.0_f32, 0.0, 0.0];
        assert_eq!(cosine_similarity(&a, &b), 0.0);
    }

    #[test]
    fn test_cosine_result_clamped() {
        let a = vec![1.0_f32, 0.0];
        let b = vec![-1.0_f32, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!(sim >= -1.0 && sim <= 1.0);
    }
}
