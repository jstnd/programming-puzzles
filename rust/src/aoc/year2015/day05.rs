use fancy_regex::Regex;

pub fn part1(input: &str) -> i32 {
    let vowels = Regex::new(r"(.*[aeiou]){3}").unwrap();
    let doubles = Regex::new(r"(.)\1").unwrap();
    let excludes = Regex::new(r"(ab|cd|pq|xy)").unwrap();

    input
        .lines()
        .filter(|line| {
            vowels.is_match(line).unwrap()
                && doubles.is_match(line).unwrap()
                && !excludes.is_match(line).unwrap()
        })
        .count() as i32
}

pub fn part2(input: &str) -> i32 {
    let pairs = Regex::new(r"(..).*\1").unwrap();
    let repeats = Regex::new(r"(.).\1").unwrap();

    input
        .lines()
        .filter(|line| pairs.is_match(line).unwrap() && repeats.is_match(line).unwrap())
        .count() as i32
}
