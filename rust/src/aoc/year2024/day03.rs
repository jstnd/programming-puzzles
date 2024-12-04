use fancy_regex::Regex;

pub fn part1(input: &str) -> u32 {
    let mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    mul.captures_iter(input)
        .map(|capture| {
            let capture = capture.unwrap();
            let left: u32 = capture.get(1).unwrap().as_str().parse().unwrap();
            let right: u32 = capture.get(2).unwrap().as_str().parse().unwrap();

            left * right
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mul = Regex::new(r"do\(\)|don\'t\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut enabled = true;

    mul.captures_iter(input)
        .map(|capture| {
            let capture = capture.unwrap();

            match capture.get(0).unwrap().as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if enabled {
                        let left: u32 = capture.get(1).unwrap().as_str().parse().unwrap();
                        let right: u32 = capture.get(2).unwrap().as_str().parse().unwrap();

                        return left * right;
                    }
                }
            }

            0
        })
        .sum()
}
