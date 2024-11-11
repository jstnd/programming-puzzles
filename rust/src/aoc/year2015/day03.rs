use std::collections::HashSet;

use crate::util::point::Point;

pub fn part1(input: &str) -> i32 {
    deliver(input, 1)
}

pub fn part2(input: &str) -> i32 {
    deliver(input, 2)
}

fn deliver(input: &str, part: u8) -> i32 {
    let mut santa = Point::origin();
    let mut robo = Point::origin();
    let mut set: HashSet<Point> = HashSet::with_capacity(10_000);
    set.insert(santa);

    for (i, b) in input.bytes().enumerate() {
        if part == 2 && i % 2 == 0 {
            robo += Point::from(b);

            set.insert(robo);
        } else {
            santa += Point::from(b);

            set.insert(santa);
        }
    }

    set.len() as i32
}
