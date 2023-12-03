use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day3.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();

    let mut parts: Vec<u64> = Vec::new();
    let mut number = "".to_string();
    let mut is_part = false;
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                number.push(c);
                if !is_part {
                    is_part = symbol_adjacent(i as i64, j as i64, &lines);
                }
            } else {
                if is_part {
                    parts.push(number.parse().unwrap());
                }
                number = "".to_string();
                is_part = false;
            }
        }
        if is_part {
            parts.push(number.parse().unwrap());
        }
        number = "".to_string();
        is_part = false;
    }

    let result: u64 = parts.iter().sum();

    println!("{}", result);
}

fn symbol_adjacent(i: i64, j: i64, lines: &Vec<String>) -> bool {
    let i_start = max(0, i - 1) as usize;
    let i_end = min((lines.len() - 1) as i64, i + 1) as usize;

    let j_start = max(0, j - 1) as usize;
    let j_end = min((lines.len() - 1) as i64, j + 1) as usize;

    for i in i_start..=i_end {
        let chars: Vec<char> = lines[i].chars().collect();
        for j in j_start..=j_end {
            if chars[j] != '.' && !chars[j].is_ascii_digit() {
                return true;
            }
        }
    }

    return false;
}
