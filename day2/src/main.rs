use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}

#[derive(Debug)]
struct Entry {
    lower: usize,
    upper: usize,
    incl: char,
    pass: String,
}

fn main() {
    let mut entries: Vec<Entry> = Vec::new();
    if let Ok(lines) = read_lines("./input") {
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
    }

    println!("Star 1:");
    let mut valid: usize = 0;
    let mut invalid: usize = 0;
    for entry in &entries {
        print!("{:?}: ", entry);
        let count = entry.pass.matches(entry.incl).count();
        if count >= entry.lower && count <= entry.upper {
            println!("count {} VALID!", count);
            valid = valid + 1;
        } else {
            println!("count {} INVALID!", count);
            invalid = invalid + 1;
        }
    }
    println!("VALID: {}, INVALID: {}", valid, invalid);

    println!("Star 2:");
    let mut valid2: usize = 0;
    let mut invalid2: usize = 0;
    for entry in &entries {
        print!("{:?}: ", entry);
        let match1 = entry.pass.chars().nth(entry.lower - 1).unwrap() == entry.incl;
        let match2 = entry.pass.chars().nth(entry.upper - 1).unwrap() == entry.incl;
        if (match1 && !match2) || (!match1 && match2) {  
            println!("VALID!");
            valid2 = valid2 + 1;
        } else {
            println!("INVALID!");
            invalid2 = invalid2 + 1;
        }
    }
    println!("VALID: {}, INVALID: {}", valid2, invalid2);
}
