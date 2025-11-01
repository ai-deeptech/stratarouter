//! Comprehensive end-to-end integration tests

use stratarouter_core::{Router, RouterConfig, Route};
use std::collections::HashMap;

#[test]
fn test_full_routing_pipeline() {
    let mut router = Router::new(RouterConfig::default());
    
    // Add multiple routes
    let mut route1 = Route::new("billing");
    route1.description = "Billing and payments".into();
    route1.examples = vec![
        "Where's my invoice?".into(),
        "I need a receipt".into(),
    ];
    route1.keywords = vec!["invoice".into(), "payment".into(), "bill".into()];
    
    let mut route2 = Route::new("support");
    route2.description = "Technical support".into();
    route2.examples = vec![
        "App is crashing".into(),
        "Can't login".into(),
    ];
    route2.keywords = vec!["error".into(), "crash".into(), "bug".into()];
    
    let mut route3 = Route::new("sales");
    route3.description = "Sales and pricing".into();
    route3.examples = vec![
        "What's the price?".into(),
        "I want to upgrade".into(),
    ];
    route3.keywords = vec!["price".into(), "upgrade".into(), "plan".into()];
    
    router.add_route(route1).unwrap();
    router.add_route(route2).unwrap();
    router.add_route(route3).unwrap();
    
    assert_eq!(router.route_count(), 3);
    
    // Build index with mock embeddings
    let embeddings = vec![
        vec![0.8, 0.2, 0.1, 0.0].into_iter().cycle().take(384).collect(),
        vec![0.2, 0.8, 0.1, 0.0].into_iter().cycle().take(384).collect(),
        vec![0.1, 0.2, 0.8, 0.0].into_iter().cycle().take(384).collect(),
    ];
    
    router.build_index(embeddings).unwrap();
    assert!(router.is_index_built());
    
    // Test routing
    let query_embedding: Vec<f32> = vec![0.75, 0.25, 0.0, 0.0]
        .into_iter()
        .cycle()
        .take(384)
        .collect();
    
    let result = router.route("I need my invoice", &query_embedding).unwrap();
    
    assert!(!result.route_id.is_empty());
    assert!(result.scores.confidence >= 0.0 && result.scores.confidence <= 1.0);
    assert!(result.latency_ms > 0);
}

#[test]
fn test_routing_with_calibration_enabled() {
    let config = RouterConfig {
        enable_calibration: true,
        ..Default::default()
    };
    
    let mut router = Router::new(config);
    
    let mut route = Route::new("test");
    route.description = "Test route".into();
    route.examples = vec!["test query".into()];
    
    router.add_route(route).unwrap();
    
    let embeddings = vec![vec![0.5; 384]];
    router.build_index(embeddings).unwrap();
    
    let result = router.route("test query", &[0.5; 384]).unwrap();
    
    assert_eq!(result.route_id, "test");
    assert!(result.scores.confidence >= 0.0);
}

#[test]
fn test_routing_with_calibration_disabled() {
    let config = RouterConfig {
        enable_calibration: false,
        ..Default::default()
    };
    
    let mut router = Router::new(config);
    
    let mut route = Route::new("test");
    route.description = "Test route".into();
    
    router.add_route(route).unwrap();
    router.build_index(vec![vec![0.5; 384]]).unwrap();
    
    let result = router.route("query", &[0.5; 384]).unwrap();
    assert_eq!(result.route_id, "test");
}

#[test]
fn test_multiple_queries_same_router() {
    let mut router = Router::new(RouterConfig::default());
    
    let mut route = Route::new("test");
    route.description = "Test route".into();
    route.keywords = vec!["test".into()];
    
    router.add_route(route).unwrap();
    router.build_index(vec![vec![0.5; 384]]).unwrap();
    
    // Run multiple queries
    for i in 0..10 {
        let query = format!("test query {}", i);
        let result = router.route(&query, &[0.5; 384]).unwrap();
        assert_eq!(result.route_id, "test");
        assert!(result.latency_ms > 0);
    }
}

#[test]
fn test_routing_keyword_matching() {
    let mut router = Router::new(RouterConfig::default());
    
    let mut route = Route::new("billing");
    route.description = "Billing queries".into();
    route.keywords = vec!["invoice".into(), "payment".into(), "bill".into()];
    
    router.add_route(route).unwrap();
    router.build_index(vec![vec![0.5; 384]]).unwrap();
    
    // Query with keyword should score well
    let result = router.route("Where is my invoice?", &[0.5; 384]).unwrap();
    assert_eq!(result.route_id, "billing");
    assert!(result.scores.keyword > 0.0, "Keyword score should be positive");
}

#[test]
fn test_routing_with_thresholds() {
    let mut router = Router::new(RouterConfig::default());
    
    let mut route = Route::new("high_confidence");
    route.description = "High confidence route".into();
    route.threshold = Some(0.9);
    
    router.add_route(route).unwrap();
    router.build_index(vec![vec![0.5; 384]]).unwrap();
    
    let result = router.route("query", &[0.5; 384]).unwrap();
    assert_eq!(result.route_id, "high_confidence");
}

#[test]
fn test_routing_with_metadata() {
    let mut router = Router::new(RouterConfig::default());
    
    let mut metadata = HashMap::new();
    metadata.insert("category".to_string(), "customer_service".to_string());
    metadata.insert("priority".to_string(), "high".to_string());
    
    let route = Route {
        id: "support".into(),
        description: "Support route".into(),
        examples: vec!["help".into()],
        keywords: vec![],
        patterns: vec![],
        metadata,
        threshold: None,
        tags: vec![],
    };
    
    router.add_route(route).unwrap();
    router.build_index(vec![vec![0.5; 384]]).unwrap();
    
    let result = router.route("I need help", &[0.5; 384]).unwrap();
    assert_eq!(result.route_id, "support");
}

#[test]
fn test_routing_with_tags() {
    let mut router = Router::new(RouterConfig::default());
    
    let mut route = Route::new("tagged");
    route.description = "Tagged route".into();
    route.tags = vec!["important".into(), "verified".into()];
    
    router.add_route(route).unwrap();
    router.build_index(vec![vec![0.5; 384]]).unwrap();
    
    let result = router.route("query", &[0.5; 384]).unwrap();
    assert_eq!(result.route_id, "tagged");
}

#[test]
fn test_routing_empty_query() {
    let mut router = Router::new(RouterConfig::default());
    
    let mut route = Route::new("test");
    route.description = "Test".into();
    router.add_route(route).unwrap();
    router.build_index(vec![vec![0.5; 384]]).unwrap();
    
    let result = router.route("", &[0.5; 384]);
    assert!(result.is_err());
}

#[test]
fn test_routing_mismatched_embedding_dimension() {
    let mut router = Router::new(RouterConfig::default());
    
    let mut route = Route::new("test");
    route.description = "Test".into();
    router.add_route(route).unwrap();
    router.build_index(vec![vec![0.5; 384]]).unwrap();
    
    // Wrong dimension
    let wrong_embedding = vec![0.5; 512];
    let result = router.route("query", &wrong_embedding);
    assert!(result.is_err());
}

#[test]
fn test_concurrent_routing() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    let mut router = Router::new(RouterConfig::default());
    
    let mut route = Route::new("test");
    route.description = "Test".into();
    router.add_route(route).unwrap();
    router.build_index(vec![vec![0.5; 384]]).unwrap();
    
    let router = Arc::new(Mutex::new(router));
    let mut handles = vec![];
    
    for i in 0..5 {
        let router_clone = Arc::clone(&router);
        let handle = thread::spawn(move || {
            let mut r = router_clone.lock().unwrap();
            r.route(&format!("test {}", i), &[0.5; 384]).unwrap()
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let result = handle.join().unwrap();
        assert_eq!(result.route_id, "test");
    }
}

#[test]
fn test_router_config_custom() {
    let config = RouterConfig {
        dimension: 768,
        default_threshold: 0.7,
        top_k: 10,
        enable_calibration: false,
    };
    
    let router = Router::new(config);
    assert!(!router.is_index_built());
    assert_eq!(router.route_count(), 0);
}

#[test]
fn test_build_index_dimension_validation() {
    let config = RouterConfig {
        dimension: 768,
        ..Default::default()
    };
    
    let mut router = Router::new(config);
    let mut route = Route::new("test");
    route.description = "Test".into();
    router.add_route(route).unwrap();
    
    // Wrong dimension
    let result = router.build_index(vec![vec![0.5; 384]]);
    assert!(result.is_err());
}

#[test]
fn test_large_number_of_routes() {
    let mut router = Router::new(RouterConfig::default());
    
    // Add 50 routes
    for i in 0..50 {
        let mut route = Route::new(format!("route_{}", i));
        route.description = format!("Route number {}", i);
        router.add_route(route).unwrap();
    }
    
    assert_eq!(router.route_count(), 50);
    
    // Build index
    let embeddings: Vec<Vec<f32>> = (0..50)
        .map(|i| vec![(i as f32) / 50.0; 384])
        .collect();
    
    router.build_index(embeddings).unwrap();
    
    // Query should work
    let result = router.route("test query", &[0.5; 384]).unwrap();
    assert!(!result.route_id.is_empty());
}

#[test]
fn test_routing_with_patterns() {
    let mut router = Router::new(RouterConfig::default());
    
    let mut route = Route::new("pattern_test");
    route.description = "Pattern matching test".into();
    route.patterns = vec!["urgent.*help".into(), "emergency.*".into()];
    
    router.add_route(route).unwrap();
    router.build_index(vec![vec![0.5; 384]]).unwrap();
    
    let result = router.route("urgent help needed", &[0.5; 384]).unwrap();
    assert_eq!(result.route_id, "pattern_test");
}

#[test]
fn test_router_empty_state() {
    let router = Router::new(RouterConfig::default());
    assert_eq!(router.route_count(), 0);
    assert!(!router.is_index_built());
}

#[test]
fn test_build_index_without_routes() {
    let mut router = Router::new(RouterConfig::default());
    let result = router.build_index(vec![]);
    assert!(result.is_err());
}

#[test]
fn test_route_without_index() {
    let mut router = Router::new(RouterConfig::default());
    let mut route = Route::new("test");
    route.description = "Test".into();
    router.add_route(route).unwrap();
    
    let result = router.route("query", &[0.5; 384]);
    assert!(result.is_err());
}
