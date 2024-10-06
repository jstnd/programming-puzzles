use md5::Digest;

pub fn get_input() -> String {
    crate::util::input::get_input(2015, 4)
}

pub fn part1(input: &str) -> i32 {
    search(input.trim(), "00000")
}

pub fn part2(input: &str) -> i32 {
    search(input.trim(), "000000")
}

fn search(key: &str, prefix: &str) -> i32 {
    let mut number = 0;

    loop {
        let str = format!(
            "{:x}",
            md5::Md5::digest(format!("{key}{number}").as_bytes())
        );

        if str.starts_with(prefix) {
            break;
        }

        number += 1;
    }

    number
}
