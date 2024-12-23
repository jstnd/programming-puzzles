use rustc_hash::{FxHashMap, FxHashSet};

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut number: usize = line.parse().unwrap();

            for _ in 0..2000 {
                number ^= (number * 64) % 16777216;
                number ^= (number / 32) % 16777216;
                number ^= (number * 2048) % 16777216;
            }

            number
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut sequences = FxHashMap::default();

    input.lines().for_each(|line| {
        let mut number: usize = line.parse().unwrap();
        let mut seen = FxHashSet::with_capacity_and_hasher(500, Default::default());
        let mut deltas = Vec::new();
        let mut previous = (number % 10) as i8;

        for _ in 0..2000 {
            number ^= (number * 64) % 16777216;
            number ^= (number / 32) % 16777216;
            number ^= (number * 2048) % 16777216;

            let price = (number % 10) as i8;
            deltas.push(price - previous);
            previous = price;

            if deltas.len() >= 4 {
                let len = deltas.len();
                let sequence = Sequence(
                    deltas[len - 4],
                    deltas[len - 3],
                    deltas[len - 2],
                    deltas[len - 1],
                );

                if !seen.contains(&sequence) {
                    seen.insert(sequence);
                    *sequences.entry(sequence).or_insert(0) += price as usize;
                }
            }
        }
    });

    *sequences
        .iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .1
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Sequence(i8, i8, i8, i8);
