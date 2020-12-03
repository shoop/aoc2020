use core::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug)]
struct IncorrectMapTileError;

#[derive(Debug, Clone, Copy, PartialEq)]
enum MapTile {
    Empty,
    Tree,
}

struct Map {
    tiles: Vec<MapTile>,
    width: usize,
    height: usize,
}

impl Map {
    fn tile_at(&self, x: usize, y: usize) -> MapTile {
        self.tiles[((y % self.height) * self.width) + (x % self.width)]
    }

    fn slope(&self, incr_x: usize, incr_y: usize) -> Slope<'_> {
        Slope {
            map: &self,
            incr_x,
            incr_y,
        }
    }
}

struct Slope<'a> {
    map: &'a Map,
    incr_x: usize,
    incr_y: usize,
}

impl<'a> IntoIterator for Slope<'a> {
    type Item = MapTile;
    type IntoIter = SlopeIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SlopeIntoIterator {
            slope: self,
            cur_x: 0,
            cur_y: 0,
        }
    }
}

struct SlopeIntoIterator<'a> {
    slope: Slope<'a>,
    cur_x: usize,
    cur_y: usize,
}

impl Iterator for SlopeIntoIterator<'_> {
    type Item = MapTile;

    fn next(&mut self) -> Option<MapTile> {
        if self.cur_y > self.slope.map.height {
            return None;
        }

        let result = self.slope.map.tile_at(self.cur_x, self.cur_y);
        self.cur_x += self.slope.incr_x;
        self.cur_y += self.slope.incr_y;

        Some(result)
    }
}

fn parse_map(mut map: Map, line: String) -> Result<Map, IncorrectMapTileError> {
    for c in line.chars() {
        match c {
            '.' => map.tiles.push(MapTile::Empty),
            '#' => map.tiles.push(MapTile::Tree),
            _ => return Err(IncorrectMapTileError),
        }
    }

    if map.width == 0 {
        map.width = line.chars().count();
    }

    map.height += 1;

    Ok(map)
}

fn check_slope<'a>(slope: Slope<'a>) -> u32 {
    slope.into_iter().fold(0u32, |s, x| match x {
        MapTile::Tree => s + 1,
        MapTile::Empty => s,
    })
}

fn star_one(map: &Map) -> u32 {
    check_slope(map.slope(3, 1))
}

fn star_two(map: &Map) -> u32 {
    println!("slope(1,1) : {}", check_slope(map.slope(1, 1)));
    println!("slope(3,1) : {}", check_slope(map.slope(3, 1)));
    println!("slope(5,1) : {}", check_slope(map.slope(5, 1)));
    println!("slope(7,1) : {}", check_slope(map.slope(7, 1)));
    println!("slope(1,2) : {}", check_slope(map.slope(1, 2)));
    check_slope(map.slope(1, 1))
        * check_slope(map.slope(3, 1))
        * check_slope(map.slope(5, 1))
        * check_slope(map.slope(7, 1))
        * check_slope(map.slope(1, 2))

    // let slopes = vec![
    //     map.slope(1, 1),
    //     map.slope(3, 1),
    //     map.slope(5, 1),
    //     map.slope(7, 1),
    //     map.slope(1, 2),
    // ];
    // slopes.into_iter().fold(1, |s, x| s * check_slope(x))
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let map = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .try_fold(
            Map {
                tiles: Vec::new(),
                width: 0,
                height: 0,
            },
            parse_map,
        )
        .expect("Invalid data in input file");

    println!("Star 1:");
    let nr_trees = star_one(&map);
    println!("Number of trees: {}", nr_trees);

    println!("Star 2:");
    let answer = star_two(&map);
    println!("Number of trees in slopes, multiplied: {}", answer);
}

#[cfg(test)]
mod tests {
    static TEST_MAP: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_star_one() {
        let map = TEST_MAP
            .lines()
            .map(|x| x.to_string())
            .try_fold(
                super::Map {
                    tiles: Vec::new(),
                    width: 0,
                    height: 0,
                },
                super::parse_map,
            )
            .expect("Invalid test data");

        assert_eq!(map.width, 11);
        assert_eq!(map.height, 11);

        assert_eq!(map.tile_at(0, 0), super::MapTile::Empty);
        assert_eq!(map.tile_at(0, 1), super::MapTile::Tree);
        assert_eq!(map.tile_at(11, 12), super::MapTile::Tree);

        let nr_trees = super::star_one(&map);
        assert_eq!(nr_trees, 7);
    }

    #[test]
    fn test_star_two() {
        let map = TEST_MAP
            .lines()
            .map(|x| x.to_string())
            .try_fold(
                super::Map {
                    tiles: Vec::new(),
                    width: 0,
                    height: 0,
                },
                super::parse_map,
            )
            .expect("Invalid test data");

        assert_eq!(map.width, 11);
        assert_eq!(map.height, 11);

        assert_eq!(map.tile_at(0, 0), super::MapTile::Empty);
        assert_eq!(map.tile_at(0, 1), super::MapTile::Tree);
        assert_eq!(map.tile_at(11, 12), super::MapTile::Tree);

        let nr_trees = super::star_two(&map);
        assert_eq!(nr_trees, 336);
    }
}
