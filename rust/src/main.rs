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

fn main() {
    dotenv::dotenv().ok();

    solution_aoc!(year2015, day01);
    solution_aoc!(year2015, day02);
    solution_aoc!(year2015, day03);
    solution_aoc!(year2015, day04);
}
