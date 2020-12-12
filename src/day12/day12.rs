use aoc2020::Vector2;

fn read_actions(input: &str) -> Vec<(char, i64)> {
    let mut actions = Vec::new();
    for line in input.lines() {
        let action = line.chars().next().unwrap();
        let num = line[1..].parse().unwrap();
        actions.push((action, num));
    }
    actions
}

fn manhattan(pos: Vector2) -> i64 {
    pos.x.abs() + pos.y.abs()
}

// integer cos and sin for angles that yield -1, 0 or 1
fn cos(angle: i64) -> i64 {
    let rad = (angle / 90) as f64 * std::f64::consts::FRAC_PI_2;
    rad.cos().round() as i64
}

fn sin(angle: i64) -> i64 {
    let rad = (angle / 90) as f64 * std::f64::consts::FRAC_PI_2;
    rad.sin().round() as i64
}

fn rotate(vector: Vector2, angle: i64) -> Vector2 {
    let x = vector.x * cos(angle) - vector.y * sin(angle);
    let y = vector.y * cos(angle) + vector.x * sin(angle);

    Vector2::new(x, y)
}

pub fn part1(input: &str) -> i64 {
    let actions = read_actions(input);
    let mut position = Vector2::new(0, 0);
    let mut direction = Vector2::new(1, 0);
    for (action, value) in actions {
        match action {
            'N' => {
                position += Vector2::new(0, 1) * value;
            }
            'S' => {
                position += Vector2::new(0, -1) * value;
            }
            'E' => {
                position += Vector2::new(1, 0) * value;
            }
            'W' => {
                position += Vector2::new(-1, 0) * value;
            }
            'L' => {
                direction = rotate(direction, value);
            }
            'R' => {
                direction = rotate(direction, -value);
            }
            'F' => {
                position += direction * value;
            }
            _ => {
                unreachable!();
            }
        }
    }
    manhattan(position)
}

pub fn part2(input: &str) -> i64 {
    let actions = read_actions(input);
    let mut position = Vector2::new(0, 0);
    let mut waypoint = Vector2::new(10, 1);
    for (action, value) in actions {
        match action {
            'N' => {
                waypoint += Vector2::new(0, 1) * value;
            }
            'S' => {
                waypoint += Vector2::new(0, -1) * value;
            }
            'E' => {
                waypoint += Vector2::new(1, 0) * value;
            }
            'W' => {
                waypoint += Vector2::new(-1, 0) * value;
            }
            'L' => {
                waypoint = rotate(waypoint, value);
            }
            'R' => {
                waypoint = rotate(waypoint, -value);
            }
            'F' => {
                position += waypoint * value;
            }
            _ => {
                unreachable!();
            }
        }
    }
    manhattan(position)
}

mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 904);
    }

    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 18747);
    }
}
