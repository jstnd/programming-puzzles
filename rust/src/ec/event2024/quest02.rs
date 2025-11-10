use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> i32 {
    // let (inscriptions, word_map) = parse_input(input);
    // let mut num_words = 0;

    // for (word_len, words) in word_map {
    //     for window in inscriptions.first().unwrap().windows(word_len) {
    //         if words.contains(window) {
    //             num_words += 1;
    //         }
    //     }
    // }

    // num_words
    0
}

pub fn part2(input: &str) -> i32 {
    let (words, inscription) = parse_input(input);
    let mut symbols = vec![0; inscription.len()];

    for i in 0..inscription.len() - 1 {
        let window = &inscription[i..];

        for word in &words {
            let mut reversed: Vec<u8> = word.to_vec();
            reversed.reverse();

            if window.starts_with(word) || window.starts_with(&reversed) {
                for j in 0..word.len() - 1 {
                    symbols[i + j] = 1;
                }
            }
        }
    }

    symbols.iter().sum()
}

pub fn part3(input: &str) -> i32 {
    0
}

fn parse_input(input: &str) -> (Vec<&[u8]>, &[u8]) {
    let (first, second) = input.split_once("\r\n\r\n").unwrap();
    let mut words = Vec::new();

    for word in first[6..].split(',') {
        words.push(word.as_bytes());
    }

    (words, second.as_bytes())
}
