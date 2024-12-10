use itertools::Itertools;

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (result, numbers) = line.split_once(": ").unwrap();
            let result: u64 = result.parse().unwrap();
            let numbers: Vec<u64> = numbers
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();

            if is_true(result, &numbers, &[Operation::Add, Operation::Multiply]) {
                return result;
            }

            0
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (result, numbers) = line.split_once(": ").unwrap();
            let result: u64 = result.parse().unwrap();
            let numbers: Vec<u64> = numbers
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();

            if is_true(
                result,
                &numbers,
                &[Operation::Add, Operation::Multiply, Operation::Concat],
            ) {
                return result;
            }

            0
        })
        .sum()
}

enum Operation {
    Add,
    Multiply,
    Concat,
}

fn is_true(result: u64, numbers: &[u64], operations: &[Operation]) -> bool {
    (0..numbers.len() - 1)
        .map(|_| operations.iter())
        .multi_cartesian_product()
        .any(|ops| {
            let mut current = numbers[0];

            for i in 1..numbers.len() {
                if current > result {
                    return false;
                }

                let number = numbers[i];

                current = match ops[i - 1] {
                    Operation::Add => current + number,
                    Operation::Multiply => current * number,
                    Operation::Concat => current * 10u64.pow(number.ilog10() + 1) + number,
                }
            }

            current == result
        })
}
