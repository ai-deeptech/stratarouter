//! Comprehensive test suite to achieve 95%+ coverage
//! This file adds tests for uncovered code paths identified from tarpaulin report

use std::collections::HashMap;
use stratarouter_core::{
    algorithms::{CalibrationManager, HybridScorer},
    Error, Route, RouteResult, RouteScores, Router, RouterConfig,
};

// ============================================================================
// Router Configuration Tests
// ============================================================================

#[test]
fn test_router_config_default() {
    let config = RouterConfig::default();
    assert_eq!(config.dimension, 384);
    assert_eq!(config.default_threshold, 0.5);
    assert_eq!(config.top_k, 5);
    assert!(config.enable_calibration);
}

#[test]
fn test_router_config_validation_zero_dimension() {
    let config = RouterConfig {
        dimension: 0,
        default_threshold: 0.5,
        top_k: 5,
        enable_calibration: true,
    };
    assert!(config.validate().is_err());
}

#[test]
fn test_router_config_validation_threshold_too_low() {
    let config = RouterConfig {
        dimension: 384,
        default_threshold: -0.1,
        top_k: 5,
        enable_calibration: true,
    };
    assert!(config.validate().is_err());
}

#[test]
fn test_router_config_validation_threshold_too_high() {
    let config = RouterConfig {
        dimension: 384,
        default_threshold: 1.5,
        top_k: 5,
        enable_calibration: true,
    };
    assert!(config.validate().is_err());
}

#[test]
fn test_router_config_validation_zero_top_k() {
    let config = RouterConfig {
        dimension: 384,
        default_threshold: 0.5,
        top_k: 0,
        enable_calibration: true,
    };
    assert!(config.validate().is_err());
}

// ============================================================================
// Route Tests
// ============================================================================

#[test]
fn test_route_builder_pattern() {
    let route = Route::new("billing");
    assert_eq!(route.id, "billing");
    assert!(route.description.is_empty());
    assert!(route.examples.is_empty());
    assert!(route.keywords.is_empty());
    assert!(route.patterns.is_empty());
    assert!(route.metadata.is_empty());
    assert!(route.threshold.is_none());
    assert!(route.tags.is_empty());
}

#[test]
fn test_route_validation_empty_id() {
    let route = Route::new("");
    assert!(route.validate().is_err());

    if let Err(Error::InvalidInput { message }) = route.validate() {
        assert!(message.contains("Route ID"));
    } else {
        panic!("Expected InvalidInput error");
    }
}

#[test]
fn test_route_validation_no_examples_no_description() {
    let route = Route::new("test_route");
    assert!(route.validate().is_err());

    if let Err(Error::InvalidInput { message }) = route.validate() {
        assert!(message.contains("examples") || message.contains("description"));
    } else {
        panic!("Expected InvalidInput error");
    }
}

#[test]
fn test_route_validation_with_description() {
    let mut route = Route::new("test");
    route.description = "Test route".to_string();
    assert!(route.validate().is_ok());
}

#[test]
fn test_route_validation_with_examples() {
    let mut route = Route::new("test");
    route.examples = vec!["example query".to_string()];
    assert!(route.validate().is_ok());
}

// ============================================================================
// Router Tests - Build Index Scenarios
// ============================================================================

#[test]
fn test_build_index_mismatch_route_count() {
    let mut router = Router::new(RouterConfig::default());

    let route1 = Route {
        id: "route1".into(),
        description: "Route 1".into(),
        examples: vec!["test".into()],
        keywords: vec![],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    let route2 = Route {
        id: "route2".into(),
        description: "Route 2".into(),
        examples: vec!["test".into()],
        keywords: vec![],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    router.add_route(route1).unwrap();
    router.add_route(route2).unwrap();

    // Only provide 1 embedding for 2 routes
    let embeddings = vec![vec![0.5; 384]];
    let result = router.build_index(embeddings);
    assert!(result.is_err());
}

#[test]
fn test_build_index_dimension_mismatch() {
    let mut router = Router::new(RouterConfig::default());

    let route = Route {
        id: "test".into(),
        description: "Test route".into(),
        examples: vec!["test".into()],
        keywords: vec![],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    router.add_route(route).unwrap();

    // Provide wrong dimension (256 instead of 384)
    let embeddings = vec![vec![0.5; 256]];
    let result = router.build_index(embeddings);
    assert!(matches!(result, Err(Error::DimensionMismatch { .. })));
}

#[test]
fn test_build_index_inconsistent_embedding_dimensions() {
    let mut router = Router::new(RouterConfig::default());

    let route1 = Route {
        id: "route1".into(),
        description: "Route 1".into(),
        examples: vec!["test".into()],
        keywords: vec![],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    let route2 = Route {
        id: "route2".into(),
        description: "Route 2".into(),
        examples: vec!["test".into()],
        keywords: vec![],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    router.add_route(route1).unwrap();
    router.add_route(route2).unwrap();

    // Embeddings with inconsistent dimensions
    let embeddings = vec![
        vec![0.5; 384],
        vec![0.5; 256], // Wrong dimension
    ];

    let result = router.build_index(embeddings);
    assert!(matches!(result, Err(Error::DimensionMismatch { .. })));
}

// ============================================================================
// Router Tests - Route Method Scenarios
// ============================================================================

#[test]
fn test_route_empty_text() {
    let mut router = Router::new(RouterConfig::default());

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
    router.build_index(vec![vec![0.5; 384]]).unwrap();

    let result = router.route("", &[0.5; 384]);
    assert!(matches!(result, Err(Error::InvalidInput { .. })));
}

#[test]
fn test_route_empty_embedding() {
    let mut router = Router::new(RouterConfig::default());

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
    router.build_index(vec![vec![0.5; 384]]).unwrap();

    let result = router.route("test query", &[]);
    assert!(matches!(result, Err(Error::InvalidInput { .. })));
}

#[test]
fn test_route_embedding_dimension_mismatch() {
    let mut router = Router::new(RouterConfig::default());

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
    router.build_index(vec![vec![0.5; 384]]).unwrap();

    let result = router.route("test query", &[0.5; 256]);
    assert!(matches!(result, Err(Error::DimensionMismatch { .. })));
}

#[test]
fn test_route_with_calibration_disabled() {
    let config = RouterConfig {
        dimension: 384,
        default_threshold: 0.5,
        top_k: 5,
        enable_calibration: false, // Disabled
    };

    let mut router = Router::new(config);

    let route = Route {
        id: "test".into(),
        description: "Test route".into(),
        examples: vec!["test".into()],
        keywords: vec!["test".into()],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    router.add_route(route).unwrap();
    router.build_index(vec![vec![0.5; 384]]).unwrap();

    let embedding = vec![0.5; 384];
    let result = router.route("test query", &embedding);
    assert!(result.is_ok());

    let route_result = result.unwrap();
    assert_eq!(route_result.route_id, "test");
}

#[test]
fn test_route_with_calibration_enabled() {
    let config = RouterConfig {
        dimension: 384,
        default_threshold: 0.5,
        top_k: 5,
        enable_calibration: true, // Enabled
    };

    let mut router = Router::new(config);

    let route = Route {
        id: "billing".into(),
        description: "Billing queries".into(),
        examples: vec!["invoice".into()],
        keywords: vec!["invoice".into(), "billing".into()],
        patterns: vec!["need invoice".into()],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    router.add_route(route).unwrap();
    router.build_index(vec![vec![0.6; 384]]).unwrap();

    let embedding = vec![0.6; 384];
    let result = router.route("I need my invoice", &embedding);
    assert!(result.is_ok());

    let route_result = result.unwrap();
    assert_eq!(route_result.route_id, "billing");
    assert!(route_result.scores.confidence > 0.0);
}

#[test]
fn test_route_multiple_candidates() {
    let mut router = Router::new(RouterConfig::default());

    let route1 = Route {
        id: "billing".into(),
        description: "Billing".into(),
        examples: vec!["invoice".into()],
        keywords: vec!["invoice".into()],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    let route2 = Route {
        id: "support".into(),
        description: "Support".into(),
        examples: vec!["help".into()],
        keywords: vec!["help".into()],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    let route3 = Route {
        id: "sales".into(),
        description: "Sales".into(),
        examples: vec!["buy".into()],
        keywords: vec!["buy".into()],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    router.add_route(route1).unwrap();
    router.add_route(route2).unwrap();
    router.add_route(route3).unwrap();

    let embeddings = vec![vec![0.9; 384], vec![0.3; 384], vec![0.1; 384]];

    router.build_index(embeddings).unwrap();

    let query_embedding = vec![0.85; 384];
    let result = router.route("Where is my invoice?", &query_embedding);
    assert!(result.is_ok());
}

// ============================================================================
// Error Tests
// ============================================================================

#[test]
fn test_error_route_not_found() {
    let error = Error::RouteNotFound {
        route_id: "missing_route".to_string(),
    };

    assert!(error.is_recoverable());
    assert_eq!(
        error.severity(),
        stratarouter_core::error::ErrorSeverity::Low
    );
    assert!(error.to_string().contains("missing_route"));
}

#[test]
fn test_error_index_not_built() {
    let error = Error::IndexNotBuilt;
    assert!(error.is_recoverable());
    assert_eq!(
        error.severity(),
        stratarouter_core::error::ErrorSeverity::Medium
    );
}

#[test]
fn test_error_no_routes() {
    let error = Error::NoRoutes;
    assert!(error.is_recoverable());
    assert_eq!(
        error.severity(),
        stratarouter_core::error::ErrorSeverity::Medium
    );
}

#[test]
fn test_error_io() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let error = Error::from(io_error);
    assert!(!error.is_recoverable());
    assert_eq!(
        error.severity(),
        stratarouter_core::error::ErrorSeverity::Critical
    );
}

#[test]
fn test_error_serialization() {
    let json = "{invalid json}";
    let parse_error = serde_json::from_str::<HashMap<String, String>>(json);

    if let Err(e) = parse_error {
        let error = Error::from(e);
        assert!(!error.is_recoverable());
        assert_eq!(
            error.severity(),
            stratarouter_core::error::ErrorSeverity::High
        );
    }
}

#[test]
fn test_error_unknown() {
    let error = Error::Unknown {
        message: "Something unexpected happened".to_string(),
    };

    assert!(!error.is_recoverable());
    assert_eq!(
        error.severity(),
        stratarouter_core::error::ErrorSeverity::Critical
    );
    assert!(error.to_string().contains("unexpected"));
}

// ============================================================================
// HybridScorer Tests
// ============================================================================

#[test]
fn test_hybrid_scorer_default() {
    let scorer1 = HybridScorer::new();
    let scorer2 = HybridScorer::default();

    // Both should have same weights
    assert_eq!(
        scorer1.fuse_scores(0.5, 0.5, 0.5),
        scorer2.fuse_scores(0.5, 0.5, 0.5)
    );
}

#[test]
fn test_sparse_score_empty_keywords() {
    let scorer = HybridScorer::new();
    let route = Route {
        id: "test".into(),
        description: "Test".into(),
        examples: vec![],
        keywords: vec![], // Empty keywords
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    let score = scorer.compute_sparse_score("test query", &route);
    assert_eq!(score, 0.0);
}

#[test]
fn test_sparse_score_empty_text() {
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

    let score = scorer.compute_sparse_score("", &route);
    assert_eq!(score, 0.0);
}

#[test]
fn test_sparse_score_whitespace_only() {
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

    let score = scorer.compute_sparse_score("   ", &route);
    assert_eq!(score, 0.0);
}

#[test]
fn test_sparse_score_multiple_keyword_matches() {
    let scorer = HybridScorer::new();
    let route = Route {
        id: "billing".into(),
        description: "Billing".into(),
        examples: vec![],
        keywords: vec!["invoice".into(), "payment".into(), "billing".into()],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    let score = scorer.compute_sparse_score("I need my invoice for payment", &route);
    assert!(score > 0.0);
    assert!(score <= 1.0);
}

#[test]
fn test_sparse_score_case_insensitive() {
    let scorer = HybridScorer::new();
    let route = Route {
        id: "test".into(),
        description: "Test".into(),
        examples: vec![],
        keywords: vec!["InVoIcE".into()],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    let score = scorer.compute_sparse_score("INVOICE needed", &route);
    assert!(score > 0.0);
}

#[test]
fn test_rule_score_empty_patterns() {
    let scorer = HybridScorer::new();
    let route = Route {
        id: "test".into(),
        description: "Test".into(),
        examples: vec![],
        keywords: vec![],
        patterns: vec![], // Empty patterns
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    let score = scorer.compute_rule_score("test query", &route);
    assert_eq!(score, 0.0);
}

#[test]
fn test_rule_score_case_insensitive() {
    let scorer = HybridScorer::new();
    let route = Route {
        id: "test".into(),
        description: "Test".into(),
        examples: vec![],
        keywords: vec![],
        patterns: vec!["NeEd InVoIcE".into()],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    let score = scorer.compute_rule_score("I NEED INVOICE please", &route);
    assert!(score > 0.0);
}

#[test]
fn test_rule_score_multiple_patterns() {
    let scorer = HybridScorer::new();
    let route = Route {
        id: "test".into(),
        description: "Test".into(),
        examples: vec![],
        keywords: vec![],
        patterns: vec![
            "need invoice".into(),
            "want receipt".into(),
            "billing question".into(),
        ],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    let score = scorer.compute_rule_score("I need invoice and want receipt", &route);
    assert!(score > 0.5); // Should match 2 out of 3 patterns
}

#[test]
fn test_fuse_scores_all_zeros() {
    let scorer = HybridScorer::new();
    let fused = scorer.fuse_scores(0.0, 0.0, 0.0);
    assert!(fused >= 0.0 && fused <= 1.0);
}

#[test]
fn test_fuse_scores_all_ones() {
    let scorer = HybridScorer::new();
    let fused = scorer.fuse_scores(1.0, 1.0, 1.0);
    assert!(fused > 0.5 && fused <= 1.0);
}

#[test]
fn test_fuse_scores_mixed() {
    let scorer = HybridScorer::new();
    let fused1 = scorer.fuse_scores(1.0, 0.0, 0.0);
    let fused2 = scorer.fuse_scores(0.0, 1.0, 0.0);
    let fused3 = scorer.fuse_scores(0.0, 0.0, 1.0);

    // All should be different and normalized
    assert!(fused1 > fused2); // Dense has highest weight
    assert!(fused2 > fused3); // Sparse has higher weight than rule
    assert!(fused1 >= 0.0 && fused1 <= 1.0);
    assert!(fused2 >= 0.0 && fused2 <= 1.0);
    assert!(fused3 >= 0.0 && fused3 <= 1.0);
}

// ============================================================================
// CalibrationManager Tests
// ============================================================================

#[test]
fn test_calibration_manager_default() {
    let manager1 = CalibrationManager::new();
    let manager2 = CalibrationManager::default();

    // Both should work the same
    drop(manager1);
    drop(manager2);
}

#[test]
fn test_calibration_edge_cases() {
    let mut manager = CalibrationManager::new();

    // Test at boundaries
    let (score_low, _) = manager.calibrate_for_route("test", 0.0);
    let (score_high, _) = manager.calibrate_for_route("test", 1.0);

    assert!(score_low < score_high);
    assert!(score_low >= 0.0 && score_low <= 1.0);
    assert!(score_high >= 0.0 && score_high <= 1.0);
}

#[test]
fn test_calibration_out_of_bounds() {
    let mut manager = CalibrationManager::new();

    // Test values outside [0, 1] - should be clamped
    let (score_below, _) = manager.calibrate_for_route("test", -0.5);
    let (score_above, _) = manager.calibrate_for_route("test", 1.5);

    assert!(score_below >= 0.0 && score_below <= 1.0);
    assert!(score_above >= 0.0 && score_above <= 1.0);
}

#[test]
fn test_calibration_multiple_routes() {
    let mut manager = CalibrationManager::new();

    let (score1, unc1) = manager.calibrate_for_route("route1", 0.6);
    let (score2, unc2) = manager.calibrate_for_route("route2", 0.6);
    let (score3, unc3) = manager.calibrate_for_route("route3", 0.6);

    // Should be similar for default calibrators
    assert!((score1 - score2).abs() < 0.01);
    assert!((score2 - score3).abs() < 0.01);
    assert!((unc1 - unc2).abs() < 0.01);
}

#[test]
fn test_calibration_interpolation() {
    let mut manager = CalibrationManager::new();

    // Test interpolation between calibration points
    let (score1, _) = manager.calibrate_for_route("test", 0.45);
    let (score2, _) = manager.calibrate_for_route("test", 0.50);
    let (score3, _) = manager.calibrate_for_route("test", 0.55);

    assert!(score1 < score2);
    assert!(score2 < score3);
}

// ============================================================================
// RouteScores Tests
// ============================================================================

#[test]
fn test_route_scores_zero() {
    let scores = RouteScores::zero();
    assert_eq!(scores.semantic, 0.0);
    assert_eq!(scores.keyword, 0.0);
    assert_eq!(scores.pattern, 0.0);
    assert_eq!(scores.total, 0.0);
    assert_eq!(scores.confidence, 0.0);
}

#[test]
fn test_route_scores_serialization() {
    let scores = RouteScores {
        semantic: 0.8,
        keyword: 0.6,
        pattern: 0.4,
        total: 0.7,
        confidence: 0.75,
    };

    let json = serde_json::to_string(&scores).unwrap();
    let deserialized: RouteScores = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.semantic, 0.8);
    assert_eq!(deserialized.keyword, 0.6);
    assert_eq!(deserialized.pattern, 0.4);
    assert_eq!(deserialized.total, 0.7);
    assert_eq!(deserialized.confidence, 0.75);
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_full_routing_workflow() {
    let config = RouterConfig {
        dimension: 384,
        default_threshold: 0.3,
        top_k: 3,
        enable_calibration: true,
    };

    let mut router = Router::new(config);

    // Add multiple routes
    let billing_route = Route {
        id: "billing".into(),
        description: "Billing and invoicing queries".into(),
        examples: vec!["Where is my invoice?".into()],
        keywords: vec!["invoice".into(), "billing".into(), "payment".into()],
        patterns: vec!["invoice".into(), "bill".into()],
        metadata: {
            let mut m = HashMap::new();
            m.insert("priority".into(), "high".into());
            m
        },
        threshold: Some(0.4),
        tags: vec!["finance".into()],
    };

    let support_route = Route {
        id: "support".into(),
        description: "Technical support".into(),
        examples: vec!["How do I reset password?".into()],
        keywords: vec!["help".into(), "support".into(), "issue".into()],
        patterns: vec!["can't".into(), "unable".into()],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec!["tech".into()],
    };

    router.add_route(billing_route).unwrap();
    router.add_route(support_route).unwrap();

    assert_eq!(router.route_count(), 2);
    assert!(!router.is_index_built());

    // Build index
    let embeddings = vec![
        vec![0.9; 384], // billing
        vec![0.1; 384], // support
    ];

    router.build_index(embeddings).unwrap();
    assert!(router.is_index_built());

    // Route a billing query
    let billing_embedding = vec![0.85; 384];
    let result = router
        .route("I need my invoice", &billing_embedding)
        .unwrap();

    assert_eq!(result.route_id, "billing");
    assert!(result.scores.semantic > 0.0);
    assert!(result.scores.keyword > 0.0); // Should match "invoice" keyword
    assert!(result.scores.total > 0.0);
    assert!(result.scores.confidence > 0.0);
    assert!(result.latency_ms >= 0);
}

#[test]
fn test_router_state_transitions() {
    let mut router = Router::new(RouterConfig::default());

    // Initial state
    assert_eq!(router.route_count(), 0);
    assert!(!router.is_index_built());

    // After adding route
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
    assert_eq!(router.route_count(), 1);
    assert!(!router.is_index_built());

    // After building index
    router.build_index(vec![vec![0.5; 384]]).unwrap();
    assert!(router.is_index_built());
}
