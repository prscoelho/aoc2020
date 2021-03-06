use std::collections::HashSet;

fn read_groups(input: &str) -> Vec<Vec<HashSet<char>>> {
    let mut groups = Vec::new();
    for group in input.split("\n\n") {
        let mut group_vec = Vec::new();
        for person in group.trim().split("\n") {
            let mut person_set = HashSet::new();
            for c in person.chars() {
                person_set.insert(c);
            }
            group_vec.push(person_set)
        }
        groups.push(group_vec);
    }
    groups
}

pub fn part1(input: &str) -> usize {
    let groups = read_groups(input);
    let mut res = 0;

    for group in groups {
        let mut group_set = HashSet::new();
        for person in group {
            for element in person {
                group_set.insert(element);
            }
        }
        res += group_set.len();
    }
    res
}

pub fn part2(input: &str) -> usize {
    let groups = read_groups(input);
    let mut res = 0;

    for group in groups {
        let mut group_set = group.iter().next().unwrap().clone();
        for person in group.iter().skip(1) {
            group_set.retain(|c| person.contains(c));
        }
        res += group_set.len();
    }
    res
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 6885);
    }
    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 3550);
    }
}
