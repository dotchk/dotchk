use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use dotchk::Checker;
use tokio::runtime::Runtime;

fn generate_test_domains(count: usize) -> Vec<String> {
    (0..count).map(|i| format!("test-domain-{i}.com")).collect()
}

fn benchmark_batch_sizes(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("batch_sizes");

    for size in [10, 50, 100, 500].iter() {
        let domains = generate_test_domains(*size);

        group.bench_with_input(BenchmarkId::new("domains", size), size, |b, _| {
            b.iter(|| {
                rt.block_on(async {
                    let checker = Checker::builder()
                        .max_parallel(100)
                        .timeout_ms(5000)
                        .build()
                        .await
                        .unwrap();

                    let results = checker.check_batch(black_box(domains.clone())).await;
                    black_box(results);
                })
            });
        });
    }

    group.finish();
}

fn benchmark_throughput(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("throughput");
    group.sample_size(10);

    // Test with a larger batch to measure throughput
    let domains = generate_test_domains(1000);

    group.bench_function("domains_1000", |b| {
        b.iter(|| {
            rt.block_on(async {
                let checker = Checker::builder()
                    .max_parallel(200)
                    .timeout_ms(5000)
                    .build()
                    .await
                    .unwrap();

                let results = checker.check_batch(black_box(domains.clone())).await;
                black_box(results);
            })
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_batch_sizes, benchmark_throughput);
criterion_main!(benches);
