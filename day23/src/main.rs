use std::vec::Vec;

fn _dump_state(cups: &Vec<usize>, move_nr: usize, cur_cup: usize) {
    println!("-- move {} --", move_nr);
    print!("cups:");
    for (i, c) in cups.iter().enumerate() {
        if i == cur_cup {
            print!(" ({})", c);
        } else {
            print!(" {}", c);
        }
    }
    println!();
}

fn _dump_pickup(pickup: &Vec<usize>) {
    print!("pick up:");
    for p in pickup {
        print!(" {}", p);
    }
    println!();
}

fn _dump_dest(dest: usize) {
    println!("destination: {}", dest);
}

fn play_game(cups: &mut Vec<usize>, nr_moves: usize) {
    let mut cur_cup: usize = 0;
    for _move_nr in 1..=nr_moves {
        // Debugging
        // _dump_state(cups, _move_nr, cur_cup);

        // Pick up the next three cups
        let cur_label = cups[cur_cup];
        let mut picked_up: Vec<usize> = vec![];
        for _ in 0..3 {
            let pickup_idx = (cups.iter().position(|&x| x == cur_label).unwrap() + 1) % cups.len();
            picked_up.push(cups.remove(pickup_idx));
        }
        // _dump_pickup(&picked_up);

        // Select destination cup
        let mut dest_label = cur_label - 1;
        while picked_up.contains(&dest_label) {
            dest_label -= 1;
        }

        if dest_label < *cups.iter().min().unwrap() {
            dest_label = *cups.iter().chain(picked_up.iter()).max().unwrap();
            while picked_up.contains(&dest_label) {
                dest_label -= 1;
            }
        }
        // _dump_dest(dest_label);

        // Insert cups after destination cup
        let dest_index = (cups.iter().position(|&x| x == dest_label).unwrap() + 1) % cups.len();
        for p in 0..picked_up.len() {
            cups.insert(dest_index, picked_up[picked_up.len() - 1 - p]);
        }

        cur_cup = (cups.iter().position(|&x| x == cur_label).unwrap() + 1) % cups.len();
    }

    // println!("-- final --\ncups: {:?}", cups);
}

fn score(cups: &Vec<usize>) -> usize {
    let first_index = cups.iter().position(|&x| x == 1).unwrap();
    let mut result: usize = 0;
    let max: u32 = cups.len() as u32;
    for i in 1..max {
        let label = cups[(first_index + i as usize) % cups.len()];
        result += label * 10usize.pow(max-i-1);
    }

    result
}

fn star_one(cups: &Vec<usize>) -> usize {
    let mut gamecups = cups.clone();
    play_game(&mut gamecups, 100);
    score(&gamecups)
}

fn main() {
    // Input provided as single string
    let cups: Vec<usize> = vec![9, 5, 2, 4, 3, 8, 7, 1, 6];

    let ans = star_one(&cups);
    println!("Star one: {}", ans);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_star_one() {
        // Input provided as single string
        let test_cups: Vec<usize> = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];

        let mut cups = test_cups.clone();
        super::play_game(&mut cups, 10);
        let ans = super::score(&cups);
        assert_eq!(ans, 92658374);

        let mut cups = test_cups.clone();
        super::play_game(&mut cups, 100);
        let ans = super::score(&cups);
        assert_eq!(ans, 67384529);
    }
}
