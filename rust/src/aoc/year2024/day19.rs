use rustc_hash::{FxHashMap, FxHashSet};

pub fn part1(input: &str) -> usize {
    let (patterns, designs) = parse(input);
    let mut possible = FxHashSet::default();

    designs
        .iter()
        .map(|design| is_possible(&patterns, &mut possible, design) as usize)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (patterns, designs) = parse(input);
    let mut possible = FxHashMap::default();

    designs
        .iter()
        .map(|design| is_possible_all(&patterns, &mut possible, design))
        .sum()
}

fn parse(input: &str) -> (FxHashSet<&str>, Vec<&str>) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();

    (
        FxHashSet::from_iter(patterns.split(", ")),
        designs.lines().collect(),
    )
}

fn is_possible<'a>(
    patterns: &FxHashSet<&str>,
    possible: &mut FxHashSet<&'a str>,
    design: &'a str,
) -> bool {
    for i in 1..=design.len() {
        let pattern = &design[0..i];

        // Check if this pattern has already been calculcated to be possible.
        if possible.contains(&pattern) {
            return true;
        }

        // Check if this pattern exists in our list of patterns, AND that we're
        // at the end of the design or the remainder of the design is possible.
        if patterns.contains(&pattern)
            && (pattern.len() == design.len() || is_possible(patterns, possible, &design[i..]))
        {
            // Add this design to our cache of possible designs.
            possible.insert(design);
            return true;
        }
    }

    false
}

fn is_possible_all<'a>(
    patterns: &FxHashSet<&str>,
    possible: &mut FxHashMap<&'a str, usize>,
    design: &'a str,
) -> usize {
    // Check if this design has already had its number of possible
    // arrangements calculated, and return the result if so.
    if possible.contains_key(&design) {
        return possible[&design];
    }

    let mut count = 0;

    for i in 1..=design.len() {
        let pattern = &design[0..i];

        if patterns.contains(&pattern) {
            count += if pattern.len() == design.len() {
                // If the pattern we're currently checking is at the end
                // of the design, add one to the number of possible arrangements.
                1
            } else {
                // Otherwise, calculate the number of possible arrangements for
                // the remainder of the design, add that number to our cache, and
                // add it to the total number of possible arrangements.
                let count = is_possible_all(patterns, possible, &design[i..]);
                possible.insert(&design[i..], count);
                count
            };
        }
    }

    count
}
