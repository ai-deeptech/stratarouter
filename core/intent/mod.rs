// core/intent/mod.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Formal definition of a user intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    /// Unique identifier (e.g., "classify_text", "generate_code")
    pub id: String,
    
    /// Human-readable description
    pub description: String,
    
    /// Example inputs that match this intent
    pub examples: Vec<String>,
    
    /// Optional constraints on this intent
    pub constraints: Option<IntentConstraints>,
    
    /// Tags for categorization (e.g., ["nlp", "safety-critical"])
    pub tags: Vec<String>,
}

/// Constraints specific to an intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentConstraints {
    /// Minimum confidence threshold for this intent
    pub min_confidence: f32,
    
    /// Sensitivity level (affects routing policy)
    pub sensitivity: SensitivityLevel,
    
    /// Whether this intent requires human review
    pub requires_human_review: bool,
    
    /// Maximum cost willing to pay for this intent
    pub max_cost_usd: Option<f32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SensitivityLevel {
    Public,        // Can use any provider
    Internal,      // Must use private models
    Confidential,  // Requires on-prem
    Restricted,    // HIPAA/PCI compliance needed
}

/// Registry of all known intents
pub struct IntentRegistry {
    intents: HashMap<String, Intent>,
}

impl IntentRegistry {
    pub fn new() -> Self {
        Self {
            intents: HashMap::new(),
        }
    }
    
    /// Register a new intent
    pub fn register(&mut self, intent: Intent) -> Result<(), String> {
        if self.intents.contains_key(&intent.id) {
            return Err(format!("Intent {} already registered", intent.id));
        }
        self.intents.insert(intent.id.clone(), intent);
        Ok(())
    }
    
    /// Get intent by ID
    pub fn get(&self, id: &str) -> Option<&Intent> {
        self.intents.get(id)
    }
    
    /// Find intent matching input (simplified - real impl would use ML)
    pub fn classify(&self, input: &str) -> Option<&Intent> {
        // Real implementation: embeddings + similarity search
        // For now: simple keyword matching
        for intent in self.intents.values() {
            if intent.examples.iter().any(|ex| input.contains(ex)) {
                return Some(intent);
            }
        }
        None
    }
    
    /// List all intents with a tag
    pub fn by_tag(&self, tag: &str) -> Vec<&Intent> {
        self.intents
            .values()
            .filter(|i| i.tags.contains(&tag.to_string()))
            .collect()
    }
}

// Example usage
impl Default for IntentRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        
        // Register common intents
        registry.register(Intent {
            id: "classify_text".to_string(),
            description: "Categorize text into predefined classes".to_string(),
            examples: vec![
                "Is this email spam?".to_string(),
                "What sentiment is this review?".to_string(),
            ],
            constraints: Some(IntentConstraints {
                min_confidence: 0.85,
                sensitivity: SensitivityLevel::Public,
                requires_human_review: false,
                max_cost_usd: Some(0.01),
            }),
            tags: vec!["nlp".to_string(), "classification".to_string()],
        }).ok();
        
        registry.register(Intent {
            id: "generate_code".to_string(),
            description: "Generate source code from natural language".to_string(),
            examples: vec![
                "Write a function to sort an array".to_string(),
                "Create a REST API endpoint".to_string(),
            ],
            constraints: Some(IntentConstraints {
                min_confidence: 0.75,
                sensitivity: SensitivityLevel::Internal,
                requires_human_review: true,
                max_cost_usd: Some(0.10),
            }),
            tags: vec!["codegen".to_string(), "high-stakes".to_string()],
        }).ok();
        
        registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_intent_registration() {
        let mut registry = IntentRegistry::new();
        
        let intent = Intent {
            id: "test_intent".to_string(),
            description: "Test".to_string(),
            examples: vec![],
            constraints: None,
            tags: vec![],
        };
        
        assert!(registry.register(intent.clone()).is_ok());
        assert!(registry.register(intent).is_err()); // Duplicate
    }
    
    #[test]
    fn test_intent_classification() {
        let registry = IntentRegistry::default();
        
        let intent = registry.classify("Is this email spam?");
        assert!(intent.is_some());
        assert_eq!(intent.unwrap().id, "classify_text");
    }
}