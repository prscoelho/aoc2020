use std::collections::{HashMap, HashSet};

use aoc2020::Vector2;

#[derive(Debug, Clone, Copy)]
enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

fn parse_direction(text: &str) -> (Direction, &str) {
    if text.starts_with("e") {
        (Direction::East, &text[1..])
    } else if text.starts_with("se") {
        (Direction::Southeast, &text[2..])
    } else if text.starts_with("sw") {
        (Direction::Southwest, &text[2..])
    } else if text.starts_with("w") {
        (Direction::West, &text[1..])
    } else if text.starts_with("nw") {
        (Direction::Northwest, &text[2..])
    } else if text.starts_with("ne") {
        (Direction::Northeast, &text[2..])
    } else {
        panic!("Unexpected direction: {}", text);
    }
}

fn hexagon_move(position: &Vector2, dir: Direction) -> Vector2 {
    match dir {
        Direction::East => Vector2::new(position.x + 1, position.y),
        Direction::West => Vector2::new(position.x - 1, position.y),
        // southwest and northeast don't change x axis
        Direction::Southwest => Vector2::new(position.x, position.y - 1),
        Direction::Northeast => Vector2::new(position.x, position.y + 1),
        // while northwest and southeast do
        Direction::Northwest => Vector2::new(position.x - 1, position.y + 1),
        Direction::Southeast => Vector2::new(position.x + 1, position.y - 1),
    }
}

fn read_directons(input: &str) -> Vec<Vec<Direction>> {
    let mut result = Vec::new();
    for mut line in input.lines() {
        let mut line_directions = Vec::new();
        while !line.is_empty() {
            let (dir, rest) = parse_direction(line);
            line_directions.push(dir);
            line = rest;
        }
        result.push(line_directions)
    }
    result
}

fn flip_from_directions(hexagon_directions: Vec<Vec<Direction>>) -> HashSet<Vector2> {
    let mut flipped = HashSet::new();

    for directions in hexagon_directions {
        let mut position = Vector2::new(0, 0);
        for dir in directions {
            position = hexagon_move(&position, dir);
        }
        if flipped.contains(&position) {
            flipped.remove(&position);
        } else {
            flipped.insert(position);
        }
    }
    flipped
}

pub fn part1(input: &str) -> usize {
    let hexagon_directions = read_directons(input);
    let flipped = flip_from_directions(hexagon_directions);
    flipped.len()
}

const ALL_DIRECTIONS: [Direction; 6] = [
    Direction::East,
    Direction::West,
    Direction::Northeast,
    Direction::Northwest,
    Direction::Southeast,
    Direction::Southwest,
];

fn conway(flipped: HashSet<Vector2>) -> HashSet<Vector2> {
    let mut neighbours: HashMap<Vector2, usize> = HashMap::new();

    for pos in flipped.iter() {
        for &dir in ALL_DIRECTIONS.iter() {
            let adj_pos = hexagon_move(pos, dir);
            *neighbours.entry(adj_pos).or_default() += 1;
        }
    }

    let mut result = HashSet::new();

    for (pos, neigh_count) in neighbours {
        let black = flipped.contains(&pos);
        match (black, neigh_count) {
            (true, 1..=2) => {
                result.insert(pos);
            }
            (false, 2) => {
                result.insert(pos);
            }
            _ => {}
        }
    }

    result
}

pub fn part2(input: &str) -> usize {
    let hexagon_directions = read_directons(input);
    let mut flipped = flip_from_directions(hexagon_directions);

    for _ in 0..100 {
        flipped = conway(flipped);
    }
    flipped.len()
}

mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 400);
    }
    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 3768);
    }
}
