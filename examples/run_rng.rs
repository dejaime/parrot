use parrot::Parrot;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Default to seed 42 and 1 iteration if no args provided
    let seed = if args.len() > 1 {
        args[1].parse().unwrap_or(42)
    } else {
        42
    };
    let iterations = if args.len() > 2 {
        args[2].parse().unwrap_or(1)
    } else {
        1
    };

    let mut rng = Parrot::new(seed);
    let mut last_val = 0;

    for _ in 0..iterations {
        last_val = rng.gen_range(0, 100);
    }

    println!("{last_val}");
}
