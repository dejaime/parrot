use crate::hash::fnv1a_64;

pub struct Parrot {
    state: [u64; 2],
}

impl Parrot {
    pub fn new_from_str(seed_str: &str) -> Self {
        let seed_u64 = fnv1a_64(seed_str.as_bytes());
        Self::new(seed_u64)
    }

    pub fn new(seed: u64) -> Self {
        let mut rng = Parrot { state: [0; 2] };
        rng.state[0] = if seed > 0 { seed } else { 1 };

        rng.state[1] = seed.wrapping_mul(6364136223846793005);
        for _ in 0..10 {
            rng.next();
        }
        rng
    }

    #[inline(always)]
    fn next(&mut self) -> u64 {
        // Xoroxiro is multiplication based, a 0 seed can only generate 0
        let state0 = self.state[0];
        let mut state1 = self.state[1];
        let result = state0.wrapping_add(state1);

        state1 ^= state0;
        self.state[0] = state0.rotate_left(24) ^ state1 ^ (state1 << 16);
        self.state[1] = state1.rotate_left(37);

        result
    }

    pub fn gen_range(&mut self, min: u64, max: u64) -> u64 {
        assert!(min < max, "min must be less than max");
        let range = max.wrapping_sub(min);
        let random_value = self.next();
        random_value % range + min
    }

    pub fn gen_f64(&mut self) -> f64 {
        let random_value = self.next();
        (random_value >> 11) as f64 / (1u64 << 53) as f64
    }
}
