use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    let data = std::fs::read_to_string("inputs/day12.txt").unwrap();
    c.bench_function("day12_part1", |b| {
        b.iter(|| {
            aoc2025::day12::UnderTheChrismasTree::parse(&data).valid_placements();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
