use rustc_hash::{FxHashMap, FxHashSet};

use crate::util::point::Point;

pub fn part1(input: &str) -> usize {
    let (grid, antennas) = parse(input);
    let mut antinodes = FxHashSet::with_capacity_and_hasher(500, Default::default());

    antennas.values().for_each(|points| {
        for (i, first) in points.iter().enumerate() {
            for second in points.iter().skip(i + 1) {
                let difference = *second - *first;
                let top = *first - difference;
                let bottom = *second + difference;

                if is_valid_antinode(&grid, top) {
                    antinodes.insert(top);
                }

                if is_valid_antinode(&grid, bottom) {
                    antinodes.insert(bottom);
                }
            }
        }
    });

    antinodes.len()
}

pub fn part2(input: &str) -> usize {
    let (grid, antennas) = parse(input);
    let mut antinodes = FxHashSet::with_capacity_and_hasher(1_000, Default::default());

    antennas.values().for_each(|points| {
        for (i, first) in points.iter().enumerate() {
            for second in points.iter().skip(i + 1) {
                antinodes.insert(*first);
                antinodes.insert(*second);

                let difference = *second - *first;
                let mut next = *first - difference;
                while is_valid_antinode(&grid, next) {
                    antinodes.insert(next);
                    next -= difference;
                }

                next = *second + difference;
                while is_valid_antinode(&grid, next) {
                    antinodes.insert(next);
                    next += difference;
                }
            }
        }
    });

    antinodes.len()
}

fn parse(input: &str) -> (Vec<Vec<u8>>, FxHashMap<u8, Vec<Point>>) {
    let mut antennas =
        FxHashMap::<u8, Vec<Point>>::with_capacity_and_hasher(200, Default::default());

    (
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.bytes()
                    .enumerate()
                    .map(|(x, b)| {
                        if b != b'.' {
                            antennas
                                .entry(b)
                                .or_default()
                                .push(Point::new(x as i32, y as i32));
                        }

                        b
                    })
                    .collect()
            })
            .collect(),
        antennas,
    )
}

fn is_valid_antinode(grid: &[Vec<u8>], point: Point) -> bool {
    if let Some(row) = grid.get(point.y as usize) {
        return row.get(point.x as usize).is_some();
    }

    false
}
