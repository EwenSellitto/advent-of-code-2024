use itertools::Itertools;
use std::collections::HashMap;
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

    let ordering_rules: Vec<(i32, i32)> = parts[0]
        .lines()
        .map(|line| {
            let values = extract_values(line, '|');
            (values[0], values[1])
        })
        .collect();

    let updates: Vec<Vec<i32>> = parts[1]
        .lines()
        .map(|line| extract_values(line, ','))
        .collect();

    let mut order_map: HashMap<i32, Vec<i32>> = ordering_rules
        .iter()
        .flat_map(|&(a, b)| vec![a, b])
        .unique()
        .map(|v| (v, Vec::new()))
        .collect();

    for (a, b) in ordering_rules {
        order_map.entry(b).or_default().push(a);
    }

    let valid_sum: i32 = updates
        .into_iter()
        .filter_map(|update| {
            if update.iter().enumerate().all(|(i, &num)| {
                update[i..]
                    .iter()
                    .all(|&other| !order_map.get(&num).unwrap().contains(&other))
            }) {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum();

    println!("{}", valid_sum);
}
