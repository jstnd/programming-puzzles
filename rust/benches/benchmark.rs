#![allow(unstable_features)]
#![feature(test)]

extern crate test;

macro_rules! benchmark_aoc {
    ($year:tt, $day:tt) => {
        mod $day {
            use puzzles::aoc::$year::$day::*;
            use test::Bencher;

            #[bench]
            fn part1_bench(b: &mut Bencher) {
                let year = puzzles::util::parse::extract_integer(stringify!($year)) as u16;
                let day = puzzles::util::parse::extract_integer(stringify!($day)) as u8;
                let input: String = puzzles::util::input::get_aoc_input(year, day);
                b.iter(|| part1(&input));
            }

            #[bench]
            fn part2_bench(b: &mut Bencher) {
                let year = puzzles::util::parse::extract_integer(stringify!($year)) as u16;
                let day = puzzles::util::parse::extract_integer(stringify!($day)) as u8;
                let input: String = puzzles::util::input::get_aoc_input(year, day);
                b.iter(|| part2(&input));
            }
        }
    };
}

mod aoc {
    mod year2015 {
        benchmark_aoc!(year2015, day01);
        benchmark_aoc!(year2015, day02);
        benchmark_aoc!(year2015, day03);
        benchmark_aoc!(year2015, day04);
        benchmark_aoc!(year2015, day05);
        benchmark_aoc!(year2015, day06);
        benchmark_aoc!(year2015, day07);
        benchmark_aoc!(year2015, day08);
    }

    mod year2024 {
        benchmark_aoc!(year2024, day01);
        benchmark_aoc!(year2024, day02);
        benchmark_aoc!(year2024, day03);
        benchmark_aoc!(year2024, day04);
        benchmark_aoc!(year2024, day05);
        benchmark_aoc!(year2024, day06);
        benchmark_aoc!(year2024, day07);
        benchmark_aoc!(year2024, day08);
        benchmark_aoc!(year2024, day09);
        benchmark_aoc!(year2024, day10);
        benchmark_aoc!(year2024, day11);
        benchmark_aoc!(year2024, day12);
        benchmark_aoc!(year2024, day13);
        benchmark_aoc!(year2024, day14);
        benchmark_aoc!(year2024, day15);
        benchmark_aoc!(year2024, day16);
        benchmark_aoc!(year2024, day18);
        benchmark_aoc!(year2024, day19);
        benchmark_aoc!(year2024, day22);
    }
}
