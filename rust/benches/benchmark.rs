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
                let input: String = get_input();
                b.iter(|| part1(&input));
            }

            #[bench]
            fn part2_bench(b: &mut Bencher) {
                let input: String = get_input();
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
    }
}
