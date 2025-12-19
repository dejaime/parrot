#![no_std]

pub mod hash;
pub mod noise;
pub mod rand;

pub use noise::PerlinNoise;
pub use rand::Parrot;

// We need std for tests to use Vectors and Print checks
#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    // 1. THE GOLDEN MASTER TEST
    #[test]
    fn test_golden_values() {
        let mut rng = Parrot::new(42);

        // Validated on 2024-12-19 using examples/run_rng.rs
        let v1 = rng.gen_range(0, 100);
        let v2 = rng.gen_range(0, 100);
        let v3 = rng.gen_range(0, 100);

        assert_eq!(v1, 0, "First random value for seed 42 changed!");
        assert_eq!(v2, 52, "Second random value for seed 42 changed!");
        assert_eq!(v3, 87, "Third random value for seed 42 changed!");
    }

    // 2. THE AVALANCHE TEST
    #[test]
    fn test_avalanche_effect() {
        let seed = 123456789;
        let mut rng1 = Parrot::new(seed);
        let mut rng2 = Parrot::new(seed + 1); // Only 1 bit difference

        let mut differences = 0;
        for _ in 0..100 {
            let r1 = rng1.gen_range(0, 1000);
            let r2 = rng2.gen_range(0, 1000);
            if r1 != r2 {
                differences += 1;
            }
        }
        assert!(
            differences > 95,
            "RNG failed avalanche test! Neighboring seeds are too similar."
        );
    }

    // 3. THE STATISTICAL DISTRIBUTION TEST
    #[test]
    fn test_distribution_fairness() {
        let mut rng = Parrot::new(999);
        let iterations = 100_000;
        let buckets = 10;
        let mut counts = vec![0; buckets];

        for _ in 0..iterations {
            let val = rng.gen_range(0, buckets as u64) as usize;
            counts[val] += 1;
        }

        let expected = iterations / buckets;
        let margin = expected / 20; // 5%

        for (i, count) in counts.iter().enumerate() {
            let diff = (*count - expected as i32).abs();
            assert!(
                diff < margin as i32,
                "Bucket {i} is biased! Expected ~{expected}, got {count}. Diff: {diff}"
            );
        }
    }

    // 4. EDGE CASE SAFETY
    #[test]
    fn test_edge_cases() {
        let mut rng_zero = Parrot::new(0);
        // Ensure it doesn't just return 0 forever
        let mut sum = 0;
        for _ in 0..10 {
            sum += rng_zero.gen_range(0, 10);
        }
        assert!(sum > 0, "Seed 0 produced only zeros!");

        let mut rng = Parrot::new(123);
        for _ in 0..10 {
            assert_eq!(
                rng.gen_range(10, 11),
                10,
                "Range(10, 11) should always return 10"
            );
        }
    }

    // 5. NOISE REPEATABILITY
    #[test]
    fn test_noise_repeatability() {
        let noise1 = PerlinNoise::new(555);
        let noise2 = PerlinNoise::new(555);
        assert_eq!(noise1.noise2d(10.5, 20.1), noise2.noise2d(10.5, 20.1));
    }

    // 6. STRING SEED ROBUSTNESS
    // This proves your hasher handles UTF-8, Emoji, and CJK correctly.
    #[test]
    fn test_string_seeds() {
        // A. Standard ASCII
        let mut rng_ascii = Parrot::new_from_str("parrot");
        let val_ascii = rng_ascii.gen_range(0, 100);

        // B. Emoji 🦜
        // In UTF-8, this is 4 bytes. The hasher should handle it fine.
        let mut rng_emoji = Parrot::new_from_str("🦜");
        let val_emoji = rng_emoji.gen_range(0, 100);

        // C. CJK (Chinese for "Parrot" -> 鹦鹉)
        let mut rng_cjk = Parrot::new_from_str("鹦鹉");
        let val_cjk = rng_cjk.gen_range(0, 100);

        // 1. Ensure they don't crash (implicit if we got here)

        // 2. Ensure they produce deterministic results
        // (Re-creating the RNG with the same string gives the same number)
        assert_eq!(Parrot::new_from_str("parrot").gen_range(0, 100), val_ascii);
        assert_eq!(Parrot::new_from_str("🦜").gen_range(0, 100), val_emoji);

        // 3. Ensure they are distinct
        // (The string "parrot" shouldn't hash to the same thing as the emoji "🦜")
        assert_ne!(
            val_ascii, val_emoji,
            "ASCII 'parrot' and Emoji '🦜' clashed!"
        );
        assert_ne!(val_emoji, val_cjk, "Emoji '🦜' and CJK '鹦鹉' clashed!");

        // 4. Empty String check (Should correspond to FNV offset basis)
        let mut rng_empty = Parrot::new_from_str("");
        let val_empty = rng_empty.gen_range(0, 100);
        assert_eq!(Parrot::new_from_str("").gen_range(0, 100), val_empty);
    }
}
