pub fn part1(input: &str) -> u32 {
    let input: Vec<char> = input.trim().chars().collect();
    let ends = if input[0] == input[input.len() - 1] {
        input[0].to_digit(10).unwrap()
    } else {
        0
    };

    input
        .windows(2)
        .map(|window| {
            if window[0] == window[1] {
                window[0].to_digit(10).unwrap()
            } else {
                0
            }
        })
        .sum::<u32>()
        + ends
}

pub fn part2(input: &str) -> u32 {
    let input: Vec<char> = input.trim().chars().collect();
    let steps = input.len() / 2;

    input
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let compare_index = (i + steps) % input.len();

            if *c == input[compare_index] {
                c.to_digit(10).unwrap()
            } else {
                0
            }
        })
        .sum()
}
