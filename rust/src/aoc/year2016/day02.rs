use crate::util::point::Point;

pub fn part1(input: &str) -> String {
    let keypad = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];

    solve(input, &keypad, Point::new(1, 1))
}

pub fn part2(input: &str) -> String {
    let keypad = vec![
        vec![' ', ' ', '1', ' ', ' '],
        vec![' ', '2', '3', '4', ' '],
        vec!['5', '6', '7', '8', '9'],
        vec![' ', 'A', 'B', 'C', ' '],
        vec![' ', ' ', 'D', ' ', ' '],
    ];

    solve(input, &keypad, Point::new(0, 2))
}

fn solve(input: &str, keypad: &[Vec<char>], start: Point) -> String {
    let mut code = String::new();
    let mut position = start;

    input.lines().for_each(|line| {
        line.bytes().for_each(|b| {
            let direction = Point::from(b);

            if is_valid(keypad, position + direction) {
                position += direction;
            }
        });

        code.push(keypad[position.y as usize][position.x as usize]);
    });

    code
}

fn is_valid(keypad: &[Vec<char>], point: Point) -> bool {
    if let Some(row) = keypad.get(point.y as usize) {
        return row.get(point.x as usize).is_some_and(|c| *c != ' ');
    }

    false
}
