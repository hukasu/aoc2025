use aoc2025::day9::Tile;
use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    let data = std::fs::read_to_string("inputs/day9.txt").unwrap();
    c.bench_function("day9_part1", |b| {
        b.iter(|| {
            let tiles = aoc2025::day9::Tile::read_tiles(&data);
            Tile::largest_rectangle(&tiles);
        })
    });
    c.bench_function("day9_part2", |b| {
        b.iter(|| {
            let tiles = aoc2025::day9::Tile::read_tiles(&data);
            Tile::largest_red_green_rectangle(&tiles);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
