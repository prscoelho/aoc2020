use std::{collections::HashSet, ops::RangeInclusive};

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

fn get_bounds(
    universe: &Universe,
) -> (
    RangeInclusive<i32>,
    RangeInclusive<i32>,
    RangeInclusive<i32>,
    RangeInclusive<i32>,
) {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;

    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    let mut min_z = i32::MAX;
    let mut max_z = i32::MIN;

    let mut min_w = i32::MAX;
    let mut max_w = i32::MIN;

    for (x, y, z, w) in universe.iter() {
        min_x = i32::min(min_x, x - 1);
        max_x = i32::max(max_x, x + 1);

        min_y = i32::min(min_y, y - 1);
        max_y = i32::max(max_y, y + 1);

        min_z = i32::min(min_z, z - 1);
        max_z = i32::max(max_z, z + 1);

        min_w = i32::min(min_w, w - 1);
        max_w = i32::max(max_w, w + 1);
    }

    (
        RangeInclusive::new(min_x, max_x),
        RangeInclusive::new(min_y, max_y),
        RangeInclusive::new(min_z, max_z),
        RangeInclusive::new(min_w, max_w),
    )
}

fn neighbours(universe: &Universe, position: (i32, i32, i32, i32)) -> usize {
    let mut count = 0;
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

                    if universe.contains(&neighbour) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn step4d(universe: &Universe) -> Universe {
    let mut next_universe = Universe::new();
    let (x_bounds, y_bounds, z_bounds, w_bounds) = get_bounds(&universe);
    for x in x_bounds {
        for y in y_bounds.clone() {
            for z in z_bounds.clone() {
                for w in w_bounds.clone() {
                    let pos = (x, y, z, w);
                    let n = neighbours(&universe, pos);
                    let state = universe.contains(&pos);
                    match (state, n) {
                        (true, 2..=3) => {
                            next_universe.insert(pos);
                        }
                        (false, 3) => {
                            next_universe.insert(pos);
                        }
                        _ => {}
                    };
                }
            }
        }
    }
    next_universe
}

fn step3d(universe: &Universe) -> Universe {
    let mut next_universe = Universe::new();
    let (x_bounds, y_bounds, z_bounds, _) = get_bounds(&universe);
    for x in x_bounds {
        for y in y_bounds.clone() {
            for z in z_bounds.clone() {
                let pos = (x, y, z, 0);
                let n = neighbours(&universe, pos);
                let state = universe.contains(&pos);
                match (state, n) {
                    (true, 2..=3) => {
                        next_universe.insert(pos);
                    }
                    (false, 3) => {
                        next_universe.insert(pos);
                    }
                    _ => {}
                };
            }
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

    // too heavy in debug mode, test manually with `cargo test day17 -- --ignored`
    #[ignore]
    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 2532);
    }
}
