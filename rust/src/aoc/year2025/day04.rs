use crate::util::point::Point;

const PAPER_ROLL: u8 = b'@';

pub fn part1(input: &str) -> usize {
    let grid = parse(input);
    get_removable_rolls(&grid).len()
}

pub fn part2(input: &str) -> usize {
    let mut grid = parse(input);
    let mut removed = 0;

    // Here, we'll want to continuously loop until we find no more removable rows.
    loop {
        // Find any removable rows from the current grid.
        let removable_rolls = get_removable_rolls(&grid);

        // If we found no removable rows, we can exit here.
        if removable_rolls.is_empty() {
            break;
        }

        // Remove any found removable rolls and increase our counter.
        for roll in removable_rolls {
            grid[roll.y as usize][roll.x as usize] = b'.';
            removed += 1;
        }
    }

    removed
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

fn get_removable_rolls(grid: &[Vec<u8>]) -> Vec<Point> {
    let mut removable = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char != PAPER_ROLL {
                continue;
            }

            let point = Point::new(x as i32, y as i32);
            let neighbor_rolls = Point::moore()
                .filter(|neighbor| is_roll(grid, point + *neighbor))
                .count();

            // A roll is only removable if less than 4 of its neighbors are rolls.
            if neighbor_rolls < 4 {
                removable.push(point);
            }
        }
    }

    removable
}

fn is_roll(grid: &[Vec<u8>], point: Point) -> bool {
    if let Some(row) = grid.get(point.y as usize)
        && let Some(char) = row.get(point.x as usize)
    {
        return *char == PAPER_ROLL;
    }

    false
}
