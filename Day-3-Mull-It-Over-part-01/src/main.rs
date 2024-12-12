use regex::Regex;
use std::io::{self, BufRead};

fn gather_input() -> Option<String> {
    Some(io::stdin().lock().lines().filter_map(Result::ok).collect())
}

fn main() {
    let corrupted = gather_input().unwrap();
    let get_num = Regex::new(r"\d{1,3}").unwrap();

    let sum: i32 = Regex::new(r"mul\(\d{1,3},\d{1,3}\)")
        .unwrap()
        .find_iter(&corrupted)
        .map(|m| -> i32 {
            let nums = get_num
                .find_iter(m.as_str())
                .map(|n| n.as_str().parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            nums[0] * nums[1]
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum::<i32>();

    println!("{}", sum);
}
