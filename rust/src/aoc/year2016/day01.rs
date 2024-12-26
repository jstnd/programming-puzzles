use rustc_hash::FxHashSet;

use crate::util::point::Point;

pub fn part1(input: &str) -> u32 {
    let mut position = Point::origin();
    let mut direction = b'^';

    input.trim().split(", ").for_each(|instruction| {
        match &instruction[..1] {
            "L" => direction = turn_left(direction),
            "R" => direction = turn_right(direction),
            _ => unreachable!(),
        }

        position += Point::from(direction) * instruction[1..].parse::<usize>().unwrap();
    });

    position.x.unsigned_abs() + position.y.unsigned_abs()
}

pub fn part2(input: &str) -> u32 {
    let mut visited = FxHashSet::default();
    let mut position = Point::origin();
    let mut direction = b'^';

    for instruction in input.trim().split(", ") {
        match &instruction[..1] {
            "L" => direction = turn_left(direction),
            "R" => direction = turn_right(direction),
            _ => unreachable!(),
        }

        for _ in 0..instruction[1..].parse::<usize>().unwrap() {
            position += Point::from(direction);

            if visited.contains(&position) {
                return position.x.unsigned_abs() + position.y.unsigned_abs();
            } else {
                visited.insert(position);
            }
        }
    }

    position.x.unsigned_abs() + position.y.unsigned_abs()
}

fn turn_left(direction: u8) -> u8 {
    match direction {
        b'^' => b'<',
        b'v' => b'>',
        b'<' => b'v',
        b'>' => b'^',
        _ => unreachable!(),
    }
}

fn turn_right(direction: u8) -> u8 {
    match direction {
        b'^' => b'>',
        b'v' => b'<',
        b'<' => b'^',
        b'>' => b'v',
        _ => unreachable!(),
    }
}
