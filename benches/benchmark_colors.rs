use all_colors::get_color_hex;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("return colors", |b| b.iter(|| get_color_hex(black_box(""))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
