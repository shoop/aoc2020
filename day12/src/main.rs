use std::convert::TryFrom;
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug)]
enum Operation {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    // Confirmed that turns always are 90 degrees based on input data
    North = 0,
    East = 90,
    South = 180,
    West = 270,
}

impl TryFrom<isize> for Direction {
    type Error = ();

    fn try_from(v: isize) -> Result<Self, Self::Error> {
        match v {
            x if x == Direction::North as isize => Ok(Direction::North),
            x if x == Direction::East as isize => Ok(Direction::East),
            x if x == Direction::South as isize => Ok(Direction::South),
            x if x == Direction::West as isize => Ok(Direction::West),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Ferry {
    cur_dir: Direction,
    cur_x: isize,
    cur_y: isize,
    waypoint_x: isize,
    waypoint_y: isize,
}

impl Ferry {
    fn new() -> Self {
        Ferry {
            cur_dir: Direction::East,
            cur_x: 0,
            cur_y: 0,
            waypoint_x: 10,
            waypoint_y: -1,
        }
    }

    fn run_deduced_operations(&mut self, operations: &Vec<Operation>) {
        for oper in operations {
            match oper {
                Operation::North(operand) => self.cur_y -= operand,
                Operation::South(operand) => self.cur_y += operand,
                Operation::East(operand) => self.cur_x += operand,
                Operation::West(operand) => self.cur_x -= operand,
                Operation::Left(operand) => self.cur_dir = (self.cur_dir as isize - operand).rem_euclid(360).try_into().unwrap(),
                Operation::Right(operand) => self.cur_dir = (self.cur_dir as isize + operand).rem_euclid(360).try_into().unwrap(),
                Operation::Forward(operand) => match self.cur_dir {
                    Direction::North => self.cur_y -= operand,
                    Direction::South => self.cur_y += operand,
                    Direction::East => self.cur_x += operand,
                    Direction::West => self.cur_x -= operand,
                }
            }
        }
    }

    fn run_real_operations(&mut self, operations: &Vec<Operation>) {
        for oper in operations {
            match oper {
                Operation::North(operand) => self.waypoint_y -= operand,
                Operation::South(operand) => self.waypoint_y += operand,
                Operation::East(operand) => self.waypoint_x += operand,
                Operation::West(operand) => self.waypoint_x -= operand,
                Operation::Left(operand) => match operand {
                    270 => {
                        let tmp = self.waypoint_x;
                        self.waypoint_x = -self.waypoint_y;
                        self.waypoint_y = tmp;
                    },
                    180 => {
                        self.waypoint_x = -self.waypoint_x;
                        self.waypoint_y = -self.waypoint_y;
                    },
                    90 => {
                        let tmp = self.waypoint_x;
                        self.waypoint_x = self.waypoint_y;
                        self.waypoint_y = -tmp;
                    },
                    _ => unreachable!(),
                },
                Operation::Right(operand) => match operand {
                    90 => {
                        let tmp = self.waypoint_x;
                        self.waypoint_x = -self.waypoint_y;
                        self.waypoint_y = tmp;
                    },
                    180 => {
                        self.waypoint_x = -self.waypoint_x;
                        self.waypoint_y = -self.waypoint_y;
                    },
                    270 => {
                        let tmp = self.waypoint_x;
                        self.waypoint_x = self.waypoint_y;
                        self.waypoint_y = -tmp;
                    },
                    _ => unreachable!(),
                },
                Operation::Forward(operand) => {
                    self.cur_x += operand * self.waypoint_x;
                    self.cur_y += operand * self.waypoint_y;
                },
            }
        }
    }

    fn manhattan_distance(&self) -> usize {
        TryInto::<usize>::try_into(self.cur_x.abs()).unwrap() + TryInto::<usize>::try_into(self.cur_y.abs()).unwrap()    
    }
}

#[derive(Debug)]
struct InvalidOperation;

fn operation_from_string(line: &str) -> Result<Operation, InvalidOperation> {
    let operand = line[1..].parse::<isize>().unwrap();
    return match line.chars().next().unwrap() {
        'N' => Ok(Operation::North(operand)),
        'S' => Ok(Operation::South(operand)),
        'E' => Ok(Operation::East(operand)),
        'W' => Ok(Operation::West(operand)),
        'L' => Ok(Operation::Left(operand)),
        'R' => Ok(Operation::Right(operand)),
        'F' => Ok(Operation::Forward(operand)),
        _ => Err(InvalidOperation),
    }
}

fn star_one(operations: &Vec<Operation>) -> usize {
    let mut ferry = Ferry::new();
    ferry.run_deduced_operations(operations);
    ferry.manhattan_distance()
}

fn star_two(operations: &Vec<Operation>) -> usize {
    let mut ferry = Ferry::new();
    ferry.run_real_operations(operations);
    ferry.manhattan_distance()
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let operations: Vec<Operation> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .map(|x| operation_from_string(&x).expect("Invalid operation in input file"))
        .collect();

    let ans = star_one(&operations);
    println!("Star one: {}", ans);

    let ans = star_two(&operations);
    println!("Star two: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_star_one() {
        let operations: Vec<super::Operation> = TEST_DATA
            .lines()
            .map(|x| String::from(x))
            .map(|x| super::operation_from_string(&x).expect("Invalid operation in input file"))
            .collect();

        let ans = super::star_one(&operations);
        assert_eq!(ans, 25);
    }

    #[test]
    fn test_star_two() {
        let operations: Vec<super::Operation> = TEST_DATA
            .lines()
            .map(|x| String::from(x))
            .map(|x| super::operation_from_string(&x).expect("Invalid operation in input file"))
            .collect();

        let ans = super::star_two(&operations);
        assert_eq!(ans, 286);
    }
}
