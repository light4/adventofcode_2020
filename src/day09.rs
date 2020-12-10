use anyhow::Result;

use std::fs::File;
use std::io::prelude::*;

fn read_input_content() -> Result<String> {
    let mut file = File::open("/home/light4/playground/adventofcode_2020/data/day09.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn get_inputs(content: &str) -> Vec<usize> {
    let mut result = vec![];
    for line in content.split("\n") {
        if line.len() <= 0 {
            continue;
        }
        let op = line.parse::<usize>().unwrap();
        result.push(op);
    }
    result
}

fn check_valid(inputs: &[usize], next: usize) -> bool {
    let mut valid = false;
    for i in 0..(inputs.len() - 1) {
        for j in (i + 1)..inputs.len() {
            if next == inputs[i] + inputs[j] {
                valid = true;
                break;
            }
        }
    }
    valid
}

fn find_not_valid(inputs: &[usize], count: usize) -> usize {
    let mut not_valid = 0;
    for i in count..inputs.len() {
        if !check_valid(&inputs[(i - count)..i], inputs[i]) {
            dbg!(i);
            not_valid = inputs[i];
            break;
        }
    }
    not_valid
}

fn find_set_index(inputs: &[usize], not_valid: usize) -> (usize, usize) {
    let mut start = 0;
    let mut end = 0;
    'outer: for i in 0..(inputs.len() - 1) {
        for j in (i + 1)..inputs.len() {
            let set_sum = inputs[i..j].iter().sum();
            if not_valid == set_sum {
                start = i;
                end = j;
                break 'outer;
            } else if not_valid < set_sum {
                continue 'outer;
            }
        }
    }
    (start, end)
}

fn main() -> Result<()> {
    let content = read_input_content()?;
    //     let content = r#"
    // 35
    // 20
    // 15
    // 25
    // 47
    // 40
    // 62
    // 55
    // 65
    // 95
    // 102
    // 117
    // 150
    // 182
    // 127
    // 219
    // 299
    // 277
    // 309
    // 576"#;

    let inputs = get_inputs(&content);
    // let mut cpu = CPU::new(ops);
    // cpu.run();
    // let next = inputs[14];
    // let r = check_valid(&inputs[9..14], next);
    // dbg!(&r);

    let not_valid = find_not_valid(&inputs, 25);
    dbg!(&not_valid);

    let (start, end) = find_set_index(&inputs, not_valid);
    let sets = &inputs[start..end];
    dbg!(&sets);

    let r = sets.iter().max().unwrap() + sets.iter().min().unwrap();
    dbg!(r);

    Ok(())
}

mod test {

    #[test]
    fn test_to_seat() {}
}
