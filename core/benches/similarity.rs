use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use stratarouter_core::algorithms::vector_ops::cosine_similarity as cosine_similarity_inner;

fn bench_cosine_similarity(c: &mut Criterion) {
    let mut group = c.benchmark_group("cosine_similarity");

    // Different vector sizes
    for size in [128, 384, 768, 1024, 1536].iter() {
        let a: Vec<f32> = (0..*size).map(|i| (i as f32).sin()).collect();
        let b: Vec<f32> = (0..*size).map(|i| (i as f32).cos()).collect();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |bench, _| {
            bench.iter(|| cosine_similarity_inner(black_box(&a), black_box(&b)));
        });
    }

    group.finish();
}

fn bench_batch_similarity(c: &mut Criterion) {
    let query: Vec<f32> = (0..384).map(|i| (i as f32).sin()).collect();
    let embeddings: Vec<Vec<f32>> = (0..100)
        .map(|j| (0..384).map(|i| ((i + j) as f32).cos()).collect())
        .collect();

    c.bench_function("batch_100x384", |b| {
        b.iter(|| {
            for emb in &embeddings {
                cosine_similarity_inner(black_box(&query), black_box(emb));
            }
        });
    });
}

criterion_group!(benches, bench_cosine_similarity, bench_batch_similarity);
criterion_main!(benches);
