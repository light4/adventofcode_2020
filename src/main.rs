use anyhow::Result;

use std::cmp::{max, min};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

fn read_input_content() -> Result<String> {
    let path = "/home/light4/playground/adventofcode_2020/data/day13.txt";
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

#[derive(Debug, Eq, PartialEq)]
struct Note {
    earliest_timestamp: usize,
    bus_ids: Vec<usize>,
}

impl Note {
    fn new(earliest_timestamp: usize, bus_ids: Vec<usize>) -> Self {
        Self {
            earliest_timestamp,
            bus_ids,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct WaitOption {
    bus_id: usize,
    wait_time: usize,
}

impl WaitOption {
    fn new(bus_id: usize, wait_time: usize) -> Self {
        Self { bus_id, wait_time }
    }
}

fn get_inputs(content: &str) -> Note {
    let mut bus_ids = vec![];
    let mut lines = content.trim().split("\n");
    let first_line = lines.next().unwrap();
    let second_line = lines.next().unwrap();
    let earliest_timestamp = first_line.trim().parse().unwrap();

    for num in second_line.split(",") {
        let n = num.parse();
        if let Ok(bus_id) = n {
            bus_ids.push(bus_id);
        }
    }

    Note::new(earliest_timestamp, bus_ids)
}

fn get_wait_option(note: &Note) -> WaitOption {
    let mut options = vec![];
    for bus_id in &note.bus_ids {
        let least_wait_time = (note.earliest_timestamp + bus_id - 1) / bus_id;
        let wait_time = least_wait_time * bus_id - note.earliest_timestamp;
        options.push(WaitOption::new(*bus_id, wait_time));
    }
    *options
        .iter()
        .min_by(|x, y| x.wait_time.cmp(&y.wait_time))
        .unwrap()
}

fn part_one() -> usize {
    todo!()
}

fn main() -> Result<()> {
    let content = read_input_content()?;

    let note = get_inputs(&content);
    let wait_option = get_wait_option(&note);
    let result = wait_option.wait_time * wait_option.bus_id;
    dbg!(&result);

    Ok(())
}

mod test {
    use crate::{get_inputs, get_wait_option, Note, WaitOption};

    #[test]
    fn first_test() {
        let content = r#"
939
7,13,x,x,59,x,31,19
"#;
        let note = get_inputs(content);
        assert_eq!(note, Note::new(939, vec![7, 13, 59, 31, 19]));
        let wait_option = get_wait_option(&note);
        assert_eq!(
            wait_option,
            WaitOption {
                bus_id: 59,
                wait_time: 5
            }
        );
        let result = wait_option.wait_time * wait_option.bus_id;
        assert_eq!(result, 295);
    }
}
