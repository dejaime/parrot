use parrot::Parrot;

// 1. THE GOLDEN MASTER TEST
#[test]
fn test_golden_values() {
    let mut rng = Parrot::new(42);

    // Validated on 2024-12-19 using examples/run_rng.rs
    let v1: u64 = rng.gen_range(0, 100);
    let v2: u64 = rng.gen_range(0, 100);
    let v3: u64 = rng.gen_range(0, 100);

    assert_eq!(v1, 52, "First random value for seed 42 changed!");
    assert_eq!(v2, 80, "Second random value for seed 42 changed!");
    assert_eq!(v3, 11, "Third random value for seed 42 changed!");
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

// 6. STRING SEED ROBUSTNESS (Parrot RNG part)
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

    // 1. Ensure they produce deterministic results
    // (Re-creating the RNG with the same string gives the same number)
    assert_eq!(Parrot::new_from_str("parrot").gen_range(0, 100), val_ascii);
    assert_eq!(Parrot::new_from_str("🦜").gen_range(0, 100), val_emoji);

    // 2. Ensure they are distinct
    // (The string "parrot" shouldn't hash to the same thing as the emoji "🦜")
    assert_ne!(
        val_ascii, val_emoji,
        "ASCII 'parrot' and Emoji '🦜' clashed!"
    );
    assert_ne!(val_emoji, val_cjk, "Emoji '🦜' and CJK '鹦鹉' clashed!");

    // 3. Empty String check (Should correspond to FNV offset basis)
    let mut rng_empty = Parrot::new_from_str("");
    let val_empty = rng_empty.gen_range(0, 100);
    assert_eq!(Parrot::new_from_str("").gen_range(0, 100), val_empty);
}

// 7. ALL TYPE IMPLEMENTATIONS
// This tests whether we broke any of the different supported type implementations.
#[test]
fn test_all_types_golden_master() {
    // 1. Setup: Fixed seed 42
    let mut rng = Parrot::new(42);

    // Warm the state up with a few rolls
    rng.next_u64();
    rng.next_u32();
    rng.next_f64();
    rng.next_i64();
    rng.next_i32();

    // 2. Raw Generation Methods
    let val_u64 = rng.next_u64();
    let val_u32 = rng.next_u32();
    let val_f64 = rng.next_f64();
    let val_i64 = rng.next_i64();
    let val_i32 = rng.next_i32();

    // 3. Range Generation
    // Unsigned
    let r_u8 = rng.gen_range(0u8, 255u8);
    let r_u16 = rng.gen_range(0u16, 60000u16);
    let r_u32 = rng.gen_range(0u32, 1_000_000u32);
    let r_u64 = rng.gen_range(0u64, 1_000_000_000u64);

    // Signed (Checking negative handling)
    let r_i8 = rng.gen_range(-128i8, 127i8);
    let r_i16 = rng.gen_range(-30000i16, 30000i16);
    let r_i32 = rng.gen_range(-1_000_000i32, 1_000_000i32);
    let r_i64 = rng.gen_range(-1_000_000_000i64, 1_000_000_000i64);

    // 4. THE ASSERTIONS
    assert_eq!(val_u64, 6_015_211_170_407_401_693);
    assert_eq!(val_u32, 1_217_301_087);
    assert_eq!(val_f64, 0.14723110273312523);
    assert_eq!(val_i64, 466_446_975_417_854_016);
    assert_eq!(val_i32, 370_039_161);
    assert_eq!(r_u8, 115);
    assert_eq!(r_u16, 49_597);
    assert_eq!(r_u32, 542_147);
    assert_eq!(r_u64, 270_835_505);
    assert_eq!(r_i8, -57);
    assert_eq!(r_i16, 14_089);
    assert_eq!(r_i32, -293_073);
    assert_eq!(r_i64, -732_041_528);
}

#[test]
fn test_long_sequence_integrity() {
    // 1. Setup with the standard seed
    let mut rng = Parrot::new(42);

    // 2. Accumulators for our checksum
    // We use a simple custom rolling hash to avoid external dependencies like CRC32.
    // This ensures that order matters (e.g., [A, B] produces a different hash than [B, A]).
    let mut hash: u64 = 0;

    // 3. Generate a large sequence (10,000 iterations)
    for _ in 0..10_000 {
        let val = rng.next_u64();
        // A. XOR the value into the hash
        hash ^= val;
        // B. Rotate to ensure position dependence (so 0x1 then 0x2 != 0x2 then 0x1)
        hash = hash.rotate_left(7);
        // C. A wrapping add to mix bits further
        hash = hash.wrapping_add(0x9E3779B97F4A7C15); // Golden Ratio constant
    }

    let expected_hash = 15370814124256169305;
    assert_eq!(
        hash, expected_hash,
        "Long-sequence checksum mismatch! The RNG algorithm has changed."
    );
}

#[test]
fn test_parrot_rng_alias() {
    use parrot::{Parrot, ParrotRng};

    // 1. Instantiate via Alias
    let mut rng: ParrotRng = ParrotRng::new(42);

    // 2. Verify behavior matches Parrot (Golden Value check)
    // First value for seed 42 in range [0, 100) is 52.
    assert_eq!(rng.gen_range(0u64, 100u64), 52);

    // 3. Verify Type Identity
    // Since it's a type alias, we should be able to assign it to a variable of type Parrot
    let mut rng_base: Parrot = rng;
    assert_eq!(rng_base.gen_range(0u64, 100u64), 80);
}
