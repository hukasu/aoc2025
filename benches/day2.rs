use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    let data = std::fs::read("inputs/day2.txt").unwrap();
    c.bench_function("day2_part1", |b| {
        b.iter(|| aoc2025::day2::RangeChecker::check_ranges(&String::from_utf8_lossy(&data)))
    });
    c.bench_function("day2_part2", |b| {
        b.iter(|| aoc2025::day2::RangeChecker::check_ranges_extra(&String::from_utf8_lossy(&data)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
