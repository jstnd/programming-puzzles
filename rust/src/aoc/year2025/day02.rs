use std::ops::RangeInclusive;

// use fancy_regex::Regex;

use crate::util::num;

pub fn part1(input: &str) -> usize {
    parse(input)
        .flatten()
        .filter(|id| {
            // Since we only have to find numbers that are made up of ONLY some sequence of digits repeated TWICE,
            // we can just split the number into two parts and check if the parts are equal
            let (left, right) = num::split(*id);
            left == right
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    // We can use regex here as a simple solution for finding a repeating pattern.
    // => (.+?) -> matches any character between one and unlimited times, as few times as possible
    // => \1+ -> matches the same text as found in the previous match between one and unlimited times, as many times as possible
    // let repeating = Regex::new("^(.+?)\\1+$").unwrap();

    // parse(input)
    //     .flatten()
    //     .filter(|id| repeating.is_match(&id.to_string()).unwrap())
    //     .sum()

    // We can also use a different trick where for each id, we:
    // 1. create a string with the id repeated twice
    // 2. remove the first and last characters
    // 3. check if the remaining string still contains the original id
    //
    // If the remaining string still contains the original id,
    // that means the id has a repeating pattern.
    // NOTE: see periodic strings
    parse(input)
        .flatten()
        .filter(|id| {
            // An id that is a single digit won't fit the criteria, so we return early.
            if num::len(*id) <= 1 {
                return false;
            }

            let id = id.to_string();
            let id_repeat = id.repeat(2);
            id_repeat[1..id_repeat.len() - 1].contains(&id)
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
