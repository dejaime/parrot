use crate::hash::fnv1a_64;

/// A trait for types that can be generated within a range.
pub trait RandomRange {
    /// Generates a random value in the range [`min`, `max`).
    fn generate_range(rng: &mut Parrot, min: Self, max: Self) -> Self;
}

impl RandomRange for u64 {
    fn generate_range(rng: &mut Parrot, min: u64, max: u64) -> u64 {
        assert!(min < max, "min must be less than max");
        let range = max.wrapping_sub(min);
        let random_value = rng.next_u64();
        random_value % range + min
    }
}

impl RandomRange for i64 {
    fn generate_range(rng: &mut Parrot, min: i64, max: i64) -> i64 {
        assert!(min < max, "min must be less than max");
        let range = (max as u64).wrapping_sub(min as u64);
        let random_value = rng.next_u64();
        let offset = random_value % range;
        min.wrapping_add(offset as i64)
    }
}

impl RandomRange for i32 {
    fn generate_range(rng: &mut Parrot, min: i32, max: i32) -> i32 {
        assert!(min < max, "min must be less than max");
        let range = (max as u32).wrapping_sub(min as u32);
        let random_value = rng.next_u64() as u32;
        let offset = random_value % range;
        min.wrapping_add(offset as i32)
    }
}

impl RandomRange for u32 {
    fn generate_range(rng: &mut Parrot, min: u32, max: u32) -> u32 {
        assert!(min < max, "min must be less than max");
        let range = max.wrapping_sub(min);
        let random_value = rng.next_u32();
        random_value % range + min
    }
}

impl RandomRange for u8 {
    fn generate_range(rng: &mut Parrot, min: u8, max: u8) -> u8 {
        assert!(min < max, "Parrot::gen_range: min must be < max");
        let range = max.wrapping_sub(min);
        let random_value = rng.next_u32();
        (random_value % (range as u32)) as u8 + min
    }
}

impl RandomRange for u16 {
    fn generate_range(rng: &mut Parrot, min: u16, max: u16) -> u16 {
        assert!(min < max, "Parrot::gen_range: min must be < max");
        let range = max.wrapping_sub(min);
        let random_value = rng.next_u32();
        (random_value % (range as u32)) as u16 + min
    }
}

impl RandomRange for i8 {
    fn generate_range(rng: &mut Parrot, min: i8, max: i8) -> i8 {
        assert!(min < max, "Parrot::gen_range: min must be < max");
        let range = (max as u8).wrapping_sub(min as u8);
        let random_value = rng.next_u32();
        let offset = (random_value % (range as u32)) as u8;
        min.wrapping_add(offset as i8)
    }
}

impl RandomRange for i16 {
    fn generate_range(rng: &mut Parrot, min: i16, max: i16) -> i16 {
        assert!(min < max, "Parrot::gen_range: min must be < max");
        let range = (max as u16).wrapping_sub(min as u16);
        let random_value = rng.next_u32();
        let offset = (random_value % (range as u32)) as u16;
        min.wrapping_add(offset as i16)
    }
}

impl RandomRange for f64 {
    fn generate_range(rng: &mut Parrot, min: f64, max: f64) -> f64 {
        assert!(min < max, "min must be less than max");
        min + (max - min) * rng.next_f64()
    }
}

impl RandomRange for f32 {
    fn generate_range(rng: &mut Parrot, min: f32, max: f32) -> f32 {
        assert!(min < max, "min must be less than max");
        let range = max - min;
        min + range * (rng.next_f32())
    }
}

/// A strictly deterministic, lightweight random number generator.
///
/// `Parrot` uses the **Xoroshiro128\*\*** algorithm. It is designed to be:
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
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Parrot {
    state: [u64; 2],
}

/// A type alias for [`Parrot`].
pub type ParrotRng = Parrot;

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
            rng.next_u64();
        }
        rng
    }

    #[inline(always)]
    pub fn next_u64(&mut self) -> u64 {
        let state0 = self.state[0];
        let mut state1 = self.state[1];
        
        // The xoroshiro128** scrambler: multiply, rotate, multiply
        let result = state0.wrapping_mul(5).rotate_left(7).wrapping_mul(9);

        // The state transition remains completely identical
        state1 ^= state0;
        self.state[0] = state0.rotate_left(24) ^ state1 ^ (state1 << 16);
        self.state[1] = state1.rotate_left(37);

        result
    }

    /// Generates a random `u32`.
    pub fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    /// Generates a random `i64`.
    pub fn next_i64(&mut self) -> i64 {
        self.next_u64() as i64
    }

    /// Generates a random `i32`.
    pub fn next_i32(&mut self) -> i32 {
        self.next_u64() as i32
    }

    /// Generates a random value in the range `[min, max)`.
    ///
    /// The result is inclusive of `min` and exclusive of `max`.
    /// Supports `u64`, `i64`, and `i32`.
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
    /// let n = rng.gen_range(10, 20); // 10 <= n < 20 (inferred as i32)
    /// ```
    pub fn gen_range<T: RandomRange>(&mut self, min: T, max: T) -> T {
        T::generate_range(self, min, max)
    }

    /// Generates a random value in the range `[min, max)` using a `Range` object.
    ///
    /// This is a convenience method for `gen_range` that accepts a `Range`.
    ///
    /// # Example
    ///
    /// ```
    /// use parrot::Parrot;
    ///
    /// let mut rng = Parrot::new(42);
    /// let n = rng.gen_in_range(10..20);
    /// ```
    pub fn gen_in_range<T: RandomRange>(&mut self, range: core::ops::Range<T>) -> T {
        self.gen_range(range.start, range.end)
    }

    /// Generates a boolean value with a specified probability of being true.
    ///
    /// # Example
    ///
    /// ```
    /// use parrot::Parrot;
    ///
    /// let mut rng = Parrot::new(42);
    /// let is_crit = rng.gen_bool(0.05); // 5% chance
    /// ```
    pub fn gen_bool(&mut self, probability: f64) -> bool {
        self.next_f64() < probability
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
        let random_value = self.next_u64();
        (random_value >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Generates a random `f32` in the range `[0.0, 1.0)`.
    ///
    /// Technically, the max value generated would be 0.99999976...
    ///
    /// This implementation uses 22 bits of randomness (dropping 2 bits of precision)
    /// to reduce floating point variation between platforms (effectively making it an "f30").
    pub fn next_f32(&mut self) -> f32 {
        let random_value = self.next_u64();
        (random_value >> 42) as f32 / (1u64 << 22) as f32
    }
}

#[cfg(feature = "rand-support")]
use rand_core::{Error, RngCore, SeedableRng};

#[cfg(feature = "rand-support")]
impl RngCore for Parrot {
    fn next_u32(&mut self) -> u32 {
        // We just truncate the u64. This is standard practice.
        self.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.next_u64()
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

#[cfg(feature = "rand-support")]
impl SeedableRng for Parrot {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        let s0 = u64::from_le_bytes(seed[0..8].try_into().unwrap());
        let s1 = u64::from_le_bytes(seed[8..16].try_into().unwrap());

        if s0 == 0 && s1 == 0 {
            return Parrot::new(0);
        }

        Parrot { state: [s0, s1] }
    }

    fn seed_from_u64(state: u64) -> Self {
        Self::new(state)
    }
}
