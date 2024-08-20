#![allow(unstable_features)]
#![feature(test)]

extern crate test;

macro_rules! benchmark {
    ($year:tt, $day:tt) => {
        mod $day {
            use aoc::$year::$day::*;
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

mod year2015 {
    benchmark!(year2015, day01);
    benchmark!(year2015, day02);
    benchmark!(year2015, day03);
}
