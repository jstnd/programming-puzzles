use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self::new(0, 0)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl From<u8> for Point {
    fn from(value: u8) -> Self {
        match value {
            b'^' => Point::new(0, 1),
            b'v' => Point::new(0, -1),
            b'>' => Point::new(1, 0),
            b'<' => Point::new(-1, 0),
            _ => unreachable!(),
        }
    }
}
