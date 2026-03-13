//! Nearest-neighbour index backed by a linear (brute-force) scan.
//!
//! # Implementation note
//! The current implementation performs an exhaustive O(N) scan over all
//! stored vectors. For typical workloads (< 500 routes) this is fast enough
//! to stay well within the < 10 ms p99 latency target.
//!
//! A graph-based HNSW index with O(log N) search complexity is planned for
//! a future release and will be a drop-in replacement — see ROADMAP.md.

use crate::algorithms::vector_ops::cosine_similarity;
use crate::error::{Error, Result};
use std::collections::HashMap;

/// A nearest-neighbour index backed by a linear scan over all stored vectors.
///
/// For an HNSW-based replacement (planned), see ROADMAP.md.
pub struct LinearIndex {
    dimension: usize,
    vectors: HashMap<usize, Vec<f32>>,
}

impl LinearIndex {
    /// Create a new index with the specified embedding `dimension`.
    ///
    /// # Errors
    /// Returns [`Error::InvalidInput`] if `dimension` is zero.
    pub fn new(dimension: usize) -> Result<Self> {
        if dimension == 0 {
            return Err(Error::invalid_input("Dimension must be positive"));
        }
        Ok(Self {
            dimension,
            vectors: HashMap::new(),
        })
    }

    /// Add a vector to the index under the given `id`.
    ///
    /// # Errors
    /// Returns [`Error::DimensionMismatch`] if `vector.len() != self.dimension`.
    pub fn add(&mut self, id: usize, vector: Vec<f32>) -> Result<()> {
        if vector.len() != self.dimension {
            return Err(Error::dimension_mismatch(self.dimension, vector.len()));
        }
        self.vectors.insert(id, vector);
        Ok(())
    }

    /// Search for the `k` nearest neighbours by cosine distance.
    ///
    /// Returns a `Vec<(id, distance)>` sorted by distance ascending, where
    /// `distance = 1 − cosine_similarity`.
    ///
    /// # Errors
    /// Returns [`Error::DimensionMismatch`] if `query.len() != self.dimension`.
    pub fn search(&self, query: &[f32], k: usize) -> Result<Vec<(usize, f32)>> {
        if query.len() != self.dimension {
            return Err(Error::dimension_mismatch(self.dimension, query.len()));
        }

        if self.vectors.is_empty() {
            return Ok(Vec::new());
        }

        let mut results: Vec<(usize, f32)> = self
            .vectors
            .iter()
            .map(|(id, vec)| {
                let distance = (1.0 - cosine_similarity(query, vec)).max(0.0);
                (*id, distance)
            })
            .collect();

        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(k);
        Ok(results)
    }

    /// Return the number of vectors currently stored.
    pub fn len(&self) -> usize {
        self.vectors.len()
    }

    /// Return `true` if the index contains no vectors.
    pub fn is_empty(&self) -> bool {
        self.vectors.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_routing() {
        let mut index = LinearIndex::new(3).unwrap();
        index.add(0, vec![1.0, 0.0, 0.0]).unwrap();
        index.add(1, vec![0.0, 1.0, 0.0]).unwrap();
        index.add(2, vec![0.0, 0.0, 1.0]).unwrap();

        assert_eq!(index.len(), 3);
        let results = index.search(&[0.9, 0.1, 0.0], 1).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, 0); // closest to [1,0,0]
    }

    #[test]
    fn test_top_k() {
        let mut index = LinearIndex::new(2).unwrap();
        index.add(0, vec![1.0, 0.0]).unwrap();
        index.add(1, vec![0.0, 1.0]).unwrap();
        index.add(2, vec![0.5, 0.5]).unwrap();

        let results = index.search(&[0.6, 0.4], 2).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_empty_index() {
        let index = LinearIndex::new(3).unwrap();
        assert!(index.is_empty());
        let results = index.search(&[1.0, 0.0, 0.0], 5).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_zero_dimension_returns_error() {
        assert!(LinearIndex::new(0).is_err());
    }

    #[test]
    fn test_add_dimension_mismatch_returns_error() {
        let mut index = LinearIndex::new(3).unwrap();
        let result = index.add(0, vec![1.0, 2.0]); // wrong length
        assert!(result.is_err());
    }

    #[test]
    fn test_search_dimension_mismatch_returns_error() {
        let mut index = LinearIndex::new(3).unwrap();
        index.add(0, vec![1.0, 0.0, 0.0]).unwrap();
        let result = index.search(&[1.0, 2.0], 1); // wrong length
        assert!(result.is_err());
    }
}
