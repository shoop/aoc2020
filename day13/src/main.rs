use std::io::Write;
use std::io::stdout;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn star_one(earliest: isize, buses: &Vec<isize>) -> isize {
    let (bus, mins) = buses.iter().map(|x| (x, -(earliest % x) + x)).min_by(|x, y| x.1.cmp(&y.1)).unwrap();
    bus * mins
}

fn star_two_peeked_solution(buses: &Vec<(usize, isize)>) -> isize {
    // I cheated and peeked, I did not see the fact that we need to increment
    // by a multiple of the previous bus nr in order to hold the invariant so far.
    let mut time: isize = 0;
    let mut inc: isize = 1;
    for bus in buses {
        while (time + (bus.0 as isize)) % bus.1 != 0 {
            time += inc;
        }

        inc *= bus.1;
    }

    time
}

fn star_two(buses: &Vec<(usize, isize)>) -> isize {
    // This version does not complete in reasonable time but was the best I could think of
    let max_bus = buses.iter().max_by(|(_, busa), (_, busb)| busa.cmp(busb)).unwrap();
    let mut check: isize = -(max_bus.0 as isize) + max_bus.1;
    loop {
        print!("{:15}", check);
        let result = buses.iter().fold(true, |s, (idx, b)| {
            s & (check % b == ((-(*idx as isize) + b)) % b)
        });
        if result {
            println!();
            return check;
        }
        // Check faster by ensuring we increment by the maximum possible bus nr
        check += max_bus.1;
        for _ in 0..15 {
            print!("\x1B[D");
        }
        stdout().flush().unwrap();
    }
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

fn parse_buses_with_offset(line: &str) -> Vec<(usize, isize)> {
    line.split(",")
        .enumerate()
        .map(|(idx, x)| {
            let mut result: isize = -1;
            if x != "x" {
                result = x.parse::<isize>()
                    .expect(&format!("Invalid number: {}", &x));
            }
            (idx, result)
        })
        .filter(|&x| x.1 != -1)
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

    let file = File::open("./input").expect("Unreadable input file ./input");
    let mut lines = io::BufReader::new(file)
        .lines().map(|x| x.expect("Could not read line"));
    lines.next().unwrap();
    let buses = parse_buses_with_offset(&lines.next().unwrap());

    let ans = star_two_peeked_solution(&buses);
    println!("Star two: {}", ans);

    let ans = star_two(&buses);
    println!("Star two brute-force: {}", ans);
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

    #[test]
    fn test_star_two() {
        let mut lines = TEST_DATA.lines().map(|x| String::from(x));
        lines.next().unwrap();
        let buses = super::parse_buses_with_offset(&lines.next().unwrap());
        let ans = super::star_two_peeked_solution(&buses);
        assert_eq!(ans, 1068781);
    }
}
