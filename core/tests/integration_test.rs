use stratarouter_core::{cosine_similarity, cosine_similarity_batch, Route, Router};

#[test]
fn test_end_to_end_routing() {
    let router = Router::new(3, 1000);

    let route1 = Route::new("billing".to_string(), vec![vec![1.0, 0.0, 0.0]], 0.7).unwrap();

    let route2 = Route::new("support".to_string(), vec![vec![0.0, 1.0, 0.0]], 0.7).unwrap();

    router.add(route1).unwrap();
    router.add(route2).unwrap();

    let matches = router.route(vec![1.0, 0.0, 0.0]).unwrap();
    assert!(!matches.is_empty());
    assert_eq!(matches[0].name, "billing");
    assert!(matches[0].score > 0.9);
}

#[test]
fn test_similarity_functions() {
    let a = vec![1.0, 0.0, 0.0];
    let b = vec![1.0, 0.0, 0.0];

    let sim = cosine_similarity(a.clone(), b).unwrap();
    assert!((sim - 1.0).abs() < 1e-6);

    let embeddings = vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0]];

    let results = cosine_similarity_batch(a, embeddings).unwrap();
    assert_eq!(results.len(), 2);
}
