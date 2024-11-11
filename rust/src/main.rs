macro_rules! solution_aoc {
    ($year:tt, $day:tt) => {
        let input: String = puzzles::aoc::$year::$day::get_input();

        println!("{} - {}", stringify!($year), stringify!($day));

        let instant = std::time::Instant::now();
        let result = puzzles::aoc::$year::$day::part1(&input);
        let elapsed = instant.elapsed();

        println!("    Part 1: {} - {} μs", result, elapsed.as_micros());

        let instant = std::time::Instant::now();
        let result = puzzles::aoc::$year::$day::part2(&input);
        let elapsed = instant.elapsed();

        println!("    Part 2: {} - {} μs", result, elapsed.as_micros());
    };
}

macro_rules! solution_ec {
    ($year:tt, $quest:tt) => {
        println!("{} - {}", stringify!($year), stringify!($quest));

        let input: String = puzzles::util::input::get_ec_input(2024, 1, 1);
        let instant = std::time::Instant::now();
        let result = puzzles::ec::$year::$quest::part1(&input);
        let elapsed = instant.elapsed();

        println!("    Part 1: {} - {} μs", result, elapsed.as_micros());

        let input: String = puzzles::util::input::get_ec_input(2024, 1, 2);
        let instant = std::time::Instant::now();
        let result = puzzles::ec::$year::$quest::part2(&input);
        let elapsed = instant.elapsed();

        println!("    Part 2: {} - {} μs", result, elapsed.as_micros());

        let input: String = puzzles::util::input::get_ec_input(2024, 1, 3);
        let instant = std::time::Instant::now();
        let result = puzzles::ec::$year::$quest::part3(&input);
        let elapsed = instant.elapsed();

        println!("    Part 3: {} - {} μs", result, elapsed.as_micros());
    };
}

fn main() {
    dotenv::dotenv().ok();

    print_aoc();
    print_ec();
}

fn print_aoc() {
    println!("========== Advent of Code ==========");
    solution_aoc!(year2015, day01);
    solution_aoc!(year2015, day02);
    solution_aoc!(year2015, day03);
    solution_aoc!(year2015, day04);
}

fn print_ec() {
    println!("========== Everybody Codes ==========");
    solution_ec!(year2024, quest01);
}
