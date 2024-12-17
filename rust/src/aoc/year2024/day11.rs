use rustc_hash::FxHashMap;

use crate::util::num;

pub fn part1(input: &str) -> usize {
    solve(input, 25)
}

pub fn part2(input: &str) -> usize {
    solve(input, 75)
}

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solve(input: &str, blinks: usize) -> usize {
    let stones = parse(input);
    let mut cache = FxHashMap::default();

    stones
        .iter()
        .map(|stone| num_stones(*stone, blinks, &mut cache))
        .sum()
}

fn num_stones(number: u64, blinks: usize, cache: &mut FxHashMap<(u64, usize), usize>) -> usize {
    if blinks == 0 {
        return 1;
    } else if let Some(entry) = cache.get(&(number, blinks)) {
        return *entry;
    }

    let num_stones = blink(number, 1)
        .iter()
        .map(|stone| num_stones(*stone, blinks - 1, cache))
        .sum();

    cache.insert((number, blinks), num_stones);
    num_stones
}

fn blink(number: u64, blinks: usize) -> Vec<u64> {
    let mut stones = vec![number];

    for _ in 0..blinks {
        let mut next = Vec::new();

        for &stone in stones.iter() {
            if stone == 0 {
                next.push(1);
            } else if num::len(stone) % 2 == 0 {
                let (left, right) = num::split(stone);
                next.push(left);
                next.push(right);
            } else {
                next.push(stone * 2024);
            }
        }

        stones = next;
    }

    stones
}
