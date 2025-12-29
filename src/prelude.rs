//! The parrot prelude.

pub use crate::Parrot;
pub use crate::Perlin;
pub use crate::RandomRange;
pub use crate::hash::hash;

#[cfg(feature = "bevy-support")]
pub use crate::bevy_ext::ParrotBevyExt;

#[cfg(feature = "rand-support")]
pub use rand_core::{RngCore, SeedableRng};
