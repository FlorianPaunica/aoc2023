use std::time::Instant;

use crate::util;

pub fn main() {
    let data = util::read_file("one");
    let start = Instant::now();
    let one_result = part_one(&data);
    let duration = start.elapsed();
    println!("input one result: {}", one_result);
    println!("execution time: {:?}", duration);
    let start = Instant::now();
    let two_result = part_two(&data);
    let duration = start.elapsed();
    println!("input two result: {}", two_result);
    println!("execution time: {:?}", duration);
}

fn part_two(data: &Vec<String>) -> u32 {
    let mut result = 0;
    for line in data.iter() {
        let fixed_first = replace_with_number(line, false);
        let fixed_last = replace_with_number(&fixed_first, true);
        if let Some(first_value) = find_digit(&fixed_last) {
            if let Some(second_value) = find_digit(&fixed_last.chars().rev().collect()) {
                let line_result = format!("{first_value}{second_value}")
                    .parse::<u32>()
                    .unwrap();
                result += line_result;
            }
        }
    }
    result
}

fn part_one(data: &Vec<String>) -> u32 {
    let mut result = 0;
    for line in data.iter() {
        if let Some(first_value) = find_digit(&line) {
            if let Some(second_value) = find_digit(&line.chars().rev().collect()) {
                let line_result = format!("{first_value}{second_value}")
                    .parse::<u32>()
                    .unwrap();
                result += line_result;
            }
        }
    }
    result
}

fn find_digit(data: &String) -> Option<u32> {
    data.chars().find_map(|ch| ch.to_digit(10))
}

fn replace_with_number(line: &String, reverse: bool) -> String {
    let letter_number: [(&str, &str); 9] = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let mut new_line = match reverse {
        true => line.clone().chars().rev().collect(),
        false => line.clone(),
    };

    'outer: for (i, ch) in new_line.char_indices() {
        if ch.is_numeric() {
            break;
        }
        for ln in letter_number {
            let comp = match reverse {
                true => ln.0.to_string().chars().rev().collect::<String>(),
                false => String::from(ln.0),
            };
            if new_line[i..].starts_with(&comp) {
                new_line = new_line.replacen(&comp, ln.1, 1);
                break 'outer;
            }
        }
    }
    match reverse {
        true => new_line.chars().rev().collect(),
        false => new_line,
    }
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn part_one_test() {
        let input: Vec<String> = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .iter()
            .map(|x| String::from(*x))
            .collect();
        let result = part_one(&input);
        assert_eq!(result, 142);
    }
    #[test]
    fn part_two_test() {
        let input: Vec<String> = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
        .iter()
        .map(|x| String::from(*x))
        .collect();
        let result = part_two(&input);
        assert_eq!(result, 281);
    }
}
