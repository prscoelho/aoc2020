use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

fn read_input(input: &str) -> Vec<u32> {
    let mut nums = Vec::new();
    for num in input.split(",") {
        nums.push(num.parse().unwrap());
    }
    nums
}

fn memory_game(nums: Vec<u32>, nth: u32) -> u32 {
    let mut seen: HashMap<u32, u32> = HashMap::new();

    // starting numbers
    for (idx, &num) in nums.iter().enumerate() {
        let e = seen.entry(num).or_default();
        *e = idx as u32 + 1; // turns start at 1
    }

    let mut last = *nums.last().unwrap();
    let turn = nums.len() as u32 + 1;
    for turn in turn..=nth {
        // because last index is always turn -1,
        // we can avoid storing two values inside hashmap
        let value = match seen.entry(last) {
            Entry::Occupied(mut o) => {
                let e_mut = o.get_mut();
                let value = turn - 1 - *e_mut;
                *e_mut = turn - 1;
                value
            }
            Entry::Vacant(v) => {
                v.insert(turn - 1);
                0
            }
        };
        last = value;
    }
    last
}

pub fn part1(input: &str) -> u32 {
    let nums = read_input(input);
    memory_game(nums, 2020)
}

pub fn part2(input: &str) -> u32 {
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
