use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std::vec::Vec;

#[derive(Debug)]
struct InvalidInputError;

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    field_index: Option<usize>,
    range_1: Range<isize>,
    range_2: Range<isize>,
}

#[derive(Debug, Clone)]
struct Ticket {
    values: Vec<isize>,
}

#[derive(Debug)]
struct TrainTickets {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

#[derive(Debug)]
enum ParseState {
    Rules,
    MyTicket,
    NearbyTickets,
}

fn parse_range(range: &str) -> Result<Range<isize>, InvalidInputError> {
    let mut split = range.trim().split("-");
    let start: isize = match split.next() {
        Some(val) => match val.parse::<isize>() {
            Ok(v) => v,
            Err(_) => return Err(InvalidInputError),
        },
        None => return Err(InvalidInputError),
    };

    let end: isize = match split.next() {
        Some(val) => {
            match val.parse::<isize>() {
                // Rust ranges are exclusive
                Ok(v) => v + 1,
                Err(_) => return Err(InvalidInputError),
            }
        }
        None => return Err(InvalidInputError),
    };

    if split.next() != None {
        return Err(InvalidInputError);
    }

    Ok(start..end)
}

fn parse_rule(line: &str) -> Result<Rule, InvalidInputError> {
    let mut split = line.split(":");
    let name = match split.next() {
        Some(val) => val,
        None => return Err(InvalidInputError),
    };
    let ranges = match split.next() {
        Some(val) => val.trim(),
        None => return Err(InvalidInputError),
    };
    if split.next() != None {
        return Err(InvalidInputError);
    }
    let mut split = ranges.split(" or ");
    let range_1 = match split.next() {
        Some(val) => parse_range(val)?,
        None => return Err(InvalidInputError),
    };
    let range_2 = match split.next() {
        Some(val) => parse_range(val)?,
        None => return Err(InvalidInputError),
    };
    if split.next() != None {
        return Err(InvalidInputError);
    }

    Ok(Rule {
        name: String::from(name),
        field_index: None,
        range_1,
        range_2,
    })
}

fn parse_ticket(line: &str) -> Result<Ticket, InvalidInputError> {
    let mut ticket = Ticket { values: vec![] };
    for val in line.split(",") {
        match val.parse::<isize>() {
            Ok(v) => ticket.values.push(v),
            Err(_) => return Err(InvalidInputError),
        }
    }

    Ok(ticket)
}

fn parse_lines<T>(lines: &mut T) -> Result<TrainTickets, InvalidInputError>
where
    T: Iterator<Item = String>,
{
    let mut train_tickets = TrainTickets {
        rules: vec![],
        my_ticket: Ticket { values: vec![] },
        nearby_tickets: vec![],
    };
    let mut state = ParseState::Rules;
    for line in lines {
        match state {
            ParseState::Rules => {
                if line.is_empty() {
                    state = ParseState::MyTicket;
                    continue;
                }

                let rule = parse_rule(&line)?;
                train_tickets.rules.push(rule);
            }
            ParseState::MyTicket => {
                if line == "your ticket:" {
                    // Skip the header
                    continue;
                } else if line.is_empty() {
                    state = ParseState::NearbyTickets;
                    continue;
                }

                train_tickets.my_ticket = parse_ticket(&line)?;
            }
            ParseState::NearbyTickets => {
                if line == "nearby tickets:" {
                    // Skip the header
                    continue;
                }

                let ticket = parse_ticket(&line)?;
                train_tickets.nearby_tickets.push(ticket);
            }
        }
    }

    Ok(train_tickets)
}

fn star_one(train_tickets: &TrainTickets) -> isize {
    let mut invalid_values: Vec<isize> = vec![];
    for ticket in train_tickets.nearby_tickets.iter() {
        for value in ticket.values.iter() {
            let valid = train_tickets.rules.iter().fold(false, |s, x| {
                s || x.range_1.contains(value) || x.range_2.contains(value)
            });
            if !valid {
                invalid_values.push(*value);
            }
        }
    }

    invalid_values.iter().fold(0, |s, x| s + x)
}

fn star_two(train_tickets: &TrainTickets, prefix: &str) -> isize {
    // Assert that we have the same amount of rules as we have fields
    assert_eq!(
        train_tickets.rules.len(),
        train_tickets.my_ticket.values.len()
    );

    let mut valid_tickets: Vec<Ticket> = vec![];
    for ticket in train_tickets.nearby_tickets.iter() {
        let valid = ticket.values.iter().fold(true, |s, x| {
            // All values should test true
            s &&
                // ... when it matches at least one rule
                train_tickets.rules.iter().fold(false, |s2, x2| {
                    s2 || x2.range_1.contains(x) || x2.range_2.contains(x)
                })
        });
        if valid {
            valid_tickets.push(ticket.clone());
        }
    }

    let mut possible_fields: HashMap<usize, Vec<usize>> = HashMap::new();

    for (idx, rule) in train_tickets.rules.iter().enumerate() {
        for field_index in 0..train_tickets.my_ticket.values.len() {
            let mut valid = true;
            for ticket in valid_tickets.iter() {
                if !rule.range_1.contains(&ticket.values[field_index])
                    && !rule.range_2.contains(&ticket.values[field_index])
                {
                    valid = false;
                    break;
                }
            }

            if valid {
                // Rule is valid for this field index, store that fact and break to next rule
                possible_fields.entry(field_index).or_insert(vec![]).push(idx);
            }
        }
    }

    let mut rule_mapping: HashMap<usize, usize> = HashMap::new();
    for _ in 0..train_tickets.my_ticket.values.len() {
        let candidate: Vec<(&usize, &Vec<usize>)> = possible_fields.iter().filter(|(_, v)| v.len() == 1).collect();
        assert_eq!(candidate.len(), 1);
        let field_index = candidate[0].0;
        let rule_index = candidate[0].1[0];
        *rule_mapping.entry(rule_index).or_default() = *field_index;
        for (_, v) in possible_fields.iter_mut() {
            v.retain(|&x| x != rule_index);
        }
    }

    train_tickets.rules
        .iter()
        .enumerate()
        .fold(1, |s, (idx, x)| match x.name.starts_with(prefix) {
            true => s * train_tickets.my_ticket.values[rule_mapping[&idx]],
            false => s,
        })
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"));
    let train_tickets = parse_lines(&mut lines).expect("Invalid input in file");

    let ans = star_one(&train_tickets);
    println!("Star one: {}", ans);

    let ans = star_two(&train_tickets, "departure");
    println!("Star two: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA_1: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    static TEST_DATA_2: &str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    #[test]
    fn test_star_one() {
        let mut lines = TEST_DATA_1.lines().map(|x| String::from(x));
        let train_tickets = super::parse_lines(&mut lines).expect("Invalid input in file");

        assert_eq!(train_tickets.rules[0].name, "class");
        assert_eq!(train_tickets.rules[0].range_1, 1..4);
        assert_eq!(train_tickets.rules[0].range_2, 5..8);

        assert_eq!(train_tickets.nearby_tickets[0].values.len(), 3);
        assert_eq!(train_tickets.nearby_tickets[0].values[0], 7);
        assert_eq!(train_tickets.nearby_tickets[0].values[1], 3);
        assert_eq!(train_tickets.nearby_tickets[0].values[2], 47);

        let ans = super::star_one(&train_tickets);
        assert_eq!(ans, 71);
    }

    #[test]
    fn test_star_two() {
        let mut lines = TEST_DATA_2.lines().map(|x| String::from(x));
        let train_tickets = super::parse_lines(&mut lines).expect("Invalid input in file");

        assert_eq!(train_tickets.rules[0].name, "class");
        assert_eq!(train_tickets.rules[0].range_1, 0..2);
        assert_eq!(train_tickets.rules[0].range_2, 4..20);

        let ans = super::star_two(&train_tickets, "");
        assert_eq!(ans, 1716);
    }
}
