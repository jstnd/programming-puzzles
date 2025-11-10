use std::ops::RangeInclusive;

use itertools::Itertools;

const VALID_RANGE: RangeInclusive<isize> = -1_000_000..=1_000_000;

pub fn part1(input: &str) -> String {
    let mut result = Complex::new(0, 0);
    let a = parse(input);

    for _ in 0..3 {
        result = multiply(result, result);
        result = divide(result, Complex::new(10, 10));
        result = add(result, a);
    }

    format!("[{},{}]", result.x, result.y)
}

pub fn part2(input: &str) -> usize {
    let a = parse(input);

    (0..=1_000)
        .step_by(10)
        .cartesian_product((0..=1_000).step_by(10))
        .filter(|(x, y)| engraved(a, *x, *y))
        .count()
}

pub fn part3(input: &str) -> usize {
    let a = parse(input);

    (0..=1_000)
        .cartesian_product(0..=1_000)
        .filter(|(x, y)| engraved(a, *x, *y))
        .count()
}

fn parse(input: &str) -> Complex {
    let (x, y) = input
        .trim_start_matches("A=[")
        .trim_end_matches("]")
        .split_once(",")
        .unwrap();

    Complex::new(x.parse().unwrap(), y.parse().unwrap())
}

fn engraved(a: Complex, x: isize, y: isize) -> bool {
    let point = add(a, Complex::new(x, y));
    let mut result = Complex::new(0, 0);

    (0..100).all(|_| {
        result = multiply(result, result);
        result = divide(result, Complex::new(100_000, 100_000));
        result = add(result, point);

        VALID_RANGE.contains(&result.x) && VALID_RANGE.contains(&result.y)
    })
}

fn add(lhs: Complex, rhs: Complex) -> Complex {
    let x = lhs.x + rhs.x;
    let y = lhs.y + rhs.y;
    Complex::new(x, y)
}

fn divide(lhs: Complex, rhs: Complex) -> Complex {
    let x = lhs.x / rhs.x;
    let y = lhs.y / rhs.y;
    Complex::new(x, y)
}

fn multiply(lhs: Complex, rhs: Complex) -> Complex {
    let x = lhs.x * rhs.x - lhs.y * rhs.y;
    let y = lhs.x * rhs.y + lhs.y * rhs.x;
    Complex::new(x, y)
}

#[derive(Clone, Copy)]
struct Complex {
    x: isize,
    y: isize,
}

impl Complex {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}
