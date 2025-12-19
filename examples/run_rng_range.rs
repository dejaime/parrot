use parrot::Parrot;
use std::collections::BTreeMap;
use std::env;
use std::process;

fn main() {
    // 1. Collect arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: cargo run --example run_rng_range <seed_string> <min> <max> <count>");
        eprintln!("Example: cargo run --example run_rng_range test 1 11 100000");
        process::exit(1);
    }

    // 2. Parse arguments
    let seed_str = &args[1];
    let min: u64 = args[2].parse().expect("Invalid min");
    let max: u64 = args[3].parse().expect("Invalid max");
    let count: usize = args[4].parse().expect("Invalid count");

    if min >= max {
        eprintln!("Error: min must be less than max.");
        process::exit(1);
    }

    // 3. Initialize RNG
    let mut rng = Parrot::new_from_str(seed_str);
    let mut counts: BTreeMap<u64, usize> = BTreeMap::new();

    println!("🦜 Rolling {} times (Range: [{}, {}), Seed: \"{}\")...", count, min, max, seed_str);
    
    // 4. Run the simulation
    for _ in 0..count {
        let val = rng.gen_range(min, max);
        *counts.entry(val).or_insert(0) += 1;
    }

    // 5. Output Distribution
    println!("\n{:<10} | {:<10} | {:<10}", "Value", "Count", "Percent");
    println!("{:-<10}-+-{:-<10}-+-{:-<10}", "", "", "");

    for (val, &c) in &counts {
        let percent = (c as f64 / count as f64) * 100.0;
        println!("{:<10} | {:<10} | {:<6.2}%", val, c, percent);
    }
}
