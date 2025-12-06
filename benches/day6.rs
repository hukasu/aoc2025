use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    let data = std::fs::read_to_string("inputs/day6.txt").unwrap();
    c.bench_function("day6_part1", |b| {
        b.iter(|| aoc2025::day6::Worksheet::solve_worksheet(&data))
    });
    c.bench_function("day6_part2", |b| {
        b.iter(|| aoc2025::day6::Worksheet::solve_cephalopodian_worksheet(&data))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
