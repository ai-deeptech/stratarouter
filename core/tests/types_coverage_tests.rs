//! Additional tests for types.rs to improve coverage

use std::collections::HashMap;
use stratarouter_core::{Route, RouteResult, RouteScores};

#[test]
fn test_route_new() {
    let route = Route::new("test_route");
    assert_eq!(route.id, "test_route");
    assert!(route.description.is_empty());
}

#[test]
fn test_route_validation_valid() {
    let mut route = Route::new("valid");
    route.examples.push("example".into());
    assert!(route.validate().is_ok());
}

#[test]
fn test_route_validation_with_description() {
    let mut route = Route::new("valid");
    route.description = "Test description".into();
    assert!(route.validate().is_ok());
}

#[test]
fn test_route_validation_empty_id() {
    let route = Route::new("");
    assert!(route.validate().is_err());
}

#[test]
fn test_route_validation_no_examples_or_description() {
    let route = Route::new("test");
    assert!(route.validate().is_err());
}

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
fn test_route_scores_creation() {
    let scores = RouteScores {
        semantic: 0.8,
        keyword: 0.6,
        pattern: 0.7,
        total: 0.75,
        confidence: 0.82,
    };

    assert_eq!(scores.semantic, 0.8);
    assert_eq!(scores.keyword, 0.6);
    assert_eq!(scores.pattern, 0.7);
    assert_eq!(scores.total, 0.75);
    assert_eq!(scores.confidence, 0.82);
}

#[test]
fn test_route_scores_debug() {
    let scores = RouteScores {
        semantic: 0.8,
        keyword: 0.6,
        pattern: 0.7,
        total: 0.75,
        confidence: 0.82,
    };

    let debug_str = format!("{:?}", scores);
    assert!(debug_str.contains("semantic"));
    assert!(debug_str.contains("0.8"));
}

#[test]
fn test_route_scores_clone() {
    let scores = RouteScores {
        semantic: 0.8,
        keyword: 0.6,
        pattern: 0.7,
        total: 0.75,
        confidence: 0.82,
    };

    let cloned = scores.clone();
    assert_eq!(cloned.semantic, scores.semantic);
    assert_eq!(cloned.keyword, scores.keyword);
    assert_eq!(cloned.confidence, scores.confidence);
}

#[test]
fn test_route_result_creation() {
    let result = RouteResult {
        route_id: "test_route".into(),
        scores: RouteScores::zero(),
        latency_ms: 5,
        metadata: std::collections::HashMap::new(),
    };

    assert_eq!(result.route_id, "test_route");
    assert_eq!(result.latency_ms, 5);
}

#[test]
fn test_route_result_debug() {
    let result = RouteResult {
        route_id: "test_route".into(),
        scores: RouteScores::zero(),
        latency_ms: 5,
        metadata: HashMap::new(),
    };

    let debug_str = format!("{:?}", result);
    assert!(debug_str.contains("test_route"));
}

#[test]
fn test_route_metadata_operations() {
    let mut route = Route::new("test");
    route.examples.push("test".into()); // Make valid

    // Add metadata
    route.metadata.insert("category".into(), "billing".into());
    route.metadata.insert("priority".into(), "high".into());

    assert_eq!(route.metadata.len(), 2);
    assert_eq!(route.metadata.get("category").unwrap(), "billing");

    // Update metadata
    route.metadata.insert("priority".into(), "medium".into());
    assert_eq!(route.metadata.get("priority").unwrap(), "medium");
}

#[test]
fn test_route_tags() {
    let mut route = Route::new("test");
    route.examples.push("test".into());
    route.tags = vec!["important".into(), "customer_facing".into()];

    assert_eq!(route.tags.len(), 2);
    assert!(route.tags.contains(&"important".into()));
    assert!(route.tags.contains(&"customer_facing".into()));
}

#[test]
fn test_route_patterns() {
    let mut route = Route::new("test");
    route.examples.push("test".into());
    route.patterns = vec!["pattern1".into(), "pattern2".into()];

    assert_eq!(route.patterns.len(), 2);
    assert_eq!(route.patterns[0], "pattern1");
}

#[test]
fn test_route_keywords() {
    let mut route = Route::new("test");
    route.examples.push("test".into());
    route.keywords = vec!["key1".into(), "key2".into()];

    assert_eq!(route.keywords.len(), 2);
    assert_eq!(route.keywords[0], "key1");
}

#[test]
fn test_route_examples() {
    let mut route = Route::new("test");
    route.examples = vec!["example1".into(), "example2".into()];

    assert_eq!(route.examples.len(), 2);
    assert!(route.validate().is_ok());
}

#[test]
fn test_route_threshold() {
    let mut route = Route::new("test");
    route.examples.push("test".into());
    route.threshold = Some(0.75);

    assert_eq!(route.threshold, Some(0.75));
}

#[test]
fn test_route_clone() {
    let mut route = Route::new("test");
    route.description = "desc".into();
    route.examples.push("test".into());
    route.threshold = Some(0.8);

    let cloned = route.clone();
    assert_eq!(cloned.id, route.id);
    assert_eq!(cloned.description, route.description);
    assert_eq!(cloned.threshold, route.threshold);
}

#[test]
fn test_route_empty_fields() {
    let route = Route::new("test");
    assert!(route.examples.is_empty());
    assert!(route.keywords.is_empty());
    assert!(route.patterns.is_empty());
    assert!(route.metadata.is_empty());
    assert!(route.tags.is_empty());
    assert_eq!(route.threshold, None);
}

#[test]
fn test_route_description() {
    let mut route = Route::new("test");
    route.description = "Test description".into();

    assert_eq!(route.description, "Test description");
    assert!(route.validate().is_ok());
}

#[test]
fn test_route_with_all_fields() {
    let mut metadata = HashMap::new();
    metadata.insert("key".to_string(), "value".to_string());

    let route = Route {
        id: "full_route".into(),
        description: "Full route test".into(),
        examples: vec!["ex1".into(), "ex2".into()],
        keywords: vec!["kw1".into()],
        patterns: vec!["pat1".into()],
        metadata,
        threshold: Some(0.9),
        tags: vec!["tag1".into()],
    };

    assert!(route.validate().is_ok());
    assert_eq!(route.id, "full_route");
    assert_eq!(route.examples.len(), 2);
    assert_eq!(route.keywords.len(), 1);
    assert_eq!(route.patterns.len(), 1);
    assert_eq!(route.metadata.len(), 1);
    assert_eq!(route.tags.len(), 1);
    assert_eq!(route.threshold, Some(0.9));
}
