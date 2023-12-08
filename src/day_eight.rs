use std::{collections::HashMap, time::Instant};
use num::integer::lcm;

use crate::util;

pub fn main() {
    let data = util::read_file("eight");
    let start = Instant::now();
    let one_result = part_one(&data);
    let duration = start.elapsed();
    println!("input one result: {}", one_result);
    println!("Execution time: {:?}", duration);
    let start = Instant::now();
    let two_result = part_two(&data);
    let duration = start.elapsed();
    println!("input two result: {}", two_result);
    println!("Execution time: {:?}", duration);
}

type NodeTree = HashMap<String, (String, String)>;

fn part_one(data: &Vec<String>) -> usize {
    let mut sum = 0;
    let (dirs, nodes) = parse_input(data);
    let mut current = nodes.get("AAA").unwrap();
    let mut loc = &String::from("AAA");
    while loc != "ZZZ" {
        for dir in &dirs {
            sum += 1;
            if dir == &0 {
                loc = &current.0;
                current = nodes.get(&current.0).unwrap();
            } else {
                loc = &current.1;
                current = nodes.get(&current.1).unwrap();
            }
        }
    }
    sum
}

fn part_two(data: &Vec<String>) -> usize {
    let (dirs, nodes) = parse_input(data);
    let root_keys: Vec<&String> = nodes.keys().filter(|k| k.ends_with('A')).collect();
    root_keys.iter().map(|k| {
        let mut current = nodes.get(*k).unwrap();
        let mut loc = *k;
        let mut moves = 0;
        while !loc.ends_with('Z') {
            for dir in &dirs {
                moves += 1;
                if dir == &0 {
                    loc = &current.0;
                    current = nodes.get(&current.0).unwrap();
                } else {
                    loc = &current.1;
                    current = nodes.get(&current.1).unwrap();
                }
            }
        }
        moves
    }).reduce(|a, b| lcm(a, b)).unwrap()
}

fn parse_input(data: &Vec<String>) -> (Vec<u8>, NodeTree) {
    let mut res: NodeTree = HashMap::new();
    let dirs = &data.first().unwrap();
    let dirs = dirs
        .trim()
        .chars()
        .map(|ch| match ch {
            'L' => 0u8,
            'R' => 1u8,
            _ => panic!(),
        })
        .collect();
    for line in data.iter().skip(2) {
        if line.len() != 0 {
            let (k, v) = line.split_once("=").unwrap();
            let k = String::from(k.trim());
            let (l, r) = v
                .trim()
                .replace("(", "")
                .replace(")", "")
                .split_once(",")
                .map(|(l, r)| (String::from(l.trim()), String::from(r.trim())))
                .unwrap();
            res.insert(k, (l, r));
        }
    }
    (dirs, res)
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn part_one_test() {
        let input: Vec<String> = vec![
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ]
        .iter()
        .map(|x| String::from(*x))
        .collect();
        let result = part_one(&input);
        assert_eq!(result, 6);
    }

    #[test]
    fn part_two_test() {
        let input: Vec<String> = vec![
            "LR",
            "",
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)",
        ]
        .iter()
        .map(|x| String::from(*x))
        .collect();
        let result = part_two(&input);
        assert_eq!(result, 6);
    }
}
