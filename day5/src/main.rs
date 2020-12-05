use core::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug)]
struct IncorrectBoardingPass;

#[derive(Debug)]
struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    fn from_boardingpass(boardingpass: &str) -> Result<Self, IncorrectBoardingPass> {
        let row_binary = boardingpass[..7].replace("F", "0").replace("B", "1");
        let col_binary = boardingpass[7..].replace("L", "0").replace("R", "1");
        let seat = Seat {
            row: match usize::from_str_radix(&row_binary, 2) {
                Ok(val) => val,
                Err(_) => return Err(IncorrectBoardingPass),
            },
            column: match usize::from_str_radix(&col_binary, 2) {
                Ok(val) => val,
                Err(_) => return Err(IncorrectBoardingPass),
            },
        };
        Ok(seat)
    }

    fn seat_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

fn parse_boardingpass(mut seats: Vec<Seat>, line: String) -> Result<Vec<Seat>, IncorrectBoardingPass> {
    seats.push(Seat::from_boardingpass(&line)?);
    Ok(seats)
}

fn star_one(seats: &Vec<Seat>) -> usize {
    seats.iter().fold(0, |s, x| {
        if x.seat_id() > s { x.seat_id() } else { s }
    })
}

#[derive(Debug)]
struct NoSeatFound;

fn star_two(seats: &mut Vec<Seat>) -> Result<usize, NoSeatFound> {
    seats.sort_by(|a, b| {
        a.seat_id().cmp(&b.seat_id())
    });
    let mut last_id: usize = 0;
    for seat in seats {
        if (last_id != 0) && (last_id == seat.seat_id() - 2) {
            return Ok(seat.seat_id() - 1);
        }
        last_id = seat.seat_id();
    }

    Err(NoSeatFound)
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let mut seats = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .try_fold(
            vec![],
            parse_boardingpass,
        )
        .expect("Invalid data in input file");

    println!("Star 1:");
    let highest_seat_id = star_one(&seats);
    println!("Highest seat ID: {}", highest_seat_id);

    println!("Star 2:");
    let my_seat_id = star_two(&mut seats).expect("No suitable seat found");
    println!("My seat ID: {}", my_seat_id);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

    #[test]
    fn test_star_one() {
        let seats = TEST_DATA
            .lines()
            .map(|x| x.to_string())
            .try_fold(
                vec![],
                super::parse_boardingpass,
            )
            .expect("Invalid data in input file");

        assert_eq!(seats.len(), 4);
        assert_eq!(seats[0].row, 44);
        assert_eq!(seats[0].column, 5);
        assert_eq!(seats[0].seat_id(), 357);

        assert_eq!(seats[1].row, 70);
        assert_eq!(seats[1].column, 7);
        assert_eq!(seats[1].seat_id(), 567);

        assert_eq!(seats[2].row, 14);
        assert_eq!(seats[2].column, 7);
        assert_eq!(seats[2].seat_id(), 119);

        assert_eq!(seats[3].row, 102);
        assert_eq!(seats[3].column, 4);
        assert_eq!(seats[3].seat_id(), 820);
    }
}
