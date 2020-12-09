use anyhow::Result;

use std::io::prelude::*;
use std::num::ParseIntError;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};
use std::{env::consts::FAMILY, fs::File};

#[derive(Debug, Clone)]
enum Op {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl FromStr for Op {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split_whitespace().collect();

        let op_str = coords[0];
        let num = coords[1].parse::<isize>()?;
        let op = match op_str {
            "nop" => Op::Nop(num),
            "acc" => Op::Acc(num),
            "jmp" => Op::Jmp(num),
            _ => unreachable!(),
        };
        Ok(op)
    }
}

#[derive(Debug, Default)]
struct CPU {
    accumulator: isize,
    ops: Vec<Op>,
    pc: isize,
    instructions: Vec<isize>,
    finish: bool,
}

impl CPU {
    pub fn new(ops: Vec<Op>) -> CPU {
        CPU {
            ops,
            finish: false,
            ..Default::default()
        }
    }

    pub fn run(&mut self) {
        loop {
            let op = &self.ops[self.pc as usize];

            if self.instructions.contains(&self.pc) {
                break;
            }

            self.instructions.push(self.pc);

            match op {
                Op::Nop(_) => {
                    self.pc += 1;
                }
                Op::Acc(i) => {
                    self.accumulator += i;
                    self.pc += 1;
                }
                Op::Jmp(i) => {
                    self.pc += i;
                }
            }

            if self.pc >= (self.ops.len() as isize) {
                self.finish = true;
                break;
            };
        }
    }
}

fn read_input_content() -> Result<String> {
    let mut file = File::open("/home/light4/playground/adventofcode_2020/data/day08.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn to_ops(content: &str) -> Vec<Op> {
    let mut result = vec![];
    for line in content.split("\n") {
        if line.len() <= 0 {
            continue;
        }
        let op = line.parse::<Op>().unwrap();
        result.push(op);
    }
    result
}

fn find_correct_cpu(ops: Vec<Op>) -> CPU {
    let mut cpu = CPU::default();
    for (idx, op) in ops.iter().enumerate() {
        dbg!(&idx);

        match op {
            Op::Nop(i) => {
                let mut new_ops = ops.clone();
                new_ops[idx] = Op::Jmp(*i);
                cpu = CPU::new(new_ops);
                cpu.run();
                if cpu.finish {
                    break;
                }
            }
            Op::Jmp(i) => {
                let mut new_ops = ops.clone();
                new_ops[idx] = Op::Nop(*i);
                cpu = CPU::new(new_ops);
                cpu.run();
                if cpu.finish {
                    break;
                }
            }
            Op::Acc(_) => (),
        }
    }
    cpu
}

fn main() -> Result<()> {
    let content = read_input_content()?;
    //     let content = r#"
    // nop +0
    // acc +1
    // jmp +4
    // acc +3
    // jmp -3
    // acc -99
    // acc +1
    // jmp -4
    // acc +6"#;

    let ops = to_ops(&content);
    // let mut cpu = CPU::new(ops);
    // cpu.run();

    let cpu = find_correct_cpu(ops);
    dbg!(&cpu);
    dbg!(&cpu.accumulator);

    Ok(())
}

mod test {

    #[test]
    fn test_to_seat() {}
}
