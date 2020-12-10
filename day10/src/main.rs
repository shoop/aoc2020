use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn star_one(joltages: &Vec<isize>) -> isize {
    let mut joltages: Vec<isize> = joltages.iter().copied().collect();
    joltages.sort();
    let (onesteps, threesteps): (isize, isize) = joltages.iter().fold((0, 0), |(one, three), joltage| {
        let curval = one + three * 3;
        if *joltage == curval + 1 {
            (one + 1, three)
        } else if *joltage == curval + 3 {
            (one, three + 1)
        } else {
            panic!(format!("Impossible next value {} ones {} threes {} curval {}", joltage, one, three, curval));
        }
    });

    onesteps * (threesteps + 1)
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let joltages: Vec<isize> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .map(|x| x.parse::<isize>().expect(&format!("Invalid number: {}", &x)))
        .collect();

    let ans = star_one(&joltages);
    println!("Star one: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA_1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    static TEST_DATA_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_star_one() {
        let joltages: Vec<isize> = TEST_DATA_1.lines().map(|x| String::from(x))
            .map(|x| x.parse::<isize>().expect(&format!("Invalid number: {}", &x)))
            .collect();
        let ans = super::star_one(&joltages);
        assert_eq!(ans, 7 * 5);

        let joltages: Vec<isize> = TEST_DATA_2.lines().map(|x| String::from(x))
            .map(|x| x.parse::<isize>().expect(&format!("Invalid number: {}", &x)))
            .collect();
        let ans = super::star_one(&joltages);
        assert_eq!(ans, 22 * 10);
    }
}
