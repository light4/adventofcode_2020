use anyhow::Result;

use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Seat {
    id: usize,
    row: usize,
    column: usize,
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn to_row(part: &str) -> usize {
    let mut left = 0;
    let mut right = 127;
    for i in part.chars() {
        if left == right {
            break;
        }
        let pivort = (left + right + 1) / 2;
        if i == 'F' {
            right = pivort;
        } else if i == 'B' {
            left = pivort;
        }
    }
    left
}

fn to_column(part: &str) -> usize {
    let mut left = 0;
    let mut right = 7;
    for i in part.chars() {
        if left == right {
            break;
        }
        let pivort = (left + right + 1) / 2;
        if i == 'L' {
            right = pivort;
        } else if i == 'R' {
            left = pivort;
        }
    }
    left
}

fn to_seat(pass: &str) -> Seat {
    let (row_part, column_part) = pass.split_at(7);

    let row = to_row(row_part);
    let column = to_column(column_part);
    Seat {
        id: row * 8 + column,
        row,
        column,
    }
}

fn main() -> Result<()> {
    let mut file = File::open("/home/light4/playground/adventofcode_2020/data/day05.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut seats = vec![];
    for pass in content.split_whitespace() {
        let seat = to_seat(pass);
        seats.push(seat);
    }
    seats.sort();
    let _ = seats
        .iter()
        .map(|s| {
            println!("{:?}", s.id);
            s.id
        })
        .collect::<Vec<usize>>();

    let lowest = seats.iter().min().unwrap();
    let hightest = seats.iter().max().unwrap();

    let all: usize = (lowest.id..(hightest.id + 1)).into_iter().sum::<usize>();
    println!("{:?}", all);

    let result: usize = seats.iter().map(|s| s.id).sum::<usize>();
    println!("{:?}", result);

    println!("{:?}", all - result);
    Ok(())
}

mod test {
    use crate::{to_seat, Seat};

    #[test]
    fn test_to_seat() {
        assert_eq!(
            to_seat("FBFBBFFRLR"),
            Seat {
                id: 357,
                row: 44,
                column: 5
            }
        );
        assert_eq!(
            to_seat("BFFFBBFRRR"),
            Seat {
                id: 567,
                row: 70,
                column: 7
            }
        );
        assert_eq!(
            to_seat("FFFBBBFRRR"),
            Seat {
                id: 119,
                row: 14,
                column: 7
            }
        );
        assert_eq!(
            to_seat("BBFFBBFRLL"),
            Seat {
                id: 820,
                row: 102,
                column: 4
            }
        );
    }

    #[test]
    fn test_ord() {
        assert!(
            Seat {
                id: 100,
                row: 1,
                column: 8
            } > Seat {
                id: 10,
                row: 100,
                column: 10
            }
        )
    }
}
