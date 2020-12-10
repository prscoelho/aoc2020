fn read_chain(input: &str) -> Vec<u64> {
    let mut nums = Vec::new();
    for line in input.lines() {
        nums.push(line.parse().unwrap());
    }
    nums.push(0);
    nums.sort();
    nums.push(nums.last().unwrap() + 3);
    nums
}

pub fn part1(input: &str) -> u64 {
    let nums = read_chain(input);

    let mut diff_1 = 0;
    let mut diff_3 = 0;

    for slice in nums.windows(2) {
        if slice[1] - slice[0] == 1 {
            diff_1 += 1;
        } else if slice[1] - slice[0] == 3 {
            diff_3 += 1;
        }
    }
    return diff_1 * diff_3;
}

fn count_possibilities(nums: &[u64]) -> u64 {
    let n = nums.len();
    let mut ways = vec![0; n];
    ways[0] = 1;

    for i in 1..n {
        let value_at_i = nums[i];
        for (j, _) in nums[..i]
            .iter()
            .rev()
            .take_while(|&v| v + 3 >= value_at_i)
            .enumerate()
        {
            ways[i] += ways[i - (j + 1)];
        }
    }
    ways[n - 1]
}

pub fn part2(input: &str) -> u64 {
    let nums = read_chain(input);
    count_possibilities(&nums)
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 1980);
    }

    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 4628074479616);
    }
}
