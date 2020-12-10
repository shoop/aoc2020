use std::convert::TryFrom;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn find_complete_steps(joltages: &Vec<isize>) -> (isize, isize) {
    let mut joltages: Vec<isize> = joltages.iter().copied().collect();
    joltages.sort();
    joltages.iter().fold((0, 0), |(one, three), joltage| {
        match joltage - (one + three * 3) {
            1 => (one + 1, three),
            3 => (one, three + 1),
            _ => panic!(format!("Invalid next value {}", joltage)),
        }
    })
}

fn star_one(joltages: &Vec<isize>) -> isize {
    let (onesteps, threesteps) = find_complete_steps(joltages);
    onesteps * (threesteps + 1)
}

fn star_two(joltages: &Vec<isize>) -> isize {
    let mut joltages: Vec<isize> = joltages.iter().copied().collect();
    joltages.insert(0, 0);
    joltages.sort();
    joltages.push(joltages[joltages.len()-1] + 3);

    // TODO: understand this part ;-D I got to the part where I collected the distances, but then gave up.
    // Thanks to @johnny from MNOT for the working algorithm.
    // Something to do with the amount of choices given consecutive single steps? But why?
    //    base = (nr_ones ^ 2 - nr_ones + 2) / 2
    //    result = multiply result of above formula for each length of consecutive ones
    joltages
        // Collect distances by subtracting consecutive values
        .windows(2)
        .map(|sl| sl[1] - sl[0])
        .collect::<Vec<isize>>()[..]

        // Split on the 3-distance elements
        .split(|d| d == &3)

        // Filter out every 3-distance element, and the final empty one
        .filter(|slice| slice.len() > 0 && slice[0] != 3)

        // Fold into a hashmap with keys the amount of consecutive ones
        .fold(HashMap::new(), |mut s, d| { *s.entry(isize::try_from(d.len()).unwrap()).or_insert(0) += 1; s })

        // Fold the map into the single result by the base formula above
        .iter()
        .fold(1, |result, (key, value)| {
            let base = (key.pow(2) - key + 2) / 2;
            result * base.pow(*value)
        })
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let joltages: Vec<isize> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .map(|x| {
            x.parse::<isize>()
                .expect(&format!("Invalid number: {}", &x))
        })
        .collect();

    let ans = star_one(&joltages);
    println!("Star one: {}", ans);

    let ans = star_two(&joltages);
    println!("Star two: {}", ans);
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
        let joltages: Vec<isize> = TEST_DATA_1
            .lines()
            .map(|x| String::from(x))
            .map(|x| {
                x.parse::<isize>()
                    .expect(&format!("Invalid number: {}", &x))
            })
            .collect();
        let ans = super::star_one(&joltages);
        assert_eq!(ans, 7 * 5);

        let joltages: Vec<isize> = TEST_DATA_2
            .lines()
            .map(|x| String::from(x))
            .map(|x| {
                x.parse::<isize>()
                    .expect(&format!("Invalid number: {}", &x))
            })
            .collect();
        let ans = super::star_one(&joltages);
        assert_eq!(ans, 22 * 10);
    }

    #[test]
    fn test_star_two() {
        let joltages: Vec<isize> = TEST_DATA_1
            .lines()
            .map(|x| String::from(x))
            .map(|x| {
                x.parse::<isize>()
                    .expect(&format!("Invalid number: {}", &x))
            })
            .collect();
        let ans = super::star_two(&joltages);
        assert_eq!(ans, 8);

        let joltages: Vec<isize> = TEST_DATA_2
            .lines()
            .map(|x| String::from(x))
            .map(|x| {
                x.parse::<isize>()
                    .expect(&format!("Invalid number: {}", &x))
            })
            .collect();
        let ans = super::star_two(&joltages);
        assert_eq!(ans, 19208);
    }
}
