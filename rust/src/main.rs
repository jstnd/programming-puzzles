macro_rules! solution_aoc {
    ($year:tt, $day:tt) => {
        let year = puzzles::util::parse::extract_integer(stringify!($year)) as u16;
        let day = puzzles::util::parse::extract_integer(stringify!($day)) as u8;
        let input: String = puzzles::util::input::get_aoc_input(year, day);

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
        let year = puzzles::util::parse::extract_integer(stringify!($year)) as u16;
        let quest = puzzles::util::parse::extract_integer(stringify!($quest)) as u8;

        println!("{} - {}", stringify!($year), stringify!($quest));

        let input: String = puzzles::util::input::get_ec_input(year, quest, 1);
        let instant = std::time::Instant::now();
        let result = puzzles::ec::$year::$quest::part1(&input);
        let elapsed = instant.elapsed();

        println!("    Part 1: {} - {} μs", result, elapsed.as_micros());

        let input: String = puzzles::util::input::get_ec_input(year, quest, 2);
        let instant = std::time::Instant::now();
        let result = puzzles::ec::$year::$quest::part2(&input);
        let elapsed = instant.elapsed();

        println!("    Part 2: {} - {} μs", result, elapsed.as_micros());

        let input: String = puzzles::util::input::get_ec_input(year, quest, 3);
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

    {
        solution_aoc!(year2015, day01);
        solution_aoc!(year2015, day02);
        solution_aoc!(year2015, day03);
        solution_aoc!(year2015, day04);
        solution_aoc!(year2015, day05);
        solution_aoc!(year2015, day06);
        solution_aoc!(year2015, day07);
        solution_aoc!(year2015, day08);
    }

    {
        solution_aoc!(year2016, day01);
        solution_aoc!(year2016, day02);
    }

    {
        solution_aoc!(year2017, day01);
        solution_aoc!(year2017, day02);
    }

    {
        solution_aoc!(year2019, day01);
        solution_aoc!(year2019, day02);
        solution_aoc!(year2019, day03);
    }

    {
        solution_aoc!(year2024, day01);
        solution_aoc!(year2024, day02);
        solution_aoc!(year2024, day03);
        solution_aoc!(year2024, day04);
        solution_aoc!(year2024, day05);
        solution_aoc!(year2024, day06);
        solution_aoc!(year2024, day07);
        solution_aoc!(year2024, day08);
        solution_aoc!(year2024, day09);
        solution_aoc!(year2024, day10);
        solution_aoc!(year2024, day11);
        solution_aoc!(year2024, day12);
        solution_aoc!(year2024, day13);
        solution_aoc!(year2024, day14);
        solution_aoc!(year2024, day15);
        //solution_aoc!(year2024, day16);
        solution_aoc!(year2024, day18);
        solution_aoc!(year2024, day19);
        solution_aoc!(year2024, day22);
        solution_aoc!(year2024, day24);
        solution_aoc!(year2024, day25);
    }
}

fn print_ec() {
    println!("========== Everybody Codes ==========");
    solution_ec!(year2024, quest01);
}
