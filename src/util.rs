use std::fs::read_to_string;

pub enum AocPart {
    One,
    Two,
    Test_One,
    Test_Two,
}

pub fn read_file(day: &str, part: AocPart) -> Vec<String> {
    let part = match part {
        AocPart::One => "one",
        AocPart::Two => "two",
        AocPart::Test_One => "test_one",
        AocPart::Test_Two => "test_two",
    };
    let path_name = format!("./src/problems/day_{day}_{part}.txt");
    read_to_string(&path_name)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
