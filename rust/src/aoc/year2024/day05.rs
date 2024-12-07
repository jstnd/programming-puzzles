use std::collections::HashSet;

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
                    if rules.contains(&(*right, *left)) {
                        return 0;
                    }
                }
            }

            pages[pages.len() / 2] as u16
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
                    if rules.contains(&(*right, *left)) {
                        // For incorrectly-ordered updates, sort the pages
                        // according to the given ordering rules.
                        pages.sort_by(|a, b| {
                            if rules.contains(&(*b, *a)) {
                                std::cmp::Ordering::Less
                            } else {
                                std::cmp::Ordering::Greater
                            }
                        });

                        // Then return the middle page number.
                        return pages[pages.len() / 2] as u16;
                    }
                }
            }

            0
        })
        .sum()
}

fn parse(input: &str) -> (HashSet<(u8, u8)>, &str) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = rules.lines().fold(HashSet::new(), |mut set, line| {
        let (first, second) = line.split_once('|').unwrap();
        set.insert((first.parse().unwrap(), second.parse().unwrap()));

        set
    });

    (rules, updates)
}
