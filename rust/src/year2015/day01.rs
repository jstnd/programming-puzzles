pub fn get_input() -> String {
    crate::util::input::get_input(2015, 1)
}

pub fn part1(input: &str) -> i32 {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum()
}

pub fn part2(input: &str) -> i32 {
    input
        .chars()
        .scan(0, |state, c| {
            *state += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };

            Some(*state)
        })
        .take_while(|&f| f != -1)
        .count() as i32
        + 1
}
