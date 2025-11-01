// core/cost/mod.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pricing model for a provider/model combination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingModel {
    pub provider: String,
    pub model: String,
    pub pricing: PricingStructure,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PricingStructure {
    /// Per-token pricing
    PerToken {
        input_cost_per_1k_tokens: f32,
        output_cost_per_1k_tokens: f32,
    },
    
    /// Per-request pricing
    PerRequest {
        cost_per_request: f32,
    },
    
    /// Tiered pricing
    Tiered {
        tiers: Vec<PricingTier>,
    },
    
    /// Free
    Free,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingTier {
    pub min_volume: u32, // Requests per month
    pub max_volume: Option<u32>,
    pub cost_per_request: f32,
}

/// Cost estimation engine
pub struct CostEstimator {
    pricing_models: HashMap<String, PricingModel>, // key: "provider:model"
}

impl CostEstimator {
    pub fn new() -> Self {
        Self {
            pricing_models: HashMap::new(),
        }
    }
    
    /// Load default pricing models (from config/API)
    pub fn load_defaults() -> Self {
        let mut estimator = Self::new();
        
        // OpenAI GPT-4
        estimator.register(PricingModel {
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            pricing: PricingStructure::PerToken {
                input_cost_per_1k_tokens: 0.03,
                output_cost_per_1k_tokens: 0.06,
            },
            updated_at: chrono::Utc::now(),
        });
        
        // OpenAI GPT-3.5
        estimator.register(PricingModel {
            provider: "openai".to_string(),
            model: "gpt-3.5-turbo".to_string(),
            pricing: PricingStructure::PerToken {
                input_cost_per_1k_tokens: 0.0015,
                output_cost_per_1k_tokens: 0.002,
            },
            updated_at: chrono::Utc::now(),
        });
        
        // Anthropic Claude
        estimator.register(PricingModel {
            provider: "anthropic".to_string(),
            model: "claude-3-opus".to_string(),
            pricing: PricingStructure::PerToken {
                input_cost_per_1k_tokens: 0.015,
                output_cost_per_1k_tokens: 0.075,
            },
            updated_at: chrono::Utc::now(),
        });
        
        // Free model example
        estimator.register(PricingModel {
            provider: "local".to_string(),
            model: "llama-2-7b".to_string(),
            pricing: PricingStructure::Free,
            updated_at: chrono::Utc::now(),
        });
        
        estimator
    }
    
    /// Register a pricing model
    pub fn register(&mut self, model: PricingModel) {
        let key = format!("{}:{}", model.provider, model.model);
        self.pricing_models.insert(key, model);
    }
    
    /// Estimate cost for a request
    pub fn estimate_cost(
        &self,
        provider: &str,
        model: &str,
        input_tokens: u32,
        output_tokens: u32,
    ) -> Result<f32, String> {
        let key = format!("{}:{}", provider, model);
        
        let pricing = self
            .pricing_models
            .get(&key)
            .ok_or_else(|| format!("No pricing model for {}", key))?;
        
        match &pricing.pricing {
            PricingStructure::PerToken {
                input_cost_per_1k_tokens,
                output_cost_per_1k_tokens,
            } => {
                let input_cost = (input_tokens as f32 / 1000.0) * input_cost_per_1k_tokens;
                let output_cost = (output_tokens as f32 / 1000.0) * output_cost_per_1k_tokens;
                Ok(input_cost + output_cost)
            }
            PricingStructure::PerRequest { cost_per_request } => Ok(*cost_per_request),
            PricingStructure::Free => Ok(0.0),
            PricingStructure::Tiered { .. } => {
                // Simplified - would need volume tracking
                Err("Tiered pricing requires volume context".to_string())
            }
        }
    }
    
    /// Get all models within a cost range
    pub fn find_models_under_cost(&self, max_cost: f32) -> Vec<&PricingModel> {
        self.pricing_models
            .values()
            .filter(|m| {
                // Approximate check (assume 1000 input + 500 output tokens)
                match self.estimate_cost(&m.provider, &m.model, 1000, 500) {
                    Ok(cost) => cost <= max_cost,
                    Err(_) => false,
                }
            })
            .collect()
    }
    
    /// Compare costs across models
    pub fn compare_costs(
        &self,
        models: &[(String, String)], // (provider, model) pairs
        input_tokens: u32,
        output_tokens: u32,
    ) -> Vec<CostComparison> {
        let mut comparisons: Vec<_> = models
            .iter()
            .filter_map(|(provider, model)| {
                self.estimate_cost(provider, model, input_tokens, output_tokens)
                    .ok()
                    .map(|cost| CostComparison {
                        provider: provider.clone(),
                        model: model.clone(),
                        estimated_cost: cost,
                    })
            })
            .collect();
        
        comparisons.sort_by(|a, b| {
            a.estimated_cost
                .partial_cmp(&b.estimated_cost)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        comparisons
    }
}

#[derive(Debug, Clone)]
pub struct CostComparison {
    pub provider: String,
    pub model: String,
    pub estimated_cost: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cost_estimation() {
        let estimator = CostEstimator::load_defaults();
        
        // GPT-4: 1000 input + 500 output tokens
        let cost = estimator
            .estimate_cost("openai", "gpt-4", 1000, 500)
            .unwrap();
        
        // (1000/1000 * 0.03) + (500/1000 * 0.06) = 0.03 + 0.03 = 0.06
        assert!((cost - 0.06).abs() < 0.001);
    }
    
    #[test]
    fn test_free_model() {
        let estimator = CostEstimator::load_defaults();
        
        let cost = estimator
            .estimate_cost("local", "llama-2-7b", 10000, 5000)
            .unwrap();
        
        assert_eq!(cost, 0.0);
    }
    
    #[test]
    fn test_cost_comparison() {
        let estimator = CostEstimator::load_defaults();
        
        let models = vec![
            ("openai".to_string(), "gpt-4".to_string()),
            ("openai".to_string(), "gpt-3.5-turbo".to_string()),
            ("anthropic".to_string(), "claude-3-opus".to_string()),
        ];
        
        let comparisons = estimator.compare_costs(&models, 1000, 500);
        
        // Should be sorted by cost (GPT-3.5 cheapest)
        assert_eq!(comparisons[0].model, "gpt-3.5-turbo");
    }
}