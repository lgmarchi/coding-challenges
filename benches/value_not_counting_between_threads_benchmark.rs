extern crate coding_challenges;

use coding_challenges::value_not_counting_between_threads::{
    arc_atomic_usize_solution, arc_mutex_solution, arc_rwlock_solution,
};
use criterion::{criterion_group, criterion_main, Criterion};

const SAMPLE_SIZE: usize = 10000;

fn benchmark_arc_mutex(c: &mut Criterion) {
    let mut group = c.benchmark_group("Arc Mutex Benchmark");
    group.sample_size(SAMPLE_SIZE);
    group.bench_function("Arc<Mutex<usize>>", |b| {
        b.iter(|| arc_mutex_solution());
    });
    group.finish();
}

fn benchmark_atomic_usize(c: &mut Criterion) {
    let mut group = c.benchmark_group("Arc AtomicUsize Benchmark");
    group.sample_size(SAMPLE_SIZE);
    group.bench_function("Arc<AtomicUsize>", |b| {
        b.iter(|| arc_atomic_usize_solution());
    });
    group.finish();
}

fn benchmark_rwlock(c: &mut Criterion) {
    let mut group = c.benchmark_group("Arc RwLock Benchmark");
    group.sample_size(SAMPLE_SIZE);
    group.bench_function("Arc<RwLock<usize>>", |b| {
        b.iter(|| arc_rwlock_solution());
    });
    group.finish();
}

criterion_group!(
    benches,
    benchmark_arc_mutex,
    benchmark_atomic_usize,
    benchmark_rwlock
);
criterion_main!(benches);
