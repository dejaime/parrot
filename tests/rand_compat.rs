#[cfg(feature = "rand-support")]
#[test]
fn test_compatibility_with_rand_ecosystem() {
    use parrot::Parrot;
    // We import this trait from the `rand` crate.
    // If Parrot didn't implement RngCore, this would fail to compile.
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
    println!("Original: {:?}", original);
    println!("Shuffled: {:?}", data);

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
}
