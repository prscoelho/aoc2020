use aoc2020::{parse_grid, Grid, Vector2};

fn count_neighbours(grid: &Grid, i: i64, j: i64) -> usize {
    let mut count = 0;
    for p in i - 1..=i + 1 {
        for q in j - 1..=j + 1 {
            if p == i && q == j {
                continue;
            }
            if let Some(&c) = grid.data.get(&Vector2::new(p, q)) {
                if c == '#' {
                    count += 1
                }
            }
        }
    }
    count
}

fn count_eyesight_neighbours(grid: &Grid, i: i64, j: i64) -> usize {
    let mut count = 0;
    for p in -1..=1 {
        for q in -1..=1 {
            if p == 0 && q == 0 {
                continue;
            }
            let mut current = Vector2::new(i, j);
            let adj = Vector2::new(p, q);
            current += adj;
            while let Some(&c) = grid.data.get(&current) {
                if c == '#' {
                    count += 1;
                    break;
                }
                if c == 'L' {
                    break;
                }
                current += adj;
            }
        }
    }
    count
}

fn next(grid: &Grid) -> (Grid, bool) {
    let mut result = grid.clone();
    let mut changed = false;

    for (pos, &tile) in grid.data.iter() {
        if tile == '.' {
            continue;
        }
        let n = count_neighbours(grid, pos.x, pos.y);
        match (tile, n) {
            ('L', 0) => {
                result.data.insert(*pos, '#');
                changed = true;
            }
            ('#', 4..=9) => {
                result.data.insert(*pos, 'L');
                changed = true;
            }
            _ => {}
        }
    }
    (result, changed)
}

fn next_part2(grid: &Grid) -> (Grid, bool) {
    let mut result = grid.clone();
    let mut changed = false;

    for (pos, &tile) in grid.data.iter() {
        if tile == '.' {
            continue;
        }
        let n = count_eyesight_neighbours(grid, pos.x, pos.y);
        match (tile, n) {
            ('L', 0) => {
                result.data.insert(*pos, '#');
                changed = true;
            }
            ('#', 5..=9) => {
                result.data.insert(*pos, 'L');
                changed = true;
            }
            _ => {}
        }
    }
    (result, changed)
}

fn count_occupied(grid: &Grid) -> usize {
    let mut occupied = 0;
    for &tile in grid.data.values() {
        if tile == '#' {
            occupied += 1;
        }
    }
    occupied
}

pub fn part1(input: &str) -> usize {
    let mut grid = parse_grid(input);

    loop {
        let (next_grid, changed) = next(&grid);
        grid = next_grid;
        if !changed {
            break;
        }
    }
    count_occupied(&grid)
}

pub fn part2(input: &str) -> usize {
    let mut grid = parse_grid(input);

    loop {
        let (next_grid, changed) = next_part2(&grid);
        grid = next_grid;
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
