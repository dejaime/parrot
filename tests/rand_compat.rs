#[cfg(feature = "rand-support")]
#[test]
fn test_compatibility_with_rand_ecosystem() {
    use parrot::Parrot;
    // We import this trait from the `rand` crate.
    // If Parrot didn't implement RngCore, this would fail to compile.
    use rand::SeedableRng;
    use rand::seq::SliceRandom;

    // 1. Create our Parrot RNG
    let mut rng = Parrot::new(42);

    // 2. Define a dataset
    let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let original = data.clone();

    // 3. Use a standard `rand` method (shuffle)
    // This is the moment of truth: .shuffle() requires &mut impl RngCore
    data.shuffle(&mut rng);

    // 4. Verify it actually did something
    println!("Original: {original:?}");
    println!("Shuffled: {data:?}");

    assert_ne!(data, original, "The vector should be shuffled!");

    // 5. Verify Determinism is maintained through the wrapper
    // Create a second RNG with same seed
    let mut rng2 = Parrot::new(42);
    let mut data2 = original.clone();

    // Shuffle the second vector
    data2.shuffle(&mut rng2);

    assert_eq!(
        data, data2,
        "Shuffling should be deterministic given the same seed"
    );

    // 6. Test "Splitting" (SeedableRng::from_rng)
    // This allows creating a new independent RNG from an existing one
    let mut child_rng = Parrot::from_rng(&mut rng).expect("Failed to create RNG from RNG");
    let _val = child_rng.next_u64();
    assert_ne!(rng.next_u64(), child_rng.next_u64());
}
