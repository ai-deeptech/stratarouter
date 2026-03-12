//! Main router implementation.

use crate::algorithms::{HybridScorer, CalibrationManager};
use crate::index::LinearIndex;
use crate::types::{Route, RouteResult, RouteScores};
use crate::error::{Error, Result};
use std::collections::HashMap;
use std::time::Instant;

/// Router configuration.
#[derive(Debug, Clone)]
pub struct RouterConfig {
    /// Embedding dimension (must be > 0).
    pub dimension: usize,
    /// Default confidence threshold for routing decisions.
    pub default_threshold: f32,
    /// Maximum number of candidate routes to evaluate.
    pub top_k: usize,
    /// Enable per-route confidence calibration.
    pub enable_calibration: bool,
}

impl Default for RouterConfig {
    fn default() -> Self {
        Self {
            dimension: 384,
            default_threshold: 0.5,
            top_k: 5,
            enable_calibration: true,
        }
    }
}

impl RouterConfig {
    /// Validate that all configuration values are in range.
    ///
    /// # Errors
    /// Returns [`Error::InvalidInput`] if any field is out of range.
    pub fn validate(&self) -> Result<()> {
        if self.dimension == 0 {
            return Err(Error::invalid_input("Dimension must be positive"));
        }

        if !(0.0..=1.0).contains(&self.default_threshold) {
            return Err(Error::invalid_input(
                "Threshold must be between 0 and 1",
            ));
        }

        if self.top_k == 0 {
            return Err(Error::invalid_input("top_k must be positive"));
        }

        Ok(())
    }
}

/// Semantic router — maps a query embedding to the best-matching [`Route`].
pub struct Router {
    config: RouterConfig,
    routes: HashMap<String, Route>,
    route_ids: Vec<String>,
    index: Option<LinearIndex>,
    hybrid_scorer: HybridScorer,
    calibration_manager: CalibrationManager,
}

impl Router {
    /// Create a new router with the given configuration.
    pub fn new(config: RouterConfig) -> Self {
        Self {
            config,
            routes: HashMap::new(),
            route_ids: Vec::new(),
            index: None,
            hybrid_scorer: HybridScorer::new(),
            calibration_manager: CalibrationManager::new(),
        }
    }

    /// Add a route to this router.
    ///
    /// Call [`build_index`](Self::build_index) after all routes have been
    /// added to make the router ready for queries.
    ///
    /// # Errors
    /// Returns an error if the route fails validation (e.g. empty ID).
    pub fn add_route(&mut self, route: Route) -> Result<()> {
        route.validate()?;
        self.route_ids.push(route.id.clone());
        self.routes.insert(route.id.clone(), route);
        Ok(())
    }

    /// Build the search index from pre-computed `embeddings`.
    ///
    /// The *i*-th embedding must correspond to the *i*-th route in insertion
    /// order (i.e. `route_ids[i]`).
    ///
    /// # Errors
    /// Returns an error if:
    /// - `embeddings` is empty
    /// - the number of embeddings doesn't match the number of routes
    /// - any embedding has a different dimension than [`RouterConfig::dimension`]
    pub fn build_index(&mut self, embeddings: Vec<Vec<f32>>) -> Result<()> {
        if embeddings.is_empty() {
            return Err(Error::invalid_input("No embeddings provided"));
        }

        if embeddings.len() != self.routes.len() {
            return Err(Error::invalid_input(&format!(
                "Embedding count ({}) doesn't match route count ({})",
                embeddings.len(),
                self.routes.len()
            )));
        }

        let dimension = embeddings[0].len();
        if dimension != self.config.dimension {
            return Err(Error::dimension_mismatch(self.config.dimension, dimension));
        }

        for embedding in embeddings.iter() {
            if embedding.len() != dimension {
                return Err(Error::dimension_mismatch(dimension, embedding.len()));
            }
        }

        let mut index = LinearIndex::new(dimension)?;
        for (i, embedding) in embeddings.into_iter().enumerate() {
            index.add(i, embedding)?;
        }

        self.index = Some(index);
        Ok(())
    }

    /// Route a query to the best matching route.
    ///
    /// # Errors
    /// Returns an error if:
    /// - `text` is empty
    /// - `embedding` is empty or has the wrong dimension
    /// - the index has not been built yet
    /// - no routes have been added
    pub fn route(&mut self, text: &str, embedding: &[f32]) -> Result<RouteResult> {
        let start = Instant::now();

        if text.is_empty() {
            return Err(Error::invalid_input("Query text cannot be empty"));
        }

        if embedding.is_empty() {
            return Err(Error::invalid_input("Embedding cannot be empty"));
        }

        let index = self.index.as_ref().ok_or(Error::IndexNotBuilt)?;

        if embedding.len() != self.config.dimension {
            return Err(Error::dimension_mismatch(
                self.config.dimension,
                embedding.len(),
            ));
        }

        if index.is_empty() {
            return Err(Error::NoRoutes);
        }

        let neighbors = index.search(embedding, self.config.top_k)?;

        if neighbors.is_empty() {
            return Err(Error::NoRoutes);
        }

        let mut best_route_id = String::new();
        let mut best_score = 0.0_f32;
        let mut best_scores = RouteScores::zero();

        for (idx, distance) in neighbors {
            if idx >= self.route_ids.len() {
                continue;
            }

            let route_id = &self.route_ids[idx];
            let route = match self.routes.get(route_id) {
                Some(r) => r,
                None => continue,
            };

            let dense_score = (1.0 - distance).max(0.0);
            let sparse_score = self.hybrid_scorer.compute_sparse_score(text, route);
            let rule_score = self.hybrid_scorer.compute_rule_score(text, route);
            let fused_score = self.hybrid_scorer.fuse_scores(dense_score, sparse_score, rule_score);

            let (calibrated_score, _uncertainty) = if self.config.enable_calibration {
                self.calibration_manager.calibrate_for_route(route_id, fused_score)
            } else {
                (fused_score, 0.0)
            };

            if calibrated_score > best_score {
                best_score = calibrated_score;
                best_route_id = route_id.clone();
                best_scores = RouteScores {
                    semantic: dense_score,
                    keyword: sparse_score,
                    pattern: rule_score,
                    total: fused_score,
                    confidence: calibrated_score,
                };
            }
        }

        if best_route_id.is_empty() {
            return Err(Error::NoRoutes);
        }

        let route = self.routes.get(&best_route_id).ok_or_else(|| {
            Error::RouteNotFound {
                route_id: best_route_id.clone(),
            }
        })?;

        let latency_us = start.elapsed().as_micros() as u64;
        let latency_ms = (latency_us as f64 / 1000.0).ceil() as u64;

        Ok(RouteResult {
            route_id: best_route_id,
            scores: best_scores,
            metadata: route.metadata.clone(),
            latency_ms,
        })
    }

    /// Return the number of routes currently registered.
    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    /// Return `true` if the search index has been built.
    pub fn is_index_built(&self) -> bool {
        self.index.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() {
        let config = RouterConfig::default();
        config.validate().unwrap();
        let router = Router::new(config);
        assert_eq!(router.route_count(), 0);
        assert!(!router.is_index_built());
    }

    #[test]
    fn test_add_route() {
        let mut router = Router::new(RouterConfig::default());
        let route = Route {
            id: "test".into(),
            description: "Test route".into(),
            examples: vec!["hello".into()],
            keywords: vec!["hello".into()],
            patterns: vec![],
            metadata: HashMap::new(),
            threshold: None,
            tags: vec![],
        };
        router.add_route(route).unwrap();
        assert_eq!(router.route_count(), 1);
    }

    #[test]
    fn test_config_validation() {
        assert!(RouterConfig::default().validate().is_ok());

        assert!(RouterConfig {
            dimension: 0,
            ..Default::default()
        }
        .validate()
        .is_err());

        assert!(RouterConfig {
            default_threshold: 1.5,
            ..Default::default()
        }
        .validate()
        .is_err());
    }

    #[test]
    fn test_build_index_empty() {
        let mut router = Router::new(RouterConfig::default());
        assert!(router.build_index(vec![]).is_err());
    }

    #[test]
    fn test_route_without_index() {
        let mut router = Router::new(RouterConfig::default());
        let result = router.route("test", &[0.5; 384]);
        assert!(matches!(result, Err(Error::IndexNotBuilt)));
    }
}
