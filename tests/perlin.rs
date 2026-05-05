use parrot::Perlin;

// 5. NOISE REPEATABILITY
#[test]
fn test_noise_repeatability() {
    let noise1 = Perlin::new_from_string("test-seed");
    let noise2 = Perlin::new_from_string("test-seed");
    assert_eq!(noise1.noise2d(10.5, 20.1), noise2.noise2d(10.5, 20.1));
}

#[test]
fn test_noise_string_seeds() {
    // Perlin Noise String Seed
    let noise_str = Perlin::new_from_string("parrot");
    let noise_val = noise_str.noise2d(1.0, 2.0);
    assert_eq!(
        noise_str.noise2d(1.0, 2.0),
        noise_val,
        "TEST Failed: Perlin noise from string seed was not deterministic"
    );
}

#[test]
fn test_noise_wrapping() {
    let noise = Perlin::new_from_string("wrapping-seed");

    let wrap_x = 10;
    let wrap_y = 10;

    let val1 = noise.noise2d_wrapped(1.5, 2.5, wrap_x, wrap_y);
    // Exact same relative position but one period to the right and up
    let val2 = noise.noise2d_wrapped(11.5, 12.5, wrap_x, wrap_y);
    // Same relative position but negative wrap offset
    let val3 = noise.noise2d_wrapped(-8.5, -7.5, wrap_x, wrap_y);

    assert_eq!(val1, val2, "Wrapped noise should repeat perfectly at the given period");
    assert_eq!(val1, val3, "Wrapped noise should handle negative wrapping correctly");

    // 1. Proof of Difference
    // Prove standard noise does NOT naturally repeat at a period of 10
    assert_ne!(
        noise.noise2d(1.5, 2.5),
        noise.noise2d(11.5, 12.5),
        "Standard noise should not accidentally repeat at period 10"
    );

    // 2. Asymmetric and Multi-Period Wrapping
    let asym_val1 = noise.noise2d_wrapped(3.1, 4.2, 10, 20);
    // x is 3 periods away (+30), y is -2 periods away (-40)
    let asym_val2 = noise.noise2d_wrapped(33.1, -35.8, 10, 20); 
    assert!((asym_val1 - asym_val2).abs() < 1e-10, "Asymmetric and multi-period wrapping should repeat correctly. Left: {}, Right: {}", asym_val1, asym_val2);

    // 3. The 256 Equivalence
    // Standard noise inherently wraps at 256. Verify our wrapped logic perfectly mirrors it.
    let std_val = noise.noise2d(300.7, -45.3);
    let wrap_256_val = noise.noise2d_wrapped(300.7, -45.3, 256, 256);
    assert_eq!(std_val, wrap_256_val, "noise2d_wrapped at period 256 should match standard noise2d");
}
