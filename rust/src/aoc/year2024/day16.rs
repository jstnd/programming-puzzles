use rustc_hash::{FxHashMap, FxHashSet};

use crate::util::point::Point;

pub fn part1(input: &str) -> usize {
    let (grid, start, end) = parse(input);
    let (min_cost, _) = a_star(&grid, start, end);
    min_cost
}

pub fn part2(input: &str) -> usize {
    let (grid, start, end) = parse(input);
    let (_, tiles) = a_star(&grid, start, end);

    tiles
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, Point, Point) {
    let mut start = None;
    let mut end = None;

    (
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.bytes()
                    .enumerate()
                    .map(|(x, b)| match b {
                        b'S' => {
                            start = Some(Point::new(x as i32, y as i32));
                            Tile::Path
                        }
                        b'E' => {
                            end = Some(Point::new(x as i32, y as i32));
                            Tile::Path
                        }
                        b'.' => Tile::Path,
                        b'#' => Tile::Wall,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
        start.unwrap(),
        end.unwrap(),
    )
}

#[derive(PartialEq, Eq)]
enum Tile {
    Path,
    Wall,
}

struct Node(Point, usize, u8, Vec<Point>);

fn a_star(grid: &[Vec<Tile>], start: Point, end: Point) -> (usize, usize) {
    let mut queue = vec![Node(start, 0, b'>', Vec::new())];
    let mut cost_map = FxHashMap::default();
    cost_map.insert((start, b'>'), 0);

    let mut min_cost = usize::MAX;
    let mut tiles = FxHashSet::default();

    while let Some(Node(point, cost, direction, mut history)) = queue.pop() {
        history.push(point);

        // We reached the end of the maze, return the path.
        if point == end {
            if cost < min_cost {
                min_cost = cost;
            }

            if cost == min_cost {
                tiles.extend(history);
            }

            continue;
        }

        for neighbor in Point::von_neumann() {
            let next = point + neighbor;

            // Skip this neighbor if it's not a path.
            if !is_path(grid, next) {
                continue;
            }

            let mut cost = cost + 1;
            let move_direction: u8 = neighbor.into();
            if direction != move_direction {
                cost += 1000;
            }

            if !cost_map.contains_key(&(next, move_direction))
                || cost <= cost_map[&(next, move_direction)]
            {
                cost_map.entry((next, move_direction)).insert_entry(cost);

                insert_queue(
                    &mut queue,
                    Node(next, cost, move_direction, history.clone()),
                );
            }
        }
    }

    (min_cost, tiles.len())
}

fn insert_queue(queue: &mut Vec<Node>, node: Node) {
    if queue.is_empty() {
        queue.push(node);
        return;
    }

    let mut index = 0;
    while index < queue.len() && queue[index].1 > node.1 {
        index += 1;
    }

    queue.insert(index, node);
}

fn is_path(grid: &[Vec<Tile>], point: Point) -> bool {
    if let Some(row) = grid.get(point.y as usize) {
        return row
            .get(point.x as usize)
            .is_some_and(|tile| *tile == Tile::Path);
    }

    false
}
