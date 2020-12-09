fn read_input(input: &str) -> Vec<u64> {
    let mut nums = Vec::new();
    for line in input.lines() {
        nums.push(line.parse().unwrap());
    }
    nums
}

fn has_sum_pair(list: &[u64], goal: u64) -> bool {
    for (idx, a) in list.iter().enumerate() {
        for b in list.iter().skip(idx + 1) {
            if a + b == goal {
                return true;
            }
        }
    }
    false
}

pub fn part1(input: &str) -> u64 {
    let data = read_input(input);
    let preamble = 25;

    for (idx, &val) in data.iter().enumerate().skip(preamble + 1) {
        if !has_sum_pair(&data[idx - 25..idx], val) {
            return val;
        }
    }
    unreachable!();
}

fn list_of_sums(nums: &[u64]) -> Vec<u64> {
    let mut sums = vec![0];
    let mut total = 0;
    for num in nums {
        total += num;
        sums.push(total);
    }
    sums
}

pub fn part2(input: &str) -> u64 {
    let data = read_input(input);
    let target = part1(input);

    let sums = list_of_sums(&data);
    for (idx_a, a) in sums.iter().enumerate() {
        for (idx_b, b) in sums.iter().enumerate().skip(idx_a + 1) {
            if b - a == target {
                let min = data[idx_a..idx_b].iter().min().unwrap();
                let max = data[idx_a..idx_b].iter().max().unwrap();

                return min + max;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 88311122);
    }
    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 13549369);
    }
}
