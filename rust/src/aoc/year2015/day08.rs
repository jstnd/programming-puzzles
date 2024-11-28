pub fn part1(input: &str) -> u16 {
    let mut in_escape = false;
    let mut count = 0;

    for b in input.bytes() {
        match b {
            b'x' => {
                if in_escape {
                    in_escape = false;
                    count += 3;
                }
            }
            b'\\' => {
                if in_escape {
                    in_escape = false;
                    count += 1;
                } else {
                    in_escape = true;
                }
            }
            b'\n' => count += 2,
            _ => {
                if in_escape {
                    in_escape = false;
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn part2(input: &str) -> u16 {
    input
        .bytes()
        .map(|b| match b {
            b'"' | b'\\' => 1,
            b'\n' => 2,
            _ => 0,
        })
        .sum()
}
