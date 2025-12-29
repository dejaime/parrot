#![cfg_attr(not(feature = "std"), no_std)]

//! # Parrot RNG 🦜
//!
//! **Parrot** (`parrot-rng`) is a lightweight, strictly deterministic procedural generation library.
//!
//! It is designed for game development, generative art, and embedded systems where reproducibility is critical.
//! If you run a Parrot RNG with the seed `12345` on a high-end PC, a Raspberry Pi, or in a web browser (WASM),
//! you will get the **exact same sequence of numbers**.
//!
//!
//! ## Features
//!
//! * **Deterministic:** 100% portable results across all major architectures (x86, ARM, WASM).
//! * **`#[no_std]`:** Works in embedded, resource-constrained, and bare-metal environments.
//! * **String Seeds:** Use human-readable strings like "mountain" or "level-1" to seed your generators.
//! * **Perlin Noise:** A stateless, immutable, and thread-safe 2D noise generator.
//!
//! ## Quick Start: RNG
//!
//! ```rust
//! use parrot::prelude::*;
//!
//! // Create a new RNG from a seed
//! let mut rng = Parrot::new(42);
//!
//! // Or from a string
//! let mut rng_from_str = Parrot::new_from_str("hello world");
//!
//! // Generate a number between 0 and 99
//! let x = rng.gen_range(0, 100);
//!
//! println!("Random number: {}", x);
//! ```
//!
//! ## Quick Start: Perlin Noise
//!
//! ```rust
//! use parrot::prelude::*;
//!
//! // Create a Perlin noise generator from a seed
//! let noise = Perlin::new(123);
//!
//! // Or from a string
//! let noise_from_str = Perlin::new_from_string("a-stable-seed");
//!
//! // Get a noise value at a 2D coordinate
//! let value = noise.noise2d(0.5, 0.2);
//!
//! println!("Noise value: {}", value);
//! ```

pub mod hash;
pub mod parrot;
pub mod perlin;
pub mod prelude;

#[cfg(feature = "bevy-support")]
pub mod bevy_ext;

pub use parrot::Parrot;
pub use parrot::RandomRange;
pub use perlin::Perlin;

#[cfg(feature = "bevy-support")]
pub use bevy_ext::ParrotBevyExt;
