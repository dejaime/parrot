#![cfg(feature = "bevy-support")]

use bevy::prelude::*;
use parrot::{Parrot, ParrotBevyExt};

#[test]
fn test_bevy_determinism() {
    // Two parrots with the same seed must generate the same Bevy types
    let mut rng1 = Parrot::new(999);
    let mut rng2 = Parrot::new(999);

    let v1: Vec3 = rng1.gen_range(Vec3::ZERO, Vec3::ONE);
    let v2: Vec3 = rng2.gen_range(Vec3::ZERO, Vec3::ONE);
    assert_eq!(v1, v2);

    let v3: Vec3 = rng1.gen_range(Vec3::ZERO, Vec3::ONE);
    let v4: Vec3 = rng2.gen_range(Vec3::ZERO, Vec3::ONE);
    assert_eq!(v3, v4);
}

#[test]
fn test_geometric_bounds() {
    let mut rng = Parrot::new(123);

    // Test Vec2 range
    let min = Vec2::new(10.0, 10.0);
    let max = Vec2::new(20.0, 20.0);
    for _ in 0..10 {
        let v: Vec2 = rng.gen_range(min, max);
        assert!(v.x >= 10.0 && v.x < 20.0);
        assert!(v.y >= 10.0 && v.y < 20.0);
    }

    // Test Point in Circle (Radius check)
    for _ in 0..10 {
        let p = rng.gen_point_in_circle(5.0);
        assert!(p.length() <= 5.0001); // Float tolerance
    }

    // Test Point in Sphere (Radius check)
    for _ in 0..10 {
        let p = rng.gen_point_in_sphere(3.0);
        assert!(p.length() <= 3.0001);
    }
}

#[test]
fn test_directions_are_normalized() {
    let mut rng = Parrot::new(555);

    // Dir2 and Dir3 should always be length 1.0
    for _ in 0..10 {
        let d2 = rng.gen_dir2();
        // Dir2 usually implements Deref<Target=Vec2>
        assert!((d2.length() - 1.0).abs() < 1e-5);

        let d3 = rng.gen_dir3();
        assert!((d3.length() - 1.0).abs() < 1e-5);
    }
}

#[test]
fn test_rotations() {
    let mut rng = Parrot::new(777);

    // Just verify quats are normalized (valid rotations)
    for _ in 0..10 {
        let q = rng.gen_quat();
        assert!((q.length() - 1.0).abs() < 1e-5);
    }
}
