use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

#[derive(Debug)]
struct Rule {
    name: String,
    left: RangeInclusive<u64>,
    right: RangeInclusive<u64>,
}

fn parse_ticket(line: &str) -> Vec<u64> {
    let mut ticket = Vec::new();
    for num in line.trim().split(",") {
        ticket.push(num.parse().unwrap());
    }
    ticket
}

fn parse_range(range_str: &str) -> RangeInclusive<u64> {
    let groups: Vec<_> = range_str.split("-").collect();
    let start = groups[0].parse().unwrap();
    let end = groups[1].parse().unwrap();

    RangeInclusive::new(start, end)
}

fn parse_rule(line: &str) -> Rule {
    let mut it = line.split(": ");
    let name = it.next().unwrap();

    let groups: Vec<_> = it.next().unwrap().split(" or ").collect();
    let left = parse_range(groups[0]);
    let right = parse_range(groups[1]);

    Rule {
        name: name.into(),
        left,
        right,
    }
}

fn parse_rules(input: &str) -> Vec<Rule> {
    let mut rules = Vec::new();
    for line in input.lines() {
        rules.push(parse_rule(line));
    }
    rules
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<u64>, Vec<Vec<u64>>) {
    let groups: Vec<_> = input.split("\n\n").collect();
    let rules = parse_rules(groups[0]);
    let my_ticket = parse_ticket(groups[1].split("\n").skip(1).next().unwrap());

    let other_tickets: Vec<Vec<u64>> = groups[2]
        .split("\n")
        .skip(1)
        .map(|line| parse_ticket(line))
        .collect();

    (rules, my_ticket, other_tickets)
}

fn invalid_fields(ticket: &[u64], rules: &[Rule]) -> Vec<u64> {
    let mut invalid_nums = Vec::new();
    'outer: for num in ticket {
        for rule in rules {
            if valid_field(rule, num) {
                continue 'outer;
            }
        }
        // if we finished the inner for loop and reached this point,
        // no rule contains the ticket field number
        invalid_nums.push(*num);
    }
    invalid_nums
}

fn valid_field(rule: &Rule, value: &u64) -> bool {
    rule.left.contains(value) || rule.right.contains(value)
}

pub fn part1(input: &str) -> u64 {
    let (rules, _, other_tickets) = parse_input(input);

    let mut error_rate: u64 = 0;

    for ticket in other_tickets {
        error_rate += invalid_fields(&ticket, &rules).iter().sum::<u64>();
    }
    error_rate
}

pub fn part2(input: &str) -> u64 {
    let (rules, my_ticket, other_tickets) = parse_input(input);

    let valid_tickets: Vec<_> = other_tickets
        .iter()
        .filter(|ticket| invalid_fields(ticket, &rules).len() == 0)
        .collect();

    let total_fields = rules.len();
    let mut map = HashMap::new();
    for rule in rules.iter() {
        let mut valid_options = HashSet::new();
        for field_index in 0..total_fields {
            if valid_tickets
                .iter()
                .map(|ticket| ticket[field_index])
                .all(|field| valid_field(rule, &field))
            {
                valid_options.insert(field_index);
            }
        }
        map.insert(rule.name.clone(), valid_options);
    }

    let mut name_index = HashMap::new();
    while map.len() > 0 {
        let definite: Vec<_> = map.drain_filter(|_, v| v.len() == 1).collect();
        for (k, v) in definite {
            let finished = v.into_iter().next().unwrap();
            name_index.insert(k, finished);
            for (_, indexes) in map.iter_mut() {
                if indexes.contains(&finished) {
                    indexes.remove(&finished);
                }
            }
        }
    }

    name_index
        .iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, index)| my_ticket[*index])
        .product()
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 26026);
    }

    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 1305243193339);
    }
}
