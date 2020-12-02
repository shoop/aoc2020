use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut vec: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(number) = line {
                let parsed: i32 = number.parse().unwrap();
                vec.push(parsed);
            }
        }
    }

    for i in &vec {
        for j in &vec {
            if i + j == 2020 {
                println!("{} + {} = {}", i, j, i + j);
            }
        }
    }
}
