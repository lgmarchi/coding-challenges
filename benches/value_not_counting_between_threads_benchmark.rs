extern crate coding_challenges;

use coding_challenges::value_not_counting_between_threads::{
    arc_atomic_usize_solution, arc_mutex_solution, arc_rwlock_solution,
    crossbeam_channels_solution, returning_from_threads_solutions,
    standard_channels_solution,
};
use criterion::{criterion_group, criterion_main, Criterion};

const SAMPLE_SIZE: usize = 10000;

fn benchmark_arc_mutex(c: &mut Criterion) {
    let mut group = c.benchmark_group("Arc Benchmark");
    group.sample_size(SAMPLE_SIZE);
    group.measurement_time(std::time::Duration::from_secs(60));
    group.bench_function("Arc<Mutex<usize>>", |b| {
        b.iter(|| arc_mutex_solution());
    });
    group.finish();
}

fn benchmark_atomic_usize(c: &mut Criterion) {
    let mut group = c.benchmark_group("Arc Benchmark");
    group.sample_size(SAMPLE_SIZE);
    group.measurement_time(std::time::Duration::from_secs(60));
    group.bench_function("Arc<AtomicUsize>", |b| {
        b.iter(|| arc_atomic_usize_solution());
    });
    group.finish();
}

fn benchmark_rwlock(c: &mut Criterion) {
    let mut group = c.benchmark_group("Arc Benchmark");
    group.sample_size(SAMPLE_SIZE);
    group.measurement_time(std::time::Duration::from_secs(60));
    group.bench_function("Arc<RwLock<usize>>", |b| {
        b.iter(|| arc_rwlock_solution());
    });
    group.finish();
}

fn benchmark_standard_channels(c: &mut Criterion) {
    let mut group = c.benchmark_group("Channels Benchmark");
    group.sample_size(SAMPLE_SIZE);
    group.measurement_time(std::time::Duration::from_secs(60));
    group.bench_function("Standard Channels", |b| {
        b.iter(|| standard_channels_solution());
    });
    group.finish();
}

fn benchmark_crossbeam_channels(c: &mut Criterion) {
    let mut group = c.benchmark_group("Channels Benchmark");
    group.sample_size(SAMPLE_SIZE);
    group.measurement_time(std::time::Duration::from_secs(60));
    group.bench_function("Crossbeam Channels", |b| {
        b.iter(|| crossbeam_channels_solution());
    });
    group.finish();
}

fn benchmark_returning_from_threads(c: &mut Criterion) {
    let mut group = c.benchmark_group("Returning from threads Benchmark");
    group.sample_size(SAMPLE_SIZE);
    group.measurement_time(std::time::Duration::from_secs(60));
    group.bench_function("Returning from threads", |b| {
        b.iter(|| returning_from_threads_solutions());
    });
    group.finish();
}

criterion_group!(
    benches,
    benchmark_arc_mutex,
    benchmark_atomic_usize,
    benchmark_rwlock,
    benchmark_standard_channels,
    benchmark_crossbeam_channels,
    benchmark_returning_from_threads
);
criterion_main!(benches);
