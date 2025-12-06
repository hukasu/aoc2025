use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    let data = std::fs::read_to_string("inputs/day5.txt").unwrap();
    c.bench_function("day5_part1", |b| {
        b.iter(|| aoc2025::day5::Inventory::spoiled_items(&data))
    });
    c.bench_function("day5_part2", |b| {
        b.iter(|| aoc2025::day5::Inventory::fresh_items(&data))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
