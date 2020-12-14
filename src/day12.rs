use anyhow::Result;

use std::cmp::{max, min};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

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
    waypoint: (isize, isize),
    pc: usize,
}

impl Status {
    pub fn new(actions: Vec<Action>, waypoint: (isize, isize)) -> Status {
        Status {
            actions,
            waypoint,
            ..Default::default()
        }
    }

    fn run_once(&mut self) {
        match self.actions[self.pc] {
            Action::F(i) => {
                self.location.0 += i * self.waypoint.0;
                self.location.1 += i * self.waypoint.1;
            }
            Action::L(i) => {
                // [cos, sin]
                // [-sin, cos]
                let coordinate = match (i / 90) % 4 {
                    1 => (0, 1),
                    2 => (-1, 0),
                    3 => (0, -1),
                    0 => (1, 0),
                    _ => unreachable!(),
                };
                self.waypoint = (
                    self.waypoint.0 * coordinate.0 + self.waypoint.1 * -coordinate.1,
                    self.waypoint.0 * coordinate.1 + self.waypoint.1 * coordinate.0,
                );
            }
            Action::R(i) => {
                // [cos, -sin]
                // [sin, cos]
                let coordinate = match (i / 90) % 4 {
                    1 => (0, 1),
                    2 => (-1, 0),
                    3 => (0, -1),
                    0 => (1, 0),
                    _ => unreachable!(),
                };
                self.waypoint = (
                    self.waypoint.0 * coordinate.0 + self.waypoint.1 * coordinate.1,
                    self.waypoint.0 * -coordinate.1 + self.waypoint.1 * coordinate.0,
                );
            }
            Action::N(i) => {
                self.waypoint.1 += i;
            }
            Action::S(i) => {
                self.waypoint.1 += -i;
            }
            Action::E(i) => {
                self.waypoint.0 += i;
            }
            Action::W(i) => {
                self.waypoint.0 += -i;
            }
        }
        self.pc += 1;
    }

    fn run(&mut self) {
        for _ in 0..self.actions.len() {
            self.run_once();
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

    let waypoint = (10, 1);
    Status::new(actions, waypoint)
}

fn main() -> Result<()> {
    let content = read_input_content()?;
    //     let content = r#"
    // F10
    // N3
    // F7
    // R90
    // F11"#;

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
        todo!()
    }
}
