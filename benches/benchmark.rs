use criterion::{Criterion, black_box, criterion_group, criterion_main};

#[cfg(feature = "rand-support")]
use parrot::Parrot;

#[cfg(feature = "rand-support")]
use rand::prelude::*;

#[cfg(feature = "rand-support")]
use rand::rngs::{SmallRng, StdRng};

#[cfg(feature = "rand-support")]
fn bench_rng_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("RNG Throughput");

    // 1. Benchmark Parrot (Your Library)
    group.bench_function("Parrot (next_u64)", |b| {
        let mut rng = Parrot::new(42);
        b.iter(|| {
            // black_box prevents the compiler from optimizing the loop away
            black_box(rng.next_u64())
        })
    });

    // 2. Benchmark SmallRng (The competition: fast, non-crypto)
    group.bench_function("SmallRng (next_u64)", |b| {
        let mut rng = SmallRng::seed_from_u64(42);
        b.iter(|| black_box(rng.next_u64()))
    });

    // 3. Benchmark StdRng (The standard: slower, crypto-secure)
    group.bench_function("StdRng (next_u64)", |b| {
        let mut rng = StdRng::seed_from_u64(42);
        b.iter(|| black_box(rng.next_u64()))
    });

    group.finish();
}

// Dummy for when rand-support is not enabled
#[cfg(not(feature = "rand-support"))]
fn bench_rng_generation(_: &mut Criterion) {
    println!("Skipping RNG benchmarks (enable 'rand-support' feature to run)");
}

fn bench_perlin_noise(c: &mut Criterion) {
    let mut group = c.benchmark_group("Noise Generation");
    let perlin = parrot::Perlin::new(42);

    group.bench_function("Perlin 2D", |b| {
        b.iter(|| {
            // We use constant coordinates to measure pure algorithm speed
            black_box(perlin.noise2d(10.5, 20.5))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_rng_generation, bench_perlin_noise);
criterion_main!(benches);
