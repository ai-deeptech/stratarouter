use std::collections::HashMap;
use stratarouter_core::{Route, Router, RouterConfig};

#[test]
fn test_end_to_end_routing() {
    let config = RouterConfig {
        dimension: 3,
        ..Default::default()
    };
    let mut router = Router::new(config);

    let billing = Route {
        id: "billing".into(),
        description: "Billing questions".into(),
        examples: vec!["invoice".into()],
        keywords: vec!["billing".into()],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };
    let support = Route {
        id: "support".into(),
        description: "Support questions".into(),
        examples: vec!["help".into()],
        keywords: vec!["support".into()],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    router.add_route(billing).unwrap();
    router.add_route(support).unwrap();

    // Embeddings must be supplied in insertion order: billing first, support second.
    router
        .build_index(vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0]])
        .unwrap();

    let result = router.route("billing question", &[1.0, 0.0, 0.0]).unwrap();
    assert_eq!(result.route_id, "billing");
    assert!(result.scores.confidence > 0.0);
}

#[test]
fn test_routing_no_index_returns_error() {
    use stratarouter_core::Error;

    let config = RouterConfig {
        dimension: 3,
        ..Default::default()
    };
    let mut router = Router::new(config);

    let route = Route {
        id: "test".into(),
        description: "Test".into(),
        examples: vec!["example".into()],
        keywords: vec![],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };
    router.add_route(route).unwrap();

    // Index not built yet — must return IndexNotBuilt error.
    let err = router.route("test", &[1.0, 0.0, 0.0]).unwrap_err();
    assert!(matches!(err, Error::IndexNotBuilt));
}
