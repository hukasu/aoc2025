use aoc2025::day8::JunctionBox;
use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    let data = std::fs::read_to_string("inputs/day8.txt").unwrap();
    c.bench_function("day8_part1", |b| {
        b.iter(|| {
            let junction_boxes = aoc2025::day8::JunctionBox::read_positions(&data);
            JunctionBox::largest_circuits(&junction_boxes, 1000);
        })
    });
    c.bench_function("day8_part2", |b| {
        b.iter(|| {
            let junction_boxes = aoc2025::day8::JunctionBox::read_positions(&data);
            JunctionBox::distance_to_wall(&junction_boxes);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
