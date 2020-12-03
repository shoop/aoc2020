use core::fmt::Debug;
use std::convert::TryFrom;
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

#[derive(Debug)]
enum MapTile {
    Empty,
    Tree,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<MapTile>,
    width: u32,
}

fn parse_map(line: &str, mut state: Map) -> Result<Map, IncorrectMapTileError> {
    for c in line.chars() {
        if c == '.' {
            state.tiles.push(MapTile::Empty);
        } else if c == '#' {
            state.tiles.push(MapTile::Tree);
        } else {
            return Err(IncorrectMapTileError);
        }
    }
    if state.width == 0 {
        state.width = u32::try_from(line.chars().count()).unwrap();
    }

    return Ok(state);
}

fn star_one(_map: &Map) -> u32 {
    1
}

fn main() {
    let map = process_lines(
        "./input",
        parse_map,
        Map {
            tiles: Vec::new(),
            width: 0,
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
        };
        for line in TEST_MAP.lines() {
            map = super::parse_map(line, map).expect("Invalid test data");
        }

        let nr_trees = super::star_one(&map);
        assert_eq!(nr_trees, 7);
    }
}
