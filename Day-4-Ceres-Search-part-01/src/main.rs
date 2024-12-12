use regex::Regex;
use std::io::{self, BufRead};

fn gather_input() -> Option<String> {
    Some(
        io::stdin()
            .lock()
            .lines()
            .map(|m| {
                let mut line = m.unwrap();
                line.push('\n');
                line
            })
            .collect(),
    )
}

fn find_word_diagonal(
    index_x: u32,
    index_y: u32,
    matrix: &Vec<&str>,
    size_x: usize,
    size_y: usize,
) -> u32 {
    let mut count: u32 = 0;
    const DIRECTIONS: [(i32, i32); 6] = [(1, 1), (-1, 1), (-1, -1), (1, -1), (0, 1), (0, -1)];
    const WORD: &str = "XMAS";

    for direction in DIRECTIONS.iter() {
        let mut x = index_x as i32;
        let mut y = index_y as i32;
        let mut word_index = 0;
        let mut found = true;
        while word_index < WORD.len() {
            if x < 0 || y < 0 || x >= size_x as i32 || y >= size_y as i32 {
                found = false;
                break;
            }
            if matrix[y as usize].chars().nth(x as usize).unwrap()
                != WORD.chars().nth(word_index).unwrap()
            {
                found = false;
                break;
            }
            x += direction.0;
            y += direction.1;
            word_index += 1;
        }
        if found {
            count += 1;
        }
    }
    count
}

fn main() {
    let input = gather_input().unwrap();
    let matrix: Vec<&str> = input.split("\n").collect();
    let re = Regex::new(r"XMAS").unwrap();
    let er = Regex::new(r"SAMX").unwrap();
    let mut sum = re.find_iter(&input).count() as u32 + er.find_iter(&input).count() as u32;

    let mut index_x;
    let mut index_y = 0;
    let size_y = matrix.len() - 1;
    let size_x = matrix[0].len();

    for row in &matrix {
        index_x = 0;
        for col in row.chars() {
            if col == 'X' {
                sum += find_word_diagonal(index_x, index_y, &matrix, size_x, size_y);
            }
            index_x += 1;
        }
        index_y += 1;
    }
    println!("{}", sum);
}
