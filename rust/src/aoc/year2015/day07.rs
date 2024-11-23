use std::collections::HashMap;

pub fn part1(input: &str) -> u16 {
    let (instructions, mut signals) = parse_instructions(input);
    get_signal("a", &instructions, &mut signals)
}

pub fn part2(input: &str) -> u16 {
    let a_signal = part1(input);
    let (instructions, mut signals) = parse_instructions(input);
    *signals.get_mut("b").unwrap() += a_signal;
    get_signal("a", &instructions, &mut signals)
}

fn parse_instructions(input: &str) -> (HashMap<&str, Operation>, HashMap<&str, u16>) {
    let mut instructions = HashMap::new();
    let mut signals = HashMap::new();

    for line in input.lines() {
        let parts = line.split_once(" -> ").unwrap();
        let operation: Vec<&str> = parts.0.split_whitespace().collect();

        let operation = match operation.len() {
            1 => Operation::Single(Operand::from(operation[0])),
            2 => Operation::Not(Operand::from(operation[1])),
            3 => {
                let left = Operand::from(operation[0]);
                let right = Operand::from(operation[2]);

                match operation[1] {
                    "AND" => Operation::And(left, right),
                    "OR" => Operation::Or(left, right),
                    "LSHIFT" => Operation::LShift(left, right),
                    "RSHIFT" => Operation::RShift(left, right),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        };

        instructions.insert(parts.1, operation);
        signals.insert(parts.1, 0);
    }

    (instructions, signals)
}

fn get_signal(
    wire: &str,
    instructions: &HashMap<&str, Operation>,
    signals: &mut HashMap<&str, u16>,
) -> u16 {
    if let Some(signal) = signals.get(wire) {
        if *signal > 0 {
            return *signal;
        }
    }

    *signals.get_mut(wire).unwrap() += match instructions.get(wire).unwrap() {
        Operation::Single(operand) => resolve_operand(operand, instructions, signals),
        Operation::And(left, right) => {
            resolve_operand(left, instructions, signals)
                & resolve_operand(right, instructions, signals)
        }
        Operation::Or(left, right) => {
            resolve_operand(left, instructions, signals)
                | resolve_operand(right, instructions, signals)
        }
        Operation::Not(operand) => !resolve_operand(operand, instructions, signals),
        Operation::LShift(left, right) => {
            resolve_operand(left, instructions, signals)
                << resolve_operand(right, instructions, signals)
        }
        Operation::RShift(left, right) => {
            resolve_operand(left, instructions, signals)
                >> resolve_operand(right, instructions, signals)
        }
    };

    *signals.get(wire).unwrap()
}

fn resolve_operand(
    operand: &Operand,
    instructions: &HashMap<&str, Operation>,
    signals: &mut HashMap<&str, u16>,
) -> u16 {
    match operand {
        Operand::Value(value) => *value,
        Operand::Wire(wire) => get_signal(wire, instructions, signals),
    }
}

enum Operation {
    Single(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Not(Operand),
    LShift(Operand, Operand),
    RShift(Operand, Operand),
}

enum Operand {
    Value(u16),
    Wire(String),
}

impl From<&str> for Operand {
    fn from(value: &str) -> Self {
        if let Ok(value) = value.parse::<u16>() {
            Operand::Value(value)
        } else {
            Operand::Wire(value.to_string())
        }
    }
}
