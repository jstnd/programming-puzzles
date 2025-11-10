use std::collections::VecDeque;

use crate::util::num;

pub fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|parameters| {
            eni1(parameters.a, parameters.x, parameters.m)
                + eni1(parameters.b, parameters.y, parameters.m)
                + eni1(parameters.c, parameters.z, parameters.m)
        })
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|parameters| {
            eni2(parameters.a, parameters.x, parameters.m)
                + eni2(parameters.b, parameters.y, parameters.m)
                + eni2(parameters.c, parameters.z, parameters.m)
        })
        .max()
        .unwrap()
}

pub fn part3(input: &str) -> u32 {
    0
}

fn parse(input: &str) -> Vec<Parameters> {
    input
        .lines()
        .map(|line| {
            let parameters: Vec<usize> = line
                .split_whitespace()
                .map(|p| p.split_once("=").unwrap().1.parse().unwrap())
                .collect();

            Parameters {
                a: parameters[0],
                b: parameters[1],
                c: parameters[2],
                x: parameters[3],
                y: parameters[4],
                z: parameters[5],
                m: parameters[6],
            }
        })
        .collect()
}

fn eni1(n: usize, exp: usize, modulo: usize) -> usize {
    let mut score = 1;
    let mut remainders = Vec::new();

    for _ in 0..exp {
        score = (score * n) % modulo;
        remainders.push(score);
    }

    remainders.reverse();
    num::concat(&remainders)
}

fn eni2(n: usize, exp: usize, modulo: usize) -> usize {
    let mut score = 1;
    let mut remainders = VecDeque::new();

    for _ in 0..exp {
        score = (score * n) % modulo;

        if remainders.len() >= 5 {
            remainders.pop_front();
        }

        remainders.push_back(score);
    }

    let remainders: Vec<usize> = remainders.iter().rev().copied().collect();
    num::concat(&remainders)
}

struct Parameters {
    a: usize,
    b: usize,
    c: usize,
    x: usize,
    y: usize,
    z: usize,
    m: usize,
}
