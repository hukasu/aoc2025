use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    let data = std::fs::read("inputs/day4.txt").unwrap();
    c.bench_function("day4_part1", |b| {
        b.iter(|| aoc2025::day4::FloorPlan::accessible_paper_rolls(&data))
    });
    c.bench_function("day4_part2", |b| {
        b.iter(|| aoc2025::day4::FloorPlan::removable_paper_rolls(&data))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
