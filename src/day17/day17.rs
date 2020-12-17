use std::collections::{HashMap, HashSet};

type Universe = HashSet<(i32, i32, i32, i32)>;

fn parse_plane(input: &str) -> Universe {
    let mut universe = Universe::new();

    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for c in line.chars() {
            match c {
                '#' => {
                    universe.insert((x, y, 0, 0));
                }
                '.' => {}
                _ => {
                    unreachable!()
                }
            };

            x += 1;
        }
        y += 1;
    }
    universe
}

fn step4d(universe: &Universe) -> Universe {
    let mut neighbour_count = HashMap::new();
    for position in universe.iter() {
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        if x == 0 && y == 0 && z == 0 && w == 0 {
                            continue;
                        }

                        let neighbour = (
                            x + position.0,
                            y + position.1,
                            z + position.2,
                            w + position.3,
                        );
                        *neighbour_count.entry(neighbour).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    let mut next_universe = Universe::new();
    for (position, n) in neighbour_count.iter() {
        let activated = universe.contains(position);
        match (activated, n) {
            (true, 2..=3) => {
                next_universe.insert(*position);
            }
            (false, 3) => {
                next_universe.insert(*position);
            }
            _ => {}
        }
    }
    next_universe
}

fn step3d(universe: &Universe) -> Universe {
    let mut neighbour_count = HashMap::new();
    for position in universe.iter() {
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }

                    let neighbour = (x + position.0, y + position.1, z + position.2, 0);
                    *neighbour_count.entry(neighbour).or_insert(0) += 1;
                }
            }
        }
    }

    let mut next_universe = Universe::new();
    for (position, n) in neighbour_count.iter() {
        let activated = universe.contains(position);
        match (activated, n) {
            (true, 2..=3) => {
                next_universe.insert(*position);
            }
            (false, 3) => {
                next_universe.insert(*position);
            }
            _ => {}
        }
    }
    next_universe
}

pub fn part1(input: &str) -> usize {
    let mut universe = parse_plane(input);

    for _ in 0..6 {
        universe = step3d(&universe);
    }

    universe.len()
}

pub fn part2(input: &str) -> usize {
    let mut universe = parse_plane(input);

    for _ in 0..6 {
        universe = step4d(&universe);
    }

    universe.len()
}

mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 257);
    }

    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 2532);
    }
}
