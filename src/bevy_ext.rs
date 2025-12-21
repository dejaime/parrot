use crate::Parrot;
use crate::RandomRange;
use bevy::prelude::*;

/// Extension methods for generating Bevy-specific types
pub trait ParrotBevyExt {
    /// Generates a random standard Color (sRGB).
    fn gen_color_srgb(&mut self) -> Color;

    /// Generates a random HSLA color.
    fn gen_color_hsla(
        &mut self,
        hue_range: Vec2,
        saturation_range: Vec2,
        lightness_range: Vec2,
        alpha: f32,
    ) -> Color;

    // Generates a random Oklcha color.
    fn gen_color_oklcha(
        &mut self,
        l_range: Vec2,
        c_range: Vec2,
        h_range: Vec2,
        alpha: f32,
    ) -> Color;

    /// Generates a random uniform rotation.
    fn gen_quat(&mut self) -> Quat;

    /// Generates a random 2D direction.
    fn gen_dir2(&mut self) -> Dir2;

    /// Generates a random 3D direction.
    fn gen_dir3(&mut self) -> Dir3;

    // Generates a point in a circle within `radius`.
    fn gen_point_in_circle(&mut self, radius: f32) -> Vec2;

    // Generates a point in a sphere within `radius`.
    fn gen_point_in_sphere(&mut self, radius: f32) -> Vec3;
}

impl ParrotBevyExt for Parrot {
    fn gen_color_srgb(&mut self) -> Color {
        // Generate random RGB float components [0.0, 1.0]
        Color::srgb(self.next_f32(), self.next_f32(), self.next_f32())
    }

    fn gen_color_hsla(
        &mut self,
        hue_range: Vec2,
        saturation_range: Vec2,
        lightness_range: Vec2,
        alpha: f32,
    ) -> Color {
        Color::hsla(
            self.gen_range(hue_range.x, hue_range.y),
            self.gen_range(saturation_range.x, saturation_range.y),
            self.gen_range(lightness_range.x, lightness_range.y),
            alpha,
        )
    }

    fn gen_color_oklcha(&mut self, l: Vec2, c: Vec2, h: Vec2, a: f32) -> Color {
        Color::oklcha(
            self.gen_range(l.x, l.y),
            self.gen_range(c.x, c.y),
            self.gen_range(h.x, h.y),
            a,
        )
    }

    fn gen_quat(&mut self) -> Quat {
        // Shoemake (1992) Uniform Random Rotations
        // Generates a quaternion uniformly distributed on the unit hypersphere S^3.
        let u0 = self.next_f32();
        let u1 = self.next_f32();
        let u2 = self.next_f32();

        let r = (1.0 - u0).sqrt();
        let s = u0.sqrt();
        let tau = std::f32::consts::TAU;

        Quat::from_xyzw(
            r * (tau * u1).sin(),
            r * (tau * u1).cos(),
            s * (tau * u2).sin(),
            s * (tau * u2).cos(),
        )
    }

    fn gen_dir2(&mut self) -> Dir2 {
        let angle = self.next_f32() * std::f32::consts::TAU;
        Dir2::new(Vec2::from_angle(angle)).expect(
            "Parrot Bevy: Sine and Cosine should never be simultaneously zero for any given angle",
        )
    }

    fn gen_dir3(&mut self) -> Dir3 {
        let z = self.gen_range(-1.0f32, 1.0f32);
        let phi = self.gen_range(0.0f32, std::f32::consts::TAU);
        let r = (1.0 - z * z).sqrt();
        let x = r * phi.cos();
        let y = r * phi.sin();
        Dir3::new(Vec3::new(x, y, z))
			.expect("Parrot Bevy: Sperical Coordinates applied to a unit sphere should only yield vectors with length 1")
    }

    fn gen_point_in_circle(&mut self, radius: f32) -> Vec2 {
        let r = radius * (self.next_f32()).sqrt();
        let theta = self.next_f32() * std::f32::consts::TAU;
        Vec2::new(r * theta.cos(), r * theta.sin())
    }

    fn gen_point_in_sphere(&mut self, radius: f32) -> Vec3 {
        let r = radius * (self.next_f32()).cbrt();
        let dir = self.gen_dir3();
        dir.as_vec3() * r
    }
}

impl RandomRange for IVec2 {
    fn generate_range(rng: &mut Parrot, min: Self, max: Self) -> Self {
        IVec2::new(rng.gen_range(min.x, max.x), rng.gen_range(min.y, max.y))
    }
}

impl RandomRange for Vec2 {
    fn generate_range(rng: &mut Parrot, min: Self, max: Self) -> Self {
        Vec2::new(rng.gen_range(min.x, max.x), rng.gen_range(min.y, max.y))
    }
}

impl RandomRange for Vec3 {
    fn generate_range(rng: &mut Parrot, min: Self, max: Self) -> Self {
        Vec3::new(
            rng.gen_range(min.x, max.x),
            rng.gen_range(min.y, max.y),
            rng.gen_range(min.z, max.z),
        )
    }
}

impl RandomRange for IVec3 {
    fn generate_range(rng: &mut Parrot, min: Self, max: Self) -> Self {
        IVec3::new(
            rng.gen_range(min.x, max.x),
            rng.gen_range(min.y, max.y),
            rng.gen_range(min.z, max.z),
        )
    }
}

impl RandomRange for UVec2 {
    fn generate_range(rng: &mut Parrot, min: Self, max: Self) -> Self {
        UVec2::new(rng.gen_range(min.x, max.x), rng.gen_range(min.y, max.y))
    }
}

impl RandomRange for UVec3 {
    fn generate_range(rng: &mut Parrot, min: Self, max: Self) -> Self {
        UVec3::new(
            rng.gen_range(min.x, max.x),
            rng.gen_range(min.y, max.y),
            rng.gen_range(min.z, max.z),
        )
    }
}
