use rustc_hash::{FxHashMap, FxHashSet};

use crate::util::point::Point;

pub fn part1(input: &str) -> u32 {
    let (first, second) = input.trim().split_once('\n').unwrap();
    let mut visited = FxHashSet::default();

    let central = Point::origin();
    let mut closest = u32::MAX;

    let mut position = central;
    first.split(',').for_each(|path| {
        let bytes = path.as_bytes();
        let direction = Point::from(bytes[0]);
        let length = path[1..].parse::<usize>().unwrap();

        for _ in 0..length {
            position += direction;
            visited.insert(position);
        }
    });

    position = central;
    second.split(',').for_each(|path| {
        let bytes = path.as_bytes();
        let direction = Point::from(bytes[0]);
        let length = path[1..].parse::<usize>().unwrap();

        for _ in 0..length {
            position += direction;

            if visited.contains(&position) {
                let distance = position.manhattan_distance(&central);
                if distance < closest {
                    closest = distance;
                }
            }
        }
    });

    closest
}

pub fn part2(input: &str) -> u32 {
    let (first, second) = input.trim().split_once('\n').unwrap();
    let mut visited = FxHashMap::default();

    let central = Point::origin();
    let mut fewest = u32::MAX;

    let mut position = central;
    let mut steps = 0;
    first.split(',').for_each(|path| {
        let bytes = path.as_bytes();
        let direction = Point::from(bytes[0]);
        let length = path[1..].parse::<usize>().unwrap();

        for _ in 0..length {
            position += direction;
            steps += 1;
            visited.entry(position).or_insert(steps);
        }
    });

    position = central;
    steps = 0;
    second.split(',').for_each(|path| {
        let bytes = path.as_bytes();
        let direction = Point::from(bytes[0]);
        let length = path[1..].parse::<usize>().unwrap();

        for _ in 0..length {
            position += direction;
            steps += 1;

            if visited.contains_key(&position) {
                let combined = visited[&position] + steps;
                if combined < fewest {
                    fewest = combined;
                }
            }
        }
    });

    fewest
}
