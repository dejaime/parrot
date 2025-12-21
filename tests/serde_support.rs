#![cfg(feature = "serde-support")]

use parrot::{Parrot, Perlin};
// Ensure you have serde_json in [dev-dependencies]
use serde_json;

#[test]
fn test_parrot_state_restoration() {
    let mut rng1 = Parrot::new(42);

    // 1. Advance the state slightly
    let _ = rng1.next_u64();
    let _ = rng1.next_u64();

    // 2. Serialize to JSON
    let serialized = serde_json::to_string(&rng1).expect("Failed to serialize Parrot");

    // 3. Deserialize into a new instance
    let mut rng2: Parrot = serde_json::from_str(&serialized).expect("Failed to deserialize Parrot");

    // 4. Verify both continue generating the EXACT same sequence
    assert_eq!(rng1.next_u64(), rng2.next_u64());
    assert_eq!(rng1.next_u64(), rng2.next_u64());
    assert_eq!(rng1.next_f64(), rng2.next_f64());
}

#[test]
fn test_perlin_reconstruction() {
    // 1. Create noise with a specific seed
    let perlin1 = Perlin::new(12345);

    // 2. Serialize
    // This tests the Proxy Pattern: strictly speaking, the output JSON
    // should be very short (just the seed), not a huge array.
    let serialized = serde_json::to_string(&perlin1).expect("Failed to serialize Perlin");
    println!("Perlin Serialized: {}", serialized); // Should look like: {"seed":12345}

    // 3. Deserialize
    // This triggers the From<PerlinSeed> logic which regenerates the 512-byte table.
    let perlin2: Perlin = serde_json::from_str(&serialized).expect("Failed to deserialize Perlin");

    // 4. Verify the table was regenerated correctly by checking noise output
    assert_eq!(
        perlin1.noise2d(10.5, 20.5),
        perlin2.noise2d(10.5, 20.5),
        "Noise values do not match after deserialization!"
    );
}
