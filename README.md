# Parrot 🦜

[![CI](https://github.com/dejaime/parrot/actions/workflows/ci.yml/badge.svg)](https://github.com/dejaime/parrot/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/parrot-rng.svg)](https://crates.io/crates/parrot-rng)
[![License](https://img.shields.io/crates/l/parrot-rng.svg)](https://crates.io/crates/parrot-rng)

Parrot is a lightweight, strictly deterministic procedural generation library for Rust. It is designed to produce identical results across all platforms (x86, ARM, WASM) and is fully compatible with embedded (no_std) environments.

## Features

    Lightweight: Implements Xoroshiro128+ for high-performance random number generation.

    Deterministic Perlin Noise: Spatially coherent 2D noise that uses a static permutation table (no "TV static" artifacts or mutable state).

    Embedded Ready: no_std by default, supports anything from ARM to WASM to embedded use cases.

    Thread Safe: The noise generator is immutable (&self) and can be shared across threads without locking.

## Installation

Add this to your Cargo.toml:

```toml
[dependencies]
parrot = "0.1.2"
```

## Usage
1. Repeatable Random Numbers

Generate numbers that are guaranteed to be the same on every machine for a given seed.

```rust
use parrot::Parrot;

fn main() {
    let mut rng = Parrot::new(12345);

    // Generate values
    let val = rng.gen_range(0, 100);
    let float_val = rng.gen_f64(); // 0.0 to 1.0

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
