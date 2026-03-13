use stratarouter_core::*;

#[test]
fn test_route_validation() {
    let mut route = Route::new("test");
    assert!(route.validate().is_err()); // No examples/description

    route.examples.push("test".into());
    assert!(route.validate().is_ok());
}

#[test]
fn test_route_validation_with_description() {
    let mut route = Route::new("test");
    route.description = "Test route".into();
    assert!(route.validate().is_ok());
}

#[test]
fn test_route_id_empty() {
    let route = Route::new("");
    assert!(route.validate().is_err());
}

#[test]
fn test_router_config_validation() {
    let config = RouterConfig::default();
    assert!(config.validate().is_ok());

    let invalid = RouterConfig {
        dimension: 0,
        ..Default::default()
    };
    assert!(invalid.validate().is_err());

    let invalid_threshold = RouterConfig {
        default_threshold: 1.5,
        ..Default::default()
    };
    assert!(invalid_threshold.validate().is_err());

    let invalid_top_k = RouterConfig {
        top_k: 0,
        ..Default::default()
    };
    assert!(invalid_top_k.validate().is_err());
}

#[test]
fn test_error_types() {
    let err = Error::InvalidInput {
        message: "test".into(),
    };
    assert!(err.is_recoverable());

    let err = Error::IndexNotBuilt;
    assert!(err.is_recoverable());

    let err = Error::NoRoutes;
    assert!(err.is_recoverable());
}

#[test]
fn test_route_scores() {
    let scores = RouteScores::zero();
    assert_eq!(scores.semantic, 0.0);
    assert_eq!(scores.keyword, 0.0);
    assert_eq!(scores.pattern, 0.0);
    assert_eq!(scores.total, 0.0);
    assert_eq!(scores.confidence, 0.0);
}

#[test]
fn test_error_display() {
    let err = Error::DimensionMismatch {
        expected: 384,
        actual: 256,
    };
    let display = format!("{}", err);
    assert!(display.contains("384"));
    assert!(display.contains("256"));
}

#[test]
fn test_route_new() {
    let route = Route::new("test_route");
    assert_eq!(route.id, "test_route");
    assert!(route.examples.is_empty());
    assert!(route.keywords.is_empty());
}

#[test]
fn test_router_creation() {
    let config = RouterConfig::default();
    let router = Router::new(config);
    assert_eq!(router.route_count(), 0);
    assert!(!router.is_index_built());
}
