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
            // allergen listed can match any of the ingredients on the list
            // we can narrow options down by removing elements that don't get repeated
            // every time the allergen is listed
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
    while !allergen_to_ingredient.is_empty() {
        for (&allergen, ingredients) in allergen_to_ingredient.clone().iter() {
            // if this allergen can only match one ingredient, we can add it to canonical list
            // and remove it from the rest of the elements
            // this allows us to match allergens with ingredients one by one
            if ingredients.len() == 1 {
                let ingredient_matched = ingredients.iter().next().unwrap();
                for (_, ingredients_mut) in allergen_to_ingredient.iter_mut() {
                    ingredients_mut.retain(|element| element != ingredient_matched);
                }
                canonical_list.insert(allergen, ingredient_matched);
                allergen_to_ingredient.remove(allergen);
                break;
            }
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

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 2569);
    }
    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(
            super::part2(input),
            "vmhqr,qxfzc,khpdjv,gnrpml,xrmxxvn,rfmvh,rdfr,jxh"
        );
    }
}
