use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
    let (mut left, mut right) = parse_lists(input);
    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(num1, num2)| num1.abs_diff(*num2))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let (left, right) = parse_lists(input);

    let frequencies = right.iter().fold(HashMap::new(), |mut map, element| {
        *map.entry(element).or_insert(0) += 1;
        map
    });

    left.iter()
        .map(|element| element * frequencies.get(element).unwrap_or(&0))
        .sum()
}

fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let (num1, num2) = line.split_once("   ").unwrap();
        left.push(num1.parse().unwrap());
        right.push(num2.parse().unwrap());
    }

    (left, right)
}
