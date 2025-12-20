# Parrot 🦜

[![CI](https://github.com/dejaime/parrot/actions/workflows/ci.yml/badge.svg)](https://github.com/dejaime/parrot/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/parrot-rng.svg)](https://crates.io/crates/parrot-rng)

Parrot is a no dependency, lightweight, strictly deterministic procedural generation library for Rust. It is designed to produce identical results across all platforms (x86, ARM, WASM) and is fully compatible with embedded (no_std) environments.

[![Documentation](https://docs.rs/crate/parrot-rng/latest)](https://docs.rs/crate/parrot-rng/latest)

## Features

- Lightweight: Implements Xoroshiro128+ for high-performance random number generation.

- Embedded Ready: no_std by default, supports anything from ARM to WASM to embedded use cases.

- Convenient Seeding: Use `u64` or `str` seeds for creative products and games!

- Deterministic and Thread Safe Perlin Noise: Spatially coherent 2D noise that uses a static permutation table (no "TV static" artifacts or mutable state). The noise generator is immutable (&self) and can be shared across threads without locking.

## Benchmarks 🚀

Parrot is tuned for extreme performance. On a standard modern CPU, it generates random numbers in **sub-nanosecond** time, outperforming even the standard Rust `SmallRng`.

| Generator | Time per u64 | Ops / Sec | Notes |
|-----------|--------------|-----------|-------|
| StdRng | 2.99 ns | ~334 Million | `rand::rngs::StdRng` |
| SmallRng | 0.88 ns | ~1.13 Billion | `rand::rngs::SmallRng` |
| **Parrot** | **0.88 ns** | **~1.14 Billion** | `Xoroshiro128+` |

*Benchmarks run on Linux x86_64, single-threaded on a Ryzen 7 modern CPU.*

| Generator | Time per u64 | Ops / Sec | Notes |
|-----------|--------------|-----------|-------|
| StdRng | 2.99 ns | ~334 Million | `rand::rngs::StdRng` |
| Perlin 2D | 3.28 ns | ~305 Million | Gradient Noise Lookup |

This means we are generating gradient noise almost as fast as a standard crypto RNG generates a plain integers!

To run these benchmarks yourself:
```bash
cargo bench --features rand-support
```

### Multithreaded Performance ⚡

Parrot scales linearly across cores. Because the noise generator is immutable and lock-free, adding more threads yields a near-perfect speedup.

| Threads | RNG Throughput | Perlin Throughput | Speedup |
|---------|----------------|-------------------|---------|
| 1       | 1.14 Billion/s | 121 Million/s     | 1.0x    |
| 2       | 2.22 Billion/s | 237 Million/s     | 1.96x   |
| 4       | 4.33 Billion/s | 462 Million/s     | 3.83x   |
| 8       | 8.29 Billion/s | 869 Million/s     | 7.18x   |

*Benchmarks run on Linux x86_64, on a Ryzen 7 modern CPU.*
*Up to 8 threads run on a 16-core CPU. Scaling is effectively linear until hardware saturation.*

To run these benchmarks yourself:
```bash
cargo bench --bench multithreaded --features rand-support
```

### Why is it fast?

Parrot does not need to be cryptographically secure!

Differently from a deterministic random number generator, `StdRng` is used in a lot of security sensitive contexts.

Run them with `cargo bench --features rand-support`

## Installation

Add this to your Cargo.toml:

```toml
[dependencies]
parrot = "0.4.0"
```

Or alternativelly, if you want `rand` support:

```toml
[dependencies]
parrot-rng = { version = "0.4.0", features = ["rand-support"] }
```

## Usage
1. Repeatable Random Numbers

Generate numbers that are guaranteed to be the same on every machine for a given seed.

```rust
use parrot::Parrot;

fn main() {
	let mut rng = Parrot::new_from_str("parrot");
    // Also supports u64
	//		let mut rng = Parrot::new(12345);

    // Generate values
    let val = rng.gen_range(0, 100);
    let u64_val = rng.next_u64(); // [0 to u64::MAX]
    let f64_val = rng.next_f64(); // [0.0 to 1.0)

    println!("Deterministic value: {}", val);
}
```

2. Terrain Generation (Perlin Noise)

Generate smooth, continuous noise for terrain, clouds, or textures.
```rust
use parrot::PerlinNoise;

fn main() {
    // The seed determines the "shape" of the terrain
    let noise = PerlinNoise::new(98765);

    // Get the height at coordinate (10.5, 20.0)
    // This is immutable and thread-safe!
    let height = noise.noise2d(10.5, 20.0);
    
    println!("Height: {}", height);
}
```

3. Ecosystem Compatibility (rand-support)

Requires `features = ["rand-support"]`

Parrot can act as a standard RngCore, allowing you to use it with the rest of the Rust ecosystem.
```rust
use parrot::Parrot;
use rand::seq::SliceRandom; // From the 'rand' crate

fn main() {
    let mut rng = Parrot::new(42);
    let mut deck = vec!["Ace", "King", "Queen", "Jack"];

    // Use standard rand methods!
    deck.shuffle(&mut rng);
    
    println!("Shuffled: {:?}", deck);
}
```

## Examples
The "Hello World"

This repository includes a brute-force tool to demonstrate determinism. You can find a seed that forces the RNG to generate a specific string (like "hello" or your name).

Run the finder example to discover a seed for your own word:
Bash

## Find a seed that generates "parrot"

Check the example `demo.rs`, and you can find your own test seeds with the example brute force example.

```shell
$ cargo run --release --example brute_force_finder "parrot" 100M
Found 2 matching seeds in range 0-100000000
Seeds for "parrot": [12493373, 24602289]
```

## License

Licensed under your choice of:

- MIT License
- Apache License, Version 2.0
- GPLv3
- LGPLv3
