use anyhow::Result;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn read_input_content() -> Result<String> {
    let mut file = File::open("/home/light4/playground/adventofcode_2020/data/day07.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn to_trans_map(content: &str) -> HashMap<String, Option<HashMap<String, usize>>> {
    let mut result = HashMap::new();
    for line in content.split("\n") {
        if line.len() <= 0 {
            continue;
        }

        let mut _line = line.split(" contain ");

        let first = _line
            .next()
            .unwrap()
            .split_whitespace()
            .take(2)
            .collect::<Vec<&str>>()
            .join(" ");

        let second = _line.next().unwrap();
        if second.starts_with("no") {
            result.insert(first, None);
        } else {
            let value = second
                .split(",")
                .map(|i| {
                    let mut _i = i.split_whitespace();
                    let v = _i.next().unwrap().parse::<usize>().unwrap();
                    let k = _i.take(2).collect::<Vec<&str>>().join(" ");
                    (k, v)
                })
                .collect::<HashMap<String, usize>>();
            result.insert(first, Some(value));
        }
    }
    result
}

fn find_contains(
    trans_map: &HashMap<String, Option<HashMap<String, usize>>>,
    s: &HashSet<String>,
) -> HashSet<String> {
    let mut find_next_string = HashSet::new();
    for (k, v) in trans_map {
        if let Some(m) = v {
            for i in s {
                if m.contains_key(i) {
                    if !find_next_string.contains(k) {
                        find_next_string.insert(k.to_owned());
                    }
                }
            }
        }
    }
    find_next_string
}

fn find_sets(
    trans_map: &HashMap<String, Option<HashMap<String, usize>>>,
    s: &str,
) -> HashSet<String> {
    let mut first_set = HashSet::new();
    first_set.insert(s.to_owned());

    let mut result = HashSet::new();
    let mut set = find_contains(&trans_map, &first_set);
    loop {
        if set.len() == 0 {
            break;
        }
        result = result.union(&set).map(|s| s.to_owned()).collect();
        dbg!(&set);
        set = find_contains(&trans_map, &set);
    }
    dbg!(&result);
    result
}

fn find_bags(trans_map: &HashMap<String, Option<HashMap<String, usize>>>, s: &str) -> usize {
    let mut result = 0;
    if trans_map.contains_key(s) {
        if let Some(m) = trans_map.get(s).unwrap() {
            for (k, v) in m {
                result += v + v * find_bags(trans_map, k);
            }
        }
    }
    result
}

fn main() -> Result<()> {
    let content = read_input_content()?;
    //     let content = r#"
    // light red bags contain 1 bright white bag, 2 muted yellow bags.
    // dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    // bright white bags contain 1 shiny gold bag.
    // muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    // shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    // dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    // vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    // faded blue bags contain no other bags.
    // dotted black bags contain no other bags."#
    //         .to_owned();
    let bag = "shiny gold";

    let trans_map = to_trans_map(&content);

    // let sets = find_sets(&trans_map, bag);
    // dbg!(sets.len());

    let bags = find_bags(&trans_map, bag);
    dbg!(bags);

    Ok(())
}

mod test {

    #[test]
    fn test_to_seat() {}
}
