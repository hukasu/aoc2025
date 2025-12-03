use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    let data = std::fs::read_to_string("inputs/day3.txt").unwrap();
    c.bench_function("day3_part1", |b| {
        b.iter(|| aoc2025::day3::PowerBank::best_joltage_multiple_banks(&data, 2))
    });
    c.bench_function("day3_part2", |b| {
        b.iter(|| aoc2025::day3::PowerBank::best_joltage_multiple_banks(&data, 12))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
