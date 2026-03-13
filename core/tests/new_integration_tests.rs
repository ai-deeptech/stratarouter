use std::collections::HashMap;
use stratarouter_core::{Route, Router, RouterConfig};

#[test]
fn test_end_to_end_routing() {
    let config = RouterConfig {
        dimension: 384,
        default_threshold: 0.5,
        top_k: 3,
        enable_calibration: true,
    };

    let mut router = Router::new(config);

    // Add routes
    let route1 = Route {
        id: "billing".into(),
        description: "Billing questions".into(),
        examples: vec!["Where's my invoice?".into()],
        keywords: vec!["invoice".into(), "payment".into()],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    let route2 = Route {
        id: "support".into(),
        description: "Technical support".into(),
        examples: vec!["App is broken".into()],
        keywords: vec!["bug".into(), "error".into()],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    router.add_route(route1).unwrap();
    router.add_route(route2).unwrap();

    // Build index with dummy embeddings
    let embeddings = vec![vec![1.0; 384], vec![0.5; 384]];

    router.build_index(embeddings).unwrap();

    // Route query
    let query_embedding = vec![0.9; 384];
    let result = router.route("I need my invoice", &query_embedding).unwrap();

    assert_eq!(result.route_id, "billing");
    assert!(result.scores.confidence > 0.0);
    assert!(result.scores.confidence <= 1.0);
    assert!(result.latency_ms > 0);
}

#[test]
fn test_empty_routes() {
    let mut router = Router::new(RouterConfig::default());
    let embeddings: Vec<Vec<f32>> = vec![];

    let result = router.build_index(embeddings);
    assert!(result.is_err());
}

#[test]
fn test_dimension_mismatch() {
    let config = RouterConfig {
        dimension: 384,
        ..Default::default()
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

    // Wrong dimension
    let embeddings = vec![vec![1.0; 256]];
    let result = router.build_index(embeddings);

    assert!(result.is_err());
}

#[test]
fn test_multiple_routes() {
    let mut router = Router::new(RouterConfig::default());

    for i in 0..10 {
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

    let embeddings: Vec<Vec<f32>> = (0..10).map(|_| vec![0.5; 384]).collect();

    router.build_index(embeddings).unwrap();

    let result = router.route("test", &vec![0.6; 384]).unwrap();
    assert!(!result.route_id.is_empty());
}

#[test]
fn test_confidence_bounds() {
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

    let result = router.route("test query", &vec![0.6; 384]).unwrap();
    assert!(result.scores.confidence >= 0.0);
    assert!(result.scores.confidence <= 1.0);
}

#[test]
fn test_routing_without_index() {
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

    let result = router.route("test", &vec![0.5; 384]);
    assert!(result.is_err());
}
