use rustc_hash::FxHashMap;

pub fn part1(input: &str) -> usize {
    let (mut wires, z_wires) = parse(input);
    let mut number = 0usize;

    for wire in z_wires {
        let input = get_input(&mut wires, wire);
        number = (number << 1) | input as usize;
    }

    number
}

pub fn part2(input: &str) -> u32 {
    0
}

fn parse(input: &str) -> (FxHashMap<&str, Input>, Vec<&str>) {
    let (initial, gates) = input.split_once("\n\n").unwrap();
    let mut wires = FxHashMap::default();
    let mut z_wires = Vec::new();

    initial.lines().for_each(|line| {
        let (wire, input) = line.split_once(": ").unwrap();
        wires.insert(wire, Input::Value(input.parse().unwrap()));
    });

    gates.lines().for_each(|line| {
        let (input, wire) = line.split_once(" -> ").unwrap();

        let input: Vec<&str> = input.split_whitespace().collect();
        let input = match input[1] {
            "AND" => Input::And(input[0], input[2]),
            "OR" => Input::Or(input[0], input[2]),
            "XOR" => Input::Xor(input[0], input[2]),
            _ => unreachable!(),
        };

        wires.insert(wire, input);
        if wire.starts_with('z') {
            z_wires.push(wire);
        }
    });

    z_wires.sort();
    z_wires.reverse();
    (wires, z_wires)
}

#[derive(Clone, Copy)]
enum Input<'a> {
    Value(u8),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

fn get_input<'a>(wires: &mut FxHashMap<&'a str, Input<'a>>, wire: &'a str) -> u8 {
    let input = match wires[wire] {
        Input::Value(value) => value,
        Input::And(left, right) => {
            let left = get_input(wires, left);
            let right = get_input(wires, right);
            left & right
        }
        Input::Or(left, right) => {
            let left = get_input(wires, left);
            let right = get_input(wires, right);
            left | right
        }
        Input::Xor(left, right) => {
            let left = get_input(wires, left);
            let right = get_input(wires, right);
            left ^ right
        }
    };

    wires.entry(wire).insert_entry(Input::Value(input));
    input
}
