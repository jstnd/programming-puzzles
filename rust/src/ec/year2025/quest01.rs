pub fn part1(input: &str) -> &str {
    let (names, steps) = parse(input);
    let mut name_pos = 0;

    for step in steps {
        name_pos += step;
        name_pos = name_pos.clamp(0, names.len() as i32 - 1);
    }

    names[name_pos as usize]
}

pub fn part2(input: &str) -> &str {
    let (names, steps) = parse(input);
    let mut name_pos = steps.iter().sum::<i32>();
    name_pos = name_pos.rem_euclid(names.len() as i32);

    names[name_pos as usize]
}

pub fn part3(input: &str) -> &str {
    let (mut names, steps) = parse(input);

    for step in steps {
        let name_pos = step.rem_euclid(names.len() as i32);
        names.swap(0, name_pos as usize);
    }

    names[0]
}

fn parse(input: &str) -> (Vec<&str>, Vec<i32>) {
    let (names, instructions) = input.split_once("\r\n\r\n").unwrap();

    (
        names.split(",").collect(),
        instructions
            .split(",")
            .map(|instruction| {
                let (direction, steps) = instruction.split_at(1);
                let steps = steps.parse::<i32>().unwrap();

                match direction {
                    "L" => -steps,
                    "R" => steps,
                    _ => 0,
                }
            })
            .collect(),
    )
}
