use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
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

fn star_one(numbers: &Vec<i32>) -> (i32, i32) {
    for (i, first) in numbers.iter().enumerate() {
        let mut j = i + 1;
        while j < numbers.len() {
            if first + numbers[j] == 2020 {
                return (*first, numbers[j]);
            }
            j += 1;
        }
    }
    (-1, -1)
}

fn main() {
    let lines = read_lines("./input")
        .expect("Could not read input file ./input");
    let numbers = parse_numbers(lines);

    println!("Star 1:");
    let (num1, num2) = star_one(&numbers);
    println!("{} + {} = {}", num1, num2, num1 + num2);
    println!("{} * {} = {}", num1, num2, num1 * num2);

    println!("Star 2:");
    let mut i = 0;
    while i < numbers.len() {
        let mut j = i + 1;
        while j < numbers.len() {
            let mut k = j + 1;
            while k < numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    println!("{} + {} + {} = {}", numbers[i], numbers[j], numbers[k], numbers[i] + numbers[j] + numbers[k]);
                    println!("{} * {} * {} = {}", numbers[i], numbers[j], numbers[k], numbers[i] * numbers[j] * numbers[k]);
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
