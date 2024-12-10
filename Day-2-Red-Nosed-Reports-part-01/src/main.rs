use std::io;

fn gather_input() -> Option<Vec<Vec<i32>>> {
    let stdin = io::stdin();
    let mut input: Vec<Vec<i32>> = Vec::new();

    for line in stdin.lines() {
        let line = line.ok()?;
        input.push(Vec::new());
        let split: Vec<_> = line.split_whitespace().collect();
        for s in split {
            input.last_mut()?.push(s.parse().unwrap());
        }
    }

    Some(input)
}

fn is_safe(vect: &Vec<i32>) -> bool {
    let min: i32 = *vect.iter().min().unwrap();
    let max: i32 = *vect.iter().max().unwrap();
    let symbol = vect[1] - vect[0] > 0;

    if max - min > vect.len() as i32 * 3 {
        return false;
    }
    for index in 0..(vect.len() - 1) {
        let diff = vect[index + 1] - vect[index];
        if (diff > 0) != symbol || diff.abs() > 3 || diff == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let reports = gather_input().unwrap();
    let mut safe = 0;

    for report in reports.iter() {
        if is_safe(&report) {
            println!("{:?}", report);
            safe += 1;
        }
    }
    println!("{}", safe);
}
