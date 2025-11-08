pub fn part1(input: &str) -> u32 {
    let mut crates = parse(input);
    crates.sort_unstable();
    crates.dedup();
    crates.iter().sum()
}

pub fn part2(input: &str) -> u32 {
    let mut crates = parse(input);
    crates.sort_unstable();
    crates.dedup();
    crates.iter().take(20).sum()
}

pub fn part3(input: &str) -> usize {
    let crates = parse(input);
    *crate::util::array::frequencies(&crates)
        .values()
        .max()
        .unwrap()
}

fn parse(input: &str) -> Vec<u32> {
    input
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}
