fn read_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect()
}

pub fn part1(input: &str) -> i32 {
    let numbers = read_input(input);

    for (idx1, num1) in numbers.iter().enumerate() {
        for num2 in numbers.iter().skip(idx1 + 1) {
            if num1 + num2 == 2020 {
                return num1 * num2;
            }
        }
    }
    0
}

pub fn part2(input: &str) -> i32 {
    let numbers = read_input(input);

    for (idx1, num1) in numbers.iter().enumerate() {
        for (idx2, num2) in numbers.iter().enumerate().skip(idx1 + 1) {
            for num3 in numbers.iter().skip(idx2 + 1) {
                if num1 + num2 + num3 == 2020 {
                    return num1 * num2 * num3;
                }
            }
        }
    }
    0
}
#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 955584);
    }
    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 287503934);
    }
}
