//! Performance benchmarks for StrataRouter

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;
use stratarouter_core::{Route, Router, RouterConfig};

fn create_router(num_routes: usize, dimension: usize) -> Router {
    let config = RouterConfig {
        dimension,
        default_threshold: 0.5,
        top_k: 5,
        enable_calibration: true,
    };

    let mut router = Router::new(config);

    for i in 0..num_routes {
        let route = Route {
            id: format!("route_{}", i),
            description: format!("Test route {}", i),
            examples: vec![format!("example {}", i)],
            keywords: vec![format!("keyword{}", i)],
            patterns: vec![],
            metadata: HashMap::new(),
            threshold: None,
            tags: vec![],
        };
        router.add_route(route).unwrap();
    }

    let embeddings: Vec<Vec<f32>> = (0..num_routes)
        .map(|i| {
            (0..dimension)
                .map(|j| ((i * dimension + j) as f32 * 0.01).sin())
                .collect()
        })
        .collect();

    router.build_index(embeddings).unwrap();
    router
}

fn benchmark_routing(c: &mut Criterion) {
    let mut group = c.benchmark_group("routing");

    for num_routes in [10, 50, 100, 500].iter() {
        let mut router = create_router(*num_routes, 384);
        let query_embedding: Vec<f32> = (0..384).map(|i| (i as f32 * 0.01).cos()).collect();

        group.bench_with_input(
            BenchmarkId::from_parameter(num_routes),
            num_routes,
            |bencher, _| {
                bencher.iter(|| {
                    router
                        .route(black_box("test query"), black_box(&query_embedding))
                        .unwrap()
                })
            },
        );
    }

    group.finish();
}

fn benchmark_index_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("index_build");

    for num_routes in [10, 50, 100].iter() {
        let embeddings: Vec<Vec<f32>> = (0..*num_routes)
            .map(|i| {
                (0..384)
                    .map(|j| ((i * 384 + j) as f32 * 0.01).sin())
                    .collect()
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::from_parameter(num_routes),
            &embeddings,
            |bencher, embs| {
                bencher.iter(|| {
                    let config = RouterConfig {
                        dimension: 384,
                        ..Default::default()
                    };
                    let mut router = Router::new(config);

                    // Add dummy routes with description/examples to match embeddings
                    for i in 0..embs.len() {
                        let route = Route {
                            id: format!("route_{}", i),
                            description: format!("Route {}", i),
                            examples: vec![format!("example {}", i)],
                            keywords: vec![],
                            patterns: vec![],
                            metadata: HashMap::new(),
                            threshold: None,
                            tags: vec![],
                        };
                        router.add_route(route).unwrap();
                    }

                    router.build_index(black_box(embs.clone())).unwrap()
                })
            },
        );
    }

    group.finish();
}

fn benchmark_sparse_scoring(c: &mut Criterion) {
    use stratarouter_core::algorithms::HybridScorer;

    let scorer = HybridScorer::new();
    let route = Route {
        id: "test".into(),
        description: "".into(),
        examples: vec![],
        keywords: vec!["invoice".into(), "payment".into(), "billing".into()],
        patterns: vec![],
        metadata: HashMap::new(),
        threshold: None,
        tags: vec![],
    };

    c.bench_function("sparse_scoring", |bencher| {
        bencher.iter(|| {
            scorer.compute_sparse_score(
                black_box("I need my invoice for the payment"),
                black_box(&route),
            )
        })
    });
}

criterion_group!(
    benches,
    benchmark_routing,
    benchmark_index_build,
    benchmark_sparse_scoring
);
criterion_main!(benches);
