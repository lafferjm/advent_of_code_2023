use std::fs;

pub fn read_input() -> Vec<String> {
    return fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}