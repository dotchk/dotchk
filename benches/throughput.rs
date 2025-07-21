use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use dotchk::{Checker, Pattern};
use tokio::runtime::Runtime;

fn benchmark_single_check(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("single_domain_check", |b| {
        b.iter(|| {
            rt.block_on(async {
                let checker = Checker::builder()
                    .max_parallel(1)
                    .timeout_ms(1000)
                    .build()
                    .await
                    .unwrap();

                black_box(checker.check("test-example.com").await)
            })
        })
    });
}

fn benchmark_parallel_checks(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("parallel_checks");

    for parallel in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(parallel),
            parallel,
            |b, &parallel| {
                b.iter(|| {
                    rt.block_on(async {
                        let checker = Checker::builder()
                            .max_parallel(parallel)
                            .timeout_ms(1000)
                            .build()
                            .await
                            .unwrap();

                        let domains: Vec<String> =
                            (0..100).map(|i| format!("test-{i}.com")).collect();

                        black_box(checker.check_batch(domains).await)
                    })
                })
            },
        );
    }

    group.finish();
}

fn benchmark_pattern_generation(c: &mut Criterion) {
    c.bench_function("pattern_generation_simple", |b| {
        b.iter(|| {
            let pattern = Pattern::compile("test-[a-z]{4}.com").unwrap();
            black_box(pattern.generate(Some(1000)))
        })
    });

    c.bench_function("pattern_generation_complex", |b| {
        b.iter(|| {
            let pattern = Pattern::compile("app-[a-z]{2}[0-9]{2}.com").unwrap();
            black_box(pattern.generate(Some(100)))
        })
    });
}

fn benchmark_dns_parsing(c: &mut Criterion) {
    c.bench_function("dns_query_building", |b| {
        b.iter(|| {
            for i in 0..100 {
                let domain = format!("test-{i}.com");
                // Benchmark domain validation
                black_box(domain.split('.').count());
            }
        })
    });
}

fn benchmark_tld_lookup(c: &mut Criterion) {
    use dotchk::tld::get_tld_info;

    c.bench_function("tld_lookup", |b| {
        b.iter(|| {
            black_box(get_tld_info("example.com"));
            black_box(get_tld_info("test.xyz"));
            black_box(get_tld_info("app.io"));
            black_box(get_tld_info("startup.dev"));
            black_box(get_tld_info("company.tech"));
        })
    });
}

criterion_group!(
    benches,
    benchmark_single_check,
    benchmark_parallel_checks,
    benchmark_pattern_generation,
    benchmark_dns_parsing,
    benchmark_tld_lookup
);
criterion_main!(benches);
