use crate::input_utils;

fn replace_words(line: &String) -> String {
    let new_string = line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    
    return new_string;
}

fn get_number(line: &String) -> i32 {
    let digits: Vec<char> = replace_words(&line).chars().filter(|c| c.is_digit(10)).collect();
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
    let lines = input_utils::read_input();
    let mut sum = 0;
    for line in lines {
        let number = get_number(&line);
        sum += number;
    }

    println!("{}", sum);
}
