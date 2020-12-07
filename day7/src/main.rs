use core::fmt::Debug;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct IncorrectBagSpecification;

#[derive(Debug)]
struct Bag {
    name: String,
    contents: HashMap<String, usize>,
}

fn until_err<T, E>(err: &mut &mut Result<(), E>, item: Result<T, E>) -> Option<T> {
    match item {
        Ok(item) => Some(item),
        Err(e) => {
            **err = Err(e);
            None
        }
    }
}

impl Bag {
    fn from_specification(specification: &str) -> Result<Bag, IncorrectBagSpecification> {
        // Split on "bags contain" for the name and the contents
        let mut split = specification.split(" bags contain ");
        let name = match split.next() {
            Some(first) => String::from(first),
            None => return Err(IncorrectBagSpecification),
        };
        let contents_spec = match split.next() {
            Some(second) => second,
            None => return Err(IncorrectBagSpecification),
        };
        if let Some(_) = split.next() {
            return Err(IncorrectBagSpecification);
        }

        // Split contents on "," and trim leading/trailing whitespace and fluff to get the list of contents
        let mut err = Ok(());
        let contents_vec: Vec<(String, usize)> = contents_spec
            .split(",")
            .map(|spec| {
                spec.trim()
                    .trim_end_matches(".")
                    .trim_end_matches("s")
                    .trim_end_matches(" bag")
            })
            .map(|spec| match spec {
                "no other" => Ok((String::default(), 0)),
                _ => match spec.find(" ") {
                    Some(idx) => match spec[..idx].parse::<usize>() {
                        Ok(v) => Ok((String::from(&spec[idx + 1..]), v)),
                        Err(_) => Err(IncorrectBagSpecification),
                    },
                    None => Err(IncorrectBagSpecification),
                },
            })
            .scan(&mut err, until_err)
            .collect();
        err?;

        let bag = Bag {
            name: name,
            contents: contents_vec.into_iter().filter(|x| x.1 != 0).collect(),
        };
        Ok(bag)
    }
}

fn parse_bag_specification(
    mut bags: HashMap<String, Bag>,
    line: String,
) -> Result<HashMap<String, Bag>, IncorrectBagSpecification> {
    let bag = Bag::from_specification(&line)?;
    let index = String::from(&bag.name);
    if let Some(b) = bags.get_mut(&index) {
        *b = bag;
    } else {
        bags.insert(index, bag);
    }
    Ok(bags)
}

fn can_contain(bag_name: &str, name: &str, bags: &HashMap<String, Bag>) -> bool {
    let bag = match bags.get(bag_name) {
        Some(bag) => bag,
        None => return false,
    };
    if bag.contents.contains_key(name) {
        return true;
    }
    for subbag_name in bag.contents.keys() {
        if can_contain(subbag_name, name, bags) {
            return true;
        }
    }

    false
}

fn star_one(bags: &HashMap<String, Bag>) -> usize {
    bags.iter()
        .fold(0_usize, |s, (_, b)| if can_contain(&b.name, "shiny gold", bags) { s + 1 } else { s })
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let bags = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .try_fold(HashMap::new(), parse_bag_specification)
        .expect("Invalid data in input file");

    println!("Star 1:");
    let bag_colors_for_shiny_bag = star_one(&bags);
    println!("Bag colors that can contain at least one shiny gold bag: {}", bag_colors_for_shiny_bag);
}

#[cfg(test)]
mod tests {
    use crate::HashMap;

    static TEST_DATA: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn test_star_one() {
        let bags = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .try_fold(HashMap::new(), super::parse_bag_specification)
            .expect("Invalid data in input file");

        assert_eq!(bags.len(), 9);

        assert_eq!(bags.contains_key("light red"), true);
        let bag = bags.get("light red").unwrap();
        assert_eq!(bag.name, "light red");
        assert_eq!(bag.contents.len(), 2);
        assert_eq!(bag.contents.contains_key("bright white"), true);
        assert_eq!(bag.contents.contains_key("muted yellow"), true);
        let amount1 = bag.contents.get("bright white").unwrap();
        assert_eq!(amount1, &1_usize);
        let amount2 = bag.contents.get("muted yellow").unwrap();
        assert_eq!(amount2, &2_usize);

        assert_eq!(bags.contains_key("faded blue"), true);
        let bag = bags.get("faded blue").unwrap();
        assert_eq!(bag.name, "faded blue");
        assert_eq!(bag.contents.len(), 0);

        assert_eq!(super::can_contain("light red", "bright white", &bags), true);
        assert_eq!(super::can_contain("light red", "shiny gold", &bags), true);
        assert_eq!(super::can_contain("light red", "dotted black", &bags), true);
        assert_eq!(super::can_contain("faded blue", "dotted black", &bags), false);

        assert_eq!(super::star_one(&bags), 4);
    }
}
