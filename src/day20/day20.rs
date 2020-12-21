use std::collections::HashMap;

use aoc2020::{parse_grid, Grid, Vector2};

fn parse_input(input: &str) -> HashMap<usize, Grid> {
    let mut res = HashMap::new();
    for group in input.split("\n\n") {
        let (first, grid_str) = group.split_once(":\n").unwrap();
        let num = first
            .strip_prefix("Tile ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let grid = parse_grid(grid_str);
        res.insert(num, grid);
    }
    res
}

fn borders(grid: &Grid) -> Vec<Vec<char>> {
    let mut left = Vec::new();
    for j in 0..grid.rows {
        left.push(grid.get(&Vector2::new(0, j))); // left border
    }

    let mut top = Vec::new();
    for i in 0..grid.cols {
        top.push(grid.get(&Vector2::new(i, 0))); // top row border
    }

    let mut right = Vec::new();
    for j in 0..grid.cols {
        right.push(grid.get(&Vector2::new(grid.cols - 1, j))); // right
    }

    let mut bot = Vec::new();
    for i in 0..grid.cols {
        bot.push(grid.get(&Vector2::new(i, grid.rows - 1))); // bottom border
    }

    vec![left, top, right, bot]
}

fn flip(grid: &Grid) -> Grid {
    let mut result = grid.clone();
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let new_col = grid.cols - 1 - col;
            let value = grid.get(&Vector2::new(col, row));
            result.replace(&Vector2::new(new_col, row), value);
        }
    }
    result
}

fn rotate(grid: &Grid) -> Grid {
    let mut result = grid.clone();
    for row in 0..grid.rows {
        let new_col = grid.cols - 1 - row;
        for col in 0..grid.cols {
            let value = grid.get(&Vector2::new(col, row));
            result.replace(&Vector2::new(new_col, col), value);
        }
    }
    result
}

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    grid: Grid,
    connections: [Option<usize>; 4], // left, top, right, bottom connection indices
                                     // these sides match with result order of fn borders(grid)
}

impl Tile {
    fn new(id: usize, grid: Grid) -> Tile {
        Self {
            id,
            grid,
            connections: [None, None, None, None],
        }
    }
}

fn connect_all(mut all_grids: HashMap<usize, Grid>) -> HashMap<usize, Tile> {
    let mut mappings = HashMap::new();
    let key = *all_grids.keys().next().unwrap();
    let first_grid = all_grids.get(&key).unwrap();
    let first_tile = Tile::new(key, first_grid.clone());
    mappings.insert(key, first_tile);

    let mut unfinished = Vec::new();
    unfinished.push(key);

    while let Some(current_id) = unfinished.pop() {
        let current_tile = mappings.get(&current_id).unwrap().clone();
        let current_borders = borders(&current_tile.grid);
        let mut connections_matched = 0;
        'outer: for (&other_id, other_grid) in all_grids.iter_mut() {
            if other_id == current_id {
                continue;
            }
            if current_tile.connections.contains(&Some(other_id)) {
                continue;
            }
            if connections_matched == 4 {
                break;
            }

            'inner: for (side, connection) in current_tile.connections.iter().enumerate() {
                if connection.is_none() {
                    for _ in 0..2 {
                        // unflipped/ flipped once
                        for _ in 0..4 {
                            // rotated 0, 1, 2, 3 times.
                            let other_borders = borders(other_grid);
                            let other_side = (side + 2) % 4;
                            if current_borders[side] == other_borders[other_side] {
                                if let Some(other_tile) = mappings.get_mut(&other_id) {
                                    other_tile.connections[other_side] = Some(current_id);
                                } else {
                                    let mut new_tile = Tile::new(other_id, other_grid.clone());
                                    new_tile.connections[other_side] = Some(current_id);
                                    mappings.insert(other_id, new_tile);
                                    unfinished.push(other_id);
                                }

                                mappings.get_mut(&current_id).unwrap().connections[side] =
                                    Some(other_id);
                                connections_matched += 1;

                                continue 'outer; // current_tile connected to other_tile; continue to other tiles
                            }
                            if mappings.contains_key(&other_id) {
                                continue 'inner; // we cant rotate/flip this tile as it's already in the mapping,
                                                 // but other sides might still match
                            }
                            *other_grid = rotate(&other_grid);
                        }
                        *other_grid = flip(&other_grid);
                    }
                }
            }
        }
    }

    mappings
}

pub fn part1(input: &str) -> usize {
    let tiles = parse_input(input);

    let mappings = connect_all(tiles);

    mappings
        .iter()
        .filter(|(_, tile)| tile.connections.iter().filter(|o| o.is_some()).count() == 2) // find tiles with two connections
        .map(|(&id, _)| id)
        .product()
}

fn join_tiles(mappings: HashMap<usize, Tile>) -> Grid {
    let size = mappings.values().next().unwrap().grid.cols - 2;
    let side_amount = (mappings.len() as f32).sqrt() as i64;
    let colsrows = size * side_amount;
    let map_size = colsrows * colsrows;

    let data = vec!['.'; map_size as usize];
    let mut final_grid = Grid {
        data,
        cols: colsrows,
        rows: colsrows,
    };

    let mut left = mappings
        .values()
        .filter(|t| {
            t.connections[0].is_none()
                && t.connections[1].is_none()
                && t.connections[2].is_some()
                && t.connections[3].is_some()
        })
        .next()
        .unwrap(); // find the top left tile
    for tile_j in 0..side_amount {
        let mut current = left.clone();
        for tile_i in 0..side_amount {
            for y in 1..size + 1 {
                for x in 1..size + 1 {
                    let c = current.grid.get(&Vector2::new(x, y));
                    let big_position = Vector2::new(tile_i * size + x - 1, tile_j * size + y - 1);
                    final_grid.replace(&big_position, c);
                }
            }
            // move current to next right tile
            if let Some(connection) = &current.connections[2] {
                current = mappings[connection].clone();
            }
        }
        if let Some(connection) = &left.connections[3] {
            left = &mappings[connection];
        }
    }
    final_grid
}

fn find_monsters(mut grid: Grid) -> usize {
    let monsters: [Vector2; 15] = [
        Vector2::new(0, 1),
        Vector2::new(1, 2),
        Vector2::new(4, 2),
        Vector2::new(5, 1),
        Vector2::new(6, 1),
        Vector2::new(7, 2),
        Vector2::new(10, 2),
        Vector2::new(11, 1),
        Vector2::new(12, 1),
        Vector2::new(13, 2),
        Vector2::new(16, 2),
        Vector2::new(17, 1),
        Vector2::new(18, 1),
        Vector2::new(19, 1),
        Vector2::new(18, 0),
    ];

    for _ in 0..2 {
        for _ in 0..4 {
            let mut found_count = 0;
            for j in 0..grid.rows - 2 {
                for i in 0..grid.cols - 19 {
                    let current = Vector2::new(i, j);
                    if monsters
                        .iter()
                        .map(|v| *v + current)
                        .all(|v| grid.get(&v) == '#')
                    {
                        found_count += 1;
                    }
                }
            }
            if found_count > 0 {
                return found_count;
            }
            grid = rotate(&grid);
        }
        grid = flip(&grid);
    }
    0
}

fn count_tiles(grid: &Grid) -> usize {
    grid.data.iter().filter(|&&c| c == '#').count()
}

pub fn part2(input: &str) -> usize {
    let tiles = parse_input(input);

    let mappings = connect_all(tiles);
    let final_grid = join_tiles(mappings);

    let total_tiles = count_tiles(&final_grid);
    let monsters = find_monsters(final_grid);
    total_tiles - (monsters * 15)
}
