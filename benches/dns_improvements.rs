use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dotchk::Pattern;
use std::sync::atomic::{AtomicU16, Ordering};

fn benchmark_transaction_id_generation(c: &mut Criterion) {
    c.bench_function("transaction_id_generation", |b| {
        let counter = AtomicU16::new(1);
        b.iter(|| {
            let id = counter.fetch_add(1, Ordering::Relaxed);
            if id == 0 {
                counter.store(1, Ordering::Relaxed);
            }
            black_box(id);
        });
    });
}

fn benchmark_domain_grouping(c: &mut Criterion) {
    c.bench_function("group_domains_by_tld", |b| {
        let domains: Vec<String> = (0..1000)
            .flat_map(|i| {
                vec![
                    format!("test{}.com", i),
                    format!("test{}.net", i),
                    format!("test{}.org", i),
                    format!("test{}.io", i),
                ]
            })
            .collect();

        b.iter(|| {
            let mut grouped: std::collections::HashMap<String, Vec<String>> =
                std::collections::HashMap::new();
            for domain in &domains {
                let tld = domain.split('.').next_back().unwrap_or("").to_string();
                grouped.entry(tld).or_default().push(domain.clone());
            }
            black_box(grouped);
        });
    });
}

fn benchmark_batch_preparation(c: &mut Criterion) {
    c.bench_function("prepare_dns_batch", |b| {
        let domains: Vec<String> = (0..256).map(|i| format!("test-domain-{i}.com")).collect();

        b.iter(|| {
            let mut batch = Vec::with_capacity(domains.len());
            for (i, domain) in domains.iter().enumerate() {
                // Simulate DNS query packet building
                let mut packet = Vec::with_capacity(512);
                packet.extend_from_slice(&(i as u16).to_be_bytes()); // Transaction ID
                packet.extend_from_slice(&[0x01, 0x00]); // Flags
                packet.extend_from_slice(&[0x00, 0x01]); // Questions: 1
                packet.extend_from_slice(&[0x00, 0x00]); // Answer RRs: 0
                packet.extend_from_slice(&[0x00, 0x00]); // Authority RRs: 0
                packet.extend_from_slice(&[0x00, 0x00]); // Additional RRs: 0

                // Add domain labels
                for label in domain.split('.') {
                    packet.push(label.len() as u8);
                    packet.extend_from_slice(label.as_bytes());
                }
                packet.push(0x00); // End of domain

                packet.extend_from_slice(&[0x00, 0x02]); // Type: NS
                packet.extend_from_slice(&[0x00, 0x01]); // Class: IN

                batch.push(packet);
            }
            black_box(batch);
        });
    });
}

fn benchmark_pattern_generation_count(c: &mut Criterion) {
    let mut group = c.benchmark_group("pattern_generation");

    group.bench_function("simple_pattern_count", |b| {
        b.iter(|| {
            let pattern = Pattern::compile("test-[a-z]{4}.com").unwrap();
            let domains = pattern.generate(Some(100));
            black_box(domains.len());
        });
    });

    group.bench_function("medium_pattern_count", |b| {
        b.iter(|| {
            let pattern = Pattern::compile("app-[a-z]{2}[0-9]{2}.com").unwrap();
            let domains = pattern.generate(Some(100));
            black_box(domains.len());
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_transaction_id_generation,
    benchmark_domain_grouping,
    benchmark_batch_preparation,
    benchmark_pattern_generation_count
);
criterion_main!(benches);
