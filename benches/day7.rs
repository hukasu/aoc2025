use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    let data = std::fs::read("inputs/day7.txt").unwrap();
    c.bench_function("day7_part1", |b| {
        b.iter(|| aoc2025::day7::TachyonManifold::new(&data).count_splits())
    });
    c.bench_function("day7_part2", |b| {
        b.iter(|| aoc2025::day7::TachyonManifold::new(&data).count_timelines())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
