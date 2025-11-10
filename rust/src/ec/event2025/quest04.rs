pub fn part1(input: &str) -> u64 {
    let mut turns = 1_f64;
    parse(input).windows(2).for_each(|w| {
        turns *= w[0].left / w[1].left;
    });

    (turns * 2025.).floor() as u64
}

pub fn part2(input: &str) -> u64 {
    let mut turns = 10_000_000_000_000_f64;
    parse(input).windows(2).rev().for_each(|w| {
        turns *= w[1].left / w[0].left;
    });

    turns.ceil() as u64
}

pub fn part3(input: &str) -> u64 {
    let mut turns = 1_f64;
    parse(input).windows(2).for_each(|w| {
        let first = if w[0].right != 0. {
            w[0].right
        } else {
            w[0].left
        };

        turns *= first / w[1].left;
    });

    (turns * 100.).floor() as u64
}

fn parse(input: &str) -> Vec<Gear> {
    input
        .lines()
        .map(|line| {
            if line.contains("|") {
                let (left, right) = line.split_once("|").unwrap();
                Gear {
                    left: left.parse().unwrap(),
                    right: right.parse().unwrap(),
                }
            } else {
                Gear {
                    left: line.parse().unwrap(),
                    right: 0.,
                }
            }
        })
        .collect()
}

struct Gear {
    left: f64,
    right: f64,
}
