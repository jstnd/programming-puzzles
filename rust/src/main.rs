macro_rules! solution {
    ($year:tt, $day:tt) => {
        let input: String = aoc::$year::$day::get_input();

        println!("{} - {}", stringify!($year), stringify!($day));

        let instant = std::time::Instant::now();
        let result = aoc::$year::$day::part1(&input);
        let elapsed = instant.elapsed();

        println!("    Part 1: {} - {} μs", result, elapsed.as_micros());

        let instant = std::time::Instant::now();
        let result = aoc::$year::$day::part2(&input);
        let elapsed = instant.elapsed();

        println!("    Part 2: {} - {} μs", result, elapsed.as_micros());
    };
}

fn main() {
    dotenv::dotenv().ok();

    solution!(year2015, day01);
    solution!(year2015, day02);
    solution!(year2015, day03);
}
