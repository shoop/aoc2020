use std::fs::File;
use std::fmt;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug, Copy, Clone, PartialEq)]
enum TileState {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug, Clone)]
struct Ferry {
    map: Vec<Vec<TileState>>,
}

impl Ferry {
    fn new() -> Self {
        Ferry { map: vec![] }
    }

    fn add_layout_from_line(&mut self, line: &str) {
        let row: Vec<TileState> = line.chars().map(|ch| match ch {
            '.' => TileState::Floor,
            'L' => TileState::Empty,
            '#' => TileState::Occupied,
            _ => panic!(),
        }).collect();
        self.map.push(row);
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<TileState> {
        let mut result: Vec<TileState> = vec![];

        // TODO: probably more efficient to extend the map beforehand. Oh well, this works.
        if y > 0 && x > 0 {
            result.push(self.map[y - 1][x - 1]);
        } else {
            result.push(TileState::Floor);
        }
        if y > 0 && x < self.map[y].len() {
            result.push(self.map[y - 1][x]);
        } else {
            result.push(TileState::Floor);
        }
        if y > 0 && x < self.map[y].len() - 1 {
            result.push(self.map[y - 1][x + 1]);
        } else {
            result.push(TileState::Floor);
        }
        if y < self.map.len() && x > 0 {
            result.push(self.map[y][x - 1]);
        } else {
            result.push(TileState::Floor);
        }
        if y < self.map.len() && x < self.map[y].len() - 1 {
            result.push(self.map[y][x + 1]);
        } else {
            result.push(TileState::Floor);
        }
        if y < self.map.len() - 1 && x > 0 {
            result.push(self.map[y + 1][x - 1]);
        } else {
            result.push(TileState::Floor);
        }
        if y < self.map.len() - 1 && x < self.map[y].len() {
            result.push(self.map[y + 1][x]);
        } else {
            result.push(TileState::Floor);
        }
        if y < self.map.len() - 1 && x < self.map[y].len() - 1 {
            result.push(self.map[y + 1][x + 1]);
        } else {
            result.push(TileState::Floor);
        }

        result
    }

    fn shuffle_people(&mut self) -> usize {
        let mut people_changed: usize = 0;
        let mut new_state = self.clone();

        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                match self.map[y][x] {
                    TileState::Floor => {},
                    TileState::Empty => {
                        if self.get_neighbours(x, y).iter().fold(true, |s, t| match t {
                            TileState::Occupied => false,
                            _ => s & true,
                        }) {
                            new_state.map[y][x] = TileState::Occupied;
                            people_changed += 1;
                        };
                    },
                    TileState::Occupied => {
                        if self.get_neighbours(x, y).iter().filter(|t| t == &&TileState::Occupied).count() >= 4 {
                            new_state.map[y][x] = TileState::Empty;
                            people_changed += 1;
                        }
                    }
                }
            }
        }

        self.map = new_state.map;

        people_changed
    }

    fn walk_line_until_seat(&self, start: (isize, isize), walk: &dyn Fn(isize, isize) -> (isize, isize)) -> Option<(usize, usize)> {
        let (mut cury, mut curx) = start;
        let maxy = self.map.len() as isize;
        let maxx = self.map[0].len() as isize;
        while cury >= 0 && curx >= 0 && cury < maxy && curx < maxx {
            if self.map[cury as usize][curx as usize] != TileState::Floor {
                return Some((cury as usize, curx as usize));
            }

            // Interesting, cannot destructure on assigment, https://github.com/rust-lang/rfcs/issues/372
            let result = walk(cury, curx);
            cury = result.0;
            curx = result.1;
        }

        None
    }

    fn get_first_visible_seats(&self, x: isize, y: isize) -> Vec<TileState> {
        let mut result: Vec<TileState> = vec![];

        result.push(match self.walk_line_until_seat((y - 1, x - 1), &|cury, curx| (cury - 1, curx - 1)) {
            Some((y, x)) => self.map[y][x],
            None => TileState::Floor,
        });

        result.push(match self.walk_line_until_seat((y - 1, x    ), &|cury, curx| (cury - 1, curx    )) {
            Some((y, x)) => self.map[y][x],
            None => TileState::Floor,
        });

        result.push(match self.walk_line_until_seat((y - 1, x + 1), &|cury, curx| (cury - 1, curx + 1)) {
            Some((y, x)) => self.map[y][x],
            None => TileState::Floor,
        });

        result.push(match self.walk_line_until_seat((y, x - 1    ), &|cury, curx| (cury    , curx - 1)) {
            Some((y, x)) => self.map[y][x],
            None => TileState::Floor,
        });

        result.push(match self.walk_line_until_seat((y, x + 1    ), &|cury, curx| (cury    , curx + 1)) {
            Some((y, x)) => self.map[y][x],
            None => TileState::Floor,
        });

        result.push(match self.walk_line_until_seat((y + 1, x - 1), &|cury, curx| (cury + 1, curx - 1)) {
            Some((y, x)) => self.map[y][x],
            None => TileState::Floor,
        });

        result.push(match self.walk_line_until_seat((y + 1, x    ), &|cury, curx| (cury + 1, curx    )) {
            Some((y, x)) => self.map[y][x],
            None => TileState::Floor,
        });

        result.push(match self.walk_line_until_seat((y + 1, x + 1), &|cury, curx| (cury + 1, curx + 1)) {
            Some((y, x)) => self.map[y][x],
            None => TileState::Floor,
        });

        result
    }

    fn shuffle_people_advanced(&mut self) -> usize {
        let mut people_changed: usize = 0;
        let mut new_state = self.clone();

        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                match self.map[y][x] {
                    TileState::Floor => {},
                    TileState::Empty => {
                        if self.get_first_visible_seats(x as isize, y as isize) .iter().fold(true, |s, t| match t {
                            TileState::Occupied => false,
                            _ => s & true,
                        }) {
                            new_state.map[y][x] = TileState::Occupied;
                            people_changed += 1;
                        };
                    },
                    TileState::Occupied => {
                        if self.get_first_visible_seats(x as isize, y as isize).iter().filter(|t| t == &&TileState::Occupied).count() >= 5 {
                            new_state.map[y][x] = TileState::Empty;
                            people_changed += 1;
                        }
                    }
                }
            }
        }

        self.map = new_state.map;

        people_changed
    }

    fn count_occupied_seats(&self) -> usize {
        self.map.iter().fold(0, |sum, row| sum + row.iter().fold(0, |sum, tile| if tile == &TileState::Occupied { sum + 1} else { sum }))
    }
}

impl fmt::Display for Ferry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.map {
            for tile in row {
                write!(f, "{}", match tile {
                    TileState::Floor => '.',
                    TileState::Empty => 'L',
                    TileState::Occupied => '#',
                })?;
            }
            writeln!(f)?;
        }

        fmt::Result::Ok(())
    }
}

fn star_one(ferry: &mut Ferry) -> usize {
    loop {
        let people_changed = ferry.shuffle_people();
        if people_changed == 0 {
            return ferry.count_occupied_seats();
        }
    }
}

fn star_two(ferry: &mut Ferry) -> usize {
    loop {
        let people_changed = ferry.shuffle_people_advanced();
        if people_changed == 0 {
            return ferry.count_occupied_seats();
        }
    }
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let mut ferry_star_one = Ferry::new();
    for line in io::BufReader::new(file).lines().map(|x| x.expect("Could not read line")) {
        ferry_star_one.add_layout_from_line(&line);
    }

    let mut ferry_star_two = ferry_star_one.clone();

    let ans = star_one(&mut ferry_star_one);
    println!("Star one: {}", ans);

    let ans = star_two(&mut ferry_star_two);
    println!("Star two: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_star_one() {
        let mut ferry = super::Ferry::new();
        for line in TEST_DATA.lines().map(|x| String::from(x)) {
            ferry.add_layout_from_line(&line);
        }

        let ans = super::star_one(&mut ferry);
        assert_eq!(ans, 37);
    }

    #[test]
    fn test_star_two() {
        let mut ferry = super::Ferry::new();
        for line in TEST_DATA.lines().map(|x| String::from(x)) {
            ferry.add_layout_from_line(&line);
        }

        let ans = super::star_two(&mut ferry);
        assert_eq!(ans, 26);
    }
}
