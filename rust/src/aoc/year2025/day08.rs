use itertools::Itertools;
use rustc_hash::FxHashMap;

pub fn part1(input: &str) -> usize {
    // Track each junction box and its circuit number.
    let mut circuits = FxHashMap::default();

    // Track the number of junction boxes in each circuit.
    // Since we only need to process the first 1,000 combinations,
    // the maximum number of circuits (with minimum of 2 junction boxes) will be 500.
    let mut circuit_counts = vec![0; 500];

    // Track an incrementing circuit number for unique IDs.
    let mut current_circuit = 0;

    parse(input)
        .iter()
        .tuple_combinations()
        .sorted_unstable_by(|(a, b), (c, d)| a.distance(b).total_cmp(&c.distance(d)))
        .take(1_000)
        .for_each(|(a, b)| {
            // Check if both junction boxes are currently in a circuit.
            if circuits.contains_key(a) && circuits.contains_key(b) {
                let circuit_num_a = circuits[a];
                let circuit_num_b = circuits[b];

                // If both junction boxes are in the same circuit, nothing needs to be done.
                if circuit_num_a == circuit_num_b {
                    return;
                }

                // Otherwise, we need to combine the two circuits.
                // Change all boxes in circuit B to be in circuit A instead.
                for (_, value) in circuits.iter_mut() {
                    if *value == circuit_num_b {
                        *value = circuit_num_a;
                    }
                }

                // Combine the circuit number counts.
                circuit_counts[circuit_num_a] += circuit_counts[circuit_num_b];
                circuit_counts[circuit_num_b] = 0;
                return;
            }

            // Determine the circuit for the current pair of junction boxes.
            let circuit_num = if circuits.contains_key(a) {
                circuits[a]
            } else if circuits.contains_key(b) {
                circuits[b]
            } else {
                current_circuit += 1;
                current_circuit
            };

            // Add the junction boxes to the determined circuit.
            if !circuits.contains_key(a) {
                circuits.insert(a, circuit_num);
                circuit_counts[circuit_num] += 1;
            }

            if !circuits.contains_key(b) {
                circuits.insert(b, circuit_num);
                circuit_counts[circuit_num] += 1;
            }
        });

    circuit_counts.sort_unstable_by(|a, b| b.cmp(a));
    circuit_counts.iter().take(3).product()
}

pub fn part2(input: &str) -> usize {
    // Track each junction box and its circuit number.
    let mut circuits = FxHashMap::default();

    // Track an incrementing circuit number for unique IDs.
    let mut current_circuit = 0;

    // Track the total number of circuits.
    let mut num_circuits = 0;

    let boxes = parse(input);
    let boxes = boxes
        .iter()
        .tuple_combinations()
        .sorted_unstable_by(|(a, b), (c, d)| a.distance(b).total_cmp(&c.distance(d)));

    for (a, b) in boxes {
        // Check if both junction boxes are currently in a circuit.
        if circuits.contains_key(a) && circuits.contains_key(b) {
            let circuit_num_a = circuits[a];
            let circuit_num_b = circuits[b];

            // If both junction boxes are in the same circuit, nothing needs to be done.
            if circuit_num_a == circuit_num_b {
                continue;
            }

            // Otherwise, we need to combine the two circuits.
            // Change all boxes in circuit B to be in circuit A instead.
            for (_, value) in circuits.iter_mut() {
                if *value == circuit_num_b {
                    *value = circuit_num_a;
                }
            }

            // Decrement the total number of circuits (as we have combined two circuits here).
            num_circuits -= 1;
            continue;
        }

        // Determine the circuit for the current pair of junction boxes.
        let circuit_num = if circuits.contains_key(a) {
            circuits[a]
        } else if circuits.contains_key(b) {
            circuits[b]
        } else {
            // New circuit; we'll want to increment the total number of circuits.
            num_circuits += 1;
            current_circuit += 1;
            current_circuit
        };

        // Add the junction boxes to the determined circuit.
        circuits.entry(a).or_insert(circuit_num);
        circuits.entry(b).or_insert(circuit_num);

        // Check if all junction boxes are currently part of the same circuit.
        if circuits.len() == 1_000 && num_circuits == 1 {
            return a.x * b.x;
        }
    }

    unreachable!()
}

#[derive(PartialEq, Eq, Hash)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl JunctionBox {
    /// Calculates euclidean distance between two junction boxes (sqrt was removed for better performance).
    /// https://en.wikipedia.org/wiki/Euclidean_distance#Higher_dimensions
    fn distance(&self, other: &JunctionBox) -> f64 {
        (other.x - self.x).pow(2) as f64
            + (other.y - self.y).pow(2) as f64
            + (other.z - self.z).pow(2) as f64
    }
}

fn parse(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .map(|line| {
            let position: Vec<&str> = line.split(",").collect();
            JunctionBox {
                x: position[0].parse().unwrap(),
                y: position[1].parse().unwrap(),
                z: position[2].parse().unwrap(),
            }
        })
        .collect()
}
