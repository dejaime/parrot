use crate::hash::fnv1a_64;

/// A strictly deterministic, lightweight random number generator.
///
/// `Parrot` uses the **Xoroshiro128+** algorithm. It is designed to be:
/// - **Fast:** Suitable for real-time applications (games, simulations).
/// - **Portable:** Guarantees the same sequence of numbers on any architecture (x86, ARM, WASM).
/// - **Embedded-friendly:** Uses a small state (16 bytes) and works in `no_std` environments.
///
/// # Example
///
/// ```
/// use parrot::Parrot;
///
/// let mut rng = Parrot::new(12345);
/// let val = rng.gen_range(0, 100);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parrot {
    state: [u64; 2],
}

impl Parrot {
    /// Creates a new RNG instance seeded from a string.
    ///
    /// The string is hashed using the FNV-1a algorithm to produce a 64-bit seed.
    /// This allows using human-readable seeds like "dungeon-level-1".
    ///
    /// # Example
    ///
    /// ```
    /// use parrot::Parrot;
    ///
    /// let mut rng = Parrot::new_from_str("hello world");
    /// ```
    pub fn new_from_str(seed_str: &str) -> Self {
        let seed_u64 = fnv1a_64(seed_str.as_bytes());
        Self::new(seed_u64)
    }

    /// Creates a new RNG instance from a `u64` seed.
    ///
    /// This initializes the internal state using a SplitMix64-style step and performs
    /// a warmup phase to ensure the initial output is sufficiently random.
    ///
    /// If `seed` is `0`, it is treated as `1` to avoid the invalid all-zero state.
    ///
    /// # Example
    ///
    /// ```
    /// use parrot::Parrot;
    ///
    /// let mut rng = Parrot::new(42);
    /// ```
    pub fn new(seed: u64) -> Self {
        let mut rng = Parrot { state: [0; 2] };
        rng.state[0] = if seed > 0 { seed } else { 1 };

        rng.state[1] = seed.wrapping_mul(6364136223846793005);
        for _ in 0..10 {
            rng.next();
        }
        rng
    }

    #[inline(always)]
    pub fn next_64(&mut self) -> u64 {
        // Xoroxiro is multiplication based, a 0 seed can only generate 0
        let state0 = self.state[0];
        let mut state1 = self.state[1];
        let result = state0.wrapping_add(state1);

        state1 ^= state0;
        self.state[0] = state0.rotate_left(24) ^ state1 ^ (state1 << 16);
        self.state[1] = state1.rotate_left(37);

        result
    }

    /// Generates a random `u64`.
    ///
    /// This is a shortcut for `next()`.
    #[inline(always)]
    pub fn next(&mut self) -> u64 {
        self.next_64()
    }

    /// Generates a random integer in the range `[min, max)`.
    ///
    /// The result is inclusive of `min` and exclusive of `max`.
    ///
    /// # Panics
    ///
    /// Panics if `min >= max`.
    ///
    /// # Example
    ///
    /// ```
    /// use parrot::Parrot;
    ///
    /// let mut rng = Parrot::new(42);
    /// let n = rng.gen_range(10, 20); // 10 <= n < 20
    /// ```
    pub fn gen_range(&mut self, min: u64, max: u64) -> u64 {
        assert!(min < max, "min must be less than max");
        let range = max.wrapping_sub(min);
        let random_value = self.next();
        random_value % range + min
    }

    /// Generates a random floating-point number in the range `[0.0, 1.0)`.
    ///
    /// This implementation generates 53 bits of randomness for the significand,
    /// providing a uniform distribution.
    ///
    /// # Example
    ///
    /// ```
    /// use parrot::Parrot;
    ///
    /// let mut rng = Parrot::new(42);
    /// let f = rng.next_f64(); // 0.0 <= f < 1.0
    /// ```
    pub fn next_f64(&mut self) -> f64 {
        let random_value = self.next();
        (random_value >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[cfg(feature = "rand-support")]
use rand_core::{Error, RngCore};

#[cfg(feature = "rand-support")]
impl RngCore for Parrot {
    fn next_u32(&mut self) -> u32 {
        // We just truncate the u64. This is standard practice.
        self.next() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.next()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        // rand_core provides a helper to fill byte arrays efficiently using u64 chunks
        rand_core::impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}
