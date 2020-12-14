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

// Tuple (and_mask, or_mask)
fn generate_masks(mask: &str) -> Vec<(isize, isize)> {
    let mut result: Vec<(isize, isize)> = vec![(0, 0)];
    for (idx, ch) in mask.chars().rev().enumerate() {
        match ch {
            '0' => {
                for r in result.iter_mut() {
                    r.0 = r.0 | 1 << idx;
                }
            },
            '1' => {
                for r in result.iter_mut() {
                    r.0 = r.0 | 1 << idx;
                    r.1 = r.1 | 1 << idx;
                }
            },
            'X' => {
                let mut to_add: Vec<(isize, isize)> = vec![];
                for r in result.iter_mut() {
                    // Fork into the two different options
                    r.1 = r.1 | 1 << idx;

                    let mut copy = r.clone();
                    copy.0 = copy.0 | 1 << idx;
                    to_add.push(copy);
                }

                result.append(&mut to_add);
            },
            _ => unreachable!(),
        }
    }

    result
}

fn star_two(operations: &Vec<Operation>) -> isize {
    let mut masks: Vec<(isize, isize)> = vec![];
    let mut memory: HashMap<isize, isize> = HashMap::new();
    for oper in operations {
        match oper {
            Operation::Mask(operand) => {
                masks = generate_masks(&operand);
            },
            Operation::MemSet(index, operand) => {
                for mask in &masks {
                    *memory.entry((*index | mask.1) & mask.0).or_insert(0) = *operand;
                }
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

    let ans = star_two(&operations);
    println!("Star two: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA_1: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_star_one() {
        let operations: Vec<super::Operation> = TEST_DATA_1
            .lines()
            .map(|x| String::from(x))
            .map(|x| super::parse_operation(&x).expect("Invalid operation in input file"))
            .collect();

        let ans = super::star_one(&operations);
        assert_eq!(ans, 165);
    }

    static TEST_DATA_2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_star_two() {
        let operations: Vec<super::Operation> = TEST_DATA_2
            .lines()
            .map(|x| String::from(x))
            .map(|x| super::parse_operation(&x).expect("Invalid operation in input file"))
            .collect();

        let ans = super::star_two(&operations);
        assert_eq!(ans, 208);
    }
}
