#![feature(iter_array_chunks)]

pub mod util {
    pub mod array;
    pub mod input;
    pub mod num;
    pub mod parse;
    pub mod point;
}

pub mod aoc {
    pub mod year2015 {
        pub mod day01;
        pub mod day02;
        pub mod day03;
        pub mod day04;
        pub mod day05;
        pub mod day06;
        pub mod day07;
        pub mod day08;
    }

    pub mod year2016 {
        pub mod day01;
        pub mod day02;
    }

    pub mod year2017 {
        pub mod day01;
        pub mod day02;
    }

    pub mod year2019 {
        pub mod day01;
        pub mod day02;
        pub mod day03;
        pub mod intcode;
    }

    pub mod year2024 {
        pub mod day01;
        pub mod day02;
        pub mod day03;
        pub mod day04;
        pub mod day05;
        pub mod day06;
        pub mod day07;
        pub mod day08;
        pub mod day09;
        pub mod day10;
        pub mod day11;
        pub mod day12;
        pub mod day13;
        pub mod day14;
        pub mod day15;
        pub mod day16;
        pub mod day18;
        pub mod day19;
        pub mod day22;
        pub mod day24;
        pub mod day25;
    }

    pub mod year2025 {
        pub mod day01;
    }
}

pub mod ec {
    pub mod event2024 {
        pub mod quest01;
    }

    pub mod event2025 {
        pub mod quest01;
        pub mod quest02;
        pub mod quest03;
        pub mod quest04;
        pub mod quest05;
    }

    pub mod story01 {
        pub mod quest01;
    }
}
