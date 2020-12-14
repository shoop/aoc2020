use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug)]
struct InvalidOperationError;

#[derive(Debug)]
enum Operation {
    Mask(String),
    MemSet(isize, isize),
}

fn parse_operation(line: &str) -> Result<Operation, InvalidOperationError> {
    let mut split = line.split(" = ");
    match split.next() {
        Some(val) => {
            if val == "mask" {
                if let Some(operand) = split.next() {
                    return Ok(Operation::Mask(String::from(operand)));
                }
            } else if val.starts_with("mem[") {
                if let Some(end_index) = val[4..].find("]") {
                    if let Ok(index) = val[4..end_index+4].parse::<isize>() {
                        if let Some(raw_operand) = split.next() {
                            if let Ok(operand) = raw_operand.parse::<isize>() {
                                return Ok(Operation::MemSet(index, operand));
                            }
                        }
                    }
                }
            }
        }
        None => return Err(InvalidOperationError),
    }

    Err(InvalidOperationError)
}

fn star_one(operations: &Vec<Operation>) -> isize {
    let mut and_mask: isize = 0;
    let mut or_mask: isize = 0;
    let mut memory: HashMap<isize, isize> = HashMap::new();
    for oper in operations {
        match oper {
            Operation::Mask(operand) => {
                and_mask = isize::from_str_radix(&operand.replace("X", "1"), 2).unwrap();
                or_mask = isize::from_str_radix(&operand.replace("X", "0"), 2).unwrap();
            },
            Operation::MemSet(index, operand) => {
                *memory.entry(*index).or_insert(0) = (operand & and_mask) | or_mask;
            }
        }
    }

    memory.iter().fold(0, |s, (_, val)| s + val)
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let operations: Vec<Operation> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .map(|x| parse_operation(&x).expect("Invalid operation in input file"))
        .collect();

    let ans = star_one(&operations);
    println!("Star one: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_star_one() {
        let operations: Vec<super::Operation> = TEST_DATA
            .lines()
            .map(|x| String::from(x))
            .map(|x| super::parse_operation(&x).expect("Invalid operation in input file"))
            .collect();

        let ans = super::star_one(&operations);
        assert_eq!(ans, 165);
    }
}
