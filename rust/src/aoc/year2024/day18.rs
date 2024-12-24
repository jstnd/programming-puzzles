use std::collections::VecDeque;

use crate::util::point::Point;

const DIMENSION: usize = 71;

pub fn part1(input: &str) -> usize {
    let corrupted = parse(input);
    let mut grid = [[Memory::Safe; DIMENSION]; DIMENSION];

    corrupted.iter().take(1024).for_each(|point| {
        grid[point.y as usize][point.x as usize] = Memory::Corrupted;
    });

    bfs(
        &grid,
        Point::new(0, 0),
        Point::new((DIMENSION - 1) as i32, (DIMENSION - 1) as i32),
    )
}

pub fn part2(input: &str) -> String {
    let corrupted = parse(input);
    let mut grid = [[Memory::Safe; DIMENSION]; DIMENSION];

    corrupted.iter().take(1024).for_each(|point| {
        grid[point.y as usize][point.x as usize] = Memory::Corrupted;
    });

    let byte = corrupted.iter().skip(1024).take_while(|point| {
        grid[point.y as usize][point.x as usize] = Memory::Corrupted;
        bfs(&grid, Point::new(0, 0), Point::new((DIMENSION - 1) as i32, (DIMENSION - 1) as i32)) > 0
    }).count() + 1024;

    let byte = corrupted[byte];
    format!("{},{}", byte.x, byte.y)
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Memory {
    Corrupted,
    Safe,
}

fn bfs(grid: &[[Memory; DIMENSION]; DIMENSION], start: Point, end: Point) -> usize {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut visited = [[false; DIMENSION]; DIMENSION];
    visited[start.y as usize][start.x as usize] = true;

    while let Some((point, steps)) = queue.pop_back() {
        if point == end {
            return steps;
        }

        for neighbor in Point::von_neumann() {
            let next = point + neighbor;

            if !is_safe(grid, next) || visited[next.y as usize][next.x as usize] {
                continue;
            }

            visited[next.y as usize][next.x as usize] = true;
            queue.push_front((next, steps + 1));
        }
    }

    0
}

fn is_safe(grid: &[[Memory; DIMENSION]; DIMENSION], point: Point) -> bool {
    if let Some(row) = grid.get(point.y as usize) {
        return row
            .get(point.x as usize)
            .is_some_and(|memory| *memory == Memory::Safe);
    }

    false
}
