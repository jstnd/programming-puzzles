pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut sides: Vec<i32> = line.split('x').map(|s| s.parse().unwrap()).collect();
            sides.sort_unstable();

            2 * (sides[0] * sides[1] + sides[1] * sides[2] + sides[0] * sides[2])
                + sides[0] * sides[1]
        })
        .sum()
}

pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut sides: Vec<i32> = line.split('x').map(|s| s.parse().unwrap()).collect();
            sides.sort_unstable();

            2 * (sides[0] + sides[1]) + sides.iter().product::<i32>()
        })
        .sum()
}
