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

fn find_word_diagonal(index_x: u32, index_y: u32, matrix: &Vec<&str>) -> u32 {
    let get_chars = |x, y| matrix[y as usize].chars().nth(x as usize).unwrap();

    if ((get_chars(index_x as i32 + 1, index_y as i32 + 1) == 'M')
        && (get_chars(index_x as i32 - 1, index_y as i32 - 1) == 'S')
        || (get_chars(index_x as i32 + 1, index_y as i32 + 1) == 'S')
            && (get_chars(index_x as i32 - 1, index_y as i32 - 1) == 'M'))
        && ((get_chars(index_x as i32 + 1, index_y as i32 - 1) == 'M')
            && (get_chars(index_x as i32 - 1, index_y as i32 + 1) == 'S')
            || (get_chars(index_x as i32 + 1, index_y as i32 - 1) == 'S')
                && (get_chars(index_x as i32 - 1, index_y as i32 + 1) == 'M'))
    {
        return 1;
    }
    0
}

fn main() {
    let input = gather_input().unwrap();
    let matrix: Vec<&str> = input.split("\n").collect();
    let mut sum: u32 = 0;

    let size_y = matrix.len() - 1;
    let size_x = matrix[0].len();

    if size_x < 3 || size_y < 3 {
        println!("{}", sum);
        return;
    }

    for index_y in 1..(size_y - 1) {
        for index_x in 1..(size_x - 1) {
            if matrix[index_y].chars().nth(index_x).unwrap() == 'A' {
                sum += find_word_diagonal(index_x as u32, index_y as u32, &matrix);
            }
        }
    }

    println!("{}", sum);
}
