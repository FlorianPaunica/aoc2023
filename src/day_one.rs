use crate::util;

pub fn day_one() {
    let test_one_data = util::read_file("one", util::AocPart::Test_One);
    let one_data = util::read_file("one", util::AocPart::One);
    let test_one_result = part_one(&test_one_data);
    let one_result = part_one(&one_data);
    let test_two_data = util::read_file("one", util::AocPart::Test_Two);
    let test_two_result = part_two(&test_two_data);
    let two_result = part_two(&one_data);
    println!("Test one  result: {}", test_one_result);
    println!("Input one result: {}", one_result);
    println!("Test two result: {}", test_two_result);
    println!("Input two result: {}", two_result);
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
    for ch in data.chars() {
        if ch.is_numeric() {
            return ch.to_digit(10);
        }
    }
    None
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
