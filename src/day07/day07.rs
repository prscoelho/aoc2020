use std::collections::{HashMap, HashSet};

type BagRules<'a> = HashMap<&'a str, Vec<(usize, &'a str)>>;

fn read_bags(input: &str) -> BagRules {
    let mut bags = BagRules::new();
    for line in input.lines() {
        let tokens = line.split(" bags contain ").collect::<Vec<_>>();
        let bag = tokens[0];
        let mut contains = Vec::new();
        for element in tokens[1].strip_suffix(".").unwrap().split(", ") {
            let mut words = element.splitn(2, " ");
            let num = match words.next() {
                Some("no") => break,
                Some(n) => n.parse::<usize>().unwrap(),
                None => panic!(),
            };
            let bag_contained_untrimmed = words.next().unwrap();
            let bag_contained = bag_contained_untrimmed
                .rsplitn(2, " ")
                .skip(1)
                .next()
                .unwrap();
            contains.push((num, bag_contained));
        }

        bags.insert(bag, contains);
    }

    bags
}

fn reaches_gold_bfs(bag_rules: &BagRules, start: &str) -> bool {
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut position = Vec::new();
    position.push(start);

    while let Some(current) = position.pop() {
        if current == "shiny gold" {
            return true;
        }

        if let Some(neighbours) = bag_rules.get(current) {
            for (_, neighbour) in neighbours {
                // true if set is not in visited:
                if visited.insert(neighbour) {
                    position.push(neighbour);
                }
            }
        }
    }
    false
}

pub fn part1(input: &str) -> usize {
    let bags = read_bags(input);
    let mut res = 0;
    for bag in bags.keys() {
        if reaches_gold_bfs(&bags, bag) {
            res += 1
        }
    }

    res - 1 // don't count the shiny bag element
}

fn bag_count(bag_rules: &BagRules, bag: &str) -> usize {
    let mut res = 1; // self
    if let Some(contained_elements) = bag_rules.get(bag) {
        for (count, contained_bag) in contained_elements {
            res += count * bag_count(&bag_rules, contained_bag);
        }
    }

    res
}

pub fn part2(input: &str) -> usize {
    let bags = read_bags(input);
    bag_count(&bags, "shiny gold") - 1 // don't count the shiny bag element
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 211);
    }

    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 12414);
    }
}
