use std::collections::BTreeMap;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_vec(self) -> Vector2 {
        match self {
            Direction::Up => Vector2::new(0, -1),
            Direction::Down => Vector2::new(0, 1),
            Direction::Left => Vector2::new(-1, 0),
            Direction::Right => Vector2::new(1, 0),
        }
    }
}
pub enum Rotation {
    Left,
    Right,
    Flip,
}

pub fn rotate(from: Direction, rotation: Rotation) -> Direction {
    match rotation {
        Rotation::Left => match from {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        },
        Rotation::Right => match from {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        },
        Rotation::Flip => match from {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        },
    }
}

// Why BTreeMap<Vector2>, char> and not Vec<Vec<char>>:
// Allows index into data with negative values, while
// indexing into an array requires usizes that can't be negative
// So generating neighbour values allows us to ignore bound checking.
// If it's out of bounds, it's simply not in the grid.
// Furthermore, there might be scenarios where the grid actually
// contains negative values, with the origin in the middle of the grid
#[derive(Clone)]
pub struct Grid {
    pub data: BTreeMap<Vector2, char>,
    pub cols: i64,
    pub rows: i64,
}

pub fn parse_grid(input: &str) -> Grid {
    let mut data = BTreeMap::new();

    let mut rows = 0;
    let mut cols = 0;
    for line in input.lines() {
        cols = 0;
        for c in line.chars() {
            data.insert(Vector2::new(cols, rows), c);
            cols += 1;
        }
        rows += 1;
    }
    Grid { data, rows, cols }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Vector2 {
    pub y: i64, // sort by y before x
    pub x: i64,
}

impl Vector2 {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Mul<i64> for Vector2 {
    type Output = Self;

    fn mul(self, scalar: i64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl MulAssign<i64> for Vector2 {
    fn mul_assign(&mut self, scalar: i64) {
        *self = Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
#[derive(Debug)]
pub struct CharSet(u32);

impl CharSet {
    pub fn new() -> Self {
        CharSet(0)
    }

    pub fn insert(&mut self, c: char) -> bool {
        let idx = CharSet::decode(c);
        let present = (self.0 & idx) != 0;
        self.0 |= idx;
        !present
    }

    pub fn contains(&self, c: char) -> bool {
        let idx = CharSet::decode(c);
        (self.0 & idx) != 0
    }

    fn decode(c: char) -> u32 {
        match c {
            'a'..='z' => 1 << (c as u8 - 'a' as u8),
            'A'..='Z' => 1 << (c as u8 - 'A' as u8),
            _ => {
                panic!();
            }
        }
    }
}
#[cfg(test)]
mod charset_test {
    use super::CharSet;
    #[test]
    fn insert_contains() {
        let mut set = CharSet::new();
        assert_eq!(set.contains('a'), false);

        assert_eq!(set.insert('A'), true);
        assert_eq!(set.contains('a'), true);
        assert_eq!(set.insert('A'), false);
    }
}
