use crate::cache::EmbeddingCache;
use crate::error::StrataError;
use crate::route::{Route, RouteMatch};
use dashmap::DashMap;
use pyo3::prelude::*;
use std::sync::Arc;

#[pyclass]
pub struct Router {
    routes: Arc<DashMap<String, Route>>,
    cache: EmbeddingCache,
    #[pyo3(get, set)]
    pub top_k: usize,
}

#[pymethods]
impl Router {
    #[new]
    #[pyo3(signature = (top_k=1, cache_size=1000))]
    pub fn new(top_k: usize, cache_size: usize) -> Self {
        Self {
            routes: Arc::new(DashMap::new()),
            cache: EmbeddingCache::new(cache_size),
            top_k: if top_k == 0 { 1 } else { top_k },
        }
    }
    
    pub fn add(&self, route: Route) -> PyResult<()> {
        let name = route.name.clone();
        
        if self.routes.contains_key(&name) {
            return Err(StrataError::DuplicateRoute(name).into());
        }
        
        self.routes.insert(name, route);
        Ok(())
    }
    
    pub fn remove(&self, name: String) -> PyResult<()> {
        if self.routes.remove(&name).is_none() {
            return Err(StrataError::RouteNotFound(name).into());
        }
        Ok(())
    }
    
    pub fn get(&self, name: String) -> PyResult<Route> {
        self.routes
            .get(&name)
            .map(|r| r.value().clone())
            .ok_or_else(|| StrataError::RouteNotFound(name).into())
    }
    
    pub fn route(&self, query_embedding: Vec<f32>) -> PyResult<Vec<RouteMatch>> {
        if query_embedding.is_empty() {
            return Err(StrataError::InvalidInput("Empty query embedding".to_string()).into());
        }
        
        if self.routes.is_empty() {
            return Ok(vec![]);
        }
        
        // Collect all route matches
        let mut matches: Vec<RouteMatch> = self.routes
            .iter()
            .map(|entry| {
                let route = entry.value();
                let score = route.score(&query_embedding);
                RouteMatch::new(route.name.clone(), score, route.threshold)
            })
            .collect();
        
        // Sort by score (highest first)
        matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        
        // Return top_k matches
        matches.truncate(self.top_k);
        
        Ok(matches)
    }
    
    pub fn route_with_threshold(&self, query_embedding: Vec<f32>, threshold: f32) -> PyResult<Vec<RouteMatch>> {
        if !(0.0..=1.0).contains(&threshold) {
            return Err(StrataError::InvalidThreshold(threshold).into());
        }
        
        let all_matches = self.route(query_embedding)?;
        Ok(all_matches.into_iter().filter(|m| m.score >= threshold).collect())
    }
    
    #[getter]
    pub fn num_routes(&self) -> usize {
        self.routes.len()
    }
    
    pub fn list_routes(&self) -> Vec<String> {
        self.routes.iter().map(|entry| entry.key().clone()).collect()
    }
    
    pub fn clear(&self) {
        self.routes.clear();
        self.cache.clear();
    }
    
    #[getter]
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
    
    pub fn clear_cache(&self) {
        self.cache.clear();
    }
    
    pub fn __repr__(&self) -> String {
        format!(
            "Router(routes={}, top_k={}, cache_size={})",
            self.num_routes(),
            self.top_k,
            self.cache_size()
        )
    }
}

impl Clone for Router {
    fn clone(&self) -> Self {
        Self {
            routes: Arc::clone(&self.routes),
            cache: self.cache.clone(),
            top_k: self.top_k,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() {
        let router = Router::new(5, 1000);
        assert_eq!(router.num_routes(), 0);
        assert_eq!(router.top_k, 5);
    }

    #[test]
    fn test_add_route() {
        let router = Router::new(1, 1000);
        let route = Route::new(
            "test".to_string(),
            vec![vec![1.0, 0.0]],
            0.8,
        ).unwrap();
        
        router.add(route).unwrap();
        assert_eq!(router.num_routes(), 1);
        assert!(router.list_routes().contains(&"test".to_string()));
    }

    #[test]
    fn test_route_query() {
        let router = Router::new(10, 1000);
        
        let route1 = Route::new(
            "politics".to_string(),
            vec![vec![1.0, 0.0, 0.0]],
            0.7,
        ).unwrap();
        
        let route2 = Route::new(
            "chitchat".to_string(),
            vec![vec![0.0, 1.0, 0.0]],
            0.7,
        ).unwrap();
        
        router.add(route1).unwrap();
        router.add(route2).unwrap();
        
        let matches = router.route(vec![1.0, 0.0, 0.0]).unwrap();
        assert_eq!(matches[0].name, "politics");
        assert!(matches[0].score > 0.9);
    }
}
