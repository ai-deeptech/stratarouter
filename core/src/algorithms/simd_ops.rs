//! SIMD-optimized operations

/// Cosine similarity between two vectors
/// 
/// Computes the cosine similarity using scalar operations
/// (SIMD optimization would be added in production)
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    debug_assert_eq!(a.len(), b.len(), "Vector lengths must match");
    
    if a.is_empty() {
        return 0.0;
    }
    
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;
    
    for i in 0..a.len() {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }
    
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    
    dot / (norm_a.sqrt() * norm_b.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.5, 0.5, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 0.707).abs() < 0.01);
    }
    
    #[test]
    fn test_cosine_identity() {
        let a = vec![1.0, 2.0, 3.0];
        let sim = cosine_similarity(&a, &a);
        assert!((sim - 1.0).abs() < 0.001);
    }
    
    #[test]
    fn test_cosine_orthogonal() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!(sim.abs() < 0.001);
    }
    
    #[test]
    fn test_cosine_empty() {
        let a: Vec<f32> = vec![];
        let b: Vec<f32> = vec![];
        let sim = cosine_similarity(&a, &b);
        assert_eq!(sim, 0.0);
    }
}
