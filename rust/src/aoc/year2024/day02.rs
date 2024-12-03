pub fn part1(input: &str) -> u16 {
    input
        .lines()
        .map(|line| {
            let levels = line.split_whitespace().map(|level| level.parse().unwrap());
            is_safe(levels).is_none() as u16
        })
        .sum()
}

pub fn part2(input: &str) -> u16 {
    input
        .lines()
        .map(|line| {
            let levels = line.split_whitespace().map(|level| level.parse().unwrap());

            if let Some(index) = is_safe(levels.clone()) {
                for i in (0..=index).rev() {
                    let levels = levels
                        .clone()
                        .enumerate()
                        .filter_map(|(j, level)| (i != j).then_some(level));

                    if is_safe(levels).is_none() {
                        return 1;
                    }
                }

                return 0;
            }

            1
        })
        .sum()
}

fn is_safe<I: Iterator<Item = u8>>(levels: I) -> Option<usize> {
    let mut increasing = false;
    let mut previous = 0u8;

    for (index, level) in levels.enumerate() {
        if index == 0 {
            previous = level;
            continue;
        } else if index == 1 {
            increasing = level > previous;
        }

        if !(1..=3).contains(&level.abs_diff(previous)) {
            return Some(index);
        }

        if increasing {
            if level < previous {
                return Some(index);
            }
        } else if level > previous {
            return Some(index);
        }

        previous = level;
    }

    None
}
