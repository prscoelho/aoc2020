use std::collections::HashMap;

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

fn can_create(rules: &HashMap<usize, Vec<Rule>>, literal: char) -> Vec<usize> {
    let mut result = Vec::new();
    for (rule_index, rule_vec) in rules.iter() {
        for rule in rule_vec {
            if let Rule::Literal(c) = rule {
                if *c == literal {
                    result.push(*rule_index);
                }
            }
        }
    }
    result
}

// an implementation of the CYK algorithm as described by
// https://en.wikipedia.org/wiki/CYK_algorithm
fn matches(rules: &HashMap<usize, Vec<Rule>>, input: &str) -> bool {
    // In case there are gaps in key rules. this is a bit hacky, but it's faster
    // to allocate more space than to use a hashset. but it is a trade off to
    // watch out for. A bad input could be:
    //   0: 50000
    //   50000: "a"
    // which would make us allocate 50000 * text_length^2 bytes, even though we only
    // have two elements. the puzzle input has no gaps, so it's not a real problem.
    // But example2 input has some gaps and we can easily pass that test with this
    // small modification.
    let rule_size = rules.keys().max().unwrap();

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
                for (&a, vec_rule) in rules.iter() {
                    for rule in vec_rule {
                        if let Rule::Standard(v) = rule {
                            let b = v[0];
                            let c = v[1];

                            if set[p][s][b] && set[l - p][s + p][c] {
                                set[l][s][a] = true;
                            }
                        }
                    }
                }
            }
        }
    }
    set[n][0][0]
}

fn remove_unit_rules(rules: &mut HashMap<usize, Vec<Rule>>) {
    loop {
        let reference = rules.clone();
        let mut keep_looping = false;
        for (_, rule_vec) in rules.iter_mut() {
            let removed = rule_vec
                .drain_filter(|rule| {
                    match rule {
                        Rule::Literal(_) => {
                            false // dont filter out literals
                        }
                        Rule::Standard(v) => {
                            v.len() == 1 // only filter if len == 1
                        }
                    }
                })
                .collect::<Vec<_>>();
            if removed.len() > 0 {
                keep_looping = true;
            }
            for rule in removed {
                if let Rule::Standard(v) = rule {
                    let r = v[0];
                    // we are replacing A -> B with A -> C D where C D is B -> C D
                    // or, in english, we are replacing B with wherever B leads to.
                    for rule in reference[&r].iter() {
                        rule_vec.push(rule.clone());
                    }
                }
            }
        }
        if !keep_looping {
            break;
        }
    }
}

fn remove_triple_rules(rules: &mut HashMap<usize, Vec<Rule>>) {
    loop {
        // to account for gaps in keys, we'll add new keys starting at max_rule + 1.
        // we could search for gaps if we really wanted to save space. there are no
        // gaps in the puzzle input, only in the example2 input
        let mut rules_size = *rules.keys().max().unwrap() + 1;
        let mut to_add = HashMap::new();
        for (_, rule_vec) in rules.iter_mut() {
            let removed = rule_vec
                .drain_filter(|rule| {
                    match rule {
                        Rule::Literal(_) => {
                            false // dont filter out literals
                        }
                        Rule::Standard(v) => {
                            v.len() == 3 // only filter if len == 3, we could make it more general but it's
                                         // unneeded complexity, we have no rules with more than three elements
                        }
                    }
                })
                .collect::<Vec<_>>();
            for rule in removed {
                if let Rule::Standard(mut v) = rule {
                    let first = v.remove(0);
                    let new_index = rules_size;
                    // to fix rules that are in the form of A -> B C D
                    // we change A to A -> B E
                    // and add a new rule E -> C D
                    let modified_rule = Rule::Standard(vec![first, new_index]);
                    rule_vec.push(modified_rule);

                    let new_rule = vec![Rule::Standard(v)];
                    // because we are currently iterating and mutating rules,
                    // we can't add more elements to it. so, keep a list of new rules
                    // to add and add them after we finish iterating.
                    to_add.insert(new_index, new_rule);

                    rules_size += 1;
                }
            }
        }
        if to_add.len() == 0 {
            break;
        }
        for (rule_index, rule_vec) in to_add {
            rules.insert(rule_index, rule_vec);
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut it = input.split("\n\n");
    let mut rules = parse_rules(it.next().unwrap());
    remove_triple_rules(&mut rules);
    remove_unit_rules(&mut rules);
    let strings: Vec<String> = it.next().unwrap().lines().map(|s| s.into()).collect();

    strings
        .into_iter()
        .map(|s| matches(&rules, &s))
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
    remove_triple_rules(&mut rules);
    remove_unit_rules(&mut rules);

    let strings: Vec<String> = it.next().unwrap().lines().map(|s| s.into()).collect();

    strings
        .into_iter()
        .map(|s| matches(&rules, &s))
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
