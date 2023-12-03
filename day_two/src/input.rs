use std::fs;

pub fn get_input() -> Vec<String> {
    let  lines = fs::read_to_string("./input.txt").unwrap().lines().map(String::from).collect();
    return lines;
}

pub fn parse_round(round: &str) -> u64 {
    let round: Vec<&str> = round.split(' ').collect();
    return round[1].trim().parse().unwrap();
}