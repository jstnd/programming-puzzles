use rustc_hash::FxHashSet;

use crate::util::point::Point;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

pub fn part1(input: &str) -> u32 {
    // https://github.com/rust-lang/rust-clippy/issues/13185
    #[allow(clippy::manual_inspect)]
    parse(input)
        .iter_mut()
        .map(|robot| {
            robot.position.x = (robot.position.x + 100 * (robot.velocity.x + WIDTH)) % WIDTH;
            robot.position.y = (robot.position.y + 100 * (robot.velocity.y + HEIGHT)) % HEIGHT;
            robot
        })
        .fold([0; 4], |mut quadrants, robot| {
            let Point { x, y } = robot.position;

            match (x, y) {
                (x, y) if x < WIDTH / 2 && y < HEIGHT / 2 => quadrants[0] += 1,
                (x, y) if x > WIDTH / 2 && y < HEIGHT / 2 => quadrants[1] += 1,
                (x, y) if x < WIDTH / 2 && y > HEIGHT / 2 => quadrants[2] += 1,
                (x, y) if x > WIDTH / 2 && y > HEIGHT / 2 => quadrants[3] += 1,
                _ => {}
            }

            quadrants
        })
        .iter()
        .product()
}

pub fn part2(input: &str) -> u32 {
    let mut robots = parse(input);
    let mut visited = FxHashSet::default();

    for i in 1..=WIDTH * HEIGHT {
        for robot in robots.iter_mut() {
            robot.position.x = (robot.position.x + robot.velocity.x + WIDTH) % WIDTH;
            robot.position.y = (robot.position.y + robot.velocity.y + HEIGHT) % HEIGHT;
            visited.insert(robot.position);
        }

        if visited.len() == robots.len() {
            return i as u32;
        }

        visited.clear();
    }

    0
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (position, velocity) = line.split_once(' ').unwrap();
            let (px, py) = position.trim_start_matches("p=").split_once(',').unwrap();
            let (vx, vy) = velocity.trim_start_matches("v=").split_once(',').unwrap();

            Robot {
                position: Point::new(px.parse().unwrap(), py.parse().unwrap()),
                velocity: Point::new(vx.parse().unwrap(), vy.parse().unwrap()),
            }
        })
        .collect()
}

struct Robot {
    position: Point,
    velocity: Point,
}
