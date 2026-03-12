//! Hybrid scoring: combines dense semantic similarity, BM25 keyword matching,
//! and rule-based pattern scoring into a single confidence score.

use crate::types::Route;

/// Combines dense, sparse, and rule-based signals into a single score.
pub struct HybridScorer {
    dense_weight: f32,
    sparse_weight: f32,
    rule_weight: f32,
}

impl HybridScorer {
    /// Create a scorer with empirically tuned weights.
    pub fn new() -> Self {
        Self {
            dense_weight: 0.6427,
            sparse_weight: 0.2891,
            rule_weight: 0.0682,
        }
    }

    /// Compute a BM25-inspired keyword match score in `[0, 1]`.
    pub fn compute_sparse_score(&self, text: &str, route: &Route) -> f32 {
        if route.keywords.is_empty() {
            return 0.0;
        }

        let text_lower = text.to_lowercase();
        let words: Vec<&str> = text_lower.split_whitespace().collect();

        if words.is_empty() {
            return 0.0;
        }

        let mut score = 0.0_f32;
        // BM25 hyper-parameters
        let k1 = 1.5_f32;
        let b = 0.75_f32;
        let avg_len = 20.0_f32;
        let doc_len = words.len() as f32;

        for keyword in &route.keywords {
            let kw_lower = keyword.to_lowercase();
            let tf = words.iter().filter(|w| w.contains(&kw_lower)).count() as f32;

            if tf > 0.0 {
                let norm = tf * (k1 + 1.0)
                    / (tf + k1 * (1.0 - b + b * (doc_len / avg_len)));
                score += norm;
            }
        }

        (score / route.keywords.len() as f32).min(1.0)
    }

    /// Compute a pattern match score in `[0, 1]`.
    pub fn compute_rule_score(&self, text: &str, route: &Route) -> f32 {
        if route.patterns.is_empty() {
            return 0.0;
        }

        let text_lower = text.to_lowercase();
        let matches = route
            .patterns
            .iter()
            .filter(|p| text_lower.contains(&p.to_lowercase()))
            .count();

        (matches as f32 / route.patterns.len() as f32).min(1.0)
    }

    /// Fuse the three component scores into a single value using learned
    /// weights, followed by sigmoid normalisation to `(0, 1)`.
    pub fn fuse_scores(&self, dense: f32, sparse: f32, rule: f32) -> f32 {
        let base = self.dense_weight * dense
            + self.sparse_weight * sparse
            + self.rule_weight * rule;
        1.0 / (1.0 + (-base).exp())
    }
}

impl Default for HybridScorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn test_route() -> Route {
        Route {
            id: "billing".into(),
            description: String::new(),
            examples: vec![],
            keywords: vec!["invoice".into(), "payment".into()],
            patterns: vec!["need invoice".into()],
            metadata: HashMap::new(),
            threshold: None,
            tags: vec![],
        }
    }

    #[test]
    fn test_sparse_score_match() {
        let scorer = HybridScorer::new();
        let score = scorer.compute_sparse_score("I need my invoice", &test_route());
        assert!(score > 0.0 && score <= 1.0);
    }

    #[test]
    fn test_sparse_score_no_match() {
        let scorer = HybridScorer::new();
        let score = scorer.compute_sparse_score("hello world", &test_route());
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_rule_score_match() {
        let scorer = HybridScorer::new();
        let score = scorer.compute_rule_score("I need invoice please", &test_route());
        assert!(score > 0.0 && score <= 1.0);
    }

    #[test]
    fn test_fuse_scores_in_range() {
        let scorer = HybridScorer::new();
        let fused = scorer.fuse_scores(0.8, 0.6, 0.5);
        assert!(fused > 0.0 && fused <= 1.0);
    }

    #[test]
    fn test_fuse_scores_monotone() {
        let scorer = HybridScorer::new();
        let low = scorer.fuse_scores(0.1, 0.0, 0.0);
        let high = scorer.fuse_scores(0.9, 0.8, 0.7);
        assert!(high > low);
    }
}
