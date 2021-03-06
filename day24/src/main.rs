use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug, PartialEq, Copy, Clone)]
enum TileColor {
    Black,
    White,
}

fn _print_floor(floor: &HashMap<(isize, isize), TileColor>) {
    let min_x = floor.iter().map(|(pos, _)| pos.0).min().unwrap();
    let max_x = floor.iter().map(|(pos, _)| pos.0).max().unwrap();
    let min_y = floor.iter().map(|(pos, _)| pos.1).min().unwrap();
    let max_y = floor.iter().map(|(pos, _)| pos.1).max().unwrap();

    println!("Floor from Y {}-{} X {}-{}", min_y, max_y, min_x, max_x);
    for y in min_y..=max_y {
        if y == min_y {
            print!("        ");
            for x in min_x..=max_x {
                print!("{:3} ", x);
            }
            println!();
        }
        print!("Y: {:3} ", y);
        if y.abs() % 2 == 1 {
            print!(" ");
        }
        for x in min_x..=max_x {
            if let Some(col) = floor.get(&(x, y)) {
                print!(
                    "{}",
                    match col {
                        TileColor::Black => " ##",
                        TileColor::White => " ..",
                    }
                );
            } else {
                print!("   ");
            }
        }
        println!();
    }
}

fn make_floor(lines: &Vec<String>) -> HashMap<(isize, isize), TileColor> {
    let mut floor: HashMap<(isize, isize), TileColor> = HashMap::new();

    for line in lines.iter() {
        let mut cur_pos: (isize, isize) = (0, 0);
        let mut diriter = line.chars();
        loop {
            match diriter.next() {
                Some('e') => cur_pos = (cur_pos.0 + 1, cur_pos.1),
                Some('w') => cur_pos = (cur_pos.0 - 1, cur_pos.1),
                Some('n') => match diriter.next() {
                    Some('e') => cur_pos = (cur_pos.0 + 1, cur_pos.1 - 1),
                    Some('w') => cur_pos = (cur_pos.0, cur_pos.1 - 1),
                    None | Some(_) => panic!(),
                },
                Some('s') => match diriter.next() {
                    Some('e') => cur_pos = (cur_pos.0, cur_pos.1 + 1),
                    Some('w') => cur_pos = (cur_pos.0 - 1, cur_pos.1 + 1),
                    None | Some(_) => panic!(),
                },
                None => {
                    let tile = floor.entry(cur_pos).or_insert(TileColor::White);
                    *tile = if *tile == TileColor::White {
                        TileColor::Black
                    } else {
                        TileColor::White
                    };
                    break;
                },
                Some(_) => panic!(),
            }
        }
    }

    floor
}

fn star_one(lines: &Vec<String>) -> isize {
    let floor = make_floor(lines);
    // _print_floor(&floor);

    floor
        .iter()
        .filter(|(_, col)| col == &&TileColor::Black)
        .count() as isize
}

fn star_two(lines: &Vec<String>) -> isize {
    let neighbours: Vec<(isize, isize)> = vec![(0,-1), (1,-1), (1, 0), (0, 1), (-1, 1), (-1, 0)];
    let mut floor = make_floor(lines);

    for _day in 1..=100 {
        let check = floor.clone();
        let min_x = floor.iter().map(|(pos, _)| pos.0).min().unwrap();
        let max_x = floor.iter().map(|(pos, _)| pos.0).max().unwrap();
        let min_y = floor.iter().map(|(pos, _)| pos.1).min().unwrap();
        let max_y = floor.iter().map(|(pos, _)| pos.1).max().unwrap();

        for y in min_y-1..=max_y+1 {
            for x in min_x-1..=max_x+1 {
                let black_neighbours: isize = neighbours.iter().fold(0, |s, n| {
                    s + if let Some(TileColor::Black) = check.get(&(x+n.0,y+n.1)) {
                        1
                    } else {
                        0
                    }
                });
                match check.get(&(x, y)) {
                    Some(TileColor::White) | None => {
                        if black_neighbours == 2 {
                            *floor.entry((x, y)).or_insert(TileColor::White) = TileColor::Black;
                        }
                    },
                    Some(TileColor::Black) => {
                        if black_neighbours == 0 || black_neighbours > 2 {
                            floor.remove(&(x, y));
                        }
                    },
                }
            }
        }

        println!("Day {}: {}", _day, floor
            .iter()
            .filter(|(_, col)| col == &&TileColor::Black)
            .count() as isize);
    }

    floor
        .iter()
        .filter(|(_, col)| col == &&TileColor::Black)
        .count() as isize
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .collect();

    let ans = star_one(&lines);
    println!("Star one: {}", ans);

    let ans = star_two(&lines);
    println!("Star two: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    static SIMPLE_TEST_DATA: &str = "nwwswee";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = SIMPLE_TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 1);

        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 10);
    }

    #[test]
    fn test_star_two() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_two(&lines);
        assert_eq!(ans, 2208);
    }
}
