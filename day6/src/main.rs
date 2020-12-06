// use std::collections::HashSet;
// use core::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

// #[derive(Debug)]
// struct IncorrectFormLine;

// #[derive(Debug)]
// struct Solution {
//     groups: Vec<HashSet<char>>
// }

// impl Solution {
    // fn new() -> Self {
    //     Solution { groups: vec![] }
    // }

    // fn add_answers_for_group(self, form: &str) -> Result<Solution, IncorrectFormLine> {
    //     let answers: HashSet<char> = HashSet::new();
    //     for ch in form.chars() {
    //         if ch == '\r' || ch == '\n' {
    //             continue;
    //         }
    //         if !ch.is_alphabetic() || !ch.is_lowercase() {
    //             return Err(IncorrectFormLine);
    //         }

    //         answers.insert(ch);
    //     }
    //     self.groups.push(answers);

    //     Ok(self)
    // }
// }

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let mut s: usize = 0;
    let groups: Vec<(usize, String)> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .map(|x| {
            if x.is_empty() {
                s += 1
            }
            (s, x)
        })
        .filter(|x| !x.1.is_empty())
        .collect();
    let mut group_an: Vec<String> = vec![];
    let group_an = groups.iter().fold(&mut group_an, |s, val| {
        if let Some(_) = s.get_mut(val.0) {
            s[val.0] = String::from(&s[val.0]) + &val.1;
        } else {
            s.push(String::from(&val.1));
        }

        s
    });
    let ans = group_an.iter()
        .fold(0, |s, x| {
            let mut ch: Vec<char> = x.chars().collect();
            ch.sort();
            ch.dedup();
            println!("{} unique {}", x, ch.len());
            s + ch.len()
        });
    println!("Star one: {}", ans);
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
        let mut s: usize = 0;
        let groups: Vec<(usize, &str)> = TEST_DATA
            .lines()
            .map(|x| {
                if x.is_empty() {
                    s += 1
                }
                (s, x)
            })
            .filter(|x| !x.1.is_empty())
            .collect();
        println!("{:?} {}", groups, groups.len());
        let mut group_an: Vec<String> = vec![];
        let group_an = groups.iter().fold(&mut group_an, |s, val| {
            if let Some(_) = s.get_mut(val.0) {
                s[val.0] = String::from(&s[val.0]) + val.1;
            } else {
                s.push(String::from(val.1));
            }

            s
        });
        println!("{:?}", group_an);
        assert_eq!(group_an.len(), 5);
        let ans = group_an.iter()
            .fold(0, |s, x| {
                let mut ch: Vec<char> = x.chars().collect();
                ch.sort();
                ch.dedup();
                println!("{} unique {}", x, ch.len());
                s + ch.len()
            });
        assert_eq!(ans, 11);
    }
}
