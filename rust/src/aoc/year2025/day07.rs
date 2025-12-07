use rustc_hash::{FxHashMap, FxHashSet};

use crate::util::point::Point;

const START: u8 = b'S';
const EMPTY: u8 = b'.';
const SPLITTER: u8 = b'^';

pub fn part1(input: &str) -> usize {
    // Track the beam positions.
    let mut positions = FxHashSet::default();

    // Track the number of times a beam was split.
    let mut num_splits = 0;

    input.lines().enumerate().for_each(|(y, line)| {
        line.bytes().enumerate().for_each(|(x, byte)| {
            // Create point from current position.
            let point = Point::new(x as i32, y as i32);

            match byte {
                START => {
                    // At start position; add this position to list of beam positions.
                    positions.insert(point);
                }
                EMPTY => {
                    // At empty space; check if there's a beam above this position,
                    // and add this position to list of beam positions if so.
                    if positions.contains(&(point + Point::up())) {
                        positions.insert(point);
                    }
                }
                SPLITTER => {
                    // At splitter; check if there's a beam above this position, and
                    // add the positions to the left and right of this position to list
                    // of beam positions if so, and also record that a split has occurred.
                    if positions.contains(&(point + Point::up())) {
                        positions.insert(point + Point::left());
                        positions.insert(point + Point::right());
                        num_splits += 1;
                    }
                }
                _ => unreachable!(),
            };
        });
    });

    // Return the number of times a beam was split.
    num_splits
}

pub fn part2(input: &str) -> usize {
    // Track the beam positions as well as the number of timelines active at each position.
    let mut positions = FxHashMap::default();

    // Track the input length for later use.
    let mut input_len = 0;

    input.lines().enumerate().for_each(|(y, line)| {
        // Increment the variable for tracking the input length.
        input_len += 1;

        line.bytes().enumerate().for_each(|(x, byte)| {
            // Create point from current position.
            let point = Point::new(x as i32, y as i32);

            match byte {
                START => {
                    // At start position; add this position to list of beam positions.
                    positions.insert(point, 1);
                }
                EMPTY => {
                    // At empty space; check if there's a beam above this position, and add the
                    // number of active timelines at the beam position to the current position if so.
                    let up = point + Point::up();
                    if positions.contains_key(&up) {
                        *positions.entry(point).or_default() += positions[&up];
                    }
                }
                SPLITTER => {
                    // At splitter; check if there's a beam above this position, and add the number of active
                    // timelines at the beam position to the positions left and right of the current position if so.
                    let up = point + Point::up();
                    if positions.contains_key(&up) {
                        *positions.entry(point + Point::left()).or_default() += positions[&up];
                        *positions.entry(point + Point::right()).or_default() += positions[&up];
                    }
                }
                _ => unreachable!(),
            };
        });
    });

    positions
        .iter()
        .filter_map(|(position, count)| {
            // Only include the counts for the last row of positions.
            (position.y as usize == input_len - 1).then_some(count)
        })
        .sum()
}
