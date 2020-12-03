use core::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug, PartialEq)]
struct IncorrectMapTileError;

type Processor<S, E> = fn(&str, S) -> Result<S, E>;

fn process_lines<S, E>(
    iter: impl Iterator<Item = std::string::String>,
    processor: Processor<S, E>,
    initial: S,
) -> Result<S, E> {
    let mut result = initial;
    for read in iter {
        result = processor(&read, result)?;
    }
    Ok(result)
}

fn process_lines_from_file<S, E>(
    file: &std::fs::File,
    processor: Processor<S, E>,
    initial: S,
) -> Result<S, E> {
    let iter = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"));
    process_lines(iter, processor, initial)
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

    Ok(map)
}

fn check_slope(map: &Map, incr_x: usize, incr_y: usize) -> u32 {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut tree_count: u32 = 0;

    while y < map.height {
        if map.tile_at(x, y) == MapTile::Tree {
            tree_count += 1;
        }
        x += incr_x;
        y += incr_y;
    }

    tree_count
}

fn star_one(map: &Map) -> u32 {
    check_slope(map, 3, 1)
}

fn star_two(map: &Map) -> u32 {
    check_slope(map, 1, 1)
        * check_slope(map, 3, 1)
        * check_slope(map, 5, 1)
        * check_slope(map, 7, 1)
        * check_slope(map, 1, 2)
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let map = process_lines_from_file(
        &file,
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

    println!("Star 2:");
    let answer = star_two(&map);
    println!("Number of trees in slopes, multiplied: {}", answer);
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
        let map = super::process_lines(
            TEST_MAP.lines().map(|x| x.to_string()),
            super::parse_map,
            Map {
                tiles: Vec::new(),
                width: 0,
                height: 0,
            },
        )
        .expect("Invalid test data");

        assert_eq!(map.width, 11);
        assert_eq!(map.height, 11);

        assert_eq!(map.tile_at(0, 0), MapTile::Empty);
        assert_eq!(map.tile_at(0, 1), MapTile::Tree);
        assert_eq!(map.tile_at(11, 12), MapTile::Tree);

        let nr_trees = super::star_one(&map);
        assert_eq!(nr_trees, 7);
    }

    #[test]
    fn test_star_two() {
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

        let nr_trees = super::star_two(&map);
        assert_eq!(nr_trees, 336);
    }
}
