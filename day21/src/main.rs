use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug)]
struct Rule {
    ingredients: Vec<String>,
    allergens: HashSet<String>,
}

fn parse_rule(line: &str) -> Rule {
    let mut split = line.trim_end_matches(")").split(" (contains");
    Rule {
        ingredients: split
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.to_string())
            .collect(),
        allergens: split
            .next()
            .unwrap()
            .split(&[' ', ','][..])
            .filter(|a| !a.is_empty())
            .map(|x| x.to_string())
            .collect(),
    }
}

fn parse_rules(_lines: Vec<String>) -> Vec<Rule> {
    let mut rules: Vec<Rule> = vec![];
    for line in _lines {
        rules.push(parse_rule(&line));
    }

    rules
}

fn find_unique_allergens(rules: &Vec<Rule>) -> HashSet<String> {
    let mut unique_allergens: HashSet<String> = HashSet::new();
    for allergen in rules[0].allergens.iter() {
        unique_allergens.insert(allergen.clone());
    }

    for rule in rules {
        for allergen in rule.allergens.iter() {
            unique_allergens.insert(allergen.clone());
        }
    }

    unique_allergens
}

fn find_unique_ingredients(rules: &Vec<Rule>) -> HashSet<String> {
    let mut unique_ingredients: HashSet<String> = HashSet::new();
    for ingredient in rules[0].ingredients.iter() {
        unique_ingredients.insert(ingredient.clone());
    }

    for rule in rules {
        for ingredient in rule.ingredients.iter() {
            unique_ingredients.insert(ingredient.clone());
        }
    }

    unique_ingredients
}

fn find_inallergic_ingredients(rules: &Vec<Rule>) -> HashSet<String> {
    let unique_allergens = find_unique_allergens(rules);
    let mut unique_ingredients = find_unique_ingredients(rules);

    for allergen in unique_allergens.iter() {
        // TODO: fix string allocations
        let mut possible_ingredients: HashSet<String> = HashSet::new();
        for rule in rules.iter() {
            let mut ing_hash: HashSet<String> = HashSet::new();
            for ing in rule.ingredients.iter() {
                ing_hash.insert(ing.clone());
            }
            if rule.allergens.contains(&allergen.to_string()) {
                if possible_ingredients.len() == 0 {
                    possible_ingredients = possible_ingredients.union(&ing_hash).map(|x| x.to_string()).collect();
                } else {
                    possible_ingredients = possible_ingredients.intersection(&ing_hash).map(|x| x.to_string()).collect();
                }
            }
        }

        unique_ingredients = unique_ingredients.difference(&possible_ingredients).map(|x| x.to_string()).collect();
    }

    unique_ingredients
}

fn star_one(rules: &Vec<Rule>) -> isize {
    let inallergic_ingredients = find_inallergic_ingredients(rules);

    let mut result: isize = 0;
    for ing in inallergic_ingredients {
        for rule in rules.iter() {
            result += rule.ingredients.iter().filter(|&i| i == &ing).count() as isize;
        }
    }

    result
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let rules: Vec<Rule> = parse_rules(
        io::BufReader::new(file)
            .lines()
            .map(|x| x.expect("Could not read line"))
            .collect(),
    );

    let ans = star_one(&rules);
    println!("Star one: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";

    #[test]
    fn test_star_one() {
        let rules: Vec<super::Rule> =
            super::parse_rules(TEST_DATA.lines().map(|x| x.to_string()).collect());

        let ans = super::star_one(&rules);
        assert_eq!(ans, 5);
    }
}
