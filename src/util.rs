use std::fs::read_to_string;

pub fn read_file(day: &str) -> Vec<String> {
    let path_name = format!("./src/problems/day_{day}.txt");
    read_to_string(&path_name)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
