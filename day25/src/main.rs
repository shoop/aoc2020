use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn calc_ek_using_modular_pow(b: usize, e: usize) -> usize {
    let modulus = 20201227;
    let mut c: usize = 1;
    for _ in 0..e {
        c = (c * b) % modulus
    }

    c
}

fn calc_e_using_modular_pow(key: usize) -> usize {
    let modulus = 20201227;
    let mut c: usize = 1;
    let mut e: usize = 0;
    while c != key {
        c = (c * 7) % modulus;
        e += 1
    }

    e
}

fn star_one(pubkeys: Vec<usize>) -> usize {
    // pubkey_card = 7^loopsize % 2020227
    let card_pubkey = pubkeys[0];
    let door_pubkey = pubkeys[1];
    let card_loopsize = calc_e_using_modular_pow(card_pubkey);
    let door_loopsize = calc_e_using_modular_pow(door_pubkey);
    let card_privkey = calc_ek_using_modular_pow(door_pubkey, card_loopsize);
    let door_privkey = calc_ek_using_modular_pow(card_pubkey, door_loopsize);
    assert_eq!(card_privkey, door_privkey);
    println!("Card: pubkey {} loopsize {} privkey {}", card_pubkey, card_loopsize, card_privkey);
    println!("Door: pubkey {} loopsize {} privkey {}", door_pubkey, door_loopsize, door_privkey);

    card_privkey
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let pubkeys: Vec<usize> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line").parse::<usize>().expect("Invalid input data"))
        .collect();

    let ans = star_one(pubkeys);
    println!("Star one: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "5764801
17807724";

    #[test]
    fn test_star_one() {
        let pubkeys: Vec<usize> = TEST_DATA
            .lines()
            .map(|x| x.parse::<usize>().expect("Invalid test data"))
            .collect();

        let ans = super::star_one(pubkeys);
        assert_eq!(ans, 14897079);
    }
}
