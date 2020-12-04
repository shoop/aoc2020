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

fn is_parseable_year_between(elem: &str, lbound: usize, ubound: usize) -> bool {
    if elem.chars().count() != 4 || !elem.chars().all(char::is_numeric) {
        return false;
    }
    let elem_parsed = elem.parse::<usize>();
    if elem_parsed.is_err() {
        return false;
    }
    let elem_parsed = elem_parsed.unwrap();
    if elem_parsed < lbound || elem_parsed > ubound {
        return false;
    }

    true
}

impl Passport {
    fn is_somewhat_valid(&self) -> bool {
        self.byr != None
            && self.iyr != None
            && self.eyr != None
            && self.hgt != None
            && self.hcl != None
            && self.ecl != None
            && self.pid != None
    }

    fn is_somewhat_more_valid(&self) -> bool {
        if !self.is_somewhat_valid() {
            return false;
        }

        // byr
        if !is_parseable_year_between(self.byr.as_ref().unwrap(), 1920, 2002) {
            return false;
        }

        // iyr
        if !is_parseable_year_between(self.iyr.as_ref().unwrap(), 2010, 2020) {
            return false;
        }

        // eyr
        if !is_parseable_year_between(self.eyr.as_ref().unwrap(), 2020, 2030) {
            return false;
        }

        // hgt
        let hgt = self.hgt.as_ref().unwrap();
        let hgt_nr: &str;
        let hgt_lbound: usize;
        let hgt_ubound: usize;
        if hgt.ends_with("cm") {
            hgt_nr = &hgt[..hgt.len()-2];
            hgt_lbound = 150;
            hgt_ubound = 193;
        } else if hgt.ends_with("in") {
            hgt_nr = &hgt[..hgt.len()-2];
            hgt_lbound = 59;
            hgt_ubound = 76;
        } else {
            return false;
        }
        let hgt_parsed = hgt_nr.parse::<usize>();
        if hgt_parsed.is_err() {
            return false;
        }
        let hgt_parsed = hgt_parsed.unwrap();
        if hgt_parsed < hgt_lbound || hgt_parsed > hgt_ubound {
            return false;
        }

        // hcl
        let hcl = self.hcl.as_ref().unwrap();
        if hcl.chars().count() != 7 {
            return false;
        }
        if hcl.chars().next() != Some('#') {
            return false;
        }
        if !(hcl[1..6].chars().all(|x| char::is_ascii_hexdigit(&x))) {
            return false;
        }

        // ecl
        let ecl = self.ecl.as_ref().unwrap();
        match &ecl[..] {
            "amb" => (),
            "blu" => (),
            "brn" => (),
            "gry" => (),
            "grn" => (),
            "hzl" => (),
            "oth" => (),
            _ => return false,
        }

        // pid
        let pid = self.pid.as_ref().unwrap();
        if pid.chars().count() != 9 {
            return false;
        }
        if !(pid.chars().all(|x| char::is_ascii_digit(&x))) {
            return false;
        }

        true
    }
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

fn star_one(passports: &Vec<Passport>) -> usize {
    passports.iter().fold(
        0usize,
        |s, x| if x.is_somewhat_valid() { s + 1 } else { s }
    )
}

fn star_two(passports: &Vec<Passport>) -> usize {
    passports.iter().fold(
        0usize,
        |s, x| if x.is_somewhat_more_valid() { s + 1 } else { s }
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

    println!("Star 2:");
    let nr_valid = star_two(&passports);
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

    static TEST_INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    static TEST_VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

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

    #[test]
    fn test_star_two() {
        let mut last_line_empty = false;
        let mut passports = TEST_INVALID
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
        assert_eq!(passports[0].byr, Some(String::from("1926")));

        let nr_valid = super::star_two(&passports);
        assert_eq!(nr_valid, 0);

        let mut passports = TEST_VALID
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
        assert_eq!(passports[0].byr, Some(String::from("1980")));

        let nr_valid = super::star_two(&passports);
        assert_eq!(nr_valid, 4);
    }
}
