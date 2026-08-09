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

    assert_eq!(
        val1, val2,
        "Wrapped noise should repeat perfectly at the given period"
    );
    assert_eq!(
        val1, val3,
        "Wrapped noise should handle negative wrapping correctly"
    );

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
    assert!(
        (asym_val1 - asym_val2).abs() < 1e-10,
        "Asymmetric and multi-period wrapping should repeat correctly. Left: {}, Right: {}",
        asym_val1,
        asym_val2
    );

    // 3. The 256 Equivalence
    // Standard noise inherently wraps at 256. Verify our wrapped logic perfectly mirrors it.
    let std_val = noise.noise2d(300.7, -45.3);
    let wrap_256_val = noise.noise2d_wrapped(300.7, -45.3, 256, 256);
    assert_eq!(
        std_val, wrap_256_val,
        "noise2d_wrapped at period 256 should match standard noise2d"
    );
}

// 6. 3D NOISE
#[test]
fn test_noise3d_repeatability() {
    let noise1 = Perlin::new_from_string("test-seed");
    let noise2 = Perlin::new_from_string("test-seed");
    assert_eq!(
        noise1.noise3d(10.5, 20.1, -4.7),
        noise2.noise3d(10.5, 20.1, -4.7)
    );
}

#[test]
fn test_noise3d_is_zero_on_the_lattice() {
    // Gradient noise is zero at every lattice point by construction: each
    // corner's offset vector is zero there. This is the cheapest check that the
    // eight corners are wired to the right hashes.
    let noise = Perlin::new(42);
    for x in -3..3 {
        for y in -3..3 {
            for z in -3..3 {
                let val = noise.noise3d(x as f64, y as f64, z as f64);
                assert!(
                    val.abs() < 1e-12,
                    "noise3d({x}, {y}, {z}) was {val}, expected 0.0"
                );
            }
        }
    }
}

#[test]
fn test_noise3d_range_and_spread() {
    let noise = Perlin::new(1234);
    let (mut min, mut max) = (f64::MAX, f64::MIN);
    let mut sum = 0.0;
    let samples = 100_000;
    for i in 0..samples {
        let t = i as f64;
        let val = noise.noise3d(t * 0.0137, t * 0.0219, t * 0.0071);
        assert!(
            (-1.0..=1.0).contains(&val),
            "noise3d escaped [-1, 1] with {val}"
        );
        min = min.min(val);
        max = max.max(val);
        sum += val;
    }
    // A field pinned near zero would satisfy the bound above and be useless.
    assert!(
        min < -0.5 && max > 0.5,
        "noise3d only spanned {min}..{max}, which is not a usable field"
    );
    let mean = sum / samples as f64;
    assert!(mean.abs() < 0.05, "noise3d is biased: mean was {mean}");
}

#[test]
fn test_noise3d_is_continuous() {
    // Neighbouring samples must not jump: a discontinuity in the field is a
    // cliff in whatever terrain is generated from it.
    let noise = Perlin::new_from_string("continuity");
    for i in 0..5_000 {
        let t = i as f64 * 0.037;
        let (x, y, z) = (t, t * 0.61, t * -0.29);
        let step = 1e-4;
        let delta = (noise.noise3d(x + step, y, z) - noise.noise3d(x, y, z)).abs();
        assert!(
            delta < 0.01,
            "noise3d jumped {delta} across a step of {step} near ({x}, {y}, {z})"
        );
    }
}

#[test]
fn test_noise3d_varies_along_every_axis() {
    // A field that silently ignored one of its inputs would pass every test
    // above, so check each axis moves the value on its own.
    //
    // Sampled over many points rather than one: the first version of this test
    // compared the cube centres (0.5, 0.5, 0.5) and (0.5, 0.5, 1.5) and failed,
    // because at a cube centre every fade is exactly 0.5 and the result is a
    // plain mean of the eight corner gradients — a value that collides
    // readily. The field was fine; the test was measuring one unlucky point.
    let noise = Perlin::new(99);
    for (axis, delta) in [
        ("x", (1.0, 0.0, 0.0)),
        ("y", (0.0, 1.0, 0.0)),
        ("z", (0.0, 0.0, 1.0)),
    ] {
        let mut differing = 0;
        let samples = 500;
        for i in 0..samples {
            let (x, y, z) = (i as f64 * 0.31, i as f64 * -0.17, i as f64 * 0.23);
            let moved = noise.noise3d(x + delta.0 * 0.37, y + delta.1 * 0.37, z + delta.2 * 0.37);
            if (moved - noise.noise3d(x, y, z)).abs() > 1e-9 {
                differing += 1;
            }
        }
        assert!(
            differing > samples * 9 / 10,
            "moving along {axis} changed the field in only {differing}/{samples} samples"
        );
    }
}
