use std::ops::RangeInclusive;

use aoc2020::{parse_grid, Grid, Vector2};

// instead of iterating over every element in the grid, we search once for all seat indexes
fn create_seats(grid: &Grid) -> Vec<usize> {
    let mut seats = Vec::new();
    for (idx, &tile) in grid.data.iter().enumerate() {
        if tile == 'L' {
            seats.push(idx);
        }
    }
    seats
}

// instead of looking around each seat and wasting time on non seat neighbours,
// search once for which neighbours are seats and only iterate over those, this
// is specially better for part 2 where we might be wasting a lot of iterations
// searching for the same neighbours every step.
fn create_adjacent_neighbours(grid: &Grid, seats: &[usize]) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    for &seat in seats {
        let mut seat_neighbours = Vec::new();
        let pos = Vector2::new(seat as i64 % grid.cols, seat as i64 / grid.cols);

        for &adj in NEIGHBOURS.iter() {
            let neighbour = pos + adj;
            if grid.in_bounds(&neighbour) && grid.get(&neighbour) == 'L' {
                seat_neighbours.push(grid.index(&neighbour));
            }
        }
        result.push(seat_neighbours);
    }
    result
}

fn create_eyesight_neighbours(grid: &Grid, seats: &[usize]) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    for &seat in seats {
        let mut seat_neighbours = Vec::new();
        let pos = Vector2::new(seat as i64 % grid.cols, seat as i64 / grid.cols);

        for adj in NEIGHBOURS.iter() {
            let mut current = pos + *adj;
            while grid.in_bounds(&current) {
                let tile = grid.get(&current);
                if tile == 'L' {
                    seat_neighbours.push(grid.index(&current));
                    break;
                }
                current += *adj;
            }
        }
        result.push(seat_neighbours);
    }
    result
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

fn count_neighbours(grid: &Grid, neighbours: &[usize]) -> usize {
    neighbours
        .iter()
        .map(|&idx| grid.data[idx])
        .filter(|&c| c == '#')
        .count()
}

fn next(
    grid: &Grid,
    seats: &[usize],
    adjacencies: &Vec<Vec<usize>>,
    empty_range: RangeInclusive<usize>,
    result: &mut Grid,
) -> bool {
    let mut changed = false;

    for (idx, &seat) in seats.iter().enumerate() {
        let n = count_neighbours(grid, &adjacencies[idx]);
        let tile = grid.data[seat];
        match (tile, n) {
            ('L', 0) => {
                result.data[seat] = '#';
                changed = true;
            }
            ('#', n) if empty_range.contains(&n) => {
                result.data[seat] = 'L';
                changed = true;
            }
            _ => {
                result.data[seat] = tile;
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

    let seats = create_seats(&grid);
    let adj = create_adjacent_neighbours(&grid, &seats);
    loop {
        let changed = next(&grid, &seats, &adj, 4..=9, &mut next_grid);
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
    let seats = create_seats(&grid);
    let adj = create_eyesight_neighbours(&grid, &seats);

    loop {
        let changed = next(&grid, &seats, &adj, 5..=9, &mut next_grid);
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
