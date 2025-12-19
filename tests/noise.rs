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
        "Perlin noise from string seed is not deterministic"
    );
}
