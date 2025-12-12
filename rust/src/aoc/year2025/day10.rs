use itertools::{Itertools, Position};
use microlp::{ComparisonOp, LinearExpr, OptimizationDirection, Problem, Variable};

pub fn part1(input: &str) -> usize {
    parse(input).iter().map(fewest_light_presses).sum()
}

pub fn part2(input: &str) -> usize {
    parse(input).iter().map(fewest_joltage_presses).sum()
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
                                    // Build a bitmask out of the light pattern.
                                    // As an example, this is a visual of this process for a light pattern of [#.]:
                                    //
                                    //   acc = 0b0000_0000, b = #
                                    //   => (0b0000_0000 << 1) | 1 as u16
                                    //   => 0b0000_0000 | 0b0000_0001
                                    //   => 0b0000_0001
                                    //
                                    //   acc = 0b0000_0001, b = .
                                    //   => (0b0000_0001 << 1) | 0 as u16
                                    //   => 0b0000_0010 | 0b0000_0000
                                    //   => 0b0000_0010

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
                                // Build a bitmask out of each button group.
                                // As an example, this is a visual of this process for a button group of (0, 2):
                                //
                                //   acc = 0b0000_0000, button = 0
                                //   => 0b0000_0000 ^ (1 << 0)
                                //   => 0b0000_0000 ^ 0b0000_0001
                                //   => 0b0000_0001
                                //
                                //   acc = 0b0000_0001, button = 2
                                //   => 0b0000_0001 ^ (1 << 2)
                                //   => 0b0000_0001 ^ 0b0000_0100
                                //   => 0b0000_0101

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
        .find(|button_groups| {
            // Since the button groups are in bitmask form, we can combine
            // them using bitwise XOR to find the resulting light pattern.
            let lights = button_groups.iter().fold(0, |acc, button| acc ^ *button);

            // Determine if the resulting light pattern from combining all
            // the button groups matches the pattern for the machine.
            lights == machine.lights
        })
        .map(|button_group| button_group.len())
        .unwrap()
}

fn fewest_joltage_presses(machine: &Machine) -> usize {
    let mut problem = Problem::new(OptimizationDirection::Minimize);

    // Add a variable for each button group.
    let variables: Vec<Variable> = machine
        .buttons
        .iter()
        .map(|_| {
            // The coefficient will only be 1, and the button group can be pressed 0 or more times.
            problem.add_integer_var(1.0, (0, i32::MAX))
        })
        .collect();

    // Add an expression to the problem for each joltage value.
    for (i, &joltage) in machine.joltages.iter().enumerate() {
        let mut expression = LinearExpr::empty();

        // Check every button group to see if it would affect the current joltage's position if pressed.
        for (button_group, &variable) in machine.buttons.iter().zip(&variables) {
            // To find out if a button group would affect the current joltage's position,
            // we can use bitwise OR and left shifting with the button group's mask to
            // see if there's a 1 at the joltage's position in the mask.
            //
            // For example, if we want to find out if the button group (0, 2, 4)
            // would affect a joltage at position 2, we can do the following:
            //
            //   (0, 2, 4) (mask)          => 0b0001_0101
            //   1 << 2 (joltage position) => 0b0000_0100
            //   bitwise AND               => 0b0000_0100
            //
            // Since there's a 1 bit at position 2 in each binary number, the
            // bitwise AND results in a non-zero number which effectively tells
            // us that the button group would affect a joltage at position 2.
            if button_group & (1 << i) != 0 {
                expression.add(variable, 1.0);
            }
        }

        // Add a constraint that the built expression must equal the current joltage value.
        problem.add_constraint(expression, ComparisonOp::Eq, joltage as f64);
    }

    problem.solve().unwrap().objective().round() as usize
}
