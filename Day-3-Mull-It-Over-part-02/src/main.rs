use regex::Regex;
use std::io::{self, BufRead};

fn gather_input() -> Option<String> {
    Some(io::stdin().lock().lines().filter_map(Result::ok).collect())
}

fn main() {
    let corrupted = gather_input().unwrap();
    let valid_items = Regex::new(r"don't\(\)|do\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();
    let extract_numbers = Regex::new(r"\d{1,3}").unwrap();
    let mut sum = 0;

    let cleaned = valid_items
        .find_iter(&corrupted)
        .map(|m| m.as_str())
        .collect::<Vec<&str>>()
        .join("");
    let tokens = cleaned.split_inclusive(")").collect::<Vec<&str>>();
    let mut enabled = true;

    for token in &tokens {
        if token.contains("mul") && enabled {
            let numbers = extract_numbers
                .find_iter(token)
                .map(|m| m.as_str())
                .collect::<Vec<&str>>();
            sum += numbers[0].parse::<i32>().unwrap() * numbers[1].parse::<i32>().unwrap();
        } else if token.contains("do(") {
            enabled = true;
        } else if token.contains("don't") {
            enabled = false;
        }
    }

    println!("{}", sum);
}
