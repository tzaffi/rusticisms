use sublist::{sublist, Comparison};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sublist_benchmark(c: &mut Criterion) {
    let v1: Vec<u64> = black_box((10..1_000_001).collect());
    let v2: Vec<u64> = black_box((1..1_000_000).collect());

    c.bench_function("huge_sublist_not_in_huge_list", |b| {
        b.iter(|| Comparison::Unequal == sublist(&v1, &v2));
    });
}

criterion_group!(benches, sublist_benchmark);
criterion_main!(benches);
