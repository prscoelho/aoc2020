fn parse_input(input: &str) -> Vec<usize> {
    let mut res = Vec::new();

    for c in input.trim().chars() {
        res.push(c as usize - '0' as usize);
    }
    res
}

fn skip(links: &mut [usize], start: usize, n: usize) {
    let lowest = 1;
    let highest = links.len() - 1;

    let mut current = start;

    for _ in 0..n {
        let mut dst = current - 1;
        let picked = take(&links, current, 3);

        while dst < lowest || picked.contains(&dst) {
            dst = dst.saturating_sub(1);
            if dst < lowest {
                dst = highest;
            }
        }
        // current points to picked_next
        let picked_next = links[picked[2]];
        links[current] = picked_next;

        // dst points to picked[0]
        // and picked[2] points to dst_next
        let dst_next = links[dst];
        links[dst] = picked[0];
        links[picked[2]] = dst_next;

        // move cursor clockwise
        current = links[current];
    }
}

pub fn part1(input: &str) -> String {
    let nums = parse_input(input);
    let mut links = connect(&nums);
    let start = *nums.iter().next().unwrap();
    skip(&mut links, start, 100);
    let result_numbers = take(&links, 1, nums.len() - 1);

    let mut result = String::new();
    for n in result_numbers {
        result.push((n as u8 + '0' as u8) as char);
    }
    result
}

// Credit to rocurley for the idea to simulate a linked list with a vec of indices
// https://github.com/rocurley/aoc2020/blob/master/src/day23.rs
fn connect(nums: &[usize]) -> Vec<usize> {
    let mut links = vec![0; nums.len() + 1];

    for window in nums.windows(2) {
        links[window[0]] = window[1];
    }
    let first = *nums.iter().next().unwrap();
    let last = *nums.iter().rev().next().unwrap();

    links[last] = first;

    links
}

fn take(links: &[usize], from: usize, n: usize) -> Vec<usize> {
    let mut res = Vec::new();
    let mut current = from;
    for _ in 0..n {
        current = links[current];
        res.push(current);
    }
    res
}

pub fn part2(input: &str) -> usize {
    let mut nums = parse_input(input);
    let highest = *nums.iter().max().unwrap();
    for i in highest + 1..=1_000_000 {
        nums.push(i);
    }
    let mut links = connect(&nums);
    let start = *nums.iter().next().unwrap();

    skip(&mut links, start, 10_000_000);

    take(&links, 1, 2).iter().product()
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), "27956483");
    }

    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 18930983775);
    }
}
