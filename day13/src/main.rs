use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn star_one(earliest: isize, buses: &Vec<isize>) -> isize {
    let (bus, mins) = buses.iter().map(|x| (x, -(earliest % x) + x)).min_by(|x, y| x.1.cmp(&y.1)).unwrap();
    bus * mins
}

fn parse_buses(line: &str) -> Vec<isize> {
    line.split(",")
        .map(|x| {
            let mut result: isize = -1;
            if x != "x" {
                result = x.parse::<isize>()
                    .expect(&format!("Invalid number: {}", &x));
            }
            result
        })
        .filter(|&x| x != -1)
        .collect()
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let mut lines = io::BufReader::new(file)
        .lines().map(|x| x.expect("Could not read line"));
    let earliest = lines.next().unwrap().parse::<isize>().expect("Invalid first line");
    let buses = parse_buses(&lines.next().unwrap());

    let ans = star_one(earliest, &buses);
    println!("Star one: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_star_one() {
        let mut lines = TEST_DATA.lines().map(|x| String::from(x));
        let earliest = lines.next().unwrap().parse::<isize>().expect("Invalid first line");
        let buses = super::parse_buses(&lines.next().unwrap());
        let ans = super::star_one(earliest, &buses);
        assert_eq!(ans, 59 * 5);
    }
}
