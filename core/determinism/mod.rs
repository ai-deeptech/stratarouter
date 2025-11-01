// core/determinism/mod.rs

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Snapshot of routing state for reproducibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingSnapshot {
    /// Snapshot ID
    pub id: String,
    
    /// Timestamp when snapshot was taken
    pub timestamp: DateTime<Utc>,
    
    /// Version of Core that created this snapshot
    pub core_version: String,
    
    /// Intent registry state
    pub intent_registry: Vec<IntentSnapshot>,
    
    /// Policy registry state
    pub policy_registry: HashMap<String, String>, // name -> policy hash
    
    /// Route definitions
    pub routes: Vec<RouteSnapshot>,
    
    /// Model performance metrics at snapshot time
    pub performance_data: HashMap<String, PerformanceSnapshot>,
    
    /// Hash of entire snapshot for integrity
    pub snapshot_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentSnapshot {
    pub id: String,
    pub description: String,
    pub examples_hash: String, // Hash of examples list
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteSnapshot {
    pub route_id: String,
    pub provider: String,
    pub model: String,
    pub confidence_baseline: f32,
    pub cost_baseline: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    pub provider: String,
    pub model: String,
    pub avg_latency_ms: u64,
    pub success_rate: f32,
    pub avg_cost_usd: f32,
    pub sample_count: usize,
}

impl RoutingSnapshot {
    /// Create a new snapshot from current state
    pub fn create(
        core_version: String,
        intents: Vec<IntentSnapshot>,
        policies: HashMap<String, String>,
        routes: Vec<RouteSnapshot>,
        performance: HashMap<String, PerformanceSnapshot>,
    ) -> Self {
        let mut snapshot = Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            core_version,
            intent_registry: intents,
            policy_registry: policies,
            routes,
            performance_data: performance,
            snapshot_hash: String::new(), // Computed below
        };
        
        // Compute hash
        snapshot.snapshot_hash = snapshot.compute_hash();
        snapshot
    }
    
    /// Compute hash of snapshot for integrity
    fn compute_hash(&self) -> String {
        let mut hasher = Sha256::new();
        
        // Hash all components
        hasher.update(self.id.as_bytes());
        hasher.update(self.core_version.as_bytes());
        hasher.update(serde_json::to_string(&self.intent_registry).unwrap());
        hasher.update(serde_json::to_string(&self.policy_registry).unwrap());
        hasher.update(serde_json::to_string(&self.routes).unwrap());
        hasher.update(serde_json::to_string(&self.performance_data).unwrap());
        
        format!("{:x}", hasher.finalize())
    }
    
    /// Verify snapshot integrity
    pub fn verify(&self) -> bool {
        self.snapshot_hash == self.compute_hash()
    }
    
    /// Export to JSON
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    /// Import from JSON
    pub fn import_json(json: &str) -> Result<Self, serde_json::Error> {
        let snapshot: Self = serde_json::from_str(json)?;
        if !snapshot.verify() {
            panic!("Snapshot integrity check failed");
        }
        Ok(snapshot)
    }
}

/// Configuration for deterministic routing
#[derive(Debug, Clone)]
pub struct DeterministicConfig {
    /// Random seed for reproducible routing
    pub seed: Option<u64>,
    
    /// Use specific snapshot for frozen state
    pub snapshot: Option<RoutingSnapshot>,
    
    /// Disable any non-deterministic features
    pub strict_mode: bool,
}

impl DeterministicConfig {
    pub fn disabled() -> Self {
        Self {
            seed: None,
            snapshot: None,
            strict_mode: false,
        }
    }
    
    pub fn with_seed(seed: u64) -> Self {
        Self {
            seed: Some(seed),
            snapshot: None,
            strict_mode: true,
        }
    }
    
    pub fn with_snapshot(snapshot: RoutingSnapshot) -> Self {
        Self {
            seed: None,
            snapshot: Some(snapshot),
            strict_mode: true,
        }
    }
}

/// Router interface with determinism support
pub struct DeterministicRouter {
    config: DeterministicConfig,
}

impl DeterministicRouter {
    pub fn new(config: DeterministicConfig) -> Self {
        Self { config }
    }
    
    /// Route with determinism guarantees
    pub fn route_deterministic(
        &self,
        input: &str,
        intent_id: &str,
    ) -> RoutingDecision {
        if let Some(ref snapshot) = self.config.snapshot {
            // Use frozen state from snapshot
            self.route_from_snapshot(input, intent_id, snapshot)
        } else if let Some(seed) = self.config.seed {
            // Use seeded randomness
            self.route_with_seed(input, intent_id, seed)
        } else {
            // Normal non-deterministic routing
            self.route_normal(input, intent_id)
        }
    }
    
    fn route_from_snapshot(
        &self,
        input: &str,
        intent_id: &str,
        snapshot: &RoutingSnapshot,
    ) -> RoutingDecision {
        // Find routes for this intent from snapshot
        let routes: Vec<_> = snapshot
            .routes
            .iter()
            .filter(|r| {
                // In real impl, match intent to route
                true
            })
            .collect();
        
        if routes.is_empty() {
            panic!("No routes found in snapshot for intent {}", intent_id);
        }
        
        // Use frozen confidence/cost baselines
        RoutingDecision {
            decision_id: uuid::Uuid::new_v4().to_string(),
            primary_route: routes[0].route_id.clone(),
            confidence: routes[0].confidence_baseline,
            estimated_cost: routes[0].cost_baseline,
            snapshot_id: Some(snapshot.id.clone()),
        }
    }
    
    fn route_with_seed(
        &self,
        input: &str,
        intent_id: &str,
        seed: u64,
    ) -> RoutingDecision {
        // Use seed for reproducible randomness
        // (e.g., for tie-breaking, sampling)
        use rand::{Rng, SeedableRng};
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        
        // Deterministic route selection
        let confidence = 0.85 + (rng.gen::<f32>() * 0.10); // Reproducible
        
        RoutingDecision {
            decision_id: uuid::Uuid::new_v4().to_string(),
            primary_route: format!("route_{}", seed % 100),
            confidence,
            estimated_cost: 0.03,
            snapshot_id: None,
        }
    }
    
    fn route_normal(&self, input: &str, intent_id: &str) -> RoutingDecision {
        // Non-deterministic routing (production mode)
        RoutingDecision {
            decision_id: uuid::Uuid::new_v4().to_string(),
            primary_route: "default_route".to_string(),
            confidence: 0.90,
            estimated_cost: 0.03,
            snapshot_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingDecision {
    pub decision_id: String,
    pub primary_route: String,
    pub confidence: f32,
    pub estimated_cost: f32,
    pub snapshot_id: Option<String>, // Present if using snapshot
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_snapshot_integrity() {
        let snapshot = RoutingSnapshot::create(
            "1.0.0".to_string(),
            vec![],
            HashMap::new(),
            vec![],
            HashMap::new(),
        );
        
        assert!(snapshot.verify());
        
        // Modify and check fails
        let mut modified = snapshot.clone();
        modified.core_version = "2.0.0".to_string();
        assert!(!modified.verify());
    }
    
    #[test]
    fn test_deterministic_routing_with_seed() {
        let config = DeterministicConfig::with_seed(42);
        let router = DeterministicRouter::new(config);
        
        // Same seed = same result
        let result1 = router.route_deterministic("input", "intent");
        let result2 = router.route_deterministic("input", "intent");
        
        assert_eq!(result1.confidence, result2.confidence);
        assert_eq!(result1.primary_route, result2.primary_route);
    }
    
    #[test]
    fn test_snapshot_export_import() {
        let snapshot = RoutingSnapshot::create(
            "1.0.0".to_string(),
            vec![],
            HashMap::new(),
            vec![],
            HashMap::new(),
        );
        
        let json = snapshot.export_json().unwrap();
        let imported = RoutingSnapshot::import_json(&json).unwrap();
        
        assert_eq!(snapshot.id, imported.id);
        assert!(imported.verify());
    }
}