use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use day15::code::Run;


pub fn part1_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Part 1");
    group.sample_size(10);
    group.bench_with_input(BenchmarkId::new("Part 1", "none"), &"input.txt", |b, _| {
        b.iter(|| Run::part1());
    });
}

pub fn part2_benchmark(c: &mut Criterion) {
    c.bench_with_input(BenchmarkId::new("Part 2", "none"), &"input.txt", |b, _| {
        b.iter(|| Run::part2());
    });
}

criterion_group!(benches, part1_benchmark, part2_benchmark);
criterion_main!(benches);