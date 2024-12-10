use std::io;

fn gather_input() -> Option<(Vec<i32>, Vec<i32>)> {
    let stdin = io::stdin();
    let mut input_r: Vec<i32> = Vec::new();
    let mut input_l: Vec<i32> = Vec::new();

    for line in stdin.lines() {
        let line = line.ok()?;
        let split: Vec<_> = line.split("   ").collect();

        input_r.push(split[0].parse().unwrap());
        input_l.push(split[1].parse().unwrap());
    }

    Some((input_r, input_l))
}

fn main() -> io::Result<()> {
    let (mut input_r, mut input_l) = gather_input().unwrap();
    let mut sum = 0;

    if input_r.is_empty() || input_l.is_empty() || input_r.len() != input_l.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Input lists are empty or not the same length.",
        ));
    }

    input_r.sort();
    input_l.sort();

    for i in 0..input_r.len() {
        sum += (input_r[i] - input_l[i]).abs();
    }
    println!("{}", sum);
    Ok(())
}
