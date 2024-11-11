pub fn part1(input: &str) -> i32 {
    input.bytes().map(|b| if b == b'(' { 1 } else { -1 }).sum()
}

pub fn part2(input: &str) -> i32 {
    input
        .bytes()
        .scan(0, |state, b| {
            *state += if b == b'(' { 1 } else { -1 };

            Some(*state)
        })
        .take_while(|&f| f != -1)
        .count() as i32
        + 1
}
