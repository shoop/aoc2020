use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn find_sum_components(sum: isize, numbers: &Vec<isize>) -> Option<(usize, usize)> {
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j && numbers[i] + numbers[j] == sum {
                return Some((i, j))
            }
        }
    }

    None
}

fn star_one(numbers: &Vec<isize>, window: usize) -> Option<isize> {
    let mut start = 0_usize;
    loop {
        if start + window > numbers.len() {
            return None;
        }

        let possibles: Vec<isize> = numbers[start..start+window].iter().copied().collect();
        let target = numbers[start+window];
        let indices = find_sum_components(target, &possibles);
        match indices {
            Some(_) => start += 1,
            None => return Some(target),
        };
    }
}

fn star_two(numbers: &Vec<isize>, window: usize) -> Option<isize> {
    let target = star_one(numbers, window).expect("No solution for star one found");
    for range in 2..numbers.len() {
        for i in 0..numbers.len() {
            for j in i+range..numbers.len() {
                let sum = numbers[i..j].iter().fold(0, |s, x| s + x);
                if sum == target {
                    let min = numbers[i..j].iter().min().expect("No minimum in range");
                    let max = numbers[i..j].iter().max().expect("No maximum in range");
                    return Some(min + max);
                }
            }
        }
    }

    None
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let numbers: Vec<isize> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .map(|x| x.parse::<isize>().expect(&format!("Invalid number: {}", &x)))
        .collect();

    let ans = star_one(&numbers, 25);
    println!("Star one: {}", ans.expect("No answer found"));

    let ans = star_two(&numbers, 25);
    println!("Star two: {}", ans.expect("No answer found"));
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_star_one() {
        let numbers: Vec<isize> = TEST_DATA.lines().map(|x| String::from(x))
            .map(|x| x.parse::<isize>().expect(&format!("Invalid number: {}", &x)))
            .collect();
        let ans = super::star_one(&numbers, 5).expect("No answer found");
        assert_eq!(ans, 127);
    }

    #[test]
    fn test_star_two() {
        let numbers: Vec<isize> = TEST_DATA.lines().map(|x| String::from(x))
            .map(|x| x.parse::<isize>().expect(&format!("Invalid number: {}", &x)))
            .collect();
        let ans = super::star_two(&numbers, 5).expect("No answer found");
        assert_eq!(ans, 62);
    }
}
