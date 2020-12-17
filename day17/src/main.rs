use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::ops::Range;

#[derive(Debug)]
struct InvalidSpecificationError;

#[derive(Debug, Copy, Clone, PartialEq)]
enum CubeState {
    Inactive = 0,
    Active = 1,
}

#[derive(Debug)]
struct PocketDimension {
    cubes: HashMap<isize, HashMap<isize, HashMap<isize, CubeState>>>,
}

impl fmt::Display for PocketDimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x_range, y_range, z_range) = self.find_max_active();
        for z in z_range {
            writeln!(f, "z={}", z)?;
            for y in y_range.clone() {
                for x in x_range.clone() {
                    match self.state_at(x, y, z) {
                        CubeState::Active => write!(f, "#")?,
                        CubeState::Inactive => write!(f, ".")?,
                    }
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl PocketDimension {
    fn new() -> Self {
        PocketDimension {
            cubes: HashMap::new(),
        }
    }

    fn state_at(&self, x: isize, y: isize, z: isize) -> CubeState {
        match self.cubes.get(&z) {
            Some(ymap) => match ymap.get(&y) {
                Some(xmap) => match xmap.get(&x) {
                    Some(state) => *state,
                    None => CubeState::Inactive,
                },
                None => CubeState::Inactive,
            },
            None => CubeState::Inactive,
        }
    }

    fn set_state(&mut self, x: isize, y: isize, z: isize, new_state: CubeState) {
        match new_state {
            CubeState::Active => {
                *(*(*self.cubes.entry(z).or_insert(HashMap::new()))
                    .entry(y)
                    .or_insert(HashMap::new()))
                .entry(x)
                .or_insert(CubeState::Active) = new_state
            }
            CubeState::Inactive => {
                let ymap = match self.cubes.get_mut(&z) {
                    Some(map) => map,
                    None => return,
                };
                let xmap = match ymap.get_mut(&y) {
                    Some(map) => map,
                    None => return,
                };
                match xmap.get(&x) {
                    Some(_) => {
                        xmap.remove(&x);
                        if xmap.is_empty() {
                            ymap.remove(&y);
                            if ymap.is_empty() {
                                self.cubes.remove(&z);
                            }
                        }
                    }
                    None => return,
                }
            }
        }
    }

    fn get_active_neighbours(&self, x: isize, y: isize, z: isize) -> isize {
        let mut active: isize = 0;
        for z_neigh in -1..=1 {
            for y_neigh in -1..=1 {
                for x_neigh in -1..=1 {
                    // Don't compare to ourselves
                    if x_neigh == 0 && y_neigh == 0 && z_neigh == 0 {
                        continue;
                    }

                    if self.state_at(x + x_neigh, y + y_neigh, z + z_neigh) == CubeState::Active {
                        active += 1;
                    }
                }
            }
        }

        active
    }

    fn find_max_active(&self) -> (Range<isize>, Range<isize>, Range<isize>) {
        let mut x_range: Range<isize> = Range {
            start: isize::MAX,
            end: isize::MIN,
        };
        let mut y_range: Range<isize> = Range {
            start: isize::MAX,
            end: isize::MIN,
        };
        let mut z_range: Range<isize> = Range {
            start: isize::MAX,
            end: isize::MIN,
        };
        for (_, ymap) in self.cubes.iter() {
            for (_, xmap) in ymap {
                x_range.start = cmp::min(x_range.start, *xmap.keys().min().unwrap());
                x_range.end = cmp::max(x_range.end, *xmap.keys().max().unwrap() + 1);
            }
            y_range.start = cmp::min(y_range.start, *ymap.keys().min().unwrap());
            y_range.end = cmp::max(y_range.end, *ymap.keys().max().unwrap() + 1);
        }
        z_range.start = *self.cubes.keys().min().unwrap();
        z_range.end = *self.cubes.keys().max().unwrap() + 1;

        (x_range, y_range, z_range)
    }

    fn run_cycle(&mut self) {
        let mut new_dimension = PocketDimension::new();

        // Loop over all active layers -1, 0, +1 in order
        let (mut x_range, mut y_range, mut z_range) = self.find_max_active();
        x_range.start -= 1;
        x_range.end += 1;
        y_range.start -= 1;
        y_range.end += 1;
        z_range.start -= 1;
        z_range.end += 1;

        for z in z_range {
            for y in y_range.clone() {
                for x in x_range.clone() {
                    match self.state_at(x, y, z) {
                        CubeState::Active => match self.get_active_neighbours(x, y, z) {
                            2..=3 => new_dimension.set_state(x, y, z, CubeState::Active),
                            _ => new_dimension.set_state(x, y, z, CubeState::Inactive),
                        },
                        CubeState::Inactive => match self.get_active_neighbours(x, y, z) {
                            3 => new_dimension.set_state(x, y, z, CubeState::Active),
                            _ => new_dimension.set_state(x, y, z, CubeState::Inactive),
                        },
                    }
                }
            }
        }

        self.cubes = new_dimension.cubes;
    }

    fn count_active_cubes(&self) -> isize {
        self.cubes.iter().fold(0, |total, (_, ymap)| {
            total
                + ymap
                    .iter()
                    .fold(0, |ysum, (_, xmap)| ysum + xmap.len() as isize)
        })
    }
}

fn parse_input(input: &str) -> Result<PocketDimension, InvalidSpecificationError> {
    let mut p = PocketDimension::new();

    // Input is 2D so only loop over x, y for every line
    for (y, line) in input.split("\n").enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => p.set_state(x as isize, y as isize, 0, CubeState::Inactive),
                '#' => p.set_state(x as isize, y as isize, 0, CubeState::Active),
                _ => return Err(InvalidSpecificationError),
            }
        }
    }

    Ok(p)
}

fn star_one(dimension: &mut PocketDimension) -> isize {
    for _ in 0..6 {
        dimension.run_cycle();
    }

    dimension.count_active_cubes()
}

fn main() {
    let input = fs::read_to_string("./input").expect("Unreadable input file ./input");
    let mut dimension = parse_input(&input).expect("Error in input file");

    let ans = star_one(&mut dimension);
    println!("Star one: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = ".#.
..#
###";

    #[test]
    fn test_star_one() {
        let mut dimension = super::parse_input(TEST_DATA).expect("Error in test data");

        let ans = super::star_one(&mut dimension);
        assert_eq!(ans, 112);
    }
}
