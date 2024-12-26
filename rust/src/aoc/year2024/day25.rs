pub fn part1(input: &str) -> u32 {
    let (locks, keys) = parse(input);
    let mut pairs = 0;

    for lock in locks.iter() {
        for key in keys.iter() {
            let is_fit = lock.iter().zip(key).all(|(h1, h2)| h1 + h2 < 8);
            if is_fit {
                pairs += 1;
            }
        }
    }

    pairs
}

pub fn part2(input: &str) -> u32 {
    0
}

fn parse(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    input.split("\n\n").for_each(|schematic| {
        let mut heights = vec![0u8; 5];
        let mut is_lock = None;

        schematic.lines().for_each(|line| {
            line.bytes().enumerate().for_each(|(i, b)| {
                if is_lock.is_none() {
                    is_lock = Some(b == b'#');
                }

                heights[i] += match b {
                    b'#' => 1,
                    _ => 0,
                };
            });
        });

        let is_lock = is_lock.unwrap();
        if is_lock {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    });

    (locks, keys)
}
