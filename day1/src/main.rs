use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

#[derive(Debug, PartialEq)]
struct CorrectNumbersNotFoundError;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_numbers(lines: io::Lines<io::BufReader<File>>) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    for line in lines {
        if let Ok(number) = line {
            let parsed: i32 = number.parse().unwrap();
            numbers.push(parsed);
        }
    }
    numbers
}

fn star_one(numbers: &Vec<i32>) -> Result<(i32, i32), CorrectNumbersNotFoundError> {
    for (i, first) in numbers.iter().enumerate() {
        for (_, second) in numbers.iter().enumerate().skip(i) {
            if first + second == 2020 {
                return Ok((*first, *second));
            }
        }
    }
    Err(CorrectNumbersNotFoundError)
}

fn star_two(numbers: &Vec<i32>) -> Result<(i32, i32, i32), CorrectNumbersNotFoundError> {
    for (i, first) in numbers.iter().enumerate() {
        for (j, second) in numbers.iter().enumerate().skip(i) {
            for (_, third) in numbers.iter().enumerate().skip(j) {
                if first + second + third == 2020 {
                    return Ok((*first, *second, *third));
                }
            }
        }
    }
    Err(CorrectNumbersNotFoundError)
}

fn main() {
    let lines = read_lines("./input").expect("Could not read input file ./input");
    let numbers = parse_numbers(lines);

    println!("Star 1:");
    let (num1, num2) = star_one(&numbers).expect("Invalid input data");
    println!("{} + {} = {}", num1, num2, num1 + num2);
    println!("{} * {} = {}", num1, num2, num1 * num2);

    println!("Star 2:");
    let (num1, num2, num3) = star_two(&numbers).expect("Invalid input data");
    println!("{} + {} + {} = {}", num1, num2, num3, num1 + num2 + num3);
    println!("{} * {} * {} = {}", num1, num2, num3, num1 * num2 * num3);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_star_one() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];
        let (num1, num2) = super::star_one(&numbers).expect("Invalid test data");
        assert_eq!(num1 + num2, 2020);
        assert_eq!(num1 * num2, 514579);

        let numbers = vec![1, 2];
        let result = super::star_one(&numbers);
        assert_eq!(result, Err(super::CorrectNumbersNotFoundError));
    }

    #[test]
    fn test_star_two() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];
        let (num1, num2, num3) = super::star_two(&numbers).expect("Invalid test data");
        assert_eq!(num1 + num2 + num3, 2020);
        assert_eq!(num1 * num2 * num3, 241861950);

        let numbers = vec![1, 2, 3];
        let result = super::star_two(&numbers);
        assert_eq!(result, Err(super::CorrectNumbersNotFoundError));
    }
}
