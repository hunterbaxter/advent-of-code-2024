use advent_of_code_2024::day1::{
    input_generator_v0, input_generator_v1, part1_v0, part2_v0, part2_v1,
};
use advent_of_code_2024::read_file;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day01_generator_v0(c: &mut Criterion) {
    let contents = read_file("input/2024", 1);
    c.bench_function("day01_generator_v0", |b| {
        b.iter(|| input_generator_v0(black_box(&contents)))
    });
}

pub fn day01_generator_v1(c: &mut Criterion) {
    let contents = read_file("input/2024", 1);
    c.bench_function("day01_generator_v0", |b| {
        b.iter(|| input_generator_v1(black_box(&contents)))
    });
}

pub fn day01_p1_v0_bench(c: &mut Criterion) {
    let contents = read_file("input/2024", 1);
    c.bench_function("day01_part1_v0", |b| {
        b.iter(|| part1_v0(black_box(&contents)))
    });
}

pub fn day01_p2_v0_bench(c: &mut Criterion) {
    let contents = read_file("input/2024", 1);
    c.bench_function("day01_part2_v0", |b| {
        b.iter(|| part2_v0(black_box(&contents)))
    });
}

pub fn day01_p2_v1_bench(c: &mut Criterion) {
    let contents = read_file("input/2024", 1);
    c.bench_function("day01_part2_v1", |b| {
        b.iter(|| part2_v1(black_box(&contents)))
    });
}

criterion_group!(
    benches,
    // day01_generator_v0,
    // day01_generator_v1,
    // day01_p1_v0_bench,
    day01_p2_v0_bench,
    day01_p2_v1_bench
);
criterion_main!(benches);
