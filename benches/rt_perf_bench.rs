#[macro_use]
extern crate criterion;

use criterion::Criterion;
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Screenshot Renders");

    group.sample_size(10);

    group.bench_function("Basic Scene Screenshot 1080p", |b| {});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
