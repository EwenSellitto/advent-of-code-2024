use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn gather_input() -> String {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap() + "\n")
        .collect()
}

fn extract_values(s: &str, sep: char) -> Vec<i32> {
    s.split(sep).map(|n| n.parse().unwrap()).collect()
}

fn main() {
    let input = gather_input();
    let parts: Vec<&str> = input.split("\n\n").collect();

    // First time seeing this datatype i'm ******* in love with it
    let ordering_rules: HashSet<(i32, i32)> = parts[0]
        .lines()
        .map(|line| (line[0..2].parse().unwrap(), line[3..].parse().unwrap()))
        .collect();
    let updates: Vec<Vec<i32>> = parts[1]
        .lines()
        .map(|line| extract_values(line, ','))
        .collect();

    let compare = |a: &i32, b: &i32| {
        if ordering_rules.contains(&(*a, *b)) {
            std::cmp::Ordering::Less
        } else if ordering_rules.contains(&(*b, *a)) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    };

    let mut sum = 0;
    for mut update in updates.clone() {
        sum += if update.is_sorted_by(|a, b| compare(a, b) != Ordering::Greater) {
            0
        } else {
            update.sort_by(compare);
            update[update.len() / 2]
        };
    }
    println!("{}", sum);
}
