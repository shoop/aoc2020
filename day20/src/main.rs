use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Peekable;
use std::vec::Vec;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Pixel {
    Off = 0,
    On = 1,
    DontCare = 2,
    Monster = 3,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Symmetry {
    Identity0,
    Rotate090,
    Rotate180,
    Rotate270,
    FlipNSIdn,
    FlipNS090,
    FlipNS180,
    FlipNS270,
}

#[derive(Debug, Clone)]
struct Tile {
    id: usize, // Better Option<usize> but too many edge cases for this assignment
    image: Vec<Vec<Pixel>>,
    orientation: Symmetry,
    borders: HashMap<Symmetry, HashMap<Direction, usize>>,
}

#[derive(Debug)]
struct ParseError;

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tile {} ({:?}):", self.id, self.orientation)?;
        for y in self.image.iter() {
            writeln!(
                f,
                "{}",
                y.iter()
                    .map(|x| match x {
                        Pixel::On => "#",
                        Pixel::Off => ".",
                        Pixel::DontCare => "/",
                        Pixel::Monster => "O",
                    })
                    .collect::<Vec<&str>>()
                    .join("")
            )?;
        }
        writeln!(f)

        // let symmetries = vec![
        //     Symmetry::Identity0,
        //     Symmetry::Rotate090,
        //     Symmetry::Rotate180,
        //     Symmetry::Rotate270,
        //     Symmetry::FlipNSIdn,
        //     Symmetry::FlipNS090,
        //     Symmetry::FlipNS180,
        //     Symmetry::FlipNS270,
        // ];
        // for sym in symmetries {
        //     if let Some(br) = self.borders.get(&sym) {
        //         writeln!(
        //             f,
        //             "{:25?} N {:4} E {:4} S {:4} W {:4}",
        //             sym,
        //             br.get(&Direction::North).unwrap(),
        //             br.get(&Direction::East).unwrap(),
        //             br.get(&Direction::South).unwrap(),
        //             br.get(&Direction::West).unwrap()
        //         )?;
        //     }
        // }

        // writeln!(f)
    }
}

impl Tile {
    fn new() -> Self {
        Tile {
            id: 0,
            image: vec![],
            orientation: Symmetry::Identity0,
            borders: HashMap::new(),
        }
    }

    fn calculate_borders(&self) -> Result<HashMap<Direction, usize>, ParseError> {
        // Deliberately fixed to 10 positions
        if self.image.len() != 10 {
            return Err(ParseError);
        }

        let mut result = HashMap::new();

        // North
        let mut border: usize = 0;
        for pow in (0..10).rev() {
            border += (self.image[0][9 - pow] as usize) << pow;
        }
        result.insert(Direction::North, border);

        // East
        let mut border: usize = 0;
        for pow in (0..10).rev() {
            border += (self.image[9 - pow][9] as usize) << pow;
        }
        result.insert(Direction::East, border);

        // South
        let mut border: usize = 0;
        for pow in (0..10).rev() {
            border += (self.image[9][9 - pow] as usize) << pow;
        }
        result.insert(Direction::South, border);

        // West
        let mut border: usize = 0;
        for pow in (0..10).rev() {
            border += (self.image[9 - pow][0] as usize) << pow;
        }
        result.insert(Direction::West, border);
        Ok(result)
    }

    fn rotate90cw(&mut self) {
        let mut new_image: Vec<Vec<Pixel>> = vec![vec![Pixel::Off; 10]; 10];
        for y in 0..10 {
            for x in 0..10 {
                new_image[y][x] = self.image[9 - x][y];
            }
        }
        self.image = new_image;
        self.orientation = match self.orientation {
            Symmetry::Identity0 => Symmetry::Rotate090,
            Symmetry::Rotate090 => Symmetry::Rotate180,
            Symmetry::Rotate180 => Symmetry::Rotate270,
            Symmetry::Rotate270 => Symmetry::Identity0,
            Symmetry::FlipNSIdn => Symmetry::FlipNS090,
            Symmetry::FlipNS090 => Symmetry::FlipNS180,
            Symmetry::FlipNS180 => Symmetry::FlipNS270,
            Symmetry::FlipNS270 => Symmetry::FlipNSIdn,
        };
    }

    fn flipns(&mut self) {
        if self.orientation != Symmetry::Identity0 && self.orientation != Symmetry::FlipNSIdn {
            panic!();
        }

        let mut new_image: Vec<Vec<Pixel>> = vec![vec![Pixel::Off; 10]; 10];
        for y in 0..10 {
            for x in 0..10 {
                new_image[y][x] = self.image[9 - y][x];
            }
        }
        self.image = new_image;
        self.orientation = match self.orientation {
            Symmetry::Identity0 => Symmetry::FlipNSIdn,
            Symmetry::FlipNSIdn => Symmetry::Identity0,
            _ => unreachable!(),
        };
    }

    fn rotate_to(&mut self, symmetry: &Symmetry) {
        // No use in implementing arbitrary from/to
        assert_eq!(self.orientation, Symmetry::Identity0);

        match symmetry {
            Symmetry::Identity0 => return,
            Symmetry::Rotate090 => self.rotate90cw(),
            Symmetry::Rotate180 => {
                self.rotate90cw();
                self.rotate90cw();
            }
            Symmetry::Rotate270 => {
                self.rotate90cw();
                self.rotate90cw();
                self.rotate90cw();
            }
            Symmetry::FlipNSIdn => self.flipns(),
            Symmetry::FlipNS090 => {
                self.flipns();
                self.rotate90cw();
            }
            Symmetry::FlipNS180 => {
                self.flipns();
                self.rotate90cw();
                self.rotate90cw();
            }
            Symmetry::FlipNS270 => {
                self.flipns();
                self.rotate90cw();
                self.rotate90cw();
                self.rotate90cw();
            }
        }
    }

    fn from_lines_iter<I>(iter: &mut I) -> Result<Self, ParseError>
    where
        I: Iterator,
        I::Item: Borrow<str>,
    {
        let mut result = Tile::new();

        for bline in iter {
            let line = bline.borrow();
            if line.is_empty() {
                break;
            }

            if line.starts_with("Tile ") {
                result.id = line[5..line.find(':').unwrap()]
                    .parse::<usize>()
                    .map_err(|_| ParseError)?;
                continue;
            }

            let pixels: Vec<Pixel> = line
                .chars()
                .map(|ch| match ch {
                    '.' => Ok(Pixel::Off),
                    '#' => Ok(Pixel::On),
                    _ => Err(ParseError),
                })
                .collect::<Result<Vec<Pixel>, ParseError>>()?;
            result.image.push(pixels);
        }

        // Calculate all borders -- TODO calc all in one
        result
            .borders
            .insert(result.orientation, result.calculate_borders()?);
        result.rotate90cw();
        result
            .borders
            .insert(result.orientation, result.calculate_borders()?);
        result.rotate90cw();
        result
            .borders
            .insert(result.orientation, result.calculate_borders()?);
        result.rotate90cw();
        result
            .borders
            .insert(result.orientation, result.calculate_borders()?);
        result.rotate90cw();
        result.flipns();
        result
            .borders
            .insert(result.orientation, result.calculate_borders()?);
        result.rotate90cw();
        result
            .borders
            .insert(result.orientation, result.calculate_borders()?);
        result.rotate90cw();
        result
            .borders
            .insert(result.orientation, result.calculate_borders()?);
        result.rotate90cw();
        result
            .borders
            .insert(result.orientation, result.calculate_borders()?);
        result.rotate90cw();
        result.flipns();
        return Ok(result);
    }
}

#[derive(Debug)]
struct WorldMap {
    pixels: Vec<Vec<Pixel>>,
}

impl std::fmt::Display for WorldMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in self.pixels.iter() {
            for pix in x.iter() {
                write!(f, "{}", match pix {
                    Pixel::On => "#",
                    Pixel::Off => ".",
                    Pixel::DontCare => "/",
                    Pixel::Monster => "O",
                })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl WorldMap {
    fn from_tilemap(tilemap: &HashMap<(isize, isize), Tile>) -> Self {
        // Find min/max X and Y in tilemap
        // TODO: don't loop every time ?
        let &min_x = tilemap.keys().map(|(x, _)| x).min().unwrap();
        let &max_x = tilemap.keys().map(|(x, _)| x).max().unwrap();
        let &min_y = tilemap.keys().map(|(_, y)| y).min().unwrap();
        let &max_y = tilemap.keys().map(|(_, y)| y).max().unwrap();

        // Tiles are 10x10 so without border 8x8.
        let mut large_map: Vec<Vec<Pixel>> = vec![vec![Pixel::Off; (max_x-min_x+1) as usize * 8]; (max_y-min_y+1) as usize * 8];
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let tile = tilemap.get(&(x, y)).unwrap();
                for tile_y in 1..=8 {
                    for tile_x in 1..=8 {
                        let target_y = ((y + (0 - min_y)) * 8) as usize + tile_y - 1;
                        let target_x = ((x + (0 - min_x)) * 8) as usize + tile_x - 1;
                        large_map[target_y][target_x] = tile.image[tile_y][tile_x];
                    }
                }
            }
        }

        WorldMap {
            pixels: large_map,
        }
    }

    fn rotate90cw(&mut self) {
        let mut new_pixels: Vec<Vec<Pixel>> = vec![vec![Pixel::Off; self.pixels[0].len()]; self.pixels.len()];
        for y in 0..self.pixels.len() {
            for x in 0..self.pixels[0].len() {
                new_pixels[y][x] = self.pixels[self.pixels.len() - 1 - x][y];
            }
        }
        self.pixels = new_pixels;
    }

    fn flipns(&mut self) {
        let mut new_pixels: Vec<Vec<Pixel>> = vec![vec![Pixel::Off; self.pixels[0].len()]; self.pixels.len()];
        for y in 0..self.pixels.len() {
            for x in 0..self.pixels[0].len() {
                new_pixels[y][x] = self.pixels[self.pixels.len() - 1 - y][x];
            }
        }
        self.pixels = new_pixels;
    }

    fn find_monsters(&mut self) -> bool {
        let monster_str = "..................#.
#....##....##....###
.#..#..#..#..#..#...";
        let _monster: Vec<Vec<Pixel>> = monster_str.split("\n").map(|line| {
                line.chars().map(|ch| match ch {
                    '#' => Pixel::On,
                    '.' => Pixel::DontCare,
                    _ => panic!(),
                }).collect()
            }).collect();
        let mut found = false;
        let mut mutated_pixels: Vec<Vec<Pixel>> = self.pixels.clone();

        for y in 0..self.pixels.len() - _monster.len() {
            for x in 0..(self.pixels.len() - _monster[0].len()) {
                let mut complete = true;

                'check: for (dy, ml) in _monster.iter().enumerate() {
                    for (dx, mch) in ml.iter().enumerate() {
                        let ch = self.pixels[y + dy][x + dx];
                        complete = complete && match mch {
                            Pixel::DontCare => true,
                            Pixel::On => ch == Pixel::On,
                            _ => unreachable!(),
                        };
                        if !complete {
                            break 'check;
                        }
                    }
                }

                if complete {
                    // We found a complete monster! Mark it on the mutated map
                    found = true;

                    for (dy, ml) in _monster.iter().enumerate() {
                        for (dx, mch) in ml.iter().enumerate() {
                            if mch == &Pixel::On {
                                mutated_pixels[y + dy][x + dx] = Pixel::Monster;
                            }
                        }
                    }
                }
            }
        }

        if found {
            self.pixels = mutated_pixels;
        }
        
        found
    }

    fn roughness(&self) -> usize {
        let mut result: usize = 0;
        for y in self.pixels.iter() {
            for x in y.iter() {
                if x == &Pixel::On {
                    result += 1;
                }
            }
        }

        result
    }
}

fn parse_input<I>(iter: &mut Peekable<I>) -> Result<Vec<Tile>, ParseError>
where
    I: Iterator,
    I::Item: Borrow<str>,
{
    let mut tiles: Vec<Tile> = vec![];
    while iter.peek().is_some() {
        let tile = Tile::from_lines_iter(iter)?;
        tiles.push(tile);
    }
    Ok(tiles)
}

fn make_tilemap(tiles: &Vec<Tile>) -> HashMap<(isize, isize), Tile> {
    let mut tilemap: HashMap<(isize, isize), Tile> = HashMap::new();
    let mut coords_to_check: Vec<(isize, isize)> = vec![];
    let directions = vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    let symmetries = vec![
        Symmetry::Identity0,
        Symmetry::Rotate090,
        Symmetry::Rotate180,
        Symmetry::Rotate270,
        Symmetry::FlipNSIdn,
        Symmetry::FlipNS090,
        Symmetry::FlipNS180,
        Symmetry::FlipNS270,
    ];
    let mut placed_tiles: Vec<usize> = vec![];

    // Arbitratily begin with the first tile, non-rotated
    tilemap.insert((0, 0), tiles[0].clone());
    coords_to_check.push((0, 0));
    placed_tiles.push(tiles[0].id);
    while coords_to_check.len() > 0 {
        // Get the tile at cur_coord and determine the tiles next to it
        let cur_coord = coords_to_check.pop().unwrap();
        // TODO: ugly clone but I need to be able to mutate tilemap below in the loop :-(
        let cur_tile = tilemap[&cur_coord].clone();

        // Place tiles next to the tile we're examining, if possible
        for dir in directions.iter() {
            let (border_to_check, new_coords) = match dir {
                Direction::North => (Direction::South, (cur_coord.0, cur_coord.1 - 1)),
                Direction::East => (Direction::West, (cur_coord.0 + 1, cur_coord.1)),
                Direction::South => (Direction::North, (cur_coord.0, cur_coord.1 + 1)),
                Direction::West => (Direction::East, (cur_coord.0 - 1, cur_coord.1)),
            };

            // Don't recheck for coords where a tile is already placed
            if tilemap.get(&new_coords).is_some() {
                continue;
            }

            let tile_borders = cur_tile.borders.get(&cur_tile.orientation).unwrap();

            let matches: Vec<(&Tile, &Symmetry)> = tiles
                .iter()
                .filter_map(|t| {
                    match symmetries.iter().find(|sym| {
                        // Skip already placed tiles
                        !placed_tiles.contains(&t.id)
                            && tile_borders.get(dir)
                                == t.borders.get(&sym).unwrap().get(&border_to_check)
                    }) {
                        Some(sym) => Some((t, sym)),
                        None => None,
                    }
                })
                .collect();
            match matches.len() {
                // No match for this, apparently it's an edge
                0 => {}
                // One match exactly, place it as specified
                1 => {
                    let (found, sym) = matches[0];
                    let mut placing = found.clone();
                    placing.rotate_to(&sym);
                    placed_tiles.push(placing.id);
                    tilemap.insert(new_coords, placing);
                    coords_to_check.push(new_coords);
                }
                _ => panic!(format!(
                    "Multiple matches for dir {:?} matches {:?} tile {}",
                    dir, matches, cur_tile
                )),
            }
        }
    }

    tilemap
}

fn _print_tilemap(tilemap: &HashMap<(isize, isize), Tile>) {
    // Find min/max X and Y in tilemap
    // TODO: don't loop every time ?
    let &min_x = tilemap.keys().map(|(x, _)| x).min().unwrap();
    let &max_x = tilemap.keys().map(|(x, _)| x).max().unwrap();
    let &min_y = tilemap.keys().map(|(_, y)| y).min().unwrap();
    let &max_y = tilemap.keys().map(|(_, y)| y).max().unwrap();

    println!("x from {} to {}", min_x, max_x);
    println!("y from {} to {}", min_y, max_y);

    println!("IDs per X/Y:");
    for y in min_y..=max_y {
        if y == min_y {
            print!("    X ");

            // Print X header
            for x in min_x..=max_x {
                print!(" {:4}", x);
            }
            println!();
        }
        print!("Y {:3}:", y);
        for x in min_x..=max_x {
            if let Some(tile) = tilemap.get(&(x, y)) {
                print!(" {:4}", tile.id);
            } else {
                print!("     ");
            }
        }
        println!();
    }
    println!();
}

fn star_one(tiles: &Vec<Tile>) -> usize {
    let tilemap = make_tilemap(tiles);

    // Find min/max X and Y in tilemap
    // TODO: don't loop every time ?
    let &min_x = tilemap.keys().map(|(x, _)| x).min().unwrap();
    let &max_x = tilemap.keys().map(|(x, _)| x).max().unwrap();
    let &min_y = tilemap.keys().map(|(_, y)| y).min().unwrap();
    let &max_y = tilemap.keys().map(|(_, y)| y).max().unwrap();

    let result = tilemap.get(&(min_x, min_y)).unwrap().id
        * tilemap.get(&(min_x, max_y)).unwrap().id
        * tilemap.get(&(max_x, min_y)).unwrap().id
        * tilemap.get(&(max_x, max_y)).unwrap().id;

    result
}

fn star_two(tiles: &Vec<Tile>) -> usize {
    let tilemap = make_tilemap(tiles);
    let mut map = WorldMap::from_tilemap(&tilemap);

    'outer: for _ in 0..2 {
        for _ in 0..4 {
            if map.find_monsters() {
                break 'outer;
            }
            map.rotate90cw();
        }
        map.flipns();
    }

    // println!("{}", map);
    let roughness = map.roughness();

    roughness
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .peekable();
    let tiles = parse_input(&mut lines).expect("Invalid input data");
    let ans = star_one(&tiles);
    println!("Star one: {}", ans);

    let ans = star_two(&tiles);
    println!("Star two: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn test_star_one() {
        let mut lines = TEST_DATA.lines().map(|x| x.to_string()).peekable();
        let tiles = super::parse_input(&mut lines).expect("Invalid test data");
        let ans = super::star_one(&tiles);
        assert_eq!(ans, 20899048083289);
    }

    #[test]
    fn test_star_two() {
        let mut lines = TEST_DATA.lines().map(|x| x.to_string()).peekable();
        let tiles = super::parse_input(&mut lines).expect("Invalid test data");
        let ans = super::star_two(&tiles);
        assert_eq!(ans, 273);
    }
}
