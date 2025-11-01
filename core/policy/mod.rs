// core/policy/mod.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Routing policy that constrains decision-making
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingPolicy {
    /// Minimum confidence score to accept a route
    pub min_confidence: f32,
    
    /// Whether to allow fallback to lower-confidence routes
    pub allow_fallback: bool,
    
    /// Whether this route requires human review
    pub require_human_review: bool,
    
    /// Cost policy
    pub cost_policy: CostPolicy,
    
    /// Compliance requirements
    pub compliance: CompliancePolicy,
    
    /// Performance requirements
    pub performance: PerformancePolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostPolicy {
    /// Maximum cost per request (USD)
    pub max_cost_per_request: f32,
    
    /// Maximum total cost per hour (USD)
    pub max_cost_per_hour: Option<f32>,
    
    /// Preferred cost tier
    pub preferred_tier: CostTier,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CostTier {
    Free,       // Free models only
    Budget,     // < $0.01 per request
    Standard,   // < $0.10 per request
    Premium,    // Unlimited
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompliancePolicy {
    /// Data residency requirements (e.g., ["EU", "US"])
    pub allowed_regions: Vec<String>,
    
    /// Required certifications (e.g., ["SOC2", "HIPAA"])
    pub required_certifications: Vec<String>,
    
    /// Whether to allow cloud providers
    pub allow_cloud: bool,
    
    /// Prohibited providers (e.g., ["provider_x"])
    pub blocked_providers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformancePolicy {
    /// Maximum acceptable latency (milliseconds)
    pub max_latency_ms: u64,
    
    /// Minimum acceptable throughput (requests/sec)
    pub min_throughput: Option<f32>,
    
    /// Whether to prefer faster routes over cheaper ones
    pub optimize_for_speed: bool,
}

/// Policy evaluation result
#[derive(Debug)]
pub enum PolicyViolation {
    ConfidenceTooLow { actual: f32, required: f32 },
    CostExceeded { actual: f32, limit: f32 },
    ComplianceViolation { reason: String },
    PerformanceUnacceptable { reason: String },
    RequiresHumanReview,
}

impl RoutingPolicy {
    /// Create a default permissive policy
    pub fn permissive() -> Self {
        Self {
            min_confidence: 0.0,
            allow_fallback: true,
            require_human_review: false,
            cost_policy: CostPolicy {
                max_cost_per_request: f32::MAX,
                max_cost_per_hour: None,
                preferred_tier: CostTier::Premium,
            },
            compliance: CompliancePolicy {
                allowed_regions: vec![],
                required_certifications: vec![],
                allow_cloud: true,
                blocked_providers: vec![],
            },
            performance: PerformancePolicy {
                max_latency_ms: u64::MAX,
                min_throughput: None,
                optimize_for_speed: false,
            },
        }
    }
    
    /// Create a strict enterprise policy
    pub fn enterprise() -> Self {
        Self {
            min_confidence: 0.85,
            allow_fallback: true,
            require_human_review: false,
            cost_policy: CostPolicy {
                max_cost_per_request: 0.10,
                max_cost_per_hour: Some(100.0),
                preferred_tier: CostTier::Standard,
            },
            compliance: CompliancePolicy {
                allowed_regions: vec!["US".to_string(), "EU".to_string()],
                required_certifications: vec!["SOC2".to_string()],
                allow_cloud: true,
                blocked_providers: vec![],
            },
            performance: PerformancePolicy {
                max_latency_ms: 5000,
                min_throughput: Some(10.0),
                optimize_for_speed: false,
            },
        }
    }
    
    /// Validate a routing decision against policy
    pub fn validate(
        &self,
        confidence: f32,
        cost_usd: f32,
        provider: &str,
        region: &str,
        latency_ms: u64,
    ) -> Result<(), Vec<PolicyViolation>> {
        let mut violations = Vec::new();
        
        // Check confidence
        if confidence < self.min_confidence {
            violations.push(PolicyViolation::ConfidenceTooLow {
                actual: confidence,
                required: self.min_confidence,
            });
        }
        
        // Check cost
        if cost_usd > self.cost_policy.max_cost_per_request {
            violations.push(PolicyViolation::CostExceeded {
                actual: cost_usd,
                limit: self.cost_policy.max_cost_per_request,
            });
        }
        
        // Check compliance
        if !self.compliance.allowed_regions.is_empty()
            && !self.compliance.allowed_regions.contains(&region.to_string())
        {
            violations.push(PolicyViolation::ComplianceViolation {
                reason: format!("Region {} not allowed", region),
            });
        }
        
        if self.compliance.blocked_providers.contains(&provider.to_string()) {
            violations.push(PolicyViolation::ComplianceViolation {
                reason: format!("Provider {} is blocked", provider),
            });
        }
        
        // Check performance
        if latency_ms > self.performance.max_latency_ms {
            violations.push(PolicyViolation::PerformanceUnacceptable {
                reason: format!("Latency {}ms exceeds limit {}ms", 
                    latency_ms, self.performance.max_latency_ms),
            });
        }
        
        // Check human review requirement
        if self.require_human_review {
            violations.push(PolicyViolation::RequiresHumanReview);
        }
        
        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }
}

/// Policy registry for different contexts
pub struct PolicyRegistry {
    policies: HashMap<String, RoutingPolicy>,
    default_policy: RoutingPolicy,
}

impl PolicyRegistry {
    pub fn new(default_policy: RoutingPolicy) -> Self {
        Self {
            policies: HashMap::new(),
            default_policy,
        }
    }
    
    /// Register a named policy
    pub fn register(&mut self, name: String, policy: RoutingPolicy) {
        self.policies.insert(name, policy);
    }
    
    /// Get policy by name, or default
    pub fn get(&self, name: Option<&str>) -> &RoutingPolicy {
        name.and_then(|n| self.policies.get(n))
            .unwrap_or(&self.default_policy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_permissive_policy() {
        let policy = RoutingPolicy::permissive();
        let result = policy.validate(0.1, 100.0, "any", "any", 999999);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_enterprise_policy_violations() {
        let policy = RoutingPolicy::enterprise();
        
        // Should fail on confidence and cost
        let result = policy.validate(0.5, 1.0, "provider", "US", 1000);
        assert!(result.is_err());
        
        let violations = result.unwrap_err();
        assert_eq!(violations.len(), 2); // Confidence + Cost
    }
    
    #[test]
    fn test_compliance_violation() {
        let policy = RoutingPolicy::enterprise();
        
        // Should fail on region
        let result = policy.validate(0.9, 0.05, "provider", "CN", 1000);
        assert!(result.is_err());
    }
}