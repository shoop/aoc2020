use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}

fn parse_lines(lines: io::Lines<io::BufReader<File>>) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    for line in lines {
        if let Ok(rawrule) = line {
            let mut rulevec: Vec<&str> = rawrule.split(":").collect();
            assert_eq!(rulevec.len(), 2);
            let pass = crop_letters(rulevec.pop().unwrap(), 1);
            let rule = rulevec.pop().unwrap();
            let mut boundsvec: Vec<&str> = rule.split(" ").collect();
            assert_eq!(boundsvec.len(), 2);
            let incl = boundsvec.pop().unwrap().chars().next().unwrap();
            let bounds = boundsvec.pop().unwrap();
            let mut rangevec: Vec<&str> = bounds.split("-").collect();
            assert_eq!(rangevec.len(), 2);
            let upper: usize = rangevec.pop().unwrap().parse().unwrap();
            let lower: usize = rangevec.pop().unwrap().parse().unwrap();

            entries.push(Entry {
                lower: lower,
                upper: upper,
                incl: incl,
                pass: pass.to_string(),
            })
        }
    }
    entries
}

#[derive(Debug)]
struct Entry {
    lower: usize,
    upper: usize,
    incl: char,
    pass: String,
}

fn star_one(entries: &Vec<Entry>) -> (usize, usize) {
    let mut valid: usize = 0;
    let mut invalid: usize = 0;
    for entry in entries {
        let count = entry.pass.matches(entry.incl).count();
        if count >= entry.lower && count <= entry.upper {
            valid = valid + 1;
        } else {
            invalid = invalid + 1;
        }
    }
    (valid, invalid)
}

fn star_two(entries: &Vec<Entry>) -> (usize, usize) {
    let mut valid: usize = 0;
    let mut invalid: usize = 0;
    for entry in entries {
        let match1 = entry.pass.chars().nth(entry.lower - 1).unwrap() == entry.incl;
        let match2 = entry.pass.chars().nth(entry.upper - 1).unwrap() == entry.incl;
        if (match1 && !match2) || (!match1 && match2) {
            valid = valid + 1;
        } else {
            invalid = invalid + 1;
        }
    }
    (valid, invalid)
}

fn main() {
    let lines = read_lines("./input").expect("Could not read input file ./input");
    let entries = parse_lines(lines);

    println!("Star 1:");
    let (valid, invalid) = star_one(&entries);
    println!("VALID: {}, INVALID: {}", valid, invalid);

    println!("Star 2:");
    let (valid, invalid) = star_two(&entries);
    println!("VALID: {}, INVALID: {}", valid, invalid);
}

#[cfg(test)]
mod tests {
    use super::Entry;

    #[test]
    fn test_star_one() {
        let entries = vec![
            Entry {
                lower: 1,
                upper: 3,
                incl: 'a',
                pass: String::from("abcde"),
            },
            Entry {
                lower: 1,
                upper: 3,
                incl: 'b',
                pass: String::from("cdefg"),
            },
            Entry {
                lower: 2,
                upper: 9,
                incl: 'c',
                pass: String::from("ccccccccc"),
            },
        ];

        let (valid, invalid) = super::star_one(&entries);
        assert_eq!(valid, 2);
        assert_eq!(invalid, 1);
    }

    #[test]
    fn test_star_two() {
        let entries = vec![
            Entry {
                lower: 1,
                upper: 3,
                incl: 'a',
                pass: String::from("abcde"),
            },
            Entry {
                lower: 1,
                upper: 3,
                incl: 'b',
                pass: String::from("cdefg"),
            },
            Entry {
                lower: 2,
                upper: 9,
                incl: 'c',
                pass: String::from("ccccccccc"),
            },
        ];

        let (valid, invalid) = super::star_two(&entries);
        assert_eq!(valid, 1);
        assert_eq!(invalid, 2);
    }
}
