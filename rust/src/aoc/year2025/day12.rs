pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| line.contains('x'))
        .filter(|line| {
            let (dimensions, shapes) = line.split_once(": ").unwrap();
            let (width, length) = dimensions.split_once("x").unwrap();
            let width: usize = width.parse().unwrap();
            let length: usize = length.parse().unwrap();
            let shapes: usize = shapes
                .split_whitespace()
                .map(|shape| shape.parse::<usize>().unwrap())
                .sum();

            // Naive solution; just essentially look for if there's a 3x3 space for each present.
            (width * length) >= 9 * shapes
        })
        .count()
}

pub fn part2(_: &str) -> usize {
    0
}
