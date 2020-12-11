use aoc2020::{parse_grid, Grid, Vector2};

fn count_neighbours(grid: &Grid, pos: Vector2) -> usize {
    let mut count = 0;
    for &adj in NEIGHBOURS.iter() {
        let neighbour = pos + adj;
        if grid.in_bounds(&neighbour) && grid.get(&neighbour) == '#' {
            count += 1;
        }
    }
    count
}

const NEIGHBOURS: [Vector2; 8] = [
    Vector2::new(-1, -1),
    Vector2::new(-1, 0),
    Vector2::new(-1, 1),
    Vector2::new(0, -1),
    Vector2::new(0, 1),
    Vector2::new(1, -1),
    Vector2::new(1, 0),
    Vector2::new(1, 1),
];

fn count_eyesight_neighbours(grid: &Grid, pos: Vector2) -> usize {
    let mut count = 0;
    for adj in NEIGHBOURS.iter() {
        let mut current = pos + *adj;
        while grid.in_bounds(&current) {
            let tile = grid.get(&current);
            if tile == '#' {
                count += 1;
                break;
            }
            if tile == 'L' {
                break;
            }
            current += *adj;
        }
    }
    count
}

fn next(grid: &Grid, result: &mut Grid) -> bool {
    let mut changed = false;

    for y in 0..grid.rows {
        for x in 0..grid.cols {
            let pos = Vector2::new(x, y);
            let tile = grid.get(&pos);
            if tile == '.' {
                continue;
            }
            let n = count_neighbours(grid, pos);
            match (tile, n) {
                ('L', 0) => {
                    result.replace(&pos, '#');
                    changed = true;
                }
                ('#', 4..=9) => {
                    result.replace(&pos, 'L');
                    changed = true;
                }
                _ => {
                    result.replace(&pos, tile);
                }
            }
        }
    }
    changed
}

fn next_part2(grid: &Grid, result: &mut Grid) -> bool {
    let mut changed = false;

    for y in 0..grid.rows {
        for x in 0..grid.cols {
            let pos = Vector2::new(x, y);
            let tile = grid.get(&pos);
            if tile == '.' {
                continue;
            }
            let n = count_eyesight_neighbours(grid, pos);
            match (tile, n) {
                ('L', 0) => {
                    result.replace(&pos, '#');
                    changed = true;
                }
                ('#', 5..=9) => {
                    result.replace(&pos, 'L');
                    changed = true;
                }
                _ => {
                    result.replace(&pos, tile);
                }
            }
        }
    }
    changed
}

fn count_occupied(grid: &Grid) -> usize {
    let mut occupied = 0;
    for &tile in grid.data.iter() {
        if tile == '#' {
            occupied += 1;
        }
    }
    occupied
}

pub fn part1(input: &str) -> usize {
    let mut grid = parse_grid(input);
    let mut next_grid = grid.clone();

    loop {
        let changed = next(&grid, &mut next_grid);
        std::mem::swap(&mut grid, &mut next_grid);
        if !changed {
            break;
        }
    }
    count_occupied(&grid)
}

pub fn part2(input: &str) -> usize {
    let mut grid = parse_grid(input);
    let mut next_grid = grid.clone();

    loop {
        let changed = next_part2(&grid, &mut next_grid);
        std::mem::swap(&mut grid, &mut next_grid);
        if !changed {
            break;
        }
    }
    count_occupied(&grid)
}
#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 2438);
    }

    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 2174);
    }
}
