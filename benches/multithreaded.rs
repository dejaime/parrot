use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main}; // <-- Added black_box
use parrot::{Parrot, Perlin};
use std::sync::Arc;
use std::thread;

// We need num_cpus to know how far to scale
// Ensure num_cpus is in [dev-dependencies]

#[cfg(feature = "rand-support")]
use rand_core::RngCore;

// 1. PARROT SCALING
fn bench_parrot_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parrot Scaling (Throughput)");

    // Total operations to distribute across threads
    let total_ops = 1_000_000;
    group.throughput(Throughput::Elements(total_ops));

    let max_threads = num_cpus::get();
    let mut thread_counts = vec![1, 2, 4, 8, 16];
    thread_counts.retain(|&t| t <= max_threads);
    if !thread_counts.contains(&max_threads) {
        thread_counts.push(max_threads);
    }
    thread_counts.sort();
    thread_counts.dedup();

    for threads in thread_counts {
        group.bench_with_input(
            BenchmarkId::from_parameter(threads),
            &threads,
            |b, &t_count| {
                b.iter_custom(|iters| {
                    let batch_size = total_ops * iters;
                    let ops_per_thread = batch_size / t_count as u64;

                    let start = std::time::Instant::now();

                    thread::scope(|s| {
                        for _ in 0..t_count {
                            s.spawn(move || {
                                let mut rng = Parrot::new(42);
                                for _ in 0..ops_per_thread {
                                    // FIX: Use black_box to prevent optimization
                                    #[cfg(feature = "rand-support")]
                                    {
                                        black_box(rng.next_u64());
                                    }

                                    #[cfg(not(feature = "rand-support"))]
                                    {
                                        black_box(rng.next_f64());
                                    }
                                }
                            });
                        }
                    });

                    start.elapsed()
                })
            },
        );
    }
    group.finish();
}

// 2. PERLIN SCALING
fn bench_perlin_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("Perlin Scaling (Throughput)");
    let total_ops = 100_000;
    group.throughput(Throughput::Elements(total_ops));

    let perlin = Arc::new(Perlin::new(12345));

    let max_threads = num_cpus::get();
    let mut thread_counts = vec![1, 2, 4, 8, 16];
    thread_counts.retain(|&t| t <= max_threads);
    if !thread_counts.contains(&max_threads) {
        thread_counts.push(max_threads);
    }
    thread_counts.dedup();

    for threads in thread_counts {
        group.bench_with_input(
            BenchmarkId::from_parameter(threads),
            &threads,
            |b, &t_count| {
                let perlin_ref = perlin.clone();

                b.iter_custom(move |iters| {
                    let batch_size = total_ops * iters;
                    let ops_per_thread = batch_size / t_count as u64;

                    let start = std::time::Instant::now();

                    thread::scope(|s| {
                        for _ in 0..t_count {
                            let p = perlin_ref.clone();
                            s.spawn(move || {
                                for i in 0..ops_per_thread {
                                    let f = i as f64 * 0.01;
                                    // FIX: Use black_box here too
                                    black_box(p.noise2d(f, f));
                                }
                            });
                        }
                    });

                    start.elapsed()
                })
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_parrot_scaling, bench_perlin_scaling);
criterion_main!(benches);
