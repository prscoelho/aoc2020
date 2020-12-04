use std::collections::BTreeMap;

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

use std::ops::{Add, AddAssign};

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
