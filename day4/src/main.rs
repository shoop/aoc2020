use core::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug)]
struct PassportParsingError;

#[derive(Debug)]
struct Passport {
    byr: Option<String>, // (Birth Year)
    iyr: Option<String>, // (Issue Year)
    eyr: Option<String>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<String>, // (Country ID)
}

fn parse_passportlist(mut passports: Vec<Passport>, line: String) -> Result<Vec<Passport>, PassportParsingError> {
    if passports.len() == 0 || line.trim().is_empty() {
        passports.push(Passport { byr: None, iyr: None, eyr: None, hgt: None, hcl: None, ecl: None, pid: None, cid: None })
    }
    let pass = passports.last_mut().unwrap();
    for elem in line.split_whitespace() {
        let mut kvp = elem.split(":");
        let name = kvp.next().ok_or(PassportParsingError)?;
        let value = kvp.next().ok_or(PassportParsingError)?;
        if kvp.next() != None {
            return Err(PassportParsingError);
        }

        match name {
            "byr" => pass.byr = Some(String::from(value)),
            "iyr" => pass.iyr = Some(String::from(value)),
            "eyr" => pass.eyr = Some(String::from(value)),
            "hgt" => pass.hgt = Some(String::from(value)),
            "hcl" => pass.hcl = Some(String::from(value)),
            "ecl" => pass.ecl = Some(String::from(value)),
            "pid" => pass.pid = Some(String::from(value)),
            "cid" => pass.cid = Some(String::from(value)),
            _ => return Err(PassportParsingError),
        }
    }

    Ok(passports)        
}

fn is_valid(passport: &Passport) -> bool {
    passport.byr != None
        && passport.iyr != None
        && passport.eyr != None
        && passport.hgt != None
        && passport.hcl != None
        && passport.ecl != None
        && passport.pid != None
}

fn star_one(passports: &Vec<Passport>) -> usize {
    passports.iter().fold(
        0usize,
        |s, x| if is_valid(x) { s + 1 } else { s }
    )
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let mut last_line_empty = false;    
    let mut passports = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .try_fold(
            vec![],
            |s, x| {
                last_line_empty = x.trim().is_empty();
                parse_passportlist(s, x)
            },
        )
        .expect("Invalid data in input file");
    if last_line_empty {
        passports.pop();
    }        

    println!("Star 1:");
    let nr_valid = star_one(&passports);
    println!("Number of valid passport: {}", nr_valid);
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in

";

    #[test]
    fn test_star_one() {
        let mut last_line_empty = false;
        let mut passports = TEST_INPUT
            .lines()
            .map(|x| x.to_string())
            .try_fold(
                vec![],
                |s, x| {
                    last_line_empty = x.trim().is_empty();
                    super::parse_passportlist(s, x)
                },
            )
            .expect("Invalid test data");
        if last_line_empty {
            passports.pop();
        }

        assert_eq!(passports.len(), 4);
        assert_eq!(passports[0].byr, Some(String::from("1937")));

        let nr_valid = super::star_one(&passports);
        assert_eq!(nr_valid, 2);
    }
}
