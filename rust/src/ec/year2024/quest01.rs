pub fn part1(input: &str) -> i32 {
    input.bytes().map(get_potions).sum()
}

pub fn part2(input: &str) -> i32 {
    input
        .bytes()
        .array_chunks()
        .map(|[b1, b2]| {
            let mut potions = get_potions(b1) + get_potions(b2);

            if b1 != b'x' && b2 != b'x' {
                potions += 2;
            }

            potions
        })
        .sum()
}

pub fn part3(input: &str) -> i32 {
    input
        .bytes()
        .array_chunks()
        .map(|[b1, b2, b3]| {
            let mut potions = get_potions(b1) + get_potions(b2) + get_potions(b3);

            let num_creatures = (b1 != b'x') as u8 + (b2 != b'x') as u8 + (b3 != b'x') as u8;

            if num_creatures == 2 {
                potions += 2;
            } else if num_creatures == 3 {
                potions += 6;
            }

            potions
        })
        .sum()
}

fn get_potions(creature: u8) -> i32 {
    match creature {
        b'B' => 1,
        b'C' => 3,
        b'D' => 5,
        _ => 0,
    }
}
