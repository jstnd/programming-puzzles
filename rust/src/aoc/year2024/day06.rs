use std::collections::HashSet;

use crate::util::point::Point;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Guard {
    direction: u8,
    position: Point,
}

pub fn part1(input: &str) -> usize {
    let (grid, mut guard) = parse_grid(input);
    let mut positions = HashSet::with_capacity(10_000);
    positions.insert(guard.position);

    loop {
        let next = guard.position + Point::from(guard.direction);

        if let Some(row) = grid.get(next.y as usize) {
            if let Some(position) = row.get(next.x as usize) {
                match position {
                    b'.' => {
                        guard.position = next;
                        positions.insert(next);
                    }
                    b'#' => guard.direction = rotate(guard.direction),
                    _ => unreachable!(),
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    positions.len()
}

pub fn part2(input: &str) -> usize {
    let (mut grid, mut guard) = parse_grid(input);
    let mut positions = HashSet::new();
    let initial = guard;

    loop {
        let next = guard.position + Point::from(guard.direction);

        if let Some(row) = grid.get(next.y as usize) {
            if let Some(position) = row.get(next.x as usize) {
                match position {
                    b'.' => {
                        if !positions.contains(&next) {
                            grid[next.y as usize][next.x as usize] = b'#';

                            if is_loop(&grid, initial) {
                                positions.insert(next);
                            }

                            grid[next.y as usize][next.x as usize] = b'.';
                        }

                        guard.position = next;
                    }
                    b'#' => guard.direction = rotate(guard.direction),
                    _ => unreachable!(),
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    positions.len()
}

fn parse_grid(input: &str) -> (Vec<Vec<u8>>, Guard) {
    let mut guard = None;

    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, b)| {
                    if b == b'^' {
                        guard = Some(Guard {
                            direction: b,
                            position: Point::new(x as i32, y as i32),
                        });

                        b'.'
                    } else {
                        b
                    }
                })
                .collect()
        })
        .collect();

    (grid, guard.unwrap())
}

fn rotate(direction: u8) -> u8 {
    match direction {
        b'^' => b'>',
        b'>' => b'v',
        b'v' => b'<',
        b'<' => b'^',
        _ => unreachable!(),
    }
}

fn is_loop(grid: &[Vec<u8>], mut guard: Guard) -> bool {
    let mut turns = HashSet::new();

    loop {
        let next = guard.position + Point::from(guard.direction);

        if let Some(row) = grid.get(next.y as usize) {
            if let Some(position) = row.get(next.x as usize) {
                match position {
                    b'.' => guard.position = next,
                    b'#' => {
                        guard.direction = rotate(guard.direction);

                        if turns.contains(&guard) {
                            return true;
                        }

                        turns.insert(guard);
                    }
                    _ => unreachable!(),
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    false
}
