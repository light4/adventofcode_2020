use anyhow::Result;

use std::io::prelude::*;
use std::{fs::File, todo};

fn read_input_content() -> Result<String> {
    let path = "/home/light4/playground/adventofcode_2020/data/day10.txt";
    let mut file = File::open(path)?;
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
    result.sort();
    result.insert(0, 0);
    result.push(result.last().unwrap() + 3);
    result
}

fn get_joltages(inputs: &[usize]) -> (usize, usize) {
    let mut one = 0;
    let mut three = 0;
    let length = inputs.len();
    for i in 0..(length - 1) {
        if inputs[i + 1] - inputs[i] == 1 {
            one += 1;
        } else if inputs[i + 1] - inputs[i] == 3 {
            three += 1;
        }
    }
    (one, three)
}

fn part_one(content: &str) -> usize {
    let inputs = get_inputs(&content);
    let (one, three) = get_joltages(&inputs);
    one * three
}

fn can_remove(inputs: &[usize], idx: usize) -> bool {
    todo!()
}

fn get_parted(inputs: &[usize]) -> Vec<Vec<usize>> {
    let mut j = 0;

    let mut result = vec![];
    for i in 0..(inputs.len() - 1) {
        if inputs[i + 1] - inputs[i] == 3 {
            let part = &inputs[j..i + 1];
            if part.len() > 2 {
                result.push(part.to_owned());
            }
            j = i + 1;
        }
    }
    result
}

fn get_arranged_count(inputs: &[usize]) -> usize {
    todo!()
}

fn part_two(content: &str) -> usize {
    todo!()
}

fn main() -> Result<()> {
    let content = read_input_content()?;
    //     let content = r#"
    // 16
    // 10
    // 15
    // 5
    // 1
    // 11
    // 7
    // 19
    // 6
    // 12
    // 4"#;
    //     let content = r#"
    // 28
    // 33
    // 18
    // 42
    // 31
    // 14
    // 46
    // 20
    // 48
    // 47
    // 24
    // 23
    // 49
    // 45
    // 19
    // 38
    // 39
    // 11
    // 1
    // 32
    // 25
    // 35
    // 8
    // 17
    // 7
    // 9
    // 4
    // 2
    // 34
    // 10
    // 3"#;

    // let result = part_one(&content);

    let inputs = get_inputs(&content);
    dbg!(&inputs);
    let result = get_parted(&inputs);
    dbg!(&result);
    let r = result.iter().map(|i| i.len()).collect::<Vec<usize>>();
    dbg!(&r);
    let mut part_two_result: u64 = 1;
    for i in r {
        if i == 5 {
            part_two_result *= 7;
        } else if i == 4 {
            part_two_result *= 4;
        } else if i == 3 {
            part_two_result *= 2;
        }
    }
    dbg!(part_two_result);

    Ok(())
}

mod test {
    use crate::{part_one, part_two};

    #[test]
    fn first_test() {
        let content = r#"
16
10
15
5
1
11
7
19
6
12
4"#;
        assert_eq!(part_one(content), 5 * 7);
        assert_eq!(part_two(content), 8);
    }
    #[test]
    fn second_test() {
        let content = r#"
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;
        assert_eq!(part_one(content), 22 * 10);
        assert_eq!(part_two(content), 19208);
    }
}
