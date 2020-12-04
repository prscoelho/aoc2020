struct Policy {
    range: (usize, usize),
    policy_char: char,
    password: String,
}
fn read_input(input: &str) -> Vec<Policy> {
    let mut result = Vec::new();
    for line in input.trim().lines() {
        let tokens = line.split_ascii_whitespace().collect::<Vec<_>>();
        let range_del = tokens[0].find("-").unwrap();

        let left = tokens[0][0..range_del].parse::<usize>().unwrap();
        let right = tokens[0][range_del + 1..].parse::<usize>().unwrap();

        let policy_char = tokens[1].chars().next().unwrap();

        let password = tokens[2];
        result.push(Policy {
            range: (left, right),
            policy_char,
            password: password.into(),
        })
    }
    result
}

fn count_char(input: &str, target: char) -> usize {
    let mut count = 0;
    for c in input.chars() {
        if c == target {
            count += 1;
        }
    }
    count
}

fn valid_policy1(pol: &Policy) -> bool {
    let count = count_char(&pol.password, pol.policy_char);
    count >= pol.range.0 && count <= pol.range.1
}

fn valid_policy2(pol: &Policy) -> bool {
    let left = pol.range.0 - 1;
    let right = pol.range.1 - 1;
    let chars: Vec<char> = pol.password.chars().collect();
    (chars[left] == pol.policy_char) ^ (chars[right] == pol.policy_char)
}

pub fn part1(input: &str) -> usize {
    let policies = read_input(input);
    policies.iter().filter(|p| valid_policy1(p)).count()
}

pub fn part2(input: &str) -> usize {
    let policies = read_input(input);
    policies.iter().filter(|p| valid_policy2(p)).count()
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 424);
    }

    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 747);
    }
}
