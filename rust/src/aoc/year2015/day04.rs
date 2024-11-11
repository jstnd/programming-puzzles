use std::fmt::Write;

use md5::Digest;

pub fn part1(input: &str) -> i32 {
    search(input.trim(), 1)
}

pub fn part2(input: &str) -> i32 {
    search(input.trim(), 2)
}

fn search(key: &str, part: u8) -> i32 {
    let mut input = key.to_string();
    let len = input.len();
    let mut hasher = md5::Md5::new();

    for i in 0.. {
        input.truncate(len);
        write!(&mut input, "{}", i).unwrap();
        hasher.update(&input);

        let hash = hasher.finalize_reset();

        if part == 1 {
            if hash[0] == 0 && hash[1] == 0 && hash[2] >> 4 == 0 {
                return i;
            }
        } else if hash.starts_with(&[0, 0, 0]) {
            return i;
        }
    }

    unreachable!()
}
