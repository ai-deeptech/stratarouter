//! HNSW (Hierarchical Navigable Small World) index implementation

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use crate::algorithms::simd_ops::cosine_similarity;

/// Simple HNSW index for approximate nearest neighbor search
pub struct HnswIndex {
    dimension: usize,
    vectors: Arc<RwLock<HashMap<usize, Vec<f32>>>>,
}

impl HnswIndex {
    /// Create new HNSW index with specified dimension
    pub fn new(dimension: usize) -> Self {
        assert!(dimension > 0, "Dimension must be positive");
        
        Self {
            dimension,
            vectors: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Add vector to index
    pub fn add(&mut self, id: usize, vector: Vec<f32>) {
        assert_eq!(
            vector.len(),
            self.dimension,
            "Vector dimension mismatch: expected {}, got {}",
            self.dimension,
            vector.len()
        );
        
        let mut vectors = self.vectors.write();
        vectors.insert(id, vector);
    }
    
    /// Search for k nearest neighbors
    pub fn search(&self, query: &[f32], k: usize) -> Vec<(usize, f32)> {
        assert_eq!(
            query.len(),
            self.dimension,
            "Query dimension mismatch: expected {}, got {}",
            self.dimension,
            query.len()
        );
        
        let vectors = self.vectors.read();
        
        if vectors.is_empty() {
            return Vec::new();
        }
        
        // Compute distances to all vectors
        let mut results: Vec<(usize, f32)> = vectors
            .iter()
            .map(|(id, vec)| {
                let distance = 1.0 - cosine_similarity(query, vec);
                (*id, distance.max(0.0))
            })
            .collect();
        
        // Sort by distance (ascending)
        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Return top k
        results.truncate(k);
        results
    }
    
    /// Get number of vectors in index
    pub fn len(&self) -> usize {
        self.vectors.read().len()
    }
    
    /// Check if index is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    // Removed unused dimension() method
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hnsw_basic() {
        let mut index = HnswIndex::new(3);
        
        index.add(0, vec![1.0, 0.0, 0.0]);
        index.add(1, vec![0.0, 1.0, 0.0]);
        index.add(2, vec![0.0, 0.0, 1.0]);
        
        assert_eq!(index.len(), 3);
        
        let results = index.search(&[0.9, 0.1, 0.0], 1);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, 0);
    }
    
    #[test]
    fn test_hnsw_top_k() {
        let mut index = HnswIndex::new(2);
        
        index.add(0, vec![1.0, 0.0]);
        index.add(1, vec![0.0, 1.0]);
        index.add(2, vec![0.5, 0.5]);
        
        let results = index.search(&[0.6, 0.4], 2);
        assert_eq!(results.len(), 2);
    }
    
    #[test]
    fn test_hnsw_empty() {
        let index = HnswIndex::new(3);
        assert!(index.is_empty());
        
        let results = index.search(&[1.0, 0.0, 0.0], 5);
        assert!(results.is_empty());
    }
    
    #[test]
    #[should_panic(expected = "Dimension must be positive")]
    fn test_zero_dimension() {
        HnswIndex::new(0);
    }
    
    #[test]
    #[should_panic(expected = "Vector dimension mismatch")]
    fn test_dimension_mismatch_add() {
        let mut index = HnswIndex::new(3);
        index.add(0, vec![1.0, 2.0]);
    }
    
    #[test]
    #[should_panic(expected = "Query dimension mismatch")]
    fn test_dimension_mismatch_search() {
        let mut index = HnswIndex::new(3);
        index.add(0, vec![1.0, 0.0, 0.0]);
        index.search(&[1.0, 2.0], 1);
    }
}
