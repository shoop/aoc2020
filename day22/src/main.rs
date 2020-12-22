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

fn star_one(deck1: &Vec<usize>, deck2: &Vec<usize>) -> usize {
    let mut p1deck = deck1.clone();
    let mut p2deck = deck2.clone();

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

fn play_game(game_nr: &mut usize, deck1: &Vec<usize>, deck2: &Vec<usize>) -> (usize, usize) {
    let mut p1deck = deck1.clone();
    let mut p2deck = deck2.clone();
    let mut prev_rounds: Vec<(Vec<usize>, Vec<usize>)> = vec![];
    let this_game_nr = game_nr.clone();
    let mut round_nr = 1;

    println!("=== Game {} ===", this_game_nr);

    while p1deck.len() > 0 && p2deck.len() > 0 {
        println!("-- Round {} (Game {}) --", round_nr, this_game_nr);
        println!("Player 1's deck: {:?}", p1deck);
        println!("Player 2's deck: {:?}", p2deck);

        // Check on previous rounds
        for round in prev_rounds.iter() {
            if round.0 == p1deck && round.1 == p2deck {
                println!("Infinite recursion detected, P1 wins by default");
                let mut result = 0;
                for (idx, val) in p1deck.iter().rev().enumerate() {
                    result += (idx + 1) * val;
                }
                return (1, result);
            }
        }

        // Add current round to previous
        prev_rounds.push((p1deck.clone(), p2deck.clone()));

        let p1draw = p1deck.remove(0);
        let p2draw = p2deck.remove(0);

        println!("Player 1 plays: {}", p1draw);
        println!("Player 2 plays: {}", p2draw);

        if p1deck.len() >= p1draw && p2deck.len() >= p2draw {
            println!("Playing a sub-game to determine the winner...");

            // Play sub game with only the # cards as determined by the draw
            let subdeck1 = p1deck[..p1draw].to_vec().clone();
            let subdeck2 = p2deck[..p2draw].to_vec().clone();
            *game_nr += 1;
            
            let (winner, _) = play_game(game_nr, &subdeck1, &subdeck2);
            println!("...anyway, back to game {}.", this_game_nr);
            if winner == 1 {
                println!("Player 1 wins round {} of game {}!", round_nr, this_game_nr);
                p1deck.push(p1draw);
                p1deck.push(p2draw);
            } else {
                println!("Player 2 wins round {} of game {}!", round_nr, this_game_nr);
                p2deck.push(p2draw);
                p2deck.push(p1draw);
            }
        } else if p1draw > p2draw {
            println!("Player 1 wins round {} of game {}!", round_nr, this_game_nr);
            p1deck.push(p1draw);
            p1deck.push(p2draw);
        } else if p1draw == p2draw {
            panic!();
        } else {
            println!("Player 2 wins round {} of game {}!", round_nr, this_game_nr);
            p2deck.push(p2draw);
            p2deck.push(p1draw);
        }

        round_nr += 1;
    }

    if this_game_nr == 1 {
        println!("== Post-game results ==");
        println!("Player 1's deck: {:?}", p1deck);
        println!("Player 2's deck: {:?}", p2deck);
    }

    let mut winner: usize = 1;
    let mut winner_deck = &p1deck;
    if p1deck.len() == 0 {
        winner = 2;
        winner_deck = &p2deck;
    }

    if this_game_nr > 1 {
        println!("The winner of game {} is player {}!", this_game_nr, winner);
    }

    let mut result = 0;
    for (idx, val) in winner_deck.iter().rev().enumerate() {
        result += (idx + 1) * val;
    }

    (winner, result)
}

fn star_two(deck1: &Vec<usize>, deck2: &Vec<usize>) -> usize {
    let mut game_nr = 1;
    play_game(&mut game_nr, deck1, deck2).1
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

    let ans = star_two(&deck1, &deck2);
    println!("Star two: {}", ans);
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

        let ans = super::star_one(&deck1, &deck2);
        assert_eq!(ans, 306);
    }

    static TEST_DATA_INF: &str = "Player 1:
43
19

Player 2:
2
29
14";

    #[test]
    fn test_star_two() {
        let lines: Vec<String> = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .collect();
        let (deck1, deck2) = super::parse_lines(lines);

        let ans = super::star_two(&deck1, &deck2);
        assert_eq!(ans, 291);

        let lines: Vec<String> = TEST_DATA_INF
            .lines()
            .map(|x| x.to_string())
            .collect();
        let (deck1, deck2) = super::parse_lines(lines);

        let ans = super::star_two(&deck1, &deck2);
        assert_eq!(ans, 105);
    }
}
