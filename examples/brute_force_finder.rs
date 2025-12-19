use num_cpus::get;
use std::env;
use std::sync::mpsc;
use std::thread;

// Use your library
use parrot::Parrot;

fn main() {
    // 1. Parse CLI arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --example brute_force_finder <word> [range]");
        eprintln!("Example: cargo run --example brute_force_finder hello 100M");
        std::process::exit(1);
    }

    let target_word = &args[1];
    let range_arg = args.get(2).map(|s| s.as_str()).unwrap_or("100M"); // Default to 100M

    // Parse the range (e.g., "10k" -> 10,000)
    let end_range = parse_range(range_arg).expect("Invalid range format. Use 10, 10k, 10M, 10B");

    println!("Searching for seed to generate: \"{target_word}\"");
    println!("Search Range: 0..{end_range} seeds");

    let target_bytes = target_word.as_bytes().to_vec();

    // 2. Calculate bounds dynamically
    let min_bound = *target_bytes.iter().min().expect("Word cannot be empty") as u64;
    let max_bound = *target_bytes.iter().max().expect("Word cannot be empty") as u64 + 1;

    println!("Gen Range Bounds -> Min: {min_bound}, Max: {max_bound} (Exclusive)");

    // 3. Search Configuration
    let start_range: u64 = 0;

    let num_threads = get();
    // Safety check for small ranges to avoid division by zero or logic errors
    // Shouldn't happen
    if end_range < start_range {
        eprintln!("Error: End range must be greater than start range.");
        std::process::exit(1);
    }

    let total_seeds = (end_range - start_range) as u128 + 1;
    let seeds_per_thread = ((total_seeds / num_threads as u128) + 1) as u64;

    let (tx, rx) = mpsc::channel();

    for thread_id in 0..num_threads {
        let tx = tx.clone();
        let thread_target = target_bytes.clone();

        let thread_start = start_range + (thread_id as u64 * seeds_per_thread);
        let thread_end = (thread_start + seeds_per_thread - 1).min(end_range);

        if thread_start > end_range {
            break;
        }

        thread::spawn(move || {
            let mut matches = Vec::new();

            for seed in thread_start..=thread_end {
                let mut rng = Parrot::new(seed);
                let mut match_found = true;

                for &byte in &thread_target {
                    if rng.gen_range(min_bound, max_bound) != byte as u64 {
                        match_found = false;
                        break;
                    }
                }

                if match_found {
                    matches.push(seed);
                }
            }
            tx.send(matches).expect("Failed to send matches");
        });
    }

    drop(tx);

    let mut results: Vec<u64> = Vec::new();
    for received in rx {
        results.extend(received);
    }

    results.sort();

    println!("--------------------------------------------------");
    println!(
        "Found {} matching seeds in range {}-{}",
        results.len(),
        start_range,
        end_range
    );
    if !results.is_empty() {
        println!("Seeds for \"{target_word}\": {results:?}");
    } else {
        println!("No seeds found. Try increasing the search range.");
    }
}

/// Helper function to parse strings like "10k", "5M", "1B"
fn parse_range(input: &str) -> Result<u64, String> {
    let input = input.trim().to_uppercase();
    let mut multiplier = 1;
    let number_part;

    if input.ends_with('K') {
        multiplier = 1_000;
        number_part = &input[..input.len() - 1];
    } else if input.ends_with('M') {
        multiplier = 1_000_000;
        number_part = &input[..input.len() - 1];
    } else if input.ends_with('B') {
        multiplier = 1_000_000_000;
        number_part = &input[..input.len() - 1];
    } else {
        number_part = &input;
    }

    match number_part.parse::<u64>() {
        Ok(n) => Ok(n * multiplier),
        Err(_) => Err("Could not parse number".to_string()),
    }
}
