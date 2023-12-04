use crate::input;
use regex;

const NON_SYMBOLS: [&str; 12] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "."];

pub fn solve() {
    let lines = input::read_input();

    let character_regex = regex::Regex::new("[0-9]+").unwrap();
    let mut values: Vec<i64> = Vec::new();

    for i in 0..lines.len() {
        let matches: Vec<regex::Captures> =
            character_regex.captures_iter(lines[i].as_str()).collect();

        for capture in matches {
            let capture_match = capture.get(0).unwrap();
            let mut start = capture_match.start();
            let mut end = capture_match.end();
            let value: i64 = capture_match.as_str().parse().unwrap();

            if end < lines[i].len() {
                let next_char = lines[i].chars().nth(end).unwrap();
                if !NON_SYMBOLS.contains(&&next_char.to_string().as_str()) {
                    values.push(value);
                    continue;
                }
            }

            if start > 0 {
                let previous_char = lines[i].chars().nth(start - 1).unwrap();
                if !NON_SYMBOLS.contains(&&previous_char.to_string().as_str()) {
                    values.push(value);
                    continue;
                }
            }

            if start > 0 {
                start = start - 1;
            }

            if end < lines[i].len() {
                end = end + 1;
            }

            if i == 0 {
                let next_line_group = &lines[i + 1][start..end];
                if allowed_value(next_line_group) {
                    values.push(value);
                }
            } else if i == lines.len() - 1{
                let previous_line_group = &lines[i - 1][start..end];
                if allowed_value(previous_line_group) {
                    values.push(value);
                }
            } else {
                let next_line_group = &lines[i + 1][start..end];
                let previous_line_group = &lines[i - 1][start..end];

                if allowed_value(next_line_group) || allowed_value(previous_line_group) {
                    values.push(value);
                }
            }
        }
    }

    let sum: i64 = values.iter().sum();
    println!("{}", sum);
}

fn allowed_value(line_group: &str) -> bool {
    for character in line_group.chars() {
        if !NON_SYMBOLS.contains(&character.to_string().as_str()) {
            return true;
        }
    }

    return false;
}
