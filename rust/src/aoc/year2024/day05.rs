use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> u16 {
    let (rules, updates) = parse(input);

    updates
        .lines()
        .map(|line| {
            let pages: Vec<u8> = line.split(',').map(|page| page.parse().unwrap()).collect();

            for (i, left) in pages.iter().enumerate() {
                for right in pages.iter().skip(i + 1) {
                    // Check if an ordering rule exists where the right
                    // page should be printed before the left page.
                    if let Some(rule) = rules.get(right) {
                        if rule.contains(left) {
                            return 0;
                        }
                    }
                }
            }

            pages[(pages.len() - 1) / 2] as u16
        })
        .sum()
}

pub fn part2(input: &str) -> u16 {
    let (rules, updates) = parse(input);

    updates
        .lines()
        .map(|line| {
            let mut pages: Vec<u8> = line.split(',').map(|page| page.parse().unwrap()).collect();

            for (i, left) in pages.iter().enumerate() {
                for right in pages.iter().skip(i + 1) {
                    // Check if an ordering rule exists where the right
                    // page should be printed before the left page.
                    if let Some(rule) = rules.get(right) {
                        if rule.contains(left) {
                            // For incorrectly-ordered updates, sort the pages
                            // according to the given ordering rules.
                            pages.sort_by(|a, b| {
                                if let Some(rule) = rules.get(b) {
                                    if rule.contains(a) {
                                        return std::cmp::Ordering::Less;
                                    }
                                }

                                std::cmp::Ordering::Greater
                            });

                            // Then return the middle page number.
                            return pages[(pages.len() - 1) / 2] as u16;
                        }
                    }
                }
            }

            0
        })
        .sum()
}

fn parse(input: &str) -> (HashMap<u8, HashSet<u8>>, &str) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .fold(HashMap::<u8, HashSet<u8>>::new(), |mut map, line| {
            let (first, second) = line.split_once('|').unwrap();
            map.entry(first.parse().unwrap())
                .or_default()
                .insert(second.parse().unwrap());

            map
        });

    (rules, updates)
}
