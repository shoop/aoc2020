use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn parse_lines(lines: Vec<String>) -> (Vec<usize>, Vec<usize>) {
    let mut deck_1: Vec<usize> = vec![];
    let mut deck_2: Vec<usize> = vec![];
    let mut cur_deck = &mut deck_1;

    for line in lines.iter() {
        if line.starts_with("Player 1") {
            continue;
        }
        if line.starts_with("Player 2") {
            cur_deck = &mut deck_2;
            continue;
        }
        if line.is_empty() {
            continue;
        }

        let val = line.parse::<usize>().unwrap();
        cur_deck.push(val);
    }

    (deck_1, deck_2)
}

fn star_one(_deck1: &Vec<usize>, _deck2: &Vec<usize>) -> usize {
    let mut p1deck = _deck1.clone();
    let mut p2deck = _deck2.clone();

    while p1deck.len() > 0 && p2deck.len() > 0 {
        let p1draw = p1deck.remove(0);
        let p2draw = p2deck.remove(0);

        if p1draw > p2draw {
            p1deck.push(p1draw);
            p1deck.push(p2draw);
        } else if p1draw == p2draw {
            panic!();
        } else {
            p2deck.push(p2draw);
            p2deck.push(p1draw);
        }
    }

    let mut winner_deck = &p1deck;
    if p1deck.len() == 0 {
        winner_deck = &p2deck;
    }

    let mut result = 0;
    for (idx, val) in winner_deck.iter().rev().enumerate() {
        result += (idx + 1) * val;
    }

    result
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .collect();
    let (deck1, deck2) = parse_lines(lines);

    let ans = star_one(&deck1, &deck2);
    println!("Star one: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();
        let (deck1, deck2) = super::parse_lines(lines);
        println!("deck 1 {:?} deck 2 {:?}", deck1, deck2);

        let ans = super::star_one(&deck1, &deck2);
        assert_eq!(ans, 306);
    }
}
