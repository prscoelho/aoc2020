use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
enum Rule {
    Literal(char),        // literal
    Standard(Vec<usize>), // list of rules which rule matches
}

fn parse_rule_part(text: &str) -> Rule {
    Rule::Standard(
        text.split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>(),
    )
}

fn parse_rule(text: &str) -> Vec<Rule> {
    let mut result = Vec::new();
    if text.contains("\"") {
        result.push(Rule::Literal(text.chars().skip(1).next().unwrap()));
    } else if text.contains("|") {
        for part in text.split(" | ") {
            result.push(parse_rule_part(part));
        }
    } else {
        result.push(parse_rule_part(text));
    }
    result
}

fn parse_rules(rules_str: &str) -> HashMap<usize, Vec<Rule>> {
    let mut rules = HashMap::new();

    for line in rules_str.lines() {
        let mut it = line.split(": ");
        let left_str = it.next().unwrap();
        let right_str = it.next().unwrap();

        assert_eq!(it.next(), None);

        let left = left_str.parse().unwrap();
        let right = parse_rule(right_str);

        rules.insert(left, right);
    }

    rules
}

fn matches(rules: &HashMap<usize, Vec<Rule>>, phrase: &[char], mut with: VecDeque<usize>) -> bool {
    match (phrase.len(), with.len()) {
        (0, 0) => return true,  // matches is true only if phrase and with is empty
        (_, 0) => return false, // it can't match if phrase is empty and with is not, likewise for the reverse
        (0, _) => return false,
        _ => {}
    }

    let rule_to_expand = with.pop_front().unwrap();
    let possibilities = &rules[&rule_to_expand];
    for rule in possibilities {
        let result = match rule {
            Rule::Literal(c) => {
                if c == &phrase[0] {
                    // character matched with next rule literal, call matches with one less character and rest of with.
                    let next_with = with.clone();
                    matches(&rules, &phrase[1..], next_with)
                } else {
                    // character didn't match with the expanded rule
                    false
                }
            }
            Rule::Standard(expanded) => {
                // we werent able to match with a character, expand the popped rule and try to match [expanded, rest]
                let next_with: VecDeque<usize> =
                    expanded.iter().chain(with.iter()).copied().collect();
                if expanded.len() > phrase.len() {
                    // if the total expanded rule size is bigger than phrase size, it can't possibly match.
                    // there are no empty rules, each rule will match at least one character
                    false
                } else {
                    matches(&rules, &phrase, next_with)
                }
            }
        };
        if result {
            return true;
        }
    }
    false
}

pub fn part1(input: &str) -> usize {
    let mut it = input.split("\n\n");
    let rules = parse_rules(it.next().unwrap());
    let strings: Vec<Vec<char>> = it
        .next()
        .unwrap()
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let mut start = VecDeque::new();
    start.push_back(0);

    strings
        .into_iter()
        .map(|s| matches(&rules, &s, start.clone()))
        .filter(|&b| b)
        .count()
}

fn modify_part2(rules: &mut HashMap<usize, Vec<Rule>>) {
    let rule8 = vec![Rule::Standard(vec![42]), Rule::Standard(vec![42, 8])];
    let rule11 = vec![
        Rule::Standard(vec![42, 31]),
        Rule::Standard(vec![42, 11, 31]),
    ];
    if let Some(rule) = rules.get_mut(&8) {
        *rule = rule8;
    }
    if let Some(rule) = rules.get_mut(&11) {
        *rule = rule11;
    }
}

pub fn part2(input: &str) -> usize {
    let mut it = input.split("\n\n");
    let mut rules = parse_rules(it.next().unwrap());

    modify_part2(&mut rules);

    let strings: Vec<Vec<char>> = it
        .next()
        .unwrap()
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let mut start = VecDeque::new();
    start.push_back(0);

    strings
        .into_iter()
        .map(|s| matches(&rules, &s, start.clone()))
        .filter(|&b| b)
        .count()
}

#[cfg(test)]
mod test {
    // running the puzzle input on debug mode doesnt seem feasible,
    // but at least test the example inputs.
    #[test]
    fn example1() {
        let input = include_str!("example");
        assert_eq!(super::part1(input), 2);
    }

    #[test]
    fn example2() {
        let input = include_str!("example2");
        assert_eq!(super::part1(input), 3);
        assert_eq!(super::part2(input), 12);
    }
}
