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

            if is_true(
                result,
                numbers[0],
                &numbers[1..],
                &[Operation::Add, Operation::Multiply],
            ) {
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
                numbers[0],
                &numbers[1..],
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

fn is_true(result: u64, current: u64, numbers: &[u64], operations: &[Operation]) -> bool {
    if current > result {
        return false;
    }

    operations.iter().any(|operation| {
        let current = match operation {
            Operation::Add => current + numbers[0],
            Operation::Multiply => current * numbers[0],
            Operation::Concat => current * 10u64.pow(numbers[0].ilog10() + 1) + numbers[0],
        };

        if numbers.len() == 1 {
            return current == result;
        }

        is_true(result, current, &numbers[1..], operations)
    })
}
