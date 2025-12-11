use itertools::{Itertools, Position};

pub fn part1(input: &str) -> usize {
    parse(input).iter().map(fewest_light_presses).sum()
}

pub fn part2(input: &str) -> usize {
    let machines = parse(input);
    machines.iter().map(fewest_joltage_presses).sum()
}

#[derive(Default)]
struct Machine {
    lights: u16,
    buttons: Vec<u16>,
    joltages: Vec<u16>,
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut machine = Machine::default();

            line.split_whitespace().with_position().for_each(
                |(position, element)| match position {
                    Position::First => {
                        machine.lights =
                            element
                                .trim_matches(['[', ']'])
                                .bytes()
                                .rev()
                                .fold(0, |acc, b| {
                                    //
                                    acc << 1 | (b == b'#') as u16
                                });
                    }
                    Position::Last => {
                        machine.joltages = element
                            .trim_matches(['{', '}'])
                            .split(",")
                            .map(|joltage| joltage.parse().unwrap())
                            .collect()
                    }
                    _ => {
                        let button_group = element
                            .trim_matches(['(', ')'])
                            .split(",")
                            .map(|button| button.parse::<u8>().unwrap())
                            .fold(0, |acc, button| {
                                //
                                acc ^ (1 << button)
                            });

                        machine.buttons.push(button_group);
                    }
                },
            );

            machine
        })
        .collect()
}

fn fewest_light_presses(machine: &Machine) -> usize {
    machine
        .buttons
        .iter()
        .powerset()
        .find(|button_group| {
            //
            let lights = button_group.iter().fold(0, |acc, button| {
                //
                acc ^ *button
            });

            lights == machine.lights
        })
        .map(|button_group| button_group.len())
        .unwrap()
}

fn fewest_joltage_presses(machine: &Machine) -> usize {
    0
}
