pub fn part1(input: &str) -> usize {
    let (ranges, ingredients) = parse(input);

    // Filter for the ingredients that are in any of the given ranges.
    ingredients
        .iter()
        .filter(|&&ingredient| ranges.iter().any(|range| range.contains(ingredient)))
        .count()
}

pub fn part2(input: &str) -> usize {
    let (mut ranges, _) = parse(input);
    let mut max_high = 0;

    // Sort the ranges so they can be processed in order of their lows.
    ranges.sort_unstable_by_key(|range| range.low);
    ranges
        .iter()
        .map(|range| {
            // If the high of this range is lower than the largest range's high
            // we've seen so far, that means this range contains only IDs that we've already seen.
            // For example, the second range below would be contained inside the first range, so
            // it would only contains IDs that we already saw when processing the first range.
            //
            // First range:  3-15
            // Second range: 5-14 (here, `max_high` would be set to a value higher than this range's high)
            if range.high < max_high {
                return 0;
            }

            // Since the ranges we are given can overlap, we need to make sure we don't count an ID more than once.
            // We do this by keeping track of the largest range's high we've seen, and subtracting that from the current
            // range's high if that value is higher than the current range's low. Otherwise, we just subtract the current range's low.
            // Also, since the ranges are inclusive, we need to add 1 after the subtraction is performed.
            let num_fresh = (range.high - std::cmp::max(max_high, range.low)) + 1;

            // Update the max high to the current range's high (plus 1 due to being inclusive).
            max_high = range.high + 1;

            // Return the number of fresh IDs for this range.
            num_fresh
        })
        .sum()
}

struct FreshRange {
    /// The low of the range
    low: usize,

    /// The high of the range
    high: usize,
}

impl FreshRange {
    fn new(low: usize, high: usize) -> Self {
        Self { low, high }
    }

    fn contains(&self, ingredient: usize) -> bool {
        ingredient >= self.low && ingredient <= self.high
    }
}

fn parse(input: &str) -> (Vec<FreshRange>, Vec<usize>) {
    // The ranges and the ingredients are separated by a blank line in the input file.
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    (
        ranges
            .lines()
            .map(|range| {
                let (left, right) = range.split_once("-").unwrap();
                FreshRange::new(left.trim().parse().unwrap(), right.trim().parse().unwrap())
            })
            .collect(),
        ingredients
            .lines()
            .map(|line| line.parse().unwrap())
            .collect(),
    )
}
