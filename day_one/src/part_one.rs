use std::fs;

fn read_input() -> Vec<String> {
    return fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}

fn get_number(line: &String) -> i32 {
    let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
    let first_digit = digits[0];
    let second_digit;

    if digits.len() == 1 {
        second_digit = first_digit;
    } else {
        second_digit = digits[digits.len() - 1];
    }

    let digit_string = format!("{}{}", first_digit, second_digit);
    return digit_string.parse().unwrap();
}

pub fn solve() {
    let lines = read_input();
    let mut sum = 0;
    for line in lines {
        let number = get_number(&line);
        sum += number;
    }

    println!("{}", sum);
}