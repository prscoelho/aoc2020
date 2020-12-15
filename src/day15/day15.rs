use std::collections::{HashMap, VecDeque};

fn read_input(input: &str) -> Vec<usize> {
    let mut nums = Vec::new();
    for num in input.split(",") {
        nums.push(num.parse().unwrap());
    }
    nums
}

fn memory_game(nums: Vec<usize>, nth: usize) -> usize {
    let mut seen: HashMap<usize, VecDeque<usize>> = HashMap::new();

    // starting numbers
    for (idx, &num) in nums.iter().enumerate() {
        let e = seen.entry(num).or_default();
        e.push_back(idx + 1); // turns start at 1
    }

    let mut last = *nums.last().unwrap();
    let turn = nums.len() + 1;
    for turn in turn..=nth {
        let e = seen.entry(last).or_default();
        let value = if e.len() == 2 { e[1] - e[0] } else { 0 };
        // add value to seen
        let e = seen.entry(value).or_default();
        e.push_back(turn);
        if e.len() > 2 {
            e.pop_front();
        }
        last = value;
    }
    last
}

pub fn part1(input: &str) -> usize {
    let nums = read_input(input);
    memory_game(nums, 2020)
}

pub fn part2(input: &str) -> usize {
    let nums = read_input(input);
    memory_game(nums, 30000000)
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 852);
    }

    // part 2 is too heavy to run in non release mode
    // skip automatic testing, to test this function
    // run `cargo test -- --ignored`
    #[test]
    #[ignore]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 6007666);
    }
}
