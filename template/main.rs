use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn star_one(_lines: Vec<String>) -> isize {
    -1
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .collect();

    let ans = star_one(lines);
    println!("Star one: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();

        let ans = super::star_one(lines);
        assert_eq!(ans, 100);
    }
}
