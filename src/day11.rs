use anyhow::Result;

use std::cmp::{max, min};
use std::fmt;
use std::io::prelude::*;
use std::{fs::File, todo};

fn read_input_content() -> Result<String> {
    let path = "/home/light4/playground/adventofcode_2020/data/day11.txt";
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

#[derive(Debug, Eq, PartialEq)]
enum Grid {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

#[derive(Debug, Eq, PartialEq)]
struct SeatLayout {
    layout: Vec<Vec<Grid>>,
    row_length: usize,
    column_length: usize,
}

impl fmt::Display for SeatLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for row in &self.layout {
            for c in row {
                match c {
                    Grid::Floor => s.push('.'),
                    Grid::EmptySeat => s.push('L'),
                    Grid::OccupiedSeat => s.push('#'),
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl SeatLayout {
    pub fn new(layout: Vec<Vec<Grid>>) -> SeatLayout {
        let row_length = layout.len();
        let mut column_length = 0;
        if let Some(row) = layout.get(0) {
            column_length = row.len()
        }
        SeatLayout {
            layout,
            row_length,
            column_length,
        }
    }

    pub fn transform_index(&self, i: usize, j: usize) -> Grid {
        let idx = &self.layout[i][j];
        match idx {
            Grid::Floor => Grid::Floor,
            Grid::EmptySeat => {
                if self.adjacents(i, j) == 0 {
                    Grid::OccupiedSeat
                } else {
                    Grid::EmptySeat
                }
            }
            Grid::OccupiedSeat => {
                if self.adjacents(i, j) >= 4 {
                    Grid::EmptySeat
                } else {
                    Grid::OccupiedSeat
                }
            }
        }
    }

    pub fn new_transform_index(&self, i: usize, j: usize) -> Grid {
        let idx = &self.layout[i][j];
        match idx {
            Grid::Floor => Grid::Floor,
            Grid::EmptySeat => {
                if self.new_adjacents(i, j) == 0 {
                    Grid::OccupiedSeat
                } else {
                    Grid::EmptySeat
                }
            }
            Grid::OccupiedSeat => {
                if self.new_adjacents(i, j) >= 5 {
                    Grid::EmptySeat
                } else {
                    Grid::OccupiedSeat
                }
            }
        }
    }

    pub fn adjacents(&self, i: usize, j: usize) -> usize {
        let mut adjacents = 0;
        let i_start = if i == 0 { 0 } else { i - 1 };
        let i_end = min(i + 1, self.row_length - 1);
        let j_start = if j == 0 { 0 } else { j - 1 };
        let j_end = min(j + 1, self.column_length - 1);

        for x in i_start..=i_end {
            for y in j_start..=j_end {
                if i != x || j != y {
                    if let Some(m) = self.layout.get(x) {
                        if let Some(n) = m.get(y) {
                            match n {
                                Grid::OccupiedSeat => adjacents += 1,
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
        adjacents
    }

    pub fn new_adjacents(&self, i: usize, j: usize) -> usize {
        let mut adjacents = 0;

        // 左上
        'outer_0: for x in (0..i).rev() {
            for y in (0..j).rev() {
                if i + y == j + x {
                    match self.layout[x][y] {
                        Grid::OccupiedSeat => {
                            adjacents += 1;
                            break 'outer_0;
                        }
                        Grid::EmptySeat => break 'outer_0,
                        _ => (),
                    }
                }
            }
        }
        // dbg!("after left top", adjacents);

        // 上
        for x in (0..i).rev() {
            match self.layout[x][j] {
                Grid::OccupiedSeat => {
                    adjacents += 1;
                    break;
                }
                Grid::EmptySeat => break,
                _ => (),
            }
        }
        // dbg!("after top", adjacents);

        // 右上
        'outer_1: for x in (0..i).rev() {
            for y in min(j + 1, self.column_length)..self.column_length {
                if x + y == i + j {
                    match self.layout[x][y] {
                        Grid::OccupiedSeat => {
                            adjacents += 1;
                            break 'outer_1;
                        }
                        Grid::EmptySeat => break 'outer_1,
                        _ => (),
                    }
                }
            }
        }
        // dbg!("after right top", adjacents);

        // 右
        for y in min(j + 1, self.column_length)..self.column_length {
            match self.layout[i][y] {
                Grid::OccupiedSeat => {
                    adjacents += 1;
                    break;
                }
                Grid::EmptySeat => break,
                _ => (),
            }
        }
        // dbg!("after right", adjacents);

        // 右下
        'outer_2: for x in min(i + 1, self.row_length)..self.row_length {
            for y in min(j + 1, self.column_length)..self.column_length {
                if i + y == j + x {
                    match self.layout[x][y] {
                        Grid::OccupiedSeat => {
                            adjacents += 1;
                            break 'outer_2;
                        }
                        Grid::EmptySeat => break 'outer_2,
                        _ => (),
                    }
                }
            }
        }
        // dbg!("after right down", adjacents);

        // 下
        for x in min(i + 1, self.row_length)..self.row_length {
            match self.layout[x][j] {
                Grid::OccupiedSeat => {
                    adjacents += 1;
                    break;
                }
                Grid::EmptySeat => break,
                _ => (),
            }
        }
        // dbg!("after down", adjacents);

        // 左下
        'outer_3: for x in min(i + 1, self.row_length)..self.row_length {
            for y in (0..j).rev() {
                if x + y == i + j {
                    match self.layout[x][y] {
                        Grid::OccupiedSeat => {
                            adjacents += 1;
                            break 'outer_3;
                        }
                        Grid::EmptySeat => break 'outer_3,
                        _ => (),
                    }
                }
            }
        }
        // dbg!("after left down", adjacents);

        // 左
        for y in (0..j).rev() {
            match self.layout[i][y] {
                Grid::OccupiedSeat => {
                    adjacents += 1;
                    break;
                }
                Grid::EmptySeat => break,
                _ => (),
            }
        }
        // dbg!("after left", adjacents);

        adjacents
    }

    pub fn transform(&self) -> SeatLayout {
        let mut result = vec![];
        for i in 0..self.row_length {
            let mut new_row = vec![];
            for j in 0..self.column_length {
                new_row.push(self.new_transform_index(i, j));
            }
            result.push(new_row);
        }
        SeatLayout::new(result)
    }

    pub fn transform_to_stable(&self) -> SeatLayout {
        loop {
            let new_seat = self.transform();
            println!("{}", &new_seat);
            if &new_seat == self {
                break new_seat;
            } else {
                return new_seat.transform_to_stable();
            }
        }
    }

    pub fn occupied_seats(&self) -> usize {
        let mut result = 0;
        for i in 0..self.row_length {
            for j in 0..self.column_length {
                match self.layout[i][j] {
                    Grid::OccupiedSeat => result += 1,
                    _ => (),
                }
            }
        }
        result
    }
}

fn get_inputs(content: &str) -> SeatLayout {
    let mut result = vec![];
    for line in content.split("\n") {
        if line.len() <= 0 {
            continue;
        }
        let mut row = vec![];
        for c in line.chars() {
            match c {
                'L' => row.push(Grid::EmptySeat),
                '.' => row.push(Grid::Floor),
                '#' => row.push(Grid::OccupiedSeat),
                _ => unreachable!(),
            }
        }
        result.push(row);
    }
    SeatLayout::new(result)
}

fn part_one(inputs: &[usize]) -> usize {
    todo!()
}

fn main() -> Result<()> {
    let content = read_input_content()?;

    let seat_layout = get_inputs(&content);
    println!("{}", &seat_layout);

    let new_seat = seat_layout.transform_to_stable();
    println!("{}", &new_seat);

    let occupied_seats = new_seat.occupied_seats();
    println!("{}", occupied_seats);

    Ok(())
}

mod test {
    use crate::{get_inputs, part_one};

    #[test]
    fn first_test() {
        let content = r#"
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"#;

        let trans_once_content = r#"
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
"#;

        let trans_end_content = r#"
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
"#;

        let seat_layout = get_inputs(content);
        assert_eq!(content.trim(), seat_layout.to_string().trim());
        assert_eq!(
            trans_once_content.trim(),
            seat_layout.transform().to_string().trim()
        );
        let stable_layout = seat_layout.transform_to_stable();
        assert_eq!(trans_end_content.trim(), stable_layout.to_string().trim());
        assert_eq!(stable_layout.occupied_seats(), 37);
        // assert_eq!(part_two(content), 8);
    }

    #[test]
    fn test_new_adjacents() {
        let first = r#"
.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....
"#;
        let first_seat_layout = get_inputs(first);
        assert_eq!(first_seat_layout.new_adjacents(4, 3), 8);

        let second = r#"
.............
.L.L.#.#.#.#.
.............
"#;
        let second_seat_layout = get_inputs(second);
        assert_eq!(second_seat_layout.new_adjacents(1, 1), 0);
        assert_eq!(second_seat_layout.new_adjacents(1, 3), 1);

        let third = r#"
.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.
"#;

        let third_seat_layout = get_inputs(third);
        assert_eq!(third_seat_layout.new_adjacents(3, 3), 0);
    }
}
