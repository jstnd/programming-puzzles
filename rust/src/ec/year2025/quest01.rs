use std::cmp;

pub fn part1(input: &str) -> &str {
    let (names, instructions) = parse(input);
    let mut name_pos: usize = 0;

    for instruction in instructions {
        name_pos = match instruction.direction {
            'L' => cmp::max(0, (name_pos as i32) - (instruction.moves as i32)) as usize,
            'R' => cmp::min(names.len() - 1, name_pos + instruction.moves),
            _ => name_pos,
        };
    }

    names[name_pos]
}

pub fn part2(input: &str) -> &str {
    let (names, instructions) = parse(input);
    let num_names = names.len() as i32;
    let mut name_pos: i32 = 0;

    for instruction in instructions {
        name_pos = match instruction.direction {
            'L' => (name_pos - instruction.moves as i32).rem_euclid(num_names),
            'R' => (name_pos + instruction.moves as i32).rem_euclid(num_names),
            _ => name_pos,
        };
    }

    names[name_pos as usize]
}

pub fn part3(input: &str) -> &str {
    let (mut names, instructions) = parse(input);
    let num_names = names.len() as i32;

    for instruction in instructions {
        let name_pos = match instruction.direction {
            'L' => (-(instruction.moves as i32)).rem_euclid(num_names),
            'R' => (instruction.moves as i32).rem_euclid(num_names),
            _ => 0,
        };

        names.swap(0, name_pos as usize);
    }

    names[0]
}

fn parse(input: &str) -> (Vec<&str>, Vec<Instruction>) {
    let (names, instructions) = input.split_once("\r\n\r\n").unwrap();

    (
        names.split(",").collect(),
        instructions
            .split(",")
            .map(|instruction| {
                let mut chars = instruction.chars();
                let direction = chars.next().unwrap();
                let moves: String = chars.collect();

                Instruction {
                    direction,
                    moves: moves.parse().unwrap(),
                }
            })
            .collect(),
    )
}

struct Instruction {
    pub direction: char,
    pub moves: usize,
}
