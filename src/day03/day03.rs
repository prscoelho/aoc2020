use aoc2020::Vector2;
use aoc2020::{parse_gridmap, GridMap};

fn trees_in_slope(grid: &GridMap, slope: Vector2) -> u64 {
    let mut pos = Vector2::new(0, 0);

    let mut tree_count = 0;
    while pos.y < grid.rows {
        pos += slope;
        if pos.x >= grid.cols {
            pos.x = pos.x % grid.cols;
        }
        if let Some(&c) = grid.data.get(&pos) {
            if c == '#' {
                tree_count += 1;
            }
        }
    }
    tree_count
}

pub fn part1(input: &str) -> u64 {
    let grid = parse_gridmap(input);
    trees_in_slope(&grid, Vector2::new(3, 1))
}

pub fn part2(input: &str) -> u64 {
    let grid = parse_gridmap(input);
    let slopes = [
        Vector2::new(1, 1),
        Vector2::new(3, 1),
        Vector2::new(5, 1),
        Vector2::new(7, 1),
        Vector2::new(1, 2),
    ];
    slopes
        .iter()
        .map(|&slope| trees_in_slope(&grid, slope))
        .product()
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 254);
    }
    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 1666768320);
    }
}
