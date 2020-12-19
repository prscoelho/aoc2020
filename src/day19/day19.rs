use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Rule {
    Literal(char),             // literal
    Standard(Vec<Vec<usize>>), // list of rules which rule matches
}

fn parse_rule(text: &str) -> Rule {
    if text.contains("\"") {
        Rule::Literal(text.chars().skip(1).next().unwrap())
    } else if text.contains("|") {
        let mut v = Vec::new();
        for r in text.split(" | ") {
            let v1 = r
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>();
            v.push(v1);
        }
        Rule::Standard(v)
    } else {
        Rule::Standard(vec![text
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>()])
    }
}

fn parse_rules(rules_str: &str) -> HashMap<usize, Rule> {
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

fn can_create(rules: &HashMap<usize, Rule>, literal: char) -> Vec<usize> {
    let mut result = Vec::new();
    for (rule_index, rule) in rules.iter() {
        if let Rule::Literal(c) = rule {
            if *c == literal {
                result.push(*rule_index);
            }
        }
    }
    result
}

fn matches(rules: &HashMap<usize, Rule>, input: &str) -> bool {
    let rule_size = rules.len();
    let text: Vec<char> = input.chars().collect();
    let n = text.len();
    let mut set = vec![vec![vec![false; rule_size + 1]; n + 1]; n + 1];
    for s in 0..n {
        for rule_index in can_create(&rules, text[s]) {
            set[1][s][rule_index] = true;
        }
    }

    for l in 2..=n {
        for s in 0..=n - l + 1 {
            for p in 1..=l - 1 {
                for (&a, rule) in rules.iter() {
                    match rule {
                        Rule::Standard(vecs) => {
                            for v in vecs.iter() {
                                let b = v[0];
                                let c = v[1];

                                if set[p][s][b] && set[l - p][s + p][c] {
                                    set[l][s][a] = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    set[n][0][0]
}

pub fn part1(input: &str) -> usize {
    let mut it = input.split("\n\n");
    let rules = parse_rules(it.next().unwrap());

    let strings: Vec<String> = it.next().unwrap().lines().map(|s| s.into()).collect();

    strings
        .into_iter()
        .map(|s| matches(&rules, &s))
        .filter(|&b| b)
        .count()
}

pub fn part2(_input: &str) -> i32 {
    0
}
