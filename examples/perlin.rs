use parrot::Perlin;
use std::env;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    // 1. Parse Arguments
    // Usage: cargo run --example perlin [SEED_STRING] [WIDTH] [HEIGHT]
    let args: Vec<String> = env::args().collect();

    // Get the seed string or default to "parrot"
    let seed_input = args.get(1).map(|s| s.as_str()).unwrap_or("parrot");
    let width: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(60);
    let height: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(30);

    // 2. Resolve Seed (String -> u64)
    // If it's a number ("12345"), use it. If it's text ("hello"), hash it.
    let seed: u64 = match seed_input.parse() {
        Ok(n) => n,
        Err(_) => {
            let mut hasher = DefaultHasher::new();
            seed_input.hash(&mut hasher);
            hasher.finish()
        }
    };

    let perlin = Perlin::new(seed);
    let scale = 0.15;

    println!("Parrot Perlin Noise Demo");
    println!("Input: \"{}\" (Seed: {})", seed_input, seed);
    println!("Size: {}x{}", width, height);
    println!("{:-<1$}", "", width);

    // 3. Generate the map
    for y in 0..height {
        for x in 0..width {
            let value = perlin.noise2d(x as f64 * scale, y as f64 * scale);

            let symbol = if value > 0.6 {
                '▲' // Peak
            } else if value > 0.3 {
                '∩' // Mountain
            } else if value > 0.0 {
                '.' // Grass
            } else if value > -0.3 {
                '~' // Shallow Water
            } else {
                ' ' // Deep Water
            };

            print!("{}", symbol);
        }
        println!();
    }
}
