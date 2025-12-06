use itertools::{Itertools, Position};

use crate::util::array;

pub fn part1(input: &str) -> usize {
    parse1(input).iter().map(|problem| problem.execute()).sum()
}

pub fn part2(input: &str) -> usize {
    parse2(input).iter().map(|problem| problem.execute()).sum()
}

enum Operation {
    Add,
    Multiply,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => unreachable!(),
        }
    }
}

struct Problem {
    operands: Vec<usize>,
    operation: Operation,
}

impl Problem {
    fn execute(&self) -> usize {
        match self.operation {
            Operation::Add => self.operands.iter().sum(),
            Operation::Multiply => self.operands.iter().product(),
        }
    }
}

fn parse1(input: &str) -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();

    input.lines().with_position().for_each(|(position, line)| {
        // Each number is separated by a varying length of whitespace, so split by whitespace.
        line.split_whitespace()
            .enumerate()
            .for_each(|(i, element)| {
                if position == Position::First {
                    // Initialize the problem in our list of problems.
                    problems.push(Problem {
                        operands: Vec::new(),
                        operation: Operation::Add,
                    });
                } else if position == Position::Last {
                    // The last line in the input is the list of operations.
                    // Here, we update the problem's operation with the corresponding operation from the input.
                    problems[i].operation = Operation::from(element);
                    return;
                }

                // Add the current number to the corresponding problem's list of operands.
                problems[i].operands.push(element.parse().unwrap());
            });
    });

    problems
}

fn parse2(input: &str) -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();
    let mut numbers: Vec<Vec<char>> = Vec::new();

    input.lines().with_position().for_each(|(position, line)| {
        if position == Position::Last {
            // The last line in the input is the list of operations.
            // Here, we iterate through each operation and add a new problem to the list of problems.
            line.split_whitespace().for_each(|element| {
                problems.push(Problem {
                    operands: Vec::new(),
                    operation: Operation::from(element),
                });
            });
        } else {
            // Every other line in the input consists of digits and whitespace.
            // Add all characters inside these lines to a 2D vector for later processing.
            numbers.push(line.chars().collect());
        }
    });

    // Transpose the matrix of digits so that each row in the matrix corresponds to a number in a problem.
    //
    // ['1', '2', '3', ' ', '3', '2', '8', ' ', ' ', '5', '1', ' ', '6', '4', ' ']      ['1', ' ', ' ']
    // [' ', '4', '5', ' ', '6', '4', ' ', ' ', '3', '8', '7', ' ', '2', '3', ' ']  =>  ['2', '4', ' ']
    // [' ', ' ', '6', ' ', '9', '8', ' ', ' ', '2', '1', '5', ' ', '3', '1', '4']      ['3', '5', '6']
    //                                                                                  [' ', ' ', ' ']
    //                                                                                  ['3', '6', '9']
    //                                                                                  ['2', '4', '8']
    //                                                                                  ['8', ' ', ' ']
    //                                                                                  [' ', ' ', ' ']
    //                                                                                  [...] (truncated for brevity)
    let numbers: Vec<Vec<char>> = array::transpose(&numbers);

    // Used to track the index of where we currently are in the list of problems.
    let mut current_problem = 0;

    numbers.iter().for_each(|number| {
        // Convert the list of characters into a string.
        let number: String = number.iter().collect();

        if number.trim().is_empty() {
            // If the current row of characters is empty, that means we finished recording the
            // operands of the previous problem and we can move onto the next problem.
            current_problem += 1;
        } else {
            // Otherwise, we're still processing the operands for the current problem, so we should parse the
            // current characters into a number and include it in the list of operands of the current problem.
            problems[current_problem]
                .operands
                .push(number.trim().parse().unwrap());
        }
    });

    problems
}
