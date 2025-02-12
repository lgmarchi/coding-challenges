
extern crate coding_challenges;

use coding_challenges::two_sum::{two_sum, two_sum_o_zero};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const SAMPLE_SIZE: usize = 10000;

fn benchmark_two_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("Two Sum O(n^2)");
    group.sample_size(SAMPLE_SIZE);
    
    let input = vec![2, 7, 11, 15];
    let target = 9;

    group.bench_function("Brute Force", |b| {
        b.iter(|| two_sum(black_box(input.clone()), black_box(target)));
    });

    group.finish();
}

fn benchmark_two_sum_o_zero(c: &mut Criterion) {
    let mut group = c.benchmark_group("Two Sum O(1)");
    group.sample_size(SAMPLE_SIZE);

    let input = vec![2, 7, 11, 15];
    let target = 9;

    group.bench_function("Optimized", |b| {
        b.iter(|| two_sum_o_zero(black_box(input.clone()), black_box(target)));
    });

    group.finish();
}

fn benchmark_two_sum_with_big_input(c: &mut Criterion) {
    let mut group = c.benchmark_group("Two Sum O(n^2)");
    group.sample_size(SAMPLE_SIZE);
    
    let input: Vec<i32> = (1..10_000).collect();
    let target = 9_999;

    group.bench_function("Brute Force", |b| {
        b.iter(|| two_sum(black_box(input.clone()), black_box(target)));
    });

    group.finish();
}

fn benchmark_two_sum_o_zero_with_big_input(c: &mut Criterion) {
    let mut group = c.benchmark_group("Two Sum O(1)");
    group.sample_size(SAMPLE_SIZE);

    let input: Vec<i32> = (1..10_000).collect();
    let target = 9_999;

    group.bench_function("Optimized", |b| {
        b.iter(|| two_sum_o_zero(black_box(input.clone()), black_box(target)));
    });

    group.finish();
}


criterion_group!(benches, benchmark_two_sum, benchmark_two_sum_o_zero, benchmark_two_sum_with_big_input, benchmark_two_sum_o_zero_with_big_input);
criterion_main!(benches);
