use core::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

#[derive(Debug, PartialEq)]
struct IncorrectMapTileError;

fn process_lines<P, Q, R, S>(filename: P, processor: Q, initial: R) -> Result<R, io::Error>
where
    P: AsRef<Path>,
    Q: Fn(&str, R) -> Result<R, S>,
    S: Debug,
{
    let file = File::open(filename)?;
    let mut result: R = initial;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(read) = line {
            result = processor(&read, result).expect("Invalid input");
        }
    }
    Ok(result)
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum MapTile {
    Empty,
    Tree,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<MapTile>,
    width: usize,
    height: usize,
}

impl Map {
    fn tile_at(&self, x: usize, y: usize) -> MapTile {
        self.tiles[((y % self.height) * self.width) + (x % self.width)]
    }
}

fn parse_map(line: &str, mut map: Map) -> Result<Map, IncorrectMapTileError> {
    for c in line.chars() {
        if c == '.' {
            map.tiles.push(MapTile::Empty);
        } else if c == '#' {
            map.tiles.push(MapTile::Tree);
        } else {
            return Err(IncorrectMapTileError);
        }
    }
    if map.width == 0 {
        map.width = line.chars().count();
    }
    map.height = map.height + 1;

    return Ok(map);
}

fn star_one(map: &Map) -> u32 {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut tree_count: u32 = 0;
    while y < map.height {
        if map.tile_at(x, y) == MapTile::Tree {
            tree_count += 1;
        }
        x += 3;
        y += 1;
    }
    tree_count
}

fn main() {
    let map = process_lines(
        "./input",
        parse_map,
        Map {
            tiles: Vec::new(),
            width: 0,
            height: 0,
        },
    )
    .expect("Could not parse map");

    println!("Star 1:");
    let nr_trees = star_one(&map);
    println!("Number of trees: {}", nr_trees);
}

#[cfg(test)]
mod tests {
    use super::Map;
    use super::MapTile;

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
        let mut map = Map {
            tiles: Vec::new(),
            width: 0,
            height: 0,
        };
        for line in TEST_MAP.lines() {
            map = super::parse_map(line, map).expect("Invalid test data");
        }
        assert_eq!(map.width, 11);
        assert_eq!(map.height, 11);

        assert_eq!(map.tile_at(0, 0), MapTile::Empty);
        assert_eq!(map.tile_at(0, 1), MapTile::Tree);
        assert_eq!(map.tile_at(11, 12), MapTile::Tree);

        let nr_trees = super::star_one(&map);
        assert_eq!(nr_trees, 7);
    }
}
