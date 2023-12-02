use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day1.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let result: i64 = lines.flatten().map(|l| sum_line(l.as_str())).sum();

    println!("{}", result);
}

fn sum_line(line: &str) -> i64 {
    let first = line.chars().find(|c| c.is_ascii_digit());
    let last = line.chars().rev().find(|c| c.is_ascii_digit());

    if let (Some(first), Some(last)) = (first, last) {
        return format!("{}{}", first, last).parse::<i64>().unwrap();
    }

    return 0;
}
