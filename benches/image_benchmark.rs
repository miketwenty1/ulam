use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ulam::ulamspiral_img::generate;

pub fn criterion_benchmark(c: &mut Criterion) {
    // let mut group = c.benchmark_group("generation");
    c.bench_function("generate 10,000 x 10,000", |b| {
        b.iter(|| generate(black_box(10_000), black_box(10_000)))
    });
    // group.bench_function("generate 10,000 x 10,000 old", |b| {
    //     b.iter(|| generate_old(black_box(10_000), black_box(10_000)))
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
