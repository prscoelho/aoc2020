use std::collections::BTreeSet;

fn read_ids(input: &str) -> BTreeSet<u16> {
    let mut ids = BTreeSet::new();

    for line in input.lines() {
        // id = col * 8 + row, which is the same as col << 3 + row
        // we can just skip that and parse the entire line as u16
        let id = line.chars().fold(0, |acc, elem| {
            (acc << 1) + if elem == 'B' || elem == 'R' { 1 } else { 0 }
        });
        ids.insert(id);
    }
    ids
}

pub fn part1(input: &str) -> u16 {
    let ids = read_ids(input);
    *ids.iter().next_back().unwrap()
}

pub fn part2(input: &str) -> u16 {
    let ids = read_ids(input);
    for id in ids.iter() {
        // if the next id is not in the set and the next next id is, we found our gap
        if !ids.contains(&(id + 1)) && ids.contains(&(id + 2)) {
            return id + 1;
        }
    }
    // We should return an Option<u16>, but input is well formed and there is always an answer
    // so we should never reach this point, but panic if we do.
    unreachable!();
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 938);
    }
    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 696);
    }
}
