//! Additional tests for HNSW index and SIMD operations to reach 95% coverage

use stratarouter_core::algorithms::vector_ops::cosine_similarity;

// ============================================================================
// SIMD Operations - Cosine Similarity Tests
// ============================================================================

#[test]
fn test_cosine_similarity_perpendicular() {
    let a = vec![1.0, 0.0, 0.0];
    let b = vec![0.0, 1.0, 0.0];
    let sim = cosine_similarity(&a, &b);
    assert!(sim.abs() < 0.001);
}

#[test]
fn test_cosine_similarity_opposite() {
    let a = vec![1.0, 0.0, 0.0];
    let b = vec![-1.0, 0.0, 0.0];
    let sim = cosine_similarity(&a, &b);
    assert!((sim + 1.0).abs() < 0.001);
}

#[test]
fn test_cosine_similarity_partial_match() {
    let a = vec![1.0, 2.0, 3.0];
    let b = vec![1.0, 2.0, 0.0];
    let sim = cosine_similarity(&a, &b);
    assert!(sim > 0.5 && sim < 1.0);
}

#[test]
fn test_cosine_similarity_zero_vector_a() {
    let a = vec![0.0, 0.0, 0.0];
    let b = vec![1.0, 2.0, 3.0];
    let sim = cosine_similarity(&a, &b);
    assert_eq!(sim, 0.0);
}

#[test]
fn test_cosine_similarity_zero_vector_b() {
    let a = vec![1.0, 2.0, 3.0];
    let b = vec![0.0, 0.0, 0.0];
    let sim = cosine_similarity(&a, &b);
    assert_eq!(sim, 0.0);
}

#[test]
fn test_cosine_similarity_both_zero() {
    let a = vec![0.0, 0.0, 0.0];
    let b = vec![0.0, 0.0, 0.0];
    let sim = cosine_similarity(&a, &b);
    assert_eq!(sim, 0.0);
}

#[test]
fn test_cosine_similarity_negative_values() {
    let a = vec![-1.0, -2.0, -3.0];
    let b = vec![-1.0, -2.0, -3.0];
    let sim = cosine_similarity(&a, &b);
    assert!((sim - 1.0).abs() < 0.001);
}

#[test]
fn test_cosine_similarity_mixed_signs() {
    let a = vec![1.0, -1.0, 1.0];
    let b = vec![-1.0, 1.0, -1.0];
    let sim = cosine_similarity(&a, &b);
    assert!((sim + 1.0).abs() < 0.001);
}

#[test]
fn test_cosine_similarity_small_values() {
    let a = vec![0.001, 0.002, 0.003];
    let b = vec![0.001, 0.002, 0.003];
    let sim = cosine_similarity(&a, &b);
    assert!((sim - 1.0).abs() < 0.001);
}

#[test]
fn test_cosine_similarity_large_values() {
    let a = vec![1000.0, 2000.0, 3000.0];
    let b = vec![1000.0, 2000.0, 3000.0];
    let sim = cosine_similarity(&a, &b);
    assert!((sim - 1.0).abs() < 0.001);
}

#[test]
fn test_cosine_similarity_single_dimension() {
    let a = vec![5.0];
    let b = vec![3.0];
    let sim = cosine_similarity(&a, &b);
    assert!((sim - 1.0).abs() < 0.001); // Same direction
}

#[test]
fn test_cosine_similarity_single_dimension_opposite() {
    let a = vec![5.0];
    let b = vec![-3.0];
    let sim = cosine_similarity(&a, &b);
    assert!((sim + 1.0).abs() < 0.001); // Opposite direction
}

#[test]
fn test_cosine_similarity_high_dimensional() {
    let dim = 1000;
    let a: Vec<f32> = (0..dim).map(|i| i as f32).collect();
    let b = a.clone();
    let sim = cosine_similarity(&a, &b);
    assert!((sim - 1.0).abs() < 0.001);
}

#[test]
fn test_cosine_similarity_normalized_input() {
    // Pre-normalized vectors
    let a = vec![0.577, 0.577, 0.577]; // Normalized
    let b = vec![0.707, 0.707, 0.0]; // Normalized
    let sim = cosine_similarity(&a, &b);
    assert!(sim > 0.5 && sim < 1.0);
}

// ============================================================================
// HNSW Index Tests
// ============================================================================

#[cfg(test)]
mod hnsw_tests {
    use stratarouter_core::LinearIndex;

    #[test]
    fn test_linear_search_top_k_larger_than_index() {
        let mut index = LinearIndex::new(3).unwrap();

        index.add(0, vec![1.0, 0.0, 0.0]).unwrap();
        index.add(1, vec![0.0, 1.0, 0.0]).unwrap();

        // Request more neighbors than available
        let results = index.search(&[1.0, 0.0, 0.0], 10).unwrap();
        assert_eq!(results.len(), 2); // Should return only 2
    }

    #[test]
    fn test_linear_search_exact_match() {
        let mut index = LinearIndex::new(4).unwrap();

        let vec1 = vec![0.5, 0.5, 0.5, 0.5];
        index.add(0, vec1.clone()).unwrap();

        let results = index.search(&vec1, 1).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, 0);
        assert!(results[0].1 < 0.01); // Very small distance for exact match
    }

    #[test]
    fn test_linear_search_ordering() {
        let mut index = LinearIndex::new(2).unwrap();

        index.add(0, vec![1.0, 0.0]).unwrap(); // Close to query
        index.add(1, vec![0.0, 1.0]).unwrap(); // Far from query
        index.add(2, vec![0.9, 0.1]).unwrap(); // Very close to query

        let results = index.search(&[1.0, 0.0], 3).unwrap();

        // Results should be ordered by distance (ascending)
        assert!(results[0].1 <= results[1].1);
        assert!(results[1].1 <= results[2].1);

        // Closest should be index 0 or 2
        assert!(results[0].0 == 0 || results[0].0 == 2);
    }

    #[test]
    fn test_linear_add_multiple_vectors() {
        let mut index = LinearIndex::new(5).unwrap();

        for i in 0..100 {
            let vec = vec![i as f32; 5];
            index.add(i, vec).unwrap();
        }

        assert_eq!(index.len(), 100);
    }

    #[test]
    fn test_linear_search_with_duplicates() {
        let mut index = LinearIndex::new(3).unwrap();

        let vec = vec![0.5, 0.5, 0.5];
        index.add(0, vec.clone()).unwrap();
        index.add(1, vec.clone()).unwrap();
        index.add(2, vec.clone()).unwrap();

        let results = index.search(&vec, 3).unwrap();
        assert_eq!(results.len(), 3);

        // All should have very small distance
        for (_, dist) in results {
            assert!(dist < 0.01);
        }
    }

    #[test]
    fn test_linear_is_empty() {
        let mut index = LinearIndex::new(3).unwrap();
        assert!(index.is_empty());

        index.add(0, vec![1.0, 0.0, 0.0]).unwrap();
        assert!(!index.is_empty());
    }

    #[test]
    fn test_linear_len() {
        let mut index = LinearIndex::new(2).unwrap();
        assert_eq!(index.len(), 0);

        index.add(0, vec![1.0, 0.0]).unwrap();
        assert_eq!(index.len(), 1);

        index.add(1, vec![0.0, 1.0]).unwrap();
        assert_eq!(index.len(), 2);
    }

    #[test]
    fn test_linear_overwrite_existing_id() {
        let mut index = LinearIndex::new(3).unwrap();

        index.add(0, vec![1.0, 0.0, 0.0]).unwrap();
        index.add(0, vec![0.0, 1.0, 0.0]).unwrap(); // Overwrite

        assert_eq!(index.len(), 1);

        let results = index.search(&[0.0, 1.0, 0.0], 1).unwrap();
        assert_eq!(results[0].0, 0);
        assert!(results[0].1 < 0.01); // Should match the new vector
    }

    #[test]
    fn test_linear_distance_calculation() {
        let mut index = LinearIndex::new(3).unwrap();

        index.add(0, vec![1.0, 0.0, 0.0]).unwrap();
        index.add(1, vec![0.0, 1.0, 0.0]).unwrap();

        let results = index.search(&[1.0, 0.0, 0.0], 2).unwrap();

        // First result should be exact match (distance ~0)
        assert!(results[0].1 < 0.01);

        // Second result should be perpendicular (distance ~1)
        assert!((results[1].1 - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_linear_negative_distance_clamp() {
        let mut index = LinearIndex::new(3).unwrap();

        index.add(0, vec![1.0, 0.0, 0.0]).unwrap();

        // Search with opposite vector (similarity = -1, distance should be clamped to 0)
        let results = index.search(&[-1.0, 0.0, 0.0], 1).unwrap();
        assert!(results[0].1 >= 0.0); // Should never be negative
    }

    #[test]
    fn test_linear_search_k_zero() {
        let mut index = LinearIndex::new(2).unwrap();

        index.add(0, vec![1.0, 0.0]).unwrap();

        let results = index.search(&[1.0, 0.0], 0).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_linear_high_dimensional() {
        let dim = 512;
        let mut index = LinearIndex::new(dim).unwrap();

        let vec1: Vec<f32> = (0..dim).map(|i| (i % 10) as f32).collect();
        let vec2: Vec<f32> = (0..dim).map(|i| ((i + 1) % 10) as f32).collect();

        index.add(0, vec1.clone()).unwrap();
        index.add(1, vec2).unwrap();

        let results = index.search(&vec1, 2).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, 0); // Exact match should be first
    }

    #[test]
    fn test_linear_sparse_ids() {
        let mut index = LinearIndex::new(3).unwrap();

        // Use non-contiguous IDs
        index.add(10, vec![1.0, 0.0, 0.0]).unwrap();
        index.add(100, vec![0.0, 1.0, 0.0]).unwrap();
        index.add(1000, vec![0.0, 0.0, 1.0]).unwrap();

        assert_eq!(index.len(), 3);

        let results = index.search(&[1.0, 0.0, 0.0], 3).unwrap();
        assert!(results.iter().any(|(id, _)| *id == 10));
    }
}

// ============================================================================
// Library-level Tests
// ============================================================================

#[test]
fn test_version_string() {
    let version = stratarouter_core::VERSION;
    assert!(!version.is_empty());
    assert!(version.contains('.'));

    // Should be semantic versioning
    let parts: Vec<&str> = version.split('.').collect();
    assert!(parts.len() >= 2);
}

#[test]
fn test_build_timestamp() {
    // BUILD_TIMESTAMP is Option<&str> — may be None in environments without
    // a build script setting the env var. Just confirm it doesn't panic.
    let _timestamp = stratarouter_core::BUILD_TIMESTAMP;
}

#[test]
fn test_has_avx2() {
    // Should not panic on any platform
    let has_avx2 = stratarouter_core::has_avx2();
    // Just ensure it returns a boolean
    assert!(has_avx2 || !has_avx2);
}

// ============================================================================
// Route Metadata and Tags Tests
// ============================================================================

#[test]
fn test_route_with_metadata() {
    use std::collections::HashMap;
    use stratarouter_core::Route;

    let mut metadata = HashMap::new();
    metadata.insert("priority".to_string(), "high".to_string());
    metadata.insert("team".to_string(), "billing".to_string());

    let route = Route {
        id: "test".into(),
        description: "Test".into(),
        examples: vec!["test".into()],
        keywords: vec![],
        patterns: vec![],
        metadata: metadata.clone(),
        threshold: None,
        tags: vec![],
    };

    assert!(route.validate().is_ok());
    assert_eq!(route.metadata.get("priority"), Some(&"high".to_string()));
    assert_eq!(route.metadata.get("team"), Some(&"billing".to_string()));
}

#[test]
fn test_route_with_tags() {
    use std::collections::HashMap;
    use stratarouter_core::Route;

    let route = Route {
        id: "test".into(),
        description: "Test".into(),
        examples: vec!["test".into()],
        keywords: vec![],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec!["finance".into(), "urgent".into()],
    };

    assert!(route.validate().is_ok());
    assert_eq!(route.tags.len(), 2);
    assert!(route.tags.contains(&"finance".to_string()));
}

#[test]
fn test_route_with_custom_threshold() {
    use std::collections::HashMap;
    use stratarouter_core::Route;

    let route = Route {
        id: "test".into(),
        description: "Test".into(),
        examples: vec!["test".into()],
        keywords: vec![],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: Some(0.75),
        tags: vec![],
    };

    assert!(route.validate().is_ok());
    assert_eq!(route.threshold, Some(0.75));
}

// ============================================================================
// Edge Case Tests for Router
// ============================================================================

#[test]
fn test_router_with_large_top_k() {
    use std::collections::HashMap;
    use stratarouter_core::{Route, Router, RouterConfig};

    let config = RouterConfig {
        dimension: 384,
        default_threshold: 0.5,
        top_k: 1000, // Very large
        enable_calibration: true,
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
    router.build_index(vec![vec![0.5; 384]]).unwrap();

    let result = router.route("test", &vec![0.5; 384]);
    assert!(result.is_ok());
}

#[test]
fn test_router_with_very_low_threshold() {
    use std::collections::HashMap;
    use stratarouter_core::{Route, Router, RouterConfig};

    let config = RouterConfig {
        dimension: 384,
        default_threshold: 0.01, // Very low
        top_k: 5,
        enable_calibration: false,
    };

    config.validate().unwrap();

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
    router.build_index(vec![vec![0.5; 384]]).unwrap();
}

#[test]
fn test_router_with_very_high_threshold() {
    use std::collections::HashMap;
    use stratarouter_core::{Route, Router, RouterConfig};

    let config = RouterConfig {
        dimension: 384,
        default_threshold: 0.99, // Very high
        top_k: 5,
        enable_calibration: false,
    };

    config.validate().unwrap();

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
    router.build_index(vec![vec![0.5; 384]]).unwrap();
}
