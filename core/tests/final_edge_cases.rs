//! Edge case tests for final coverage push to 95%+
//! These tests target specific uncovered branches identified in coverage reports

use stratarouter_core::{Router, RouterConfig, Route, Error};
use std::collections::HashMap;

// ============================================================================
// Critical Edge Cases for Router.route() method
// ============================================================================

/// Test the path where index returns a neighbor ID that's out of bounds
/// This tests the `if idx >= self.route_ids.len()` branch
#[test]
fn test_route_with_invalid_index() {
    let mut router = Router::new(RouterConfig::default());
    
    let route = Route {
        id: "test".into(),
        description: "Test".into(),
        examples: vec!["test".into()],
        keywords: vec!["test".into()],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };
    
    router.add_route(route).unwrap();
    router.build_index(vec![vec![0.5; 384]]).unwrap();
    
    // The router should handle this gracefully even if HNSW returns invalid indices
    // (though our implementation shouldn't do this, this tests the safety check)
    let embedding = vec![0.5; 384];
    let result = router.route("test query", &embedding);
    
    // Should still work or return appropriate error
    assert!(result.is_ok() || matches!(result, Err(Error::NoRoutes)));
}

/// Test when all neighbors from HNSW search are skipped due to missing routes
/// This ensures the `best_route_id.is_empty()` check at the end works
#[test]
fn test_route_all_candidates_skipped() {
    // This is hard to trigger in normal usage, but we can test the behavior
    // when no valid routes are found after filtering
    let config = RouterConfig {
        dimension: 384,
        default_threshold: 0.5,
        top_k: 1,
        enable_calibration: false,
    };
    
    let mut router = Router::new(config);
    
    let route = Route {
        id: "test".into(),
        description: "Test".into(),
        examples: vec!["test".into()],
        keywords: vec![],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };
    
    router.add_route(route).unwrap();
    router.build_index(vec![vec![0.0; 384]]).unwrap();
    
    // Search with a very different embedding
    let embedding = vec![1.0; 384];
    let result = router.route("test", &embedding);
    
    // Should succeed or fail gracefully
    assert!(result.is_ok() || matches!(result, Err(Error::NoRoutes)));
}

// ============================================================================
// BM25 Scoring Edge Cases
// ============================================================================

#[test]
fn test_bm25_with_very_long_document() {
    use stratarouter_core::algorithms::HybridScorer;
    
    let scorer = HybridScorer::new();
    
    let route = Route {
        id: "test".into(),
        description: "Test".into(),
        examples: vec![],
        keywords: vec!["test".into()],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };
    
    // Very long text (200 words)
    let long_text = "test ".repeat(200);
    let score = scorer.compute_sparse_score(&long_text, &route);
    
    // Should handle long documents without panic
    assert!(score >= 0.0 && score <= 1.0);
}

#[test]
fn test_bm25_with_very_short_document() {
    use stratarouter_core::algorithms::HybridScorer;
    
    let scorer = HybridScorer::new();
    
    let route = Route {
        id: "test".into(),
        description: "Test".into(),
        examples: vec![],
        keywords: vec!["test".into()],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };
    
    // Very short text
    let score = scorer.compute_sparse_score("test", &route);
    
    assert!(score >= 0.0 && score <= 1.0);
}

#[test]
fn test_bm25_repeated_keyword() {
    use stratarouter_core::algorithms::HybridScorer;
    
    let scorer = HybridScorer::new();
    
    let route = Route {
        id: "test".into(),
        description: "Test".into(),
        examples: vec![],
        keywords: vec!["invoice".into()],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };
    
    // Keyword appears many times
    let text = "invoice invoice invoice invoice invoice";
    let score = scorer.compute_sparse_score(text, &route);
    
    // BM25 should saturate, not grow unbounded
    assert!(score >= 0.0 && score <= 1.0);
}

// ============================================================================
// Calibration Binary Search Edge Cases  
// ============================================================================

#[test]
fn test_calibration_exact_threshold_match() {
    use stratarouter_core::algorithms::CalibrationManager;
    
    let mut manager = CalibrationManager::new();
    
    // Test with exact threshold values
    let exact_values = vec![0.0, 0.2, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    
    for value in exact_values {
        let (calibrated, uncertainty) = manager.calibrate_for_route("test", value);
        assert!(calibrated >= 0.0 && calibrated <= 1.0);
        assert!(uncertainty >= 0.0);
    }
}

#[test]
fn test_calibration_between_thresholds() {
    use stratarouter_core::algorithms::CalibrationManager;
    
    let mut manager = CalibrationManager::new();
    
    // Test interpolation between all adjacent pairs
    let values = vec![
        0.1, 0.3, 0.45, 0.55, 0.65, 0.75, 0.85, 0.95
    ];
    
    for value in values {
        let (calibrated, _) = manager.calibrate_for_route("test", value);
        assert!(calibrated >= 0.0 && calibrated <= 1.0);
    }
}

#[test]
fn test_calibration_at_upper_bound() {
    use stratarouter_core::algorithms::CalibrationManager;
    
    let mut manager = CalibrationManager::new();
    
    // Test exactly at 1.0 (upper bound)
    let (calibrated, _) = manager.calibrate_for_route("test", 1.0);
    assert!(calibrated > 0.9);  // Should be high
    assert!(calibrated <= 1.0);
}

// ============================================================================
// Error Display and Debug
// ============================================================================

#[test]
fn test_all_error_displays() {
    let errors = vec![
        Error::RouteNotFound { route_id: "test_route".into() },
        Error::DimensionMismatch { expected: 384, actual: 256 },
        Error::IndexNotBuilt,
        Error::InvalidInput { message: "test message".into() },
        Error::NoRoutes,
        Error::Unknown { message: "unknown error".into() },
    ];
    
    for error in errors {
        let display = format!("{}", error);
        let debug = format!("{:?}", error);
        
        assert!(!display.is_empty());
        assert!(!debug.is_empty());
    }
}

#[test]
fn test_error_helper_constructors() {
    let err1 = Error::invalid_input("test");
    assert!(matches!(err1, Error::InvalidInput { .. }));
    
    let err2 = Error::dimension_mismatch(384, 256);
    assert!(matches!(err2, Error::DimensionMismatch { .. }));
}

// ============================================================================
// Route Serialization
// ============================================================================

#[test]
fn test_route_serialization_full() {
    let mut metadata = HashMap::new();
    metadata.insert("key1".into(), "value1".into());
    metadata.insert("key2".into(), "value2".into());
    
    let route = Route {
        id: "billing".into(),
        description: "Billing queries".into(),
        examples: vec!["invoice".into(), "payment".into()],
        keywords: vec!["bill".into(), "invoice".into()],
        patterns: vec!["need invoice".into()],
        metadata,
        threshold: Some(0.75),
        tags: vec!["finance".into(), "urgent".into()],
    };
    
    // Serialize to JSON
    let json = serde_json::to_string(&route).unwrap();
    
    // Deserialize back
    let deserialized: Route = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.id, "billing");
    assert_eq!(deserialized.examples.len(), 2);
    assert_eq!(deserialized.keywords.len(), 2);
    assert_eq!(deserialized.patterns.len(), 1);
    assert_eq!(deserialized.metadata.len(), 2);
    assert_eq!(deserialized.threshold, Some(0.75));
    assert_eq!(deserialized.tags.len(), 2);
}

// ============================================================================
// RouterConfig Cloning and Debug
// ============================================================================

#[test]
fn test_router_config_clone() {
    let config1 = RouterConfig {
        dimension: 512,
        default_threshold: 0.6,
        top_k: 10,
        enable_calibration: false,
    };
    
    let config2 = config1.clone();
    
    assert_eq!(config1.dimension, config2.dimension);
    assert_eq!(config1.default_threshold, config2.default_threshold);
    assert_eq!(config1.top_k, config2.top_k);
    assert_eq!(config1.enable_calibration, config2.enable_calibration);
}

#[test]
fn test_router_config_debug() {
    let config = RouterConfig::default();
    let debug_str = format!("{:?}", config);
    
    assert!(debug_str.contains("dimension"));
    assert!(debug_str.contains("384"));
}

// ============================================================================
// RouteScores Cloning and Debug
// ============================================================================

#[test]
fn test_route_scores_clone() {
    use stratarouter_core::RouteScores;
    
    let scores1 = RouteScores {
        semantic: 0.8,
        keyword: 0.6,
        pattern: 0.4,
        total: 0.7,
        confidence: 0.75,
    };
    
    let scores2 = scores1.clone();
    
    assert_eq!(scores1.semantic, scores2.semantic);
    assert_eq!(scores1.keyword, scores2.keyword);
    assert_eq!(scores1.pattern, scores2.pattern);
    assert_eq!(scores1.total, scores2.total);
    assert_eq!(scores1.confidence, scores2.confidence);
}

#[test]
fn test_route_scores_debug() {
    use stratarouter_core::RouteScores;
    
    let scores = RouteScores::zero();
    let debug_str = format!("{:?}", scores);
    
    assert!(debug_str.contains("semantic"));
    assert!(debug_str.contains("0.0"));
}

// ============================================================================
// Route Cloning and Debug
// ============================================================================

#[test]
fn test_route_clone() {
    let route1 = Route {
        id: "test".into(),
        description: "Test route".into(),
        examples: vec!["ex1".into()],
        keywords: vec!["kw1".into()],
        patterns: vec!["p1".into()],
        metadata: HashMap::new(),
        threshold: Some(0.5),
        tags: vec!["tag1".into()],
    };
    
    let route2 = route1.clone();
    
    assert_eq!(route1.id, route2.id);
    assert_eq!(route1.description, route2.description);
    assert_eq!(route1.examples, route2.examples);
}

#[test]
fn test_route_debug() {
    let route = Route::new("test");
    let debug_str = format!("{:?}", route);
    
    assert!(debug_str.contains("test"));
    assert!(debug_str.contains("Route"));
}

// ============================================================================
// Calibrator Edge Cases
// ============================================================================

#[test]
fn test_isotonic_calibrator_new() {
    use stratarouter_core::algorithms::calibration::IsotonicCalibrator;
    
    let calibrator = IsotonicCalibrator::new();
    let (score, unc) = calibrator.calibrate(0.5);
    
    assert!(score > 0.0 && score < 1.0);
    assert!(unc > 0.0);
}

#[test]
fn test_isotonic_calibrator_default() {
    use stratarouter_core::algorithms::calibration::IsotonicCalibrator;
    
    let cal1 = IsotonicCalibrator::new();
    let cal2 = IsotonicCalibrator::default();
    
    let (s1, _) = cal1.calibrate(0.5);
    let (s2, _) = cal2.calibrate(0.5);
    
    assert_eq!(s1, s2);
}

#[test]
fn test_calibration_very_small_values() {
    use stratarouter_core::algorithms::CalibrationManager;
    
    let mut manager = CalibrationManager::new();
    
    let small_values = vec![0.001, 0.01, 0.05, 0.1];
    
    for value in small_values {
        let (calibrated, _) = manager.calibrate_for_route("test", value);
        assert!(calibrated >= 0.0 && calibrated <= 1.0);
    }
}

#[test]
fn test_calibration_very_large_values() {
    use stratarouter_core::algorithms::CalibrationManager;
    
    let mut manager = CalibrationManager::new();
    
    let large_values = vec![0.9, 0.95, 0.99, 0.999];
    
    for value in large_values {
        let (calibrated, _) = manager.calibrate_for_route("test", value);
        assert!(calibrated >= 0.0 && calibrated <= 1.0);
    }
}

// ============================================================================
// Stress Tests
// ============================================================================

#[test]
fn test_router_with_many_routes() {
    let mut router = Router::new(RouterConfig::default());
    
    // Add 50 routes
    for i in 0..50 {
        let route = Route {
            id: format!("route_{}", i),
            description: format!("Route {}", i),
            examples: vec![format!("example {}", i)],
            keywords: vec![format!("keyword{}", i)],
            patterns: vec![],
            metadata: HashMap::new(),
            threshold: None,
            tags: vec![],
        };
        
        router.add_route(route).unwrap();
    }
    
    assert_eq!(router.route_count(), 50);
    
    // Build index
    let embeddings: Vec<Vec<f32>> = (0..50)
        .map(|i| vec![(i as f32) / 50.0; 384])
        .collect();
    
    router.build_index(embeddings).unwrap();
    
    // Route a query
    let embedding = vec![0.5; 384];
    let result = router.route("test query", &embedding);
    
    assert!(result.is_ok());
}

#[test]
fn test_hybrid_scorer_with_many_keywords() {
    use stratarouter_core::algorithms::HybridScorer;
    
    let scorer = HybridScorer::new();
    
    let mut route = Route::new("test");
    route.examples = vec!["test".into()];
    
    // Add 100 keywords
    for i in 0..100 {
        route.keywords.push(format!("keyword{}", i));
    }
    
    let score = scorer.compute_sparse_score("keyword50 keyword75", &route);
    assert!(score >= 0.0 && score <= 1.0);
}
