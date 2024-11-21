use crate::util::point::Point;

pub fn part1(input: &str) -> i32 {
    let mut lights = [[false; 1_000]; 1_000];

    for Instruction { action, start, end } in parse_instructions(input) {
        for y in start.y..=end.y {
            for x in start.x..=end.x {
                let y = y as usize;
                let x = x as usize;

                match action {
                    Action::Toggle => lights[y][x] = !lights[y][x],
                    Action::On => lights[y][x] = true,
                    Action::Off => lights[y][x] = false,
                }
            }
        }
    }

    lights.iter().flatten().filter(|light| **light).count() as i32
}

pub fn part2(input: &str) -> i32 {
    let mut lights = vec![[0u32; 1_000]; 1_000];

    for Instruction { action, start, end } in parse_instructions(input) {
        for y in start.y..=end.y {
            for x in start.x..=end.x {
                let y = y as usize;
                let x = x as usize;

                match action {
                    Action::Toggle => lights[y][x] += 2,
                    Action::On => lights[y][x] += 1,
                    Action::Off => {
                        if lights[y][x] > 0 {
                            lights[y][x] -= 1;
                        }
                    }
                }
            }
        }
    }

    lights.iter().flatten().sum::<u32>() as i32
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|mut line| {
            line = line.trim_start_matches("turn ");

            let instruction_parts: Vec<&str> = line.split_whitespace().collect();
            let coordinates1 = instruction_parts[1].split_once(',').unwrap();
            let coordinates2 = instruction_parts[3].split_once(',').unwrap();

            Instruction {
                action: Action::from(instruction_parts[0]),
                start: Point::new(
                    coordinates1.0.parse().unwrap(),
                    coordinates1.1.parse().unwrap(),
                ),
                end: Point::new(
                    coordinates2.0.parse().unwrap(),
                    coordinates2.1.parse().unwrap(),
                ),
            }
        })
        .collect()
}

struct Instruction {
    action: Action,
    start: Point,
    end: Point,
}

enum Action {
    Toggle,
    On,
    Off,
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        match value {
            "toggle" => Action::Toggle,
            "on" => Action::On,
            "off" => Action::Off,
            _ => unreachable!(),
        }
    }
}
