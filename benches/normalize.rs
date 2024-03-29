use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};
use rc_path::SugarPath;

fn normalize() {
    assert_eq!(
        Path::new("/foo/../../../bar").normalize(),
        Path::new("/bar")
    );
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("normalize", |b| b.iter(normalize));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
