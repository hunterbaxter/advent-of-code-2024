use advent_of_code_2024::day1::{part1_v0, part2_v0};
use advent_of_code_2024::read_file;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day01_p1_v0_bench(c: &mut Criterion) {
    let contents = read_file("input/2024", 1);
    c.bench_function("part1", |b| b.iter(|| part1_v0(black_box(&contents))));
}

pub fn day01_p2_v0_bench(c: &mut Criterion) {
    let contents = read_file("input/2024", 1);
    c.bench_function("part2", |b| b.iter(|| part2_v0(black_box(&contents))));
}

criterion_group!(benches, day01_p1_v0_bench, day01_p2_v0_bench);
criterion_main!(benches);
