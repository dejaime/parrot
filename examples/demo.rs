use parrot::Parrot;

const ASCII_MIN: u64 = 100;
const ASCII_MAX: u64 = 120;

fn main() {
    // The seeds below were tested before and when we use their values
    //		value->u8->char they will generate the words Hello and World.
    // These were found with simple brute-force, and you can see it in
    //		example/src/seed_finder_main.rs

    // HELLO
    // Matching seeds: [96057226, 56417755, 58618187, 61210939, 62216279, 64412117, 45590555, 47404613, 17029190, 17334132, 22616296, 38457084, 77337743, 78459210, 80170250, 70017261, 84057256, 85026615, 86405665, 86705100, 34000905, 27920784]
    let mut hello_rng = Parrot::new(96057226);

    // WORLD
    // Matching seeds: [59202201, 61198765, 2721941, 2772963, 4328036, 62630473, 67352218, 12839630, 14483923, 14628966, 15608995, 17278552, 10024729, 11209489, 47402123, 49723396, 21152285, 21462886, 23867572, 31450052, 35663121, 82125860, 95694218, 96793534, 97372030, 51425101, 53546826, 25576469, 26657652, 69011995, 72861309, 73682298, 92476582, 80072229]
    let mut world_rng = Parrot::new(59202201);

    let generated = format!(
        "{}{}{}{}{} {}{}{}{}{}",
        // This `hello_rng` will always generate these values in this order
        hello_rng.gen_range(ASCII_MIN, ASCII_MAX) as u8 as char,
        hello_rng.gen_range(ASCII_MIN, ASCII_MAX) as u8 as char,
        hello_rng.gen_range(ASCII_MIN, ASCII_MAX) as u8 as char,
        hello_rng.gen_range(ASCII_MIN, ASCII_MAX) as u8 as char,
        hello_rng.gen_range(ASCII_MIN, ASCII_MAX) as u8 as char,
        // Same here with `world_rng`
        world_rng.gen_range(ASCII_MIN, ASCII_MAX) as u8 as char,
        world_rng.gen_range(ASCII_MIN, ASCII_MAX) as u8 as char,
        world_rng.gen_range(ASCII_MIN, ASCII_MAX) as u8 as char,
        world_rng.gen_range(ASCII_MIN, ASCII_MAX) as u8 as char,
        world_rng.gen_range(ASCII_MIN, ASCII_MAX) as u8 as char,
    );
    let target = "hello world";

    println!("Generated: {generated}");
    println!("Target: {target}");
    assert_eq!(generated, target);
}
