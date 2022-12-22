use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use day16::code::Run;
use day16::code2::Run2;


pub fn part1_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Part 1");
    group.sample_size(10);
    group.bench_with_input(BenchmarkId::new("", ""), &"input.txt", |b, _| {
        b.iter(|| Run2::part1());
    });
}

pub fn part2_benchmark(c: &mut Criterion) {
    c.bench_with_input(BenchmarkId::new("", ""), &"input.txt", |b, _| {
        b.iter(|| Run::part1());
    });
}

criterion_group!(benches, part1_benchmark);
criterion_main!(benches);