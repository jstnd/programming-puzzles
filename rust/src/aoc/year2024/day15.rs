use crate::util::point::Point;

pub fn part1(input: &str) -> usize {
    let (mut grid, moves, mut robot) = parse(input, 1);

    for &m in moves {
        let direction = Point::from(m);
        let next = robot + direction;

        match grid[next.y as usize][next.x as usize] {
            b'.' => {
                // Trivially move the robot to the next spot if it's empty.
                robot = next;
            }
            b'O' => {
                // If a box is in the next spot, we need to then find all boxes and
                // determine if it's possible to move them all.
                let mut boxes = vec![next];
                let mut path = next + direction;

                // Continue searching while we're finding boxes.
                while grid[path.y as usize][path.x as usize] == b'O' {
                    boxes.push(path);
                    path += direction;
                }

                // Move all the boxes and the robot only if we're not blocked.
                if grid[path.y as usize][path.x as usize] == b'.' {
                    for &b in boxes.iter().rev() {
                        let mov = b + direction;
                        grid[mov.y as usize][mov.x as usize] = b'O';
                        grid[b.y as usize][b.x as usize] = b'.';
                    }

                    robot = next;
                }
            }
            _ => {}
        }
    }

    coordinates(&grid)
}

pub fn part2(input: &str) -> usize {
    let (mut grid, moves, mut robot) = parse(input, 2);

    for &m in moves {
        let direction = Point::from(m);
        let next = robot + direction;

        match grid[next.y as usize][next.x as usize] {
            b'.' => {
                // Trivially move the robot to the next spot if it's empty.
                robot = next;
            }
            side @ b'[' | side @ b']' => {
                // If a box is in the next spot, we need to then find all boxes and
                // determine if it's possible to move them all.
                let mut boxes = vec![next];

                // Add the other side of the box to our list of boxes.
                if side == b'[' {
                    boxes.push(next + Point::right());
                } else {
                    boxes.push(next + Point::left());
                }

                // Search through boxes until we find them all.
                let mut blocked = false;

                match m {
                    b'^' | b'v' => {
                        // If we're moving up or down, finding the boxes is a little more complex
                        // than left and right as we could have two boxes behind a single box.
                        //
                        // For example, if the robot is moving up in the scenario below, we need
                        // to move both boxes behind that first box in front of the robot.
                        //
                        //     [][]
                        //      []
                        //       @
                        //

                        // Start the search with the first box.
                        let mut current = boxes.clone();

                        while current.len() > 1 {
                            let mut next = Vec::new();

                            for b in current {
                                let path = b + direction;

                                match grid[path.y as usize][path.x as usize] {
                                    b'#' => {
                                        // If we found a wall above/below the box we're searching, that
                                        // means we're blocked, and we can stop the search here.
                                        blocked = true;
                                        next.clear();
                                        break;
                                    }
                                    side @ b'[' | side @ b']' => {
                                        // If we found another box, add it to our total list of boxes and
                                        // the list of next boxes to search if we don't already have it.
                                        if !next.contains(&path) {
                                            boxes.push(path);
                                            next.push(path);

                                            if side == b'[' {
                                                boxes.push(path + Point::right());
                                                next.push(path + Point::right());
                                            } else {
                                                boxes.push(path + Point::left());
                                                next.push(path + Point::left());
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }

                            current = next;
                        }
                    }
                    b'<' | b'>' => {
                        // If we're moving left or right, finding all the boxes is simple as
                        // we just need to search in a straight line, similar to part 1.
                        //
                        //     @[][]
                        //

                        // Start the search after the first box.
                        let mut path = next + direction + direction;

                        // Continue searching while we're finding boxes.
                        while [b'[', b']'].contains(&grid[path.y as usize][path.x as usize]) {
                            boxes.push(path);
                            path += direction;
                        }

                        // If the first non-box spot is not empty, we are blocked from moving.
                        if grid[path.y as usize][path.x as usize] != b'.' {
                            blocked = true;
                        }
                    }
                    _ => {}
                }

                // Move all the boxes and the robot only if we're not blocked.
                if !blocked {
                    for &b in boxes.iter().rev() {
                        let mov = b + direction;
                        grid[mov.y as usize][mov.x as usize] = grid[b.y as usize][b.x as usize];
                        grid[b.y as usize][b.x as usize] = b'.';
                    }

                    robot = next;
                }
            }
            _ => {}
        }
    }

    coordinates(&grid)
}

fn parse(input: &str, part: u8) -> (Vec<Vec<u8>>, Vec<&u8>, Point) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut robot = None;

    (
        grid.lines()
            .enumerate()
            .map(|(y, line)| {
                if part == 1 {
                    line.bytes()
                        .enumerate()
                        .map(|(x, b)| {
                            if b == b'@' {
                                robot = Some(Point::new(x as i32, y as i32));
                                b'.'
                            } else {
                                b
                            }
                        })
                        .collect()
                } else {
                    line.bytes()
                        .enumerate()
                        .flat_map(|(x, b)| match b {
                            b'#' => [b'#', b'#'],
                            b'O' => [b'[', b']'],
                            b'.' => [b'.', b'.'],
                            b'@' => {
                                robot = Some(Point::new(x as i32 * 2, y as i32));
                                [b'.', b'.']
                            }
                            _ => unreachable!(),
                        })
                        .collect()
                }
            })
            .collect(),
        moves.lines().flat_map(|line| line.as_bytes()).collect(),
        robot.unwrap(),
    )
}

fn coordinates(grid: &[Vec<u8>]) -> usize {
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, b)| {
                    if *b == b'O' || *b == b'[' {
                        y * 100 + x
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}
