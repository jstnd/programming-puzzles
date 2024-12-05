use std::collections::HashSet;

use crate::util::point::Point;

pub fn part1(input: &str) -> u32 {
    let grid = parse_grid(input);
    let word = "XMAS".as_bytes();
    let mut count = 0u32;

    for (y, row) in grid.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == word[0] {
                let point = Point::new(x as i32, y as i32);

                for neighbor in Point::moore() {
                    if search_direction(&grid, &word[1..], point + neighbor, neighbor) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

pub fn part2(input: &str) -> u32 {
    let grid = parse_grid(input);
    let word = "MAS".as_bytes();
    let mut points = HashSet::new();
    let mut count = 0u32;

    for (y, row) in grid.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == word[0] {
                let point = Point::new(x as i32, y as i32);

                for neighbor in Point::diagonal() {
                    let point = point + neighbor;

                    if search_direction(&grid, &word[1..], point, neighbor) {
                        // This keeps track of all the 'A' points. If we run into a point that
                        // has already been seen, that means we found an instance of X-MAS.
                        if points.contains(&point) {
                            count += 1;
                        } else {
                            points.insert(point);
                        }
                    }
                }
            }
        }
    }

    count
}

fn parse_grid(input: &str) -> Vec<&[u8]> {
    input.lines().map(|line| line.as_bytes()).collect()
}

fn search_direction(grid: &Vec<&[u8]>, word: &[u8], point: Point, direction: Point) -> bool {
    if let Some(row) = grid.get(point.y as usize) {
        if let Some(c) = row.get(point.x as usize) {
            if *c == word[0] {
                if word.len() > 1 {
                    return search_direction(grid, &word[1..], point + direction, direction);
                }

                return true;
            }
        }
    }

    false
}
