use crate::error::StrataError;
use pyo3::prelude::*;

/// High-performance cosine similarity
#[pyfunction]
pub fn cosine_similarity(a: Vec<f32>, b: Vec<f32>) -> PyResult<f32> {
    if a.len() != b.len() {
        return Err(StrataError::DimensionMismatch {
            expected: a.len(),
            actual: b.len(),
        }
        .into());
    }

    let result = cosine_similarity_inner(&a, &b);
    Ok(result)
}

/// Batch cosine similarity computation
#[pyfunction]
pub fn cosine_similarity_batch(query: Vec<f32>, embeddings: Vec<Vec<f32>>) -> PyResult<Vec<f32>> {
    if embeddings.is_empty() {
        return Ok(vec![]);
    }

    let query_dim = query.len();
    
    // Validate all embeddings have same dimension
    for emb in embeddings.iter() {
        if emb.len() != query_dim {
            return Err(StrataError::DimensionMismatch {
                expected: query_dim,
                actual: emb.len(),
            }
            .into());
        }
    }

    // Sequential computation (par_iter removed for simplicity)
    let results: Vec<f32> = embeddings
        .iter()
        .map(|emb| cosine_similarity_inner(&query, emb))
        .collect();

    Ok(results)
}

/// Internal cosine similarity implementation
#[inline]
pub fn cosine_similarity_inner(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    
    (dot_product / (norm_a * norm_b)).clamp(-1.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_cosine_similarity_identical() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        let sim = cosine_similarity_inner(&a, &b);
        assert_relative_eq!(sim, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let sim = cosine_similarity_inner(&a, &b);
        assert_relative_eq!(sim, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let a = vec![1.0, 0.0];
        let b = vec![-1.0, 0.0];
        let sim = cosine_similarity_inner(&a, &b);
        assert_relative_eq!(sim, -1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_batch_similarity() {
        let query = vec![1.0, 0.0, 0.0];
        let embeddings = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.5, 0.5, 0.0],
        ];
        
        let results = cosine_similarity_batch(query, embeddings).unwrap();
        assert_eq!(results.len(), 3);
        assert_relative_eq!(results[0], 1.0, epsilon = 1e-6);
        assert_relative_eq!(results[1], 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_dimension_mismatch() {
        let a = vec![1.0, 2.0];
        let b = vec![1.0, 2.0, 3.0];
        
        assert!(cosine_similarity(a, b).is_err());
    }
}
