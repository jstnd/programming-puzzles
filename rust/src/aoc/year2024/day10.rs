use rustc_hash::FxHashSet;

use crate::util::point::Point;

pub fn part1(input: &str) -> usize {
    solve(input, 1)
}

pub fn part2(input: &str) -> usize {
    solve(input, 2)
}

fn solve(input: &str, part: u8) -> usize {
    let grid = parse(input);
    let path = b"0123456789";
    let mut score = 0;
    let mut ends = FxHashSet::default();

    for (y, row) in grid.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == path[0] {
                let point = Point::new(x as i32, y as i32);
                let mut paths = 0;

                for neighbor in Point::von_neumann() {
                    search(&grid, &path[1..], &mut paths, &mut ends, point + neighbor);
                }

                score += if part == 1 { ends.len() } else { paths };
                ends.clear();
            }
        }
    }

    score
}

fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(|line| line.as_bytes()).collect()
}

fn search(
    grid: &Vec<&[u8]>,
    path: &[u8],
    paths: &mut usize,
    ends: &mut FxHashSet<Point>,
    point: Point,
) {
    if let Some(row) = grid.get(point.y as usize) {
        if let Some(char) = row.get(point.x as usize) {
            if *char == path[0] {
                if path.len() == 1 {
                    ends.insert(point);
                    *paths += 1;
                    return;
                }

                for neighbor in Point::von_neumann() {
                    search(grid, &path[1..], paths, ends, point + neighbor);
                }
            }
        }
    }
}
