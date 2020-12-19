use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;
use regex::Regex;

#[derive(Debug)]
struct InvalidRuleError;

#[derive(Debug)]
enum Rule {
    InOrder(Vec<usize>),
    Either(Vec<usize>, Vec<usize>),
    Literal(String),
}

#[derive(Debug)]
struct SatelliteMessages {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
}

impl SatelliteMessages {
    fn new() -> Self {
        SatelliteMessages {
            rules: HashMap::new(),
            messages: vec![],
        }
    }

    fn parse_rule(&mut self, line: &str) -> Result<(), InvalidRuleError> {
        fn parse_list_of_ints(line: &str) -> Result<Vec<usize>, InvalidRuleError> {
            line.trim()
                .split_ascii_whitespace()
                .map(|n| n.parse::<usize>().map_err(|_| InvalidRuleError))
                .collect()
        }

        let mut split = line.split(":");
        let index = split
            .next()
            .ok_or(InvalidRuleError)?
            .trim()
            .parse::<usize>()
            .map_err(|_| InvalidRuleError)?;
        let raw_rule = split.next().ok_or(InvalidRuleError)?.trim();
        if raw_rule.find("\"").is_some() {
            self.rules.insert(
                index,
                Rule::Literal(String::from(raw_rule.trim_matches('\"'))),
            );
        } else if raw_rule.find("|").is_some() {
            let mut or = raw_rule.split("|");
            let left: Vec<usize> = parse_list_of_ints(or.next().ok_or(InvalidRuleError)?)?;
            let right: Vec<usize> = parse_list_of_ints(or.next().ok_or(InvalidRuleError)?)?;
            self.rules.insert(index, Rule::Either(left, right));
        } else {
            let ordered: Vec<usize> = parse_list_of_ints(raw_rule)?;
            self.rules.insert(index, Rule::InOrder(ordered));
        }

        Ok(())
    }

    fn from_lines(lines: &Vec<String>) -> Result<Self, InvalidRuleError> {
        enum ParseState {
            Rules,
            Messages,
        }

        let mut sat = SatelliteMessages::new();
        let mut state = ParseState::Rules;
        for line in lines.iter() {
            if line.trim().is_empty() {
                state = ParseState::Messages;
                continue;
            }

            match state {
                ParseState::Rules => sat.parse_rule(line)?,
                ParseState::Messages => sat.messages.push(String::from(line.trim())),
            }
        }

        Ok(sat)
    }

    fn build_regexp(&self, index: usize, result: &mut String) {
        match self.rules.get(&index).unwrap() {
            Rule::InOrder(vec) => {
                for idx in vec {
                    self.build_regexp(*idx, result);
                }
            },
            Rule::Either(left, right) => {
                result.push('(');
                for idx in left {
                    self.build_regexp(*idx, result);
                }
                result.push('|');
                for idx in right {
                    self.build_regexp(*idx, result);
                }
                result.push(')');
            },
            Rule::Literal(ch) => {
                result.push_str(&ch);
            }
        }
    }

    fn matching_messages(&self) -> usize {
        let mut regexp = String::from("^");
        self.build_regexp(0, &mut regexp);
        regexp.push('$');
        let re = Regex::new(&regexp).unwrap();
        let mut result: usize = 0;

        for msg in self.messages.iter() {
            if re.is_match(msg) {
                result += 1;
            }
        }

        result
    }
}

fn star_one(messages: &SatelliteMessages) -> usize {
    messages.matching_messages()
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .collect();

    let messages = SatelliteMessages::from_lines(&lines).expect("Invalid rule in input file");
    let ans = star_one(&messages);
    println!("Star one: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA_1: &str = "0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"

aab
abb
aba
";

    static TEST_DATA_2: &str = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA_1.lines().map(|x| x.to_string()).collect();

        let messages = super::SatelliteMessages::from_lines(&lines).expect("Invalid test data");
        let ans = super::star_one(&messages);
        assert_eq!(ans, 2);

        let lines: Vec<String> = TEST_DATA_2.lines().map(|x| x.to_string()).collect();

        let messages = super::SatelliteMessages::from_lines(&lines).expect("Invalid test data");
        let ans = super::star_one(&messages);
        assert_eq!(ans, 2);
    }
}
