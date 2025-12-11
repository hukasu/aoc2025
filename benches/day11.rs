use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    let data = std::fs::read_to_string("inputs/day11.txt").unwrap();
    c.bench_function("day11_part1", |b| {
        b.iter(|| {
            aoc2025::day11::ServerRackConnections::parse(&data).connections_to_from("you", "out");
        })
    });
    c.bench_function("day11_part2", |b| {
        b.iter(|| {
            aoc2025::day11::ServerRackConnections::parse(&data).paths_through(
                "svr",
                "out",
                &["dac", "fft"],
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
