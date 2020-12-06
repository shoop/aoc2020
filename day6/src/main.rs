use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn star_one<I>(iter: I) -> usize
where
    I: Iterator<Item = String>,
{
    let mut group_nr: usize = 0;
    let mut group_answers: Vec<String> = vec![];
    let ans = iter
        .map(|x| {
            if x.is_empty() {
                group_nr += 1
            }
            (group_nr, x)
        })
        .filter(|x| !x.1.is_empty())
        .collect::<Vec<(usize, String)>>()
        .iter()
        .fold(&mut group_answers, |s, val| {
            if let Some(_) = s.get_mut(val.0) {
                s[val.0] = String::from(&s[val.0]) + &val.1;
            } else {
                s.push(String::from(&val.1));
            }

            s
        })
        .iter()
        .fold(0, |s, x| {
            let mut ch: Vec<char> = x.chars().collect();
            ch.sort();
            ch.dedup();
            s + ch.len()
        });

    ans
}

fn star_two<I>(iter: I) -> usize
where
    I: Iterator<Item = String>,
{
    let mut group_nr: usize = 0;
    let group_answers: Vec<(usize, String)> = iter
        .map(|x| {
            if x.is_empty() {
                group_nr += 1
            }
            (group_nr, x)
        })
        .filter(|x| !x.1.is_empty())
        .collect();

    let mut group_hash: Vec<HashSet<char>> = vec![];
    for (g, v) in group_answers {
        match group_hash.get(g) {
            None => group_hash.push(v.chars().collect()),
            _ => group_hash[g] = group_hash[g].intersection(&v.chars().collect()).copied().collect()
        }
    }

    group_hash.iter().fold(0, |s, x| {
        s + x.len()
    })
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let lines = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"));

    let ans = star_one(lines);
    println!("Star one: {}", ans);

    let file = File::open("./input").expect("Unreadable input file ./input");
    let lines = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"));
    let ans = star_two(lines);
    println!("Star two: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_star_one() {
        let lines = TEST_DATA.lines().map(|x| String::from(x));
        let ans = super::star_one(lines);
        assert_eq!(ans, 11);
    }

    #[test]
    fn test_star_two() {
        let lines = TEST_DATA.lines().map(|x| String::from(x));
        let ans = super::star_two(lines);
        assert_eq!(ans, 6);
    }
}
