// from codspeed docs
use advent_of_code_2024::day1::{part1, part2};
use advent_of_code_2024::read_file;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day01_bench(c: &mut Criterion) {
    let contents = read_file("input/2024", 1);
    c.bench_function("part1", |b| b.iter(|| part1(black_box(&contents))));
}

pub fn part2_bench(c: &mut Criterion) {
    let contents = read_file("input/2024", 1);
    c.bench_function("part2", |b| b.iter(|| part2(black_box(&contents))));
}

criterion_group!(benches, day01_bench);
criterion_main!(benches);
