# Parrot 🦜

[![CI](https://github.com/dejaime/parrot/actions/workflows/ci.yml/badge.svg)](https://github.com/dejaime/parrot/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/parrot-rng.svg)](https://crates.io/crates/parrot-rng)

Parrot is a no dependency, lightweight, strictly deterministic procedural generation library for Rust. It is designed to produce identical results across all platforms (`x86`, `ARM`, `WASM`) and is fully compatible with embedded (`no_std`) environments.

[![Documentation](https://docs.rs/crate/parrot-rng/latest)](https://docs.rs/crate/parrot-rng/latest)

## Features

- **Lightweight**: Implements Xoroshiro128+ for high-performance random number generation.

- **Embedded Ready**: `no_std` by default, supports anything from ARM to WASM to embedded use cases.

- **Convenient Seeding**: Use `u64` or `str` seeds for creative products and games!

- **Deterministic and Thread Safe Perlin Noise**: Spatially coherent 2D noise that uses a static permutation table (no "TV static" artifacts or mutable state). The noise generator is immutable (&self) and can be shared across threads without locking.

- **Serde Support**: need to save and load the current generator state? Easy done with serde support.

- **Bevy Support**: first class implementations for the basic Bevy Engine types, including spatial vectors (e.g. Vec3), Colors and more.

## Installation

Add this to your Cargo.toml:

```toml
[dependencies]
parrot-rng = "0.7.3"
```

Or alternatively, if don't need `no_std` and want more features, pick and choose:

```toml
[dependencies]
parrot-rng = { version = "0.7.3", features = ["rand-support", "serde-support", "bevy-support"] }
```

## Usage
1. Repeatable Random Numbers

Generate numbers that are guaranteed to be the same on every machine for a given seed.

```rust
use parrot::prelude::*;

fn main() {
    // Use a string or a u64 seed
    let mut rng = Parrot::new_from_str("parrot");

    // Generate values
    let val = rng.gen_range(0, 100);
    let u64_val = rng.next_u64(); // [0, u64::MAX]
    let f64_val = rng.next_f64(); // [0.0, 1.0)

    println!("Deterministic value: {}, {}, {}", val, u64_val, f64_val);
}
```

2. Terrain Generation (Perlin Noise)

Generate smooth, continuous noise for terrain, clouds, or textures.
```rust
use parrot::prelude::*;

fn main() {
    // The seed determines the "shape" of the terrain
    let noise = Perlin::new(98765);

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
use parrot::prelude::*;
use rand::seq::SliceRandom; // From the 'rand' crate

fn main() {
    let mut rng = Parrot::new(42);
    let mut deck = vec!["Ace", "King", "Queen", "Jack"];

    // Use standard rand methods!
    deck.shuffle(&mut rng);
    
    println!("Shuffled: {:?}", deck);
}
```

Parrot comes with several examples to demonstrate its features, from visual terrain generation to statistical verification.
1. Procedural Terrain (Perlin Noise)

Generate an ASCII terrain map using 2D Perlin noise. You can pass any string (like your name or a location) as a seed.
You should see the exact same pattern if running on your local system with `"archipelago" 60 20`

`# Usage: cargo run --example perlin <SEED_STRING> <WIDTH> <HEIGHT>`

```rust
$ cargo run --example perlin "archipelago" 60 20
Parrot Perlin Noise Demo
Input: "archipelago" (Seed: 8873276094910301339)
Size: 60x20
------------------------------------------------------------
~...~~~.......~~~~~~~......~~~~~~~......~~~~~~~~~~~~~~...~~~
........∩∩∩∩....~~~~..∩..~~~~~~.........~~   ~~~~~~~~....~~~
.......∩∩∩∩∩∩∩∩.........~~   ~~.∩∩∩∩...~~          ~..∩∩.~  
.∩∩∩∩∩∩∩▲▲▲∩∩∩∩∩∩......~~     ~.∩∩∩∩..~~~~         ~..∩∩.~~ 
.∩∩∩∩∩∩∩∩∩∩∩∩∩∩∩∩..~~~~~~~    ~.∩∩∩∩.~~~~~~~~~      ~..∩∩.~ 
.......∩∩∩∩∩∩∩∩∩∩..~~~~~~~~   ~..∩...~~~~....~~     ~..∩∩..~
....................~~~~~.~~~ ~~....~~~~~.∩∩∩.~~    ~~..∩∩..
~~~~~~~~~~~~~~~......~~.....~~~~~~~~~~~~~.∩∩∩∩..~   ~~.∩∩∩∩.
~~~~~~~~~~~~~~~~..∩∩....∩∩∩..~~~~~~~   ~~.∩∩∩∩∩..~ ~~..∩∩▲∩∩
~        ~~~~~~~..∩∩...∩∩∩∩∩..~         ~..∩∩∩∩∩...~...∩∩▲▲∩
~        ~~~~~........∩∩∩∩∩∩∩.~         ~~.∩∩∩∩∩∩∩∩∩...∩∩▲▲▲
~~      ~~.......~....∩∩∩∩∩∩∩.~~        ~~...∩∩∩∩▲▲∩∩...∩∩∩∩
~~     ~~.......~~~~..∩∩∩.....~~~~~     ~~......∩∩▲∩∩.....∩∩
~~    ~~.......~~~~~..........~~~~~~~  ~~...~~~..∩∩∩..~~~...
~~   ~~..∩..~~~~~~~~~....~~~~~~.....~~~~....~~~~.∩∩∩.~~~~~..
~~   ~..∩∩..~~~~~~~~~...~~~ ~~~......~~.....~~~~.....~~~~~~~
~    ~~..∩..~~~~~~~~~..~~     ~~~...........~~~~.....~~~~~..
~     ~.............~~~~        ~~.....~~~~~...........~....
~     ~~............~~~          ~~....~~~~~.....~..........
~     ~~.........∩..~~~ ~~~~~    ~~.∩∩..~~~~....~~~~........
```

2. Reverse-Engineering Seeds ("Hello World")

Parrot is strictly deterministic. This means we can "brute force" a seed that forces the RNG to generate a specific sequence of numbers.

We included a tool that finds a seed which generates your specific word when mapped to ASCII characters.
Bash

### Find a seed that generates "parrot"

```shell
$ cargo run --release --example brute_force_finder "parrot" 100M
Searching for seed to generate: "parrot"
Search Range: 0..100000000 seeds
--------------------------------------------------
Found 2 matching seeds in range 0-100000000
Seeds for "parrot": [12493373, 24602289]
```

You can then verify this behavior using the demo example, which hardcodes these discovered seeds to print "Hello World".

3. Statistical Distribution Check

Verify the uniformity of the RNG by simulating a large number of die rolls.
Bash

### Roll a 10-sided die (0-9) 500,000 times

Output:
```shell
$ cargo run --release --example run_rng_range -- "test_seed" 0 10 500000

🦜 Rolling 500000 times (Range: [0, 10), Seed: "test_seed")...

Value      | Count      | Percent   
-----------+------------+-----------
0          | 50123      | 10.02 %
1          | 49890      | 9.98  %
2          | 50005      | 10.00 %
...
```
All values should hover close to 10.00%

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

This means we are generating gradient noise almost as fast as a standard RNG generates plain integers!

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

The security requirements are different for a deterministic random number generator. We do not have to be cryptographically secure as `StdRng` does.

Parrot is not supposed to be used in high security sensitive contexts, and can optimise for speed.

## Bevy Integration

Enable Bevy Engine integration with the `"bevy-support"` feature.

```toml
[dependencies]
parrot-rng = { version = "0.4.1", features = ["rand-support", "bevy-support"] }
```

### 1. Global Deterministic RNG (Resource)

Add `Parrot` as a resource to share a single random number generator across systems.

```rust
use bevy::prelude::*;
use parrot::prelude::*;

fn main() {
    App::new()
        .insert_resource(Parrot::new_from_str(player_defined_seed))
        .add_systems(Update, print_d20_roll)
        .run();
}

fn print_d20_roll(mut rng: ResMut<Parrot>) {
    // Generate a number from 1 to 20
    println!("Player rolled a: {}", rng.gen_range(1, 21)); //[1, 21)
}
```

### 2. Context-Specific RNG (Component)

Attach `Parrot` to entities to ensure their behavior remains deterministic regardless of system execution order.

```rust
use bevy::prelude::*;
use parrot::prelude::*;

#[derive(Component)]
struct Npc {
    rng: Parrot,
}

fn spawn_npc(mut commands: Commands) {
    commands.spawn(Npc {
        rng: Parrot::new(global_seed + entity_id),
    });
}

fn npc_decision(mut query: Query<&mut Npc>) {
    for mut npc in &mut query {
        if npc.rng.gen_bool(0.25) { //25%
            println!("NPC decided to move!");
        }
    }
}
```

## License

Licensed under your choice of:

- MIT License
- Apache License, Version 2.0
- GPLv3
- LGPLv3
