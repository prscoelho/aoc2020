use std::collections::{BTreeMap, HashMap, HashSet};

fn parse_input(input: &str) -> Vec<(Vec<&str>, Option<Vec<&str>>)> {
    let mut res = Vec::new();
    for line in input.trim().lines() {
        let mut it = line.split(" (contains ");
        let words_str = it.next().unwrap();
        let words: Vec<&str> = words_str.split_ascii_whitespace().collect();

        let allergens: Option<Vec<&str>> = if let Some(contains_str) = it.next() {
            let allergens_str = contains_str.strip_suffix(")").unwrap();
            Some(allergens_str.split(", ").collect())
        } else {
            None
        };
        res.push((words, allergens));
    }

    res
}

fn find_allergens<'a>(
    lines: &'a Vec<(Vec<&str>, Option<Vec<&str>>)>,
) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut allergen_to_ingredient: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (ingredients, allergens_option) in lines.iter() {
        if let Some(allergens) = allergens_option {
            for allergen in allergens.iter() {
                if let Some(possibilities) = allergen_to_ingredient.get_mut(allergen) {
                    possibilities.retain(|element| ingredients.contains(element));
                } else {
                    let possibilities = ingredients.iter().cloned().collect::<HashSet<&str>>();
                    allergen_to_ingredient.insert(allergen, possibilities);
                }
            }
        }
    }
    allergen_to_ingredient
}

pub fn part1(input: &str) -> usize {
    let lines = parse_input(input);
    let allergen_to_ingredient = find_allergens(&lines);

    let mut all_ingredients: HashMap<&str, usize> = HashMap::new();
    for (ingredients, _) in lines.iter() {
        for ingredient in ingredients {
            *all_ingredients.entry(ingredient).or_insert(0) += 1;
        }
    }
    let bad_ingredients: HashSet<&str> =
        allergen_to_ingredient.values().cloned().flatten().collect();
    all_ingredients
        .iter()
        .filter(|(&element, _)| !bad_ingredients.contains(element))
        .map(|(_, count)| count)
        .sum()
}

pub fn part2(input: &str) -> String {
    let lines = parse_input(input);
    let mut allergen_to_ingredient = find_allergens(&lines);

    let mut canonical_list: BTreeMap<&str, &str> = BTreeMap::new();
    loop {
        let mut changed = false;

        for (&allergen, ingredients) in allergen_to_ingredient.clone().iter() {
            if ingredients.len() == 1 {
                let known_bad = ingredients.iter().next().unwrap();
                for (_, ingredients_mut) in allergen_to_ingredient.iter_mut() {
                    ingredients_mut.retain(|element| element != known_bad);
                }
                canonical_list.insert(allergen, known_bad);
                allergen_to_ingredient.remove(allergen);
                changed = true;
                break;
            }
        }

        if !changed {
            break;
        }
    }

    let mut result = String::new();
    for (_, ingredient) in canonical_list {
        result.push_str(ingredient);
        result.push(',');
    }
    result.pop(); // remove last ','
    result
}
