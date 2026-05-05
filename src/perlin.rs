use crate::Parrot;
use crate::hash::fnv1a_64;

/// A deterministic Perlin noise generator.
///
/// This generator produces continuous, gradient-based noise (a.k.a. "smooth" noise).
/// The output is deterministic, meaning it will always produce the same noise for a given seed.
/// The noise values are in the range `[-1.0, 1.0]`.
///
/// The implementation is based on the "Improved Perlin Noise" paper by Ken Perlin.
/// It is stateless, immutable, and thread-safe.
///
/// # Example
///
/// ```
/// use parrot::perlin::Perlin;
///
/// // Create a new Perlin noise generator with a seed
/// let perlin = Perlin::new(42);
///
/// // Generate a 2D noise value
/// let value = perlin.noise2d(0.5, 0.2);
///
/// println!("Noise value: {}", value);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-support", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde-support", serde(from = "PerlinSeed"))] // Use a specific proxy
pub struct Perlin {
    // 512 is too big for implicit copy.
    // 512 bytes is small enough for stack/embedded usage.
    // This table replaces the on-the-fly RNG calls during generation.
    perm: [u8; 512],
    seed: u64,
}

impl Perlin {
    /// Creates a new Perlin noise generator from a 64-bit seed.
    ///
    /// # Example
    ///
    /// ```
    /// use parrot::perlin::Perlin;
    ///
    /// let perlin = Perlin::new(123);
    /// ```
    pub fn new(seed: u64) -> Self {
        let mut rng = Parrot::new(seed);

        // 1. Initialize identity permutation 0..255
        let mut p = [0u8; 256];
        for (i, x) in p.iter_mut().enumerate() {
            *x = i as u8;
        }

        // 2. Shuffle using your RepeatableRand (Fisher-Yates shuffle)
        // This ensures the terrain "shape" is determined by the seed.
        for i in (1..256).rev() {
            // rng.gen_range is inclusive on min, exclusive on max
            // We cast to usize for array indexing.
            let r = rng.gen_range(0, (i + 1) as u64) as usize;
            p.swap(i, r);
        }

        // 3. Duplicate the array to avoid buffer overflow logic during lookup
        let mut perm = [0u8; 512];

        perm[0..256].copy_from_slice(&p);
        perm[256..512].copy_from_slice(&p);

        Perlin { perm, seed }
    }

    /// Creates a new Perlin noise generator from a string seed.
    ///
    /// The string is hashed using a `FNV-1a` hasher to produce a 64-bit seed.
    /// This is useful for "named" seeds, like in Minecraft.
    ///
    /// # Example
    ///
    /// ```
    /// use parrot::perlin::Perlin;
    ///
    /// let perlin = Perlin::new_from_string("hello world");
    /// ```
    pub fn new_from_string(seed: &str) -> Self {
        let hash = fnv1a_64(seed.as_bytes());
        Self::new(hash)
    }

    #[inline(always)]
    fn fade(t: f64) -> f64 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    #[inline(always)]
    fn lerp(t: f64, a: f64, b: f64) -> f64 {
        a + t * (b - a)
    }

    #[inline(always)]
    fn grad(hash: u8, x: f64, y: f64) -> f64 {
        // Standard "Improved Perlin" gradient calculation.
        // Bitwise logic replaces the slow branching if/else chains.
        let h = hash & 15;
        let u = if h < 8 { x } else { y };
        let v = if h < 4 {
            y
        } else if h == 12 || h == 14 {
            x
        } else {
            0.0
        };
        (if h & 1 == 0 { u } else { -u }) + (if h & 2 == 0 { v } else { -v })
    }

    // Your custom floor logic (kept as requested)
    fn floor(x: f64) -> f64 {
        let i = x as i64;
        if x < 0.0 && x != i as f64 {
            (i - 1) as f64
        } else {
            i as f64
        }
    }

    // Note: &self is now immutable. This is thread-safe and much faster to use.
    /// Generates a 2D Perlin noise value for the given coordinates.
    ///
    /// The input coordinates can be any `f64` values. The noise function
    /// will wrap around integer boundaries, so the pattern repeats indefinitely.
    ///
    /// The output value is always in the range `[-1.0, 1.0]`.
    ///
    /// # Example
    ///
    /// ```
    /// use parrot::perlin::Perlin;
    ///
    /// let perlin = Perlin::new(42);
    /// let value = perlin.noise2d(10.5, -3.2);
    /// ```
    pub fn noise2d(&self, x: f64, y: f64) -> f64 {
        let x_floor = Self::floor(x);
        let y_floor = Self::floor(y);

        // Mask with 255 to stay within the permutation table bounds
        let x_int = (x_floor as i32) & 255;
        let y_int = (y_floor as i32) & 255;

        let x_frac = x - x_floor;
        let y_frac = y - y_floor;

        let u = Self::fade(x_frac);
        let v = Self::fade(y_frac);

        // HASH LOOKUP:
        // We retrieve the corner gradients from our pre-shuffled table.
        // This guarantees coordinate (10,10) always has the same gradient.
        let p = &self.perm;

        // Note: The logic here relies on p being size 512, so adding (y_int + 1) never overflows.
        let aa = p[p[x_int as usize] as usize + y_int as usize];
        let ab = p[p[x_int as usize] as usize + y_int as usize + 1];
        let ba = p[p[x_int as usize + 1] as usize + y_int as usize];
        let bb = p[p[x_int as usize + 1] as usize + y_int as usize + 1];

        // Interpolate between the 4 corners
        Self::lerp(
            v,
            Self::lerp(
                u,
                Self::grad(aa, x_frac, y_frac),
                Self::grad(ba, x_frac - 1.0, y_frac),
            ),
            Self::lerp(
                u,
                Self::grad(ab, x_frac, y_frac - 1.0),
                Self::grad(bb, x_frac - 1.0, y_frac - 1.0),
            ),
        )
    }

    /// Generates a 2D Perlin noise value that seamlessly wraps (tiles) at the given periods.
    ///
    /// `wrap_x` and `wrap_y` must be > 0. The noise function will wrap around integer
    /// boundaries at the specified periods, creating a seamless repeating pattern.
    ///
    /// **Note:** Because the internal permutation table is 256 elements long, the wrap
    /// periods must be `<= 256`.
    ///
    /// The output value is always in the range `[-1.0, 1.0]`.
    ///
    /// # Example
    ///
    /// ```
    /// use parrot::perlin::Perlin;
    ///
    /// let perlin = Perlin::new(42);
    /// let value = perlin.noise2d_wrapped(10.5, -3.2, 10, 10);
    /// ```
    pub fn noise2d_wrapped(&self, x: f64, y: f64, wrap_x: i32, wrap_y: i32) -> f64 {
        assert!(
            wrap_x > 0 && wrap_x <= 256,
            "wrap_x must be in the range 1..=256"
        );
        assert!(
            wrap_y > 0 && wrap_y <= 256,
            "wrap_y must be in the range 1..=256"
        );

        let x_floor = Self::floor(x);
        let y_floor = Self::floor(y);

        let mut x0 = x_floor as i32;
        let mut y0 = y_floor as i32;
        let mut x1 = x0 + 1;
        let mut y1 = y0 + 1;

        // Wrap coordinates and mask with 255 to stay within the permutation table bounds
        x0 = x0.rem_euclid(wrap_x) & 255;
        x1 = x1.rem_euclid(wrap_x) & 255;
        y0 = y0.rem_euclid(wrap_y) & 255;
        y1 = y1.rem_euclid(wrap_y) & 255;

        let x_frac = x - x_floor;
        let y_frac = y - y_floor;

        let u = Self::fade(x_frac);
        let v = Self::fade(y_frac);

        let p = &self.perm;

        let aa = p[p[x0 as usize] as usize + y0 as usize];
        let ab = p[p[x0 as usize] as usize + y1 as usize];
        let ba = p[p[x1 as usize] as usize + y0 as usize];
        let bb = p[p[x1 as usize] as usize + y1 as usize];

        Self::lerp(
            v,
            Self::lerp(
                u,
                Self::grad(aa, x_frac, y_frac),
                Self::grad(ba, x_frac - 1.0, y_frac),
            ),
            Self::lerp(
                u,
                Self::grad(ab, x_frac, y_frac - 1.0),
                Self::grad(bb, x_frac - 1.0, y_frac - 1.0),
            ),
        )
    }
}

impl From<u64> for Perlin {
    fn from(seed: u64) -> Self {
        Self::new(seed)
    }
}

impl From<Perlin> for u64 {
    fn from(perlin: Perlin) -> Self {
        perlin.seed
    }
}

// Proxy used for serialization. We don't need to serialize the entire
//	512 state, only the seed.
#[cfg(feature = "serde-support")]
#[derive(serde::Serialize, serde::Deserialize)]
struct PerlinSeed(u64);

#[cfg(feature = "serde-support")]
impl From<Perlin> for PerlinSeed {
    fn from(p: Perlin) -> Self {
        PerlinSeed(p.seed)
    }
}

#[cfg(feature = "serde-support")]
impl From<PerlinSeed> for Perlin {
    fn from(s: PerlinSeed) -> Self {
        Perlin::new(s.0)
    }
}

#[cfg(feature = "serde-support")]
impl serde::Serialize for Perlin {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // We construct the proxy wrapper cheaply (it's just a u64)
        // and serialize that. No cloning of the 512-byte 'perm' array!
        PerlinSeed(self.seed).serialize(serializer)
    }
}
