//! Type definitions for StrataRouter

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Route definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    /// Unique route identifier
    pub id: String,
    /// Human-readable description
    pub description: String,
    /// Example queries for this route
    pub examples: Vec<String>,
    /// Important keywords
    pub keywords: Vec<String>,
    /// Exact patterns to match
    pub patterns: Vec<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Route-specific confidence threshold
    pub threshold: Option<f32>,
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl Route {
    /// Create a new route with given ID
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            description: String::new(),
            examples: Vec::new(),
            keywords: Vec::new(),
            patterns: Vec::new(),
            metadata: HashMap::new(),
            threshold: None,
            tags: Vec::new(),
        }
    }
    
    /// Validate route configuration
    pub fn validate(&self) -> crate::Result<()> {
        if self.id.is_empty() {
            return Err(crate::Error::invalid_input("Route ID cannot be empty"));
        }
        
        if self.examples.is_empty() && self.description.is_empty() {
            return Err(crate::Error::invalid_input(
                &format!("Route '{}' must have examples or description", self.id)
            ));
        }
        
        Ok(())
    }
    
    /// Add description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
    
    /// Add examples
    pub fn with_examples(mut self, examples: Vec<String>) -> Self {
        self.examples = examples;
        self
    }
    
    /// Add keywords
    pub fn with_keywords(mut self, keywords: Vec<String>) -> Self {
        self.keywords = keywords;
        self
    }
    
    /// Add patterns
    pub fn with_patterns(mut self, patterns: Vec<String>) -> Self {
        self.patterns = patterns;
        self
    }
    
    /// Set threshold
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.threshold = Some(threshold);
        self
    }
    
    /// Add tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
    
    /// Add metadata entry
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Routing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResult {
    /// ID of the matched route
    pub route_id: String,
    /// Scores for this match
    pub scores: RouteScores,
    /// Route metadata
    pub metadata: HashMap<String, String>,
    /// Routing latency in milliseconds
    pub latency_ms: u64,
}

/// Routing scores
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RouteScores {
    /// Semantic similarity score (0.0-1.0)
    pub semantic: f32,
    /// Keyword match score (0.0-1.0)
    pub keyword: f32,
    /// Pattern match score (0.0-1.0)
    pub pattern: f32,
    /// Total hybrid score (0.0-1.0)
    pub total: f32,
    /// Calibrated confidence (0.0-1.0)
    pub confidence: f32,
}

impl RouteScores {
    /// Create new scores
    pub fn new(semantic: f32, keyword: f32, pattern: f32) -> Self {
        let total = (semantic * 0.6) + (keyword * 0.3) + (pattern * 0.1);
        Self {
            semantic,
            keyword,
            pattern,
            total,
            confidence: total,
        }
    }
    
    /// Create zero scores
    pub fn zero() -> Self {
        Self {
            semantic: 0.0,
            keyword: 0.0,
            pattern: 0.0,
            total: 0.0,
            confidence: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_route_builder() {
        let route = Route::new("test")
            .with_description("Test route")
            .with_examples(vec!["example 1".to_string()])
            .with_threshold(0.8);
        
        assert_eq!(route.id, "test");
        assert_eq!(route.description, "Test route");
        assert_eq!(route.examples.len(), 1);
        assert_eq!(route.threshold, Some(0.8));
    }
    
    #[test]
    fn test_route_validation() {
        let route = Route::new("");
        assert!(route.validate().is_err());
        
        let route = Route::new("test");
        assert!(route.validate().is_err()); // No examples or description
        
        let route = Route::new("test").with_description("Valid");
        assert!(route.validate().is_ok());
    }
    
    #[test]
    fn test_route_scores() {
        let scores = RouteScores::new(0.8, 0.6, 0.4);
        assert!(scores.total > 0.0);
        assert!(scores.total <= 1.0);
    }
    
    #[test]
    fn test_scores_zero() {
        let scores = RouteScores::zero();
        assert_eq!(scores.semantic, 0.0);
        assert_eq!(scores.keyword, 0.0);
        assert_eq!(scores.pattern, 0.0);
        assert_eq!(scores.total, 0.0);
        assert_eq!(scores.confidence, 0.0);
    }
}
