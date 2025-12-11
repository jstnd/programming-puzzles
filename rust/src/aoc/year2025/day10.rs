use std::collections::VecDeque;

use itertools::{Itertools, Position};

pub fn part1(input: &str) -> u16 {
    let machines = parse(input);
    machines.iter().map(fewest_light_presses).sum()
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

fn fewest_light_presses(machine: &Machine) -> u16 {
    let mut queue = VecDeque::new();
    queue.push_back((0_u16, 1_u16));

    while let Some((lights, presses)) = queue.pop_front() {
        for button_group in &machine.buttons {
            let next = lights ^ button_group;

            if next == machine.lights {
                return presses;
            } else {
                queue.push_back((next, presses + 1));
            }
        }
    }

    unreachable!()
}

fn fewest_joltage_presses(machine: &Machine) -> usize {
    0
}
