// core/routing/alternatives.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A single routing alternative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteAlternative {
    /// Unique identifier for this route
    pub route_id: String,
    
    /// Provider + model combination
    pub target: RoutingTarget,
    
    /// Confidence score for this route
    pub confidence: f32,
    
    /// Delta from top choice (0.0 for primary route)
    pub delta: f32,
    
    /// Estimated cost (USD)
    pub estimated_cost: f32,
    
    /// Estimated latency (ms)
    pub estimated_latency_ms: u64,
    
    /// Reason for this ranking
    pub ranking_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingTarget {
    pub provider: String,
    pub model: String,
    pub region: Option<String>,
}

/// Collection of routing alternatives with ranking guarantees
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteAlternatives {
    /// Primary route (highest confidence)
    pub primary: RouteAlternative,
    
    /// Fallback routes (ordered by confidence)
    pub fallbacks: Vec<RouteAlternative>,
    
    /// Total routes evaluated
    pub total_evaluated: usize,
    
    /// Ranking metadata
    pub ranking_metadata: RankingMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingMetadata {
    /// Ranking algorithm used
    pub algorithm: String,
    
    /// Ranking version (for reproducibility)
    pub version: String,
    
    /// Minimum confidence gap required between alternatives
    pub min_confidence_gap: f32,
    
    /// Whether ranking is deterministic
    pub is_deterministic: bool,
}

impl RouteAlternatives {
    /// Get the Nth best alternative (0 = primary)
    pub fn get(&self, index: usize) -> Option<&RouteAlternative> {
        if index == 0 {
            Some(&self.primary)
        } else {
            self.fallbacks.get(index - 1)
        }
    }
    
    /// Total number of alternatives (including primary)
    pub fn total(&self) -> usize {
        1 + self.fallbacks.len()
    }
    
    /// Check if fallback has sufficient confidence gap
    pub fn is_fallback_viable(&self, index: usize) -> bool {
        if let Some(alt) = self.get(index) {
            // Viable if within min_confidence_gap of primary
            alt.delta <= self.ranking_metadata.min_confidence_gap * 2.0
        } else {
            false
        }
    }
    
    /// Get all viable fallbacks (within 2x confidence gap)
    pub fn viable_fallbacks(&self) -> Vec<&RouteAlternative> {
        self.fallbacks
            .iter()
            .filter(|f| f.delta <= self.ranking_metadata.min_confidence_gap * 2.0)
            .collect()
    }
    
    /// Sort alternatives by different criteria
    pub fn sort_by_cost(&self) -> Vec<&RouteAlternative> {
        let mut all: Vec<&RouteAlternative> = vec![&self.primary];
        all.extend(self.fallbacks.iter());
        all.sort_by(|a, b| {
            a.estimated_cost
                .partial_cmp(&b.estimated_cost)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        all
    }
    
    pub fn sort_by_latency(&self) -> Vec<&RouteAlternative> {
        let mut all: Vec<&RouteAlternative> = vec![&self.primary];
        all.extend(self.fallbacks.iter());
        all.sort_by_key(|a| a.estimated_latency_ms);
        all
    }
}

/// Builder for creating route alternatives
pub struct RouteAlternativesBuilder {
    routes: Vec<RouteAlternative>,
    min_confidence_gap: f32,
    algorithm: String,
    version: String,
}

impl RouteAlternativesBuilder {
    pub fn new(algorithm: String, version: String) -> Self {
        Self {
            routes: Vec::new(),
            min_confidence_gap: 0.05, // Default 5% gap
            algorithm,
            version,
        }
    }
    
    pub fn min_confidence_gap(mut self, gap: f32) -> Self {
        self.min_confidence_gap = gap;
        self
    }
    
    pub fn add_route(
        mut self,
        target: RoutingTarget,
        confidence: f32,
        estimated_cost: f32,
        estimated_latency_ms: u64,
        ranking_reason: String,
    ) -> Self {
        self.routes.push(RouteAlternative {
            route_id: Uuid::new_v4().to_string(),
            target,
            confidence,
            delta: 0.0, // Will be calculated
            estimated_cost,
            estimated_latency_ms,
            ranking_reason,
        });
        self
    }
    
    pub fn build(mut self) -> Result<RouteAlternatives, String> {
        if self.routes.is_empty() {
            return Err("No routes provided".to_string());
        }
        
        // Sort by confidence (descending)
        self.routes
            .sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        
        // Calculate deltas from top choice
        let top_confidence = self.routes[0].confidence;
        for route in &mut self.routes {
            route.delta = top_confidence - route.confidence;
        }
        
        // Extract primary and fallbacks
        let primary = self.routes.remove(0);
        let fallbacks = self.routes;
        
        Ok(RouteAlternatives {
            primary,
            fallbacks,
            total_evaluated: 1 + fallbacks.len(),
            ranking_metadata: RankingMetadata {
                algorithm: self.algorithm,
                version: self.version,
                min_confidence_gap: self.min_confidence_gap,
                is_deterministic: true,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_alternatives_builder() {
        let alternatives = RouteAlternativesBuilder::new(
            "confidence_ranking".to_string(),
            "1.0".to_string(),
        )
        .add_route(
            RoutingTarget {
                provider: "openai".to_string(),
                model: "gpt-4".to_string(),
                region: None,
            },
            0.95,
            0.03,
            500,
            "Highest confidence".to_string(),
        )
        .add_route(
            RoutingTarget {
                provider: "anthropic".to_string(),
                model: "claude-3".to_string(),
                region: None,
            },
            0.90,
            0.025,
            400,
            "Lower cost".to_string(),
        )
        .build()
        .unwrap();
        
        assert_eq!(alternatives.primary.confidence, 0.95);
        assert_eq!(alternatives.fallbacks.len(), 1);
        assert_eq!(alternatives.fallbacks[0].delta, 0.05);
    }
    
    #[test]
    fn test_viable_fallbacks() {
        let alternatives = RouteAlternativesBuilder::new(
            "test".to_string(),
            "1.0".to_string(),
        )
        .min_confidence_gap(0.05)
        .add_route(
            RoutingTarget {
                provider: "p1".to_string(),
                model: "m1".to_string(),
                region: None,
            },
            0.95,
            0.03,
            500,
            "Primary".to_string(),
        )
        .add_route(
            RoutingTarget {
                provider: "p2".to_string(),
                model: "m2".to_string(),
                region: None,
            },
            0.94,
            0.02,
            400,
            "Viable".to_string(),
        )
        .add_route(
            RoutingTarget {
                provider: "p3".to_string(),
                model: "m3".to_string(),
                region: None,
            },
            0.80,
            0.01,
            300,
            "Too far".to_string(),
        )
        .build()
        .unwrap();
        
        let viable = alternatives.viable_fallbacks();
        assert_eq!(viable.len(), 1); // Only second route is viable
        assert_eq!(viable[0].target.provider, "p2");
    }
}