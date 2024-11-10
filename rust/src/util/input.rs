use std::fs;

pub fn get_aoc_input(year: u16, day: u8) -> String {
    let path = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("input/aoc/{year}/{day:02}.txt"));

    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let mut file = fs::File::create(path.clone()).unwrap();

        retrieve_aoc_input(&mut file, year, day);
    }

    fs::read_to_string(path).unwrap()
}

fn retrieve_aoc_input(file: &mut fs::File, year: u16, day: u8) {
    let url = get_aoc_input_url(year, day);

    let client = reqwest::blocking::Client::builder()
        .user_agent("https://github.com/jstnd/programming-puzzles")
        .build()
        .unwrap();

    let mut response = client
        .get(url)
        .header(
            reqwest::header::COOKIE,
            format!("session={}", std::env::var("AOC_SESSION").unwrap()),
        )
        .send()
        .unwrap();

    response.copy_to(file).unwrap();
}

fn get_aoc_input_url(year: u16, day: u8) -> String {
    format!("https://adventofcode.com/{year}/day/{day}/input")
}
