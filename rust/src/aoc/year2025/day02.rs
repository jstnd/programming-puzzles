use std::ops::RangeInclusive;

use fancy_regex::Regex;

use crate::util::num;

pub fn part1(input: &str) -> usize {
    parse(input)
        .map(|range| {
            range
                .filter(|id| {
                    // Since we only have to find numbers that are made up of ONLY some sequence of digits repeated TWICE,
                    // we can just split the number into two parts and check if the parts are equal
                    let (left, right) = num::split(*id);
                    left == right
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    // We can use regex here as a simple solution for finding a repeating pattern.
    // => (.+?) -> matches any character between one and unlimited times, as few times as possible
    // => \1+ -> matches the same text as found in the previous match between one and unlimited times, as many times as possible
    let repeating = Regex::new("^(.+?)\\1+$").unwrap();

    parse(input)
        .map(|range| {
            range
                .filter(|id| repeating.is_match(&id.to_string()).unwrap())
                .sum::<usize>()
        })
        .sum()
}

fn parse(input: &str) -> impl Iterator<Item = RangeInclusive<usize>> + use<'_> {
    input.split(",").map(|range| {
        let (left, right) = range.split_once("-").unwrap();
        let left = left.trim().parse::<usize>().unwrap();
        let right = right.trim().parse::<usize>().unwrap();
        left..=right
    })
}
