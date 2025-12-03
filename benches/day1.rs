use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    let data = std::fs::read_to_string("inputs/day1.txt").unwrap();
    c.bench_function("day1_part1", |b| {
        b.iter(|| aoc2025::day1::LockSolver::find_password(&data))
    });
    c.bench_function("day1_part2", |b| {
        b.iter(|| aoc2025::day1::LockSolver::find_password_method_0x434C49434B(&data))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
