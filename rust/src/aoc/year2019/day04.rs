use itertools::Itertools;

use crate::util::num;

pub fn part1(input: &str) -> usize {
    let (begin, end) = parse(input);
    let mut passwords = 0;

    for password in begin..=end {
        let mut adjacent = false;
        let increasing = num::digits_rev(password).tuple_windows().all(|(d2, d1)| {
            // Check if the current two digits match;
            // if so, this password meets the adjacent criteria.
            if d1 == d2 {
                adjacent = true;
            }

            // Check if the next digit is the same or increasing.
            d2 >= d1
        });

        // Check that the password met all criteria.
        if adjacent && increasing {
            passwords += 1;
        }
    }

    passwords
}

pub fn part2(input: &str) -> usize {
    let (begin, end) = parse(input);
    let mut passwords = 0;

    for password in begin..=end {
        let mut adjacent = false;
        let mut adjacent_len = 1;
        let increasing = num::digits_rev(password).tuple_windows().all(|(d2, d1)| {
            if d1 == d2 {
                // Increment the current adjacent group's length.
                adjacent_len += 1;
            } else {
                // Check if the previous adjacent group had a length of 2;
                // if so, this password meets the adjacent criteria.
                if adjacent_len == 2 {
                    adjacent = true;
                }

                // Reset the tracked adjacent group length back to 1.
                adjacent_len = 1;
            }

            // Check if the next digit is the same or increasing.
            d2 >= d1
        });

        // Check that the password met all criteria.
        // We need to check both the `adjacent` and `adjacent_len` variables,
        // in case the adjacent group that met the criteria was at the end of the password digits.
        if (adjacent || adjacent_len == 2) && increasing {
            passwords += 1;
        }
    }

    passwords
}

fn parse(input: &str) -> (usize, usize) {
    let (begin, end) = input.trim().split_once('-').unwrap();
    (begin.parse().unwrap(), end.parse().unwrap())
}
