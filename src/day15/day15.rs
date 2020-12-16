use std::collections::HashMap;

fn read_input(input: &str) -> Vec<u32> {
    let mut nums = Vec::new();
    for num in input.trim().split(",") {
        nums.push(num.parse().unwrap());
    }
    nums
}

fn memory_game(mut nums: Vec<u32>, nth: u32) -> u32 {
    let mut seen: HashMap<u32, u32> = HashMap::new();

    let turn = nums.len() as u32 + 1;
    // we dont want the last number in the seen map yet as we only add the number
    // from previous turn on the turn after it. This allows us to only keep a single
    // number inside the HashMap instead of the last two indexes. Whenever we have to
    // look up an index, we know last one is for sure on turn -1, and are only required
    // to look up the other index.
    let mut last = nums.pop().unwrap();
    // starting numbers
    for (idx, &num) in nums.iter().enumerate() {
        seen.insert(num, idx as u32 + 1); // turns start at 1
    }

    for turn in turn..=nth {
        let seen_at_option = seen.insert(last, turn - 1);
        last = match seen_at_option {
            Some(val) => turn - 1 - val,
            None => 0,
        };
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

    // part 2 is too heavy to run in debug mode
    // skip automatic testing, to test this function
    // run `cargo test -- --ignored`
    #[test]
    #[ignore]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 6007666);
    }
}
