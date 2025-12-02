use std::ops::Range;

const DIAL_LENGTH: i32 = 100;
const DIAL_RANGE: Range<i32> = 0..DIAL_LENGTH;

pub fn part1(input: &str) -> u32 {
    let mut dial = 50;
    let mut password = 0;

    for rotation in parse(input) {
        // Use rem_euclid to find the resulting remainder
        // after moving the dial the given number of notches
        dial = (dial + rotation).rem_euclid(DIAL_LENGTH);

        // Add 1 to the password if the dial is at 0
        if dial == 0 {
            password += 1;
        }
    }

    password
}

pub fn part2(input: &str) -> u32 {
    let mut dial = 50_i32;
    let mut password = 0;

    for rotation in parse(input) {
        // Manually iterate through the number of rotations
        for _ in 0..rotation.abs() {
            // Move the dial by one notch in either direction
            dial += if rotation < 0 { -1 } else { 1 };

            // Only perform the work for rem_euclid if
            // the value went outside the dial range
            if !DIAL_RANGE.contains(&dial) {
                dial = dial.rem_euclid(DIAL_LENGTH);
            }

            // Add 1 to the password if the dial is at 0
            if dial == 0 {
                password += 1;
            }
        }
    }

    password
}

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            let distance = distance.parse::<i32>().unwrap();

            match direction {
                "L" => -distance,
                "R" => distance,
                _ => unreachable!(),
            }
        })
        .collect()
}
