fn transform(subject: u64, n: usize) -> u64 {
    let mut value = subject;
    for _ in 1..n {
        value *= subject;
        value %= 20201227;
    }
    value
}

fn search_loop_size(subject: u64, goal: u64) -> usize {
    let mut value = subject;
    for i in 1.. {
        if value == goal {
            return i;
        }
        value *= subject;
        value %= 20201227;
    }
    unreachable!();
}

fn parse_input(input: &str) -> (u64, u64) {
    let mut it = input.lines();
    let card_pubkey = it.next().unwrap().parse().unwrap();
    let door_pubkey = it.next().unwrap().parse().unwrap();

    (card_pubkey, door_pubkey)
}

pub fn part1(input: &str) -> u64 {
    let (card_pubkey, door_pubkey) = parse_input(input);
    let card_loopsize = search_loop_size(7, card_pubkey);
    let encryption_key = transform(door_pubkey, card_loopsize);
    encryption_key
}

pub fn part2(_input: &str) -> String {
    "Merry Christmas".into()
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 10187657);
    }
}
