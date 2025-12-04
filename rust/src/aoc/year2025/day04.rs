use crate::util::point::Point;

const PAPER_ROLL: u8 = b'@';

pub fn part1(input: &str) -> usize {
    let grid = parse(input);
    let mut accessible = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char != PAPER_ROLL {
                continue;
            }

            let point = Point::new(x as i32, y as i32);
            let rolls = Point::moore()
                .filter(|neighbor| is_roll(&grid, point + *neighbor))
                .count();

            if rolls < 4 {
                accessible += 1;
            }
        }
    }

    accessible
}

pub fn part2(input: &str) -> usize {
    let mut grid = parse(input);
    let mut to_remove = Vec::new();
    let mut removed = 0;

    loop {
        for (y, row) in grid.iter().enumerate() {
            for (x, char) in row.iter().enumerate() {
                if *char != PAPER_ROLL {
                    continue;
                }

                let point = Point::new(x as i32, y as i32);
                let rolls = Point::moore()
                    .filter(|neighbor| is_roll(&grid, point + *neighbor))
                    .count();

                if rolls < 4 {
                    to_remove.push(point);
                }
            }
        }

        for point in to_remove.iter() {
            grid[point.y as usize][point.x as usize] = b'.';
        }

        if to_remove.is_empty() {
            break;
        }

        removed += to_remove.len();
        to_remove.clear();
    }

    removed
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

fn is_roll(grid: &[Vec<u8>], point: Point) -> bool {
    if let Some(row) = grid.get(point.y as usize)
        && let Some(char) = row.get(point.x as usize)
    {
        return *char == PAPER_ROLL;
    }

    false
}
