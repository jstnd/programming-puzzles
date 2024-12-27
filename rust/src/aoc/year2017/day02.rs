pub fn part1(input: &str) -> u32 {
    parse(input)
        .iter_mut()
        .map(|row| {
            row.sort();
            row[row.len() - 1] - row[0]
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    parse(input)
        .iter()
        .map(|row| {
            for (i, first) in row.iter().enumerate() {
                for second in row.iter().skip(i + 1) {
                    if first % second == 0 {
                        return first / second;
                    } else if second % first == 0 {
                        return second / first;
                    }
                }
            }

            0
        })
        .sum()
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect()
}
