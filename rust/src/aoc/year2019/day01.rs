pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mass: usize = line.parse().unwrap();
            mass / 3 - 2
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut mass: usize = line.parse().unwrap();
            let mut total = 0;

            while mass / 3 > 2 {
                mass = mass / 3 - 2;
                total += mass;
            }

            total
        })
        .sum()
}
