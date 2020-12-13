use anyhow::Result;

use std::cmp::{max, min};
use std::fmt;
use std::io::prelude::*;
use std::{fs::File, todo};

fn read_input_content() -> Result<String> {
    let path = "/home/light4/playground/adventofcode_2020/data/day12.txt";
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
#[derive(Debug, Eq, PartialEq)]
enum Direct {
    East,
    North,
    West,
    South,
}

#[derive(Debug, Eq, PartialEq)]
struct Direction {
    direct: Direct,
    coordinate: (isize, isize),
}

impl Default for Direction {
    fn default() -> Self {
        Direction::new(Direct::East)
    }
}

impl Direction {
    fn new(direct: Direct) -> Self {
        let coordinate = match direct {
            Direct::East => (1, 0),
            Direct::North => (0, 1),
            Direct::West => (-1, 0),
            Direct::South => (0, -1),
        };
        Self { direct, coordinate }
    }

    fn turn_left(&self) -> Self {
        match self.direct {
            Direct::East => Direction::new(Direct::North),
            Direct::North => Direction::new(Direct::West),
            Direct::West => Direction::new(Direct::South),
            Direct::South => Direction::new(Direct::East),
        }
    }

    fn turn_right(&self) -> Self {
        match self.direct {
            Direct::East => Direction::new(Direct::South),
            Direct::North => Direction::new(Direct::East),
            Direct::West => Direction::new(Direct::North),
            Direct::South => Direction::new(Direct::West),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Action {
    N(isize),
    S(isize),
    E(isize),
    W(isize),
    L(isize),
    R(isize),
    F(isize),
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Status {
    actions: Vec<Action>,
    direction: Direction,
    location: (isize, isize),
    pc: usize,
}

impl Status {
    pub fn new(actions: Vec<Action>) -> Status {
        Status {
            actions,
            ..Default::default()
        }
    }

    fn run_once(&mut self) {
        match self.actions[self.pc] {
            Action::F(i) => {
                self.location.0 += i * self.direction.coordinate.0;
                self.location.1 += i * self.direction.coordinate.1;
            }
            Action::L(i) => {
                for _ in 0..(i / 90) {
                    self.direction = self.direction.turn_left();
                }
            }
            Action::R(i) => {
                for _ in 0..(i / 90) {
                    self.direction = self.direction.turn_right();
                }
            }
            Action::N(i) => {
                let move_direction = Direction::new(Direct::North);
                self.location.0 += i * move_direction.coordinate.0;
                self.location.1 += i * move_direction.coordinate.1;
            }
            Action::S(i) => {
                let move_direction = Direction::new(Direct::South);
                self.location.0 += i * move_direction.coordinate.0;
                self.location.1 += i * move_direction.coordinate.1;
            }
            Action::E(i) => {
                let move_direction = Direction::new(Direct::East);
                self.location.0 += i * move_direction.coordinate.0;
                self.location.1 += i * move_direction.coordinate.1;
            }
            Action::W(i) => {
                let move_direction = Direction::new(Direct::West);
                self.location.0 += i * move_direction.coordinate.0;
                self.location.1 += i * move_direction.coordinate.1;
            }
        }
        self.pc += 1;
    }

    fn run(&mut self) {
        for _ in 0..self.actions.len() {
            self.run_once()
        }
    }

    fn manhattan_distance(&self) -> usize {
        (self.location.0.abs() + self.location.1.abs()) as usize
    }
}

fn get_inputs(content: &str) -> Status {
    let mut actions = vec![];
    for line in content.split("\n") {
        if line.len() <= 0 {
            continue;
        }
        let (c, num) = line.split_at(1);
        let action = match c {
            "N" => Action::N(num.parse().unwrap()),
            "S" => Action::S(num.parse().unwrap()),
            "E" => Action::E(num.parse().unwrap()),
            "W" => Action::W(num.parse().unwrap()),
            "L" => Action::L(num.parse().unwrap()),
            "R" => Action::R(num.parse().unwrap()),
            "F" => Action::F(num.parse().unwrap()),
            _ => unreachable!(),
        };
        actions.push(action);
    }
    Status::new(actions)
}

fn main() -> Result<()> {
    let content = read_input_content()?;
    //     let content = r#"
    // F10
    // N3
    // F7
    // R90
    // F11
    // "#;

    let mut status = get_inputs(&content);
    dbg!(&status);
    status.run();
    dbg!(&status);
    dbg!(&status.manhattan_distance());

    Ok(())
}

mod test {
    use crate::get_inputs;

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
