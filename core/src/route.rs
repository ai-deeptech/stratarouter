use crate::error::{Result, StrataError};
use crate::similarity::cosine_similarity_inner;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

/// A semantic route with example embeddings
#[pyclass]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Route {
    #[pyo3(get, set)]
    pub name: String,
    
    #[pyo3(get)]
    pub embeddings: Vec<Vec<f32>>,
    
    #[pyo3(get, set)]
    pub threshold: f32,
}

#[pymethods]
impl Route {
    #[new]
    #[pyo3(signature = (name, embeddings, threshold=0.82))]
    pub fn new(name: String, embeddings: Vec<Vec<f32>>, threshold: f32) -> Result<Self> {
        // Validate name
        if name.trim().is_empty() {
            return Err(StrataError::InvalidRouteName(name));
        }
        
        // Validate embeddings
        if embeddings.is_empty() {
            return Err(StrataError::EmptyRoute(name));
        }
        
        // Validate threshold
        if !(0.0..=1.0).contains(&threshold) {
            return Err(StrataError::InvalidThreshold(threshold));
        }
        
        // Validate all embeddings have same dimension
        let embedding_dim = embeddings[0].len();
        for emb in &embeddings {
            if emb.len() != embedding_dim {
                return Err(StrataError::DimensionMismatch {
                    expected: embedding_dim,
                    actual: emb.len(),
                });
            }
        }
        
        Ok(Self {
            name,
            embeddings,
            threshold,
        })
    }
    
    pub fn __repr__(&self) -> String {
        format!(
            "Route(name='{}', examples={}, threshold={})",
            self.name,
            self.embeddings.len(),
            self.threshold
        )
    }
    
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
    
    #[getter]
    pub fn num_examples(&self) -> usize {
        self.embeddings.len()
    }
    
    #[getter]
    pub fn embedding_dim(&self) -> usize {
        self.embeddings.first().map(|e| e.len()).unwrap_or(0)
    }
}

impl Route {
    pub fn score(&self, query_embedding: &[f32]) -> f32 {
        if self.embeddings.is_empty() {
            return 0.0;
        }
        
        self.embeddings
            .iter()
            .map(|emb| cosine_similarity_inner(query_embedding, emb))
            .fold(f32::NEG_INFINITY, f32::max)
    }
    
    pub fn matches(&self, query_embedding: &[f32]) -> bool {
        self.score(query_embedding) >= self.threshold
    }
}

/// Result of a route matching operation
#[pyclass]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouteMatch {
    #[pyo3(get)]
    pub name: String,
    
    #[pyo3(get)]
    pub score: f32,
    
    #[pyo3(get)]
    pub threshold: f32,
}

#[pymethods]
impl RouteMatch {
    #[new]
    pub fn new(name: String, score: f32, threshold: f32) -> Self {
        Self {
            name,
            score,
            threshold,
        }
    }
    
    pub fn __repr__(&self) -> String {
        format!(
            "RouteMatch(name='{}', score={:.4}, threshold={})",
            self.name, self.score, self.threshold
        )
    }
    
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
    
    #[getter]
    pub fn is_match(&self) -> bool {
        self.score >= self.threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_creation() {
        let route = Route::new(
            "test".to_string(),
            vec![vec![1.0, 0.0, 0.0]],
            0.8,
        ).unwrap();
        
        assert_eq!(route.name, "test");
        assert_eq!(route.num_examples(), 1);
        assert_eq!(route.embedding_dim(), 3);
    }

    #[test]
    fn test_route_scoring() {
        let route = Route::new(
            "test".to_string(),
            vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0]],
            0.8,
        ).unwrap();
        
        let score1 = route.score(&[1.0, 0.0, 0.0]);
        assert!((score1 - 1.0).abs() < 1e-6);
        
        let score2 = route.score(&[0.0, 1.0, 0.0]);
        assert!((score2 - 1.0).abs() < 1e-6);
    }
}
